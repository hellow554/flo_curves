use super::curve_line::curve_intersects_ray;
use super::fat_line::FatLine;
use crate::bezier::solve::{solve_curve_for_t, CLOSE_ENOUGH};
use crate::bezier::{overlapping_region, BezierCurve, CurveSection};
use crate::geo::{BoundingBox, Bounds, Coordinate, Coordinate2D};

use smallvec::{smallvec, SmallVec};

///
/// Determines the length of a curve's hull as a sum of squares
///
fn curve_hull_length_sq<C: BezierCurve>(curve: &CurveSection<C>) -> f64 {
    if curve.is_tiny() {
        0.0
    } else {
        let start = curve.start_point();
        let end = curve.end_point();
        let (cp1, cp2) = curve.control_points();

        let offset1 = cp1 - start;
        let offset2 = cp2 - cp1;
        let offset3 = cp2 - end;

        offset1.dot(&offset1) + offset2.dot(&offset2) + offset3.dot(&offset3)
    }
}

///
/// Given a line representing a linear section of a curve, finds the intersection with a curved section and returns the t values
///
fn intersections_with_linear_section<'a, C: BezierCurve>(
    linear_section: &CurveSection<'a, C>,
    curved_section: &CurveSection<'a, C>,
) -> SmallVec<[(f64, f64); 4]>
where
    C::Point: 'a + Coordinate2D,
{
    // Treat the linear section as a ray based on the start and the end point and find where on the curved section the ray intersects the linear section
    let ray = (linear_section.start_point(), linear_section.end_point());
    let ray_intersections = curve_intersects_ray(curved_section, &ray);

    // Attempt to find where the 't' value is for each ray intersection against the linear section
    let curve_intersections = ray_intersections
        .iter()
        .filter_map(|(curved_t, _ray_t, pos)| {
            let linear_t = solve_curve_for_t(linear_section, pos);

            linear_t.map(|linear_t| (linear_t, *curved_t))
        })
        .collect::<SmallVec<_>>();

    // Rarely: the linear section might be very short and the solver might miss that it's essentially a point
    if curve_intersections.is_empty() && !ray_intersections.is_empty() {
        // If the linear section seems short
        if linear_section
            .point_at_pos(0.0)
            .is_near_to(&linear_section.point_at_pos(1.0), 0.1)
        {
            let midpoint = linear_section.point_at_pos(0.5);
            let curve_intersections = ray_intersections
                .iter()
                .filter_map(|(curved_t, _ray_t, pos)| {
                    if pos.is_near_to(&midpoint, CLOSE_ENOUGH) {
                        Some((0.5, *curved_t))
                    } else {
                        None
                    }
                })
                .collect::<SmallVec<_>>();

            return curve_intersections;
        }
    }

    curve_intersections
}

///
/// The result of the clip operation
///
#[derive(Debug)]
enum ClipResult {
    None,
    Some((f64, f64)),
    SecondCurveIsLinear,
}

///
/// Performs the fat-line clipping algorithm on two curves, returning the t values if they overlap
///
#[inline]
fn clip<'a, C: BezierCurve>(
    curve_to_clip: &CurveSection<'a, C>,
    curve_to_clip_against: &CurveSection<'a, C>,
) -> ClipResult
where
    C::Point: 'a + Coordinate2D,
{
    // Clip against the fat line
    let fat_line = FatLine::from_curve(curve_to_clip_against);
    let clip_t = fat_line.clip_t(curve_to_clip);

    if fat_line.is_flat() {
        return ClipResult::SecondCurveIsLinear;
    }

    let clip_t = if let Some(clip_t) = clip_t {
        // Also try clipping against the perpendicular line
        let perpendicular_line = FatLine::from_curve_perpendicular(curve_to_clip_against);
        let clip_t_perpendicular = perpendicular_line.clip_t(curve_to_clip);

        // Use the perpendicular version if better
        if let Some(clip_t_perpendicular) = clip_t_perpendicular {
            // The clip that produces a shorter range is better
            let len1 = clip_t.1 - clip_t.0;
            let len2 = clip_t_perpendicular.1 - clip_t_perpendicular.0;

            if len1 < len2 {
                ClipResult::Some(clip_t)
            } else {
                ClipResult::Some(clip_t_perpendicular)
            }
        } else {
            // If the perpendicular line excludes this point then there's no overlap
            ClipResult::None
        }
    } else {
        // Failed to clip
        ClipResult::None
    };

    // t1 and t2 must not match (exact matches produce an invalid curve)
    match clip_t {
        ClipResult::Some((t1, t2)) => {
            if t1 == t2 {
                ClipResult::Some(((t1 - 0.005).max(0.0), (t2 + 0.005).min(1.0)))
            } else {
                ClipResult::Some((t1, t2))
            }
        }
        other => other,
    }
}

///
/// Given a set of intersections found on a left and right curve, joins them in a way that eliminates duplicates
///
fn join_subsections<C: BezierCurve>(
    curve1: &CurveSection<C>,
    left: SmallVec<[(f64, f64); 8]>,
    right: SmallVec<[(f64, f64); 8]>,
    accuracy_squared: f64,
) -> SmallVec<[(f64, f64); 8]>
where
    C::Point: Coordinate2D,
{
    if left.is_empty() {
        // No further work to do
        right
    } else if right.is_empty() {
        // No further work to do
        left
    } else {
        // The last intersection in left might be the same as the first in right
        let (left_t1, _left_t2) = left[left.len() - 1];
        let (right_t1, _right_t2) = right[0];

        // We use t1 and curve1 to determine this
        let left_t1 = curve1.section_t_for_original_t(left_t1);
        let right_t1 = curve1.section_t_for_original_t(right_t1);

        if (right_t1 - left_t1).abs() < 0.1 {
            // Could be the same point
            let p1 = curve1.point_at_pos(left_t1);
            let p2 = curve1.point_at_pos(right_t1);

            let offset = p2 - p1;
            let distance_squared = offset.dot(&offset);

            if distance_squared <= (accuracy_squared * 2.0) {
                // First and last points are the same: only use the version of the LHS
                let mut combined = left;
                combined.extend(right.into_iter().skip(1));
                combined
            } else {
                // Not the same points: just combine the two curves
                let mut combined = left;
                combined.extend(right);
                combined
            }
        } else {
            // Not the same points: just combine the two curves
            let mut combined = left;
            combined.extend(right);
            combined
        }
    }
}

///
/// Determines the points at which two curves intersect using the Bezier clipping algorithm
///
fn curve_intersects_curve_clip_inner<'a, C: BezierCurve>(
    curve1: CurveSection<'a, C>,
    curve2: CurveSection<'a, C>,
    accuracy_squared: f64,
) -> SmallVec<[(f64, f64); 8]>
where
    C::Point: 'a + Coordinate2D,
{
    // Overlapping curves should be treated separately (the clipping algorithm will just match all of the points)
    let overlaps = overlapping_region(&curve1, &curve2);
    if let Some(((c1_t1, c1_t2), (c2_t1, c2_t2))) = overlaps {
        // Convert the overlapping region back to t values for the original curve
        let c1_t1 = curve1.t_for_t(c1_t1);
        let c1_t2 = curve1.t_for_t(c1_t2);
        let c2_t1 = curve2.t_for_t(c2_t1);
        let c2_t2 = curve2.t_for_t(c2_t2);

        if c1_t1 == c1_t2 || c2_t1 == c2_t2 {
            // Overlapped at a single point, so only one intersection
            return smallvec![(c1_t1, c2_t1)];
        } else {
            // Overlapping curves cross at both points
            return smallvec![(c1_t1, c2_t1), (c1_t2, c2_t2)];
        }
    }

    // We'll iterate on the two curves
    let mut curve1 = curve1;
    let mut curve2 = curve2;

    // If a curve stops shrinking, we need to subdivide it to continue the match
    let mut curve1_last_len = curve_hull_length_sq(&curve1);
    let mut curve2_last_len = curve_hull_length_sq(&curve2);

    // Edge case: 0-length curves have no match
    if curve1_last_len == 0.0 {
        return smallvec![];
    }
    if curve2_last_len == 0.0 {
        return smallvec![];
    }

    // Iterate to refine the match
    loop {
        let curve2_len = if curve2_last_len > accuracy_squared {
            // Clip curve2 against curve1
            let clip_t = clip(&curve2, &curve1);
            let clip_t = match clip_t {
                ClipResult::None => {
                    return smallvec![];
                }
                ClipResult::Some(clip_t) => clip_t,
                ClipResult::SecondCurveIsLinear => {
                    return intersections_with_linear_section(&curve1, &curve2)
                        .into_iter()
                        .map(|(t1, t2)| (curve1.t_for_t(t1), curve2.t_for_t(t2)))
                        .collect();
                }
            };

            curve2 = curve2.subsection(clip_t.0, clip_t.1);

            // Work out the length of the new curve
            curve_hull_length_sq(&curve2)
        } else {
            curve2_last_len
        };

        let curve1_len = if curve1_last_len > accuracy_squared {
            // Clip curve1 against curve2
            let clip_t = clip(&curve1, &curve2);
            let clip_t = match clip_t {
                ClipResult::None => {
                    return smallvec![];
                }
                ClipResult::Some(clip_t) => clip_t,
                ClipResult::SecondCurveIsLinear => {
                    return intersections_with_linear_section(&curve2, &curve1)
                        .into_iter()
                        .map(|(t2, t1)| (curve1.t_for_t(t1), curve2.t_for_t(t2)))
                        .collect();
                }
            };

            curve1 = curve1.subsection(clip_t.0, clip_t.1);

            // Work out the length of the new curve
            curve_hull_length_sq(&curve1)
        } else {
            curve1_last_len
        };

        if curve1_len <= accuracy_squared && curve2_len <= accuracy_squared {
            // Found a point to the required accuracy: return it, in coordinates relative to the original curve
            if curve1
                .fast_bounding_box::<Bounds<_>>()
                .overlaps(&curve2.fast_bounding_box::<Bounds<_>>())
            {
                let (t_min1, t_max1) = curve1.original_curve_t_values();
                let (t_min2, t_max2) = curve2.original_curve_t_values();

                return smallvec![((t_min1 + t_max1) * 0.5, (t_min2 + t_max2) * 0.5)];
            } else {
                // Clipping algorithm found a point, but the two curves do not actually overlap, so reject them
                return smallvec![];
            }
        }

        if (curve1_last_len * 0.8) <= curve1_len && (curve2_last_len * 0.8) <= curve2_len {
            // If neither curve shrunk by 20%, then subdivide the one that shrunk the least
            if curve1_len / curve1_last_len > curve2_len / curve2_last_len {
                // Curve1 shrunk less than curve2
                let (left, right) = (curve1.subsection(0.0, 0.5), curve1.subsection(0.5, 1.0));
                let left =
                    curve_intersects_curve_clip_inner(left, curve2.clone(), accuracy_squared);
                let right = curve_intersects_curve_clip_inner(right, curve2, accuracy_squared);

                return join_subsections(&curve1, left, right, accuracy_squared);
            } else {
                // Curve2 shrunk less than curve1
                let (left, right) = (curve2.subsection(0.0, 0.5), curve2.subsection(0.5, 1.0));
                let left =
                    curve_intersects_curve_clip_inner(curve1.clone(), left, accuracy_squared);
                let right =
                    curve_intersects_curve_clip_inner(curve1.clone(), right, accuracy_squared);

                return join_subsections(&curve1, left, right, accuracy_squared);
            }
        }

        // Update the last lengths
        curve1_last_len = curve1_len;
        curve2_last_len = curve2_len;
    }
}

///
/// Determines the points at which two curves intersect using the Bezier clipping
/// algorihtm
///
pub fn curve_intersects_curve_clip<'a, C: BezierCurve>(
    curve1: &'a C,
    curve2: &'a C,
    accuracy: f64,
) -> SmallVec<[(f64, f64); 8]>
where
    C::Point: 'a + Coordinate2D,
{
    // Start with the entire span of both curves
    let curve1 = curve1.section(0.0, 1.0);
    let curve2 = curve2.section(0.0, 1.0);

    // Perform the clipping algorithm on these curves
    curve_intersects_curve_clip_inner(curve1, curve2, accuracy * accuracy)
}
