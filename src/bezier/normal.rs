use super::super::geo::{Coordinate, Coordinate2D};
use super::basis::{de_casteljau3, de_casteljau4};
use super::curve::BezierCurve;
use super::derivative::derivative4;

// TODO: normalize should be a trait associated with coordinate rather than bezier curves (move outwards)

///
/// Changes a point and its tangent into a normal
///
pub trait Normalize {
    /// Computes the normal at a point, given its tangent
    fn to_normal(point: &Self, tangent: &Self) -> Vec<f64>;
}

impl<Point: Coordinate2D> Normalize for Point {
    #[inline]
    fn to_normal(_point: &Self, tangent: &Self) -> Vec<f64> {
        vec![-tangent.y(), tangent.x()]
    }
}

/* -- TODO: we'd like to do the same as above but for 3D coordinates but can't figure out a good way in Rust's type system...
impl Normalize<Coordinate3D> for Coordinate3D {
    #[inline]
    fn to_normal(point: &Coordinate3D, tangent: &Coordinate3D) -> Vec<f64> {
        // Extract the coordinates from the points
        let (px, py, pz) = (point.x(), point.y(), point.z());
        let (tx, ty, tz) = (tangent.x(), tangent.y(), tangent.z());

        // Cross product
        let (rx, ry, rz) = (ty*pz - tz*py, tz*px-tx*pz, tx*py-ty*px);

        // Normalized cross product
        let rmag            = (rx*rx + ry*ry + rz*rz).sqrt();
        let (rx, ry, rz)    = (rx/rmag, ry/rmag, rz/rmag);

        // Rotation matrix
        let ((m11, m21, m31), (m12, m22, m32), (m13, m23, m33)) = (
            (rx*rx,     rx*ry-rz,   rx*rz+ry),
            (rx*ry+rz,  ry*ry,      ry*rz-rx),
            (rx*rz-ry,  ry*rz+rx,   rz*rz)
        );

        // Normal
        let (nx, ny, nz) = (px*m11 + py*m12 + pz*m13, px*m21 + py*m22 + py*m23, px*m31 + px*m32 + px*m33);

        vec![nx, ny, nz]
    }
}
*/

// TODO: maybe this should be a plain fn (or a struct like Tangent) instead of a trait

///
/// Trait implemented by bezier curves where we can compute the normal
///
pub trait NormalCurve: BezierCurve {
    ///
    /// Computes the tangent vector to the curve at the specified t value
    ///
    /// The vector that this returns can have any magnitude: it's only defined to be in the direction of the
    /// tangent at the specified point. You can call `to_unit_vector()` to generate a normal vector of length 1.
    ///
    /// In the event that the curve represents a point, this will return the vector (0,0)
    ///
    fn tangent_at_pos(&self, t: f64) -> Self::Point;

    ///
    /// Computes the normal vector to the curve at the specified t value
    ///
    /// The vector that this returns can have any magnitude: it's only defined to be in the direction of the
    /// normal at the specified point. You can call `to_unit_vector()` to generate a normal vector of length 1.
    ///
    /// In the event that the curve represents a point, this will return the vector (0,0)
    ///
    fn normal_at_pos(&self, t: f64) -> Self::Point;
}

impl<Curve: BezierCurve> NormalCurve for Curve
where
    Curve::Point: Normalize,
{
    fn tangent_at_pos(&self, t: f64) -> Curve::Point {
        // Extract the points that make up this curve
        let w1 = self.start_point();
        let (w2, w3) = self.control_points();
        let w4 = self.end_point();

        // If w1 == w2 or w3 == w4 there will be an anomaly at t=0.0 and t=1.0
        // (it's probably mathematically correct to say there's no tangent at these points but the result is surprising and probably useless in a practical sense)
        let t = if t == 0.0 { f64::EPSILON } else { t };
        let t = if t == 1.0 { 1.0 - f64::EPSILON } else { t };

        // Get the deriviative
        let (d1, d2, d3) = derivative4(w1, w2, w3, w4);

        // Get the tangent and the point at the specified t value
        de_casteljau3(t, d1, d2, d3)
    }

    fn normal_at_pos(&self, t: f64) -> Curve::Point {
        // Extract the points that make up this curve
        let w1 = self.start_point();
        let (w2, w3) = self.control_points();
        let w4 = self.end_point();

        // If w1 == w2 or w3 == w4 there will be an anomaly at t=0.0 and t=1.0
        // (it's probably mathematically correct to say there's no normal at these points but the result is surprising and probably useless in a practical sense)
        let t = if t == 0.0 { f64::EPSILON } else { t };
        let t = if t == 1.0 { 1.0 - f64::EPSILON } else { t };

        // Get the deriviative
        let (d1, d2, d3) = derivative4(w1, w2, w3, w4);

        // Get the tangent and the point at the specified t value
        let point = de_casteljau4(t, w1, w2, w3, w4);
        let tangent = de_casteljau3(t, d1, d2, d3);

        // Compute the normal
        let normal = Curve::Point::to_normal(&point, &tangent);

        Curve::Point::from_components(&normal)
    }
}
