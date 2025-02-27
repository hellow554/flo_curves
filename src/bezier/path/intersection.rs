use super::super::super::geo::{BoundingBox, Bounds, Coordinate2D};
use super::super::super::line::Line;
use super::super::curve::{BezierCurve, Curve};
use super::super::intersection::{curve_intersects_curve_clip, curve_intersects_line};
use super::path::BezierPath;
use super::to_curves::path_to_curves;

///
/// Determines the intersections of a path and a line
///
/// Intersections are returned as the path section index, the 't' parameter along that curve and the 't' value along the line:
/// ie: `(path_point_idx, curve_t, line_t)`.
///
pub fn path_intersects_line<'a, Path: BezierPath, L: Line<Point = Path::Point>>(
    path: &'a Path,
    line: &'a L,
) -> impl 'a + Iterator<Item = (usize, f64, f64)>
where
    Path::Point: 'a + Coordinate2D,
{
    path_to_curves::<_, Curve<_>>(path)
        .enumerate()
        .flat_map(move |(section_id, curve)| {
            curve_intersects_line(&curve, line)
                .into_iter()
                .map(move |(t, s, _pos)| (section_id, t, s))
        })
}

///
/// Determines the intersections of a path and a ray.
///
///  Return value is `(path_point_idx, curve_t, line_t)`. Ray intersections differ from line intersections
/// in that there's no requirement for the result to be within the bounds of the supplied line (so any match in the direction of the line is
/// returned).
///
/// It's possible to filter for matches that occur after the start of the line by looking for results with an `s` value >= 0
///
pub fn path_intersects_ray<'a, Path: BezierPath, L: Line<Point = Path::Point>>(
    path: &'a Path,
    line: &'a L,
) -> impl 'a + Iterator<Item = (usize, f64, f64)>
where
    Path::Point: 'a + Coordinate2D,
{
    path_to_curves::<_, Curve<_>>(path)
        .enumerate()
        .flat_map(move |(section_id, curve)| {
            curve_intersects_line(&curve, line)
                .into_iter()
                .map(move |(t, s, _pos)| (section_id, t, s))
        })
}

///
/// Finds the points where a path intersects another path
///
/// Intersections are returned as (segment index, t-value), in pairs indicating the position on the first path
/// and the position on the second path. Intersections are unordered by default.
///
/// The accuracy value indicates the maximum errors that's permitted for an intersection: the bezier curve
/// intersection algorithm is approximate.
///
pub fn path_intersects_path<'a, Path: BezierPath>(
    path1: &'a Path,
    path2: &'a Path,
    accuracy: f64,
) -> Vec<((usize, f64), (usize, f64))>
where
    Path::Point: 'a + Coordinate2D,
{
    // Convert both paths to sections: also compute the bounding boxes for quick rejection of sections with no intersections
    let path1_sections = path_to_curves::<_, Curve<_>>(path1)
        .enumerate()
        .map(|(section_id, curve)| (section_id, curve, curve.bounding_box::<Bounds<_>>()));

    let path2_sections = path_to_curves::<_, Curve<_>>(path2)
        .enumerate()
        .map(|(section_id, curve)| (section_id, curve, curve.bounding_box::<Bounds<_>>()))
        .collect::<Vec<_>>();

    // Start generating the result
    let mut result = vec![];

    // Compare the sections in path1 to the sections in path2
    // We iterate over path1 once...
    for (p1_section_id, p1_curve, p1_curve_bounds) in path1_sections {
        // But repeatedly interate over path2
        for (p2_section_id, p2_curve, p2_curve_bounds) in path2_sections.iter() {
            // Only search for intersections if these two sections have overlapping bounding boxes
            if p1_curve_bounds.overlaps(p2_curve_bounds) {
                // Determine the intersections (if any) between these two curves
                let intersections = curve_intersects_curve_clip(&p1_curve, p2_curve, accuracy);

                // Combine with the section IDs to generate the results
                result.extend(
                    intersections
                        .into_iter()
                        .map(|(t1, t2)| ((p1_section_id, t1), (*p2_section_id, t2))),
                );
            }
        }
    }

    result
}
