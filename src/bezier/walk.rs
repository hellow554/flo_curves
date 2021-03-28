use super::curve::*;
use super::length::*;
use super::section::*;

use crate::geo::*;

///
/// Walks a bezier curve by dividing it into a number of sections
///
/// These sections are uneven in length: they all advance equally by 't' value but the points will
/// be spaced according to the shape of the curve (will have an uneven distance between them) 
///
#[inline]
pub fn walk_curve_unevenly<'a, Curve: BezierCurve>(curve: &'a Curve, num_subdivisions: usize) -> impl 'a+Iterator<Item=CurveSection<'a, Curve>> {
    if num_subdivisions > 0 {
        UnevenWalkIterator {
            curve:              curve,
            step:               (1.0)/(num_subdivisions as f64),
            num_subdivisions:   num_subdivisions,
            last_subdivision:   0
        }
    } else {
        UnevenWalkIterator {
            curve:              curve,
            step:               0.0,
            num_subdivisions:   0,
            last_subdivision:   0
        }
    }
}

///
/// Walks a bezier curve by moving forward a set amount at each point. Each point may be up to `max_error` away from `distance.
///
#[inline]
pub fn walk_curve_evenly<'a, Curve: BezierCurve>(curve: &'a Curve, distance: f64, max_error: f64) -> impl 'a+Iterator<Item=CurveSection<'a, Curve>> {
    const INITIAL_INCREMENT: f64 = 0.1;

    // Too small or negative values might produce bad effects due to floating point inprecision
    let max_error   = if max_error < 1e-10  { 1e-10 } else { max_error };
    let distance    = if distance < 1e-10   { 1e-10 } else { distance };

    EvenWalkIterator {
        curve:          curve,
        curve_length:   curve_length(curve, max_error),
        last_t:         0.0, 
        last_point:     curve.start_point(),
        last_increment: INITIAL_INCREMENT,
        distance:       distance,
        max_error:      max_error
    }
}

///
/// Iterator implemenation that performs an uneven walk along a curve
///
struct UnevenWalkIterator<'a, Curve: BezierCurve> {
    /// The curve that this is iterating over
    curve:              &'a Curve,

    /// The distance between t-values
    step:               f64,

    /// The total number of subdivisions to return
    num_subdivisions:   usize,

    /// The number of the most recently returned subdivision
    last_subdivision:   usize
}

impl<'a, Curve: BezierCurve> Iterator for UnevenWalkIterator<'a, Curve> {
    type Item = CurveSection<'a, Curve>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.last_subdivision >= self.num_subdivisions {
            // No more sections
            None
        } else {
            // Update the position and work out the range of t-values to return
            let t_min = self.step * (self.last_subdivision as f64);
            self.last_subdivision += 1;
            let t_max = self.step * (self.last_subdivision as f64);

            // Generate a section for this range of values
            Some(self.curve.section(t_min, t_max))
        }
    }
}

///
/// Iterator implementation that performs an even walk along a curve
///
struct EvenWalkIterator<'a, Curve: BezierCurve> {
    /// The curve that is being walked
    curve:          &'a Curve,

    /// The total length of the curve
    curve_length:   f64,

    /// The last 't' value where a coordinate was generated
    last_t:         f64,

    /// The point generated at the last 't' value
    last_point:     Curve::Point,

    /// The last increment
    last_increment: f64,

    /// The target distance between points (as the chord length)
    distance:       f64,

    /// The maximum error in distance for the points that are generated by this iterator
    max_error:      f64
}


impl<'a, Curve: BezierCurve> Iterator for EvenWalkIterator<'a, Curve> {
    type Item = CurveSection<'a, Curve>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        // Gather values
        let curve           = self.curve;
        let curve_length    = self.curve_length;
        let distance        = self.distance;
        let max_error       = self.max_error;
        let mut t_increment = self.last_increment;
        let last_t          = self.last_t;
        let mut next_t      = last_t + t_increment;
        let last_point      = self.last_point.clone();
        let mut next_point;

        // If the curve far too short, then indicate that there are no points
        if curve_length < 1e-10 {
            return None;
        }

        // If the next point appears to be after the end of the curve, and the end of the curve i
        if next_t >= 1.0 {
            if last_point.distance_to(&curve.point_at_pos(1.0)) < distance {
                // End point is closer than the target distance
                return None;
            }
        }

        loop {
            debug_assert!(!t_increment.is_nan());

            // next_point contains the initial estimate of the position of the point at distance 't' from the current point
            next_point      = curve.point_at_pos(next_t);

            // Compute the distance to the guess and the error
            let next_distance   = last_point.distance_to(&next_point);
            let error           = distance - next_distance;

            // We've found the next point if the error drops low enough
            if error.abs() < max_error {
                break;
            }

            // Use the error to adjust the t position we're testing if it's larger than max_error
            let error_ratio     = error / curve_length;
            t_increment         = t_increment + error_ratio;
            t_increment         = if t_increment < 1e-10 { (next_t + last_t)/2.0 - last_t } else { t_increment };
            next_t              = last_t + t_increment;
        }

        // next_t -> last_t is the next point
        if next_t > 1.0 {
            return None;
        }

        // Update the coordinates
        self.last_point     = next_point;
        self.last_increment = t_increment;
        self.last_t         = next_t;

        // Return the section that we found
        Some(self.curve.section(last_t, next_t))
    }
}
