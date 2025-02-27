//!
//! # Describing coordinates
//!
//! The `Coordinate` trait provides a way to represent coordinates in arbitary numbers of dimensions. Most of the
//! types in `flo_curves` support arbitrary coordinate types through this trait.
//!
//! `Coordinate2D` coordinates are a special case of coordinates with only two dimensions. Some operations are
//! only defined for two dimensions: for example, taking the normal of a Bezier curve. The `Coord2` type is
//! supplied as a generic implementation of a 2-dimensional coordinate, though these operations will work on
//! any type for which the `Coordinate2D` trait is defined.
//!

use smallvec::{smallvec, SmallVec};

use std::ops::{Add, Mul, Sub};

///
/// Represents a value that can be used as a coordinate in a bezier curve
///
pub trait Coordinate:
    Sized
    + Copy
    + Add<Self, Output = Self>
    + Mul<f64, Output = Self>
    + Sub<Self, Output = Self>
    + PartialEq
{
    ///
    /// Creates a new coordinate from the specified set of components
    ///
    fn from_components(components: &[f64]) -> Self;

    ///
    /// Returns the origin coordinate
    ///
    fn origin() -> Self;

    ///
    /// The number of components in this coordinate
    ///
    fn len() -> usize;

    ///
    /// Retrieves the component at the specified index
    ///
    fn get(&self, index: usize) -> f64;

    ///
    /// Returns a point made up of the biggest components of the two points
    ///
    fn from_biggest_components(p1: Self, p2: Self) -> Self;

    ///
    /// Returns a point made up of the smallest components of the two points
    ///
    fn from_smallest_components(p1: Self, p2: Self) -> Self;

    ///
    /// Computes the distance between this coordinate and another of the same type
    ///
    #[inline]
    fn distance_to(&self, target: &Self) -> f64 {
        let offset = *self - *target;
        let squared_distance = offset.dot(&offset);

        f64::sqrt(squared_distance)
    }

    ///
    /// Computes the dot product for this vector along with another vector
    ///
    #[inline]
    fn dot(&self, target: &Self) -> f64 {
        let mut dot_product = 0.0;

        for component_index in 0..Self::len() {
            dot_product += self.get(component_index) * target.get(component_index);
        }

        dot_product
    }

    ///
    /// Computes the magnitude of this vector
    ///
    #[inline]
    fn magnitude(&self) -> f64 {
        f64::sqrt(self.dot(self))
    }

    ///
    /// Treating this as a vector, returns a unit vector in the same direction
    ///
    #[inline]
    fn to_unit_vector(&self) -> Self {
        let magnitude = self.magnitude();
        if magnitude == 0.0 {
            Self::origin()
        } else {
            *self * (1.0 / magnitude)
        }
    }

    ///
    /// Returns true if this coordinate has a NaN component
    ///
    #[inline]
    fn is_nan(&self) -> bool {
        for component in 0..Self::len() {
            if self.get(component).is_nan() {
                return true;
            }
        }

        false
    }

    ///
    /// Round this coordinate so that it is accurate to a certain precision
    ///
    #[inline]
    fn round(self, accuracy: f64) -> Self {
        let mut new_components: SmallVec<[_; 4]> = smallvec![];

        for component in 0..Self::len() {
            let unrounded_value = self.get(component);
            let rounded_value = (unrounded_value / accuracy).round() * accuracy;

            new_components.push(rounded_value);
        }

        Self::from_components(&new_components)
    }

    ///
    /// True if this point is within max_distance of another point
    ///
    #[inline]
    fn is_near_to(&self, other: &Self, max_distance: f64) -> bool {
        let offset = *self - *other;
        let squared_distance = offset.dot(&offset);

        squared_distance <= (max_distance * max_distance)
    }

    ///
    /// Generates a smoothed version of a set of coordinates, using the specified weights
    /// (weights should add up to 1.0).
    ///
    /// A suggested set of weights might be '[0.25, 0.5, 0.25]', which will slightly
    /// adjust each point according to its neighbours (the central weight is what's
    /// applied to the 'current' point)
    ///
    fn smooth(points: &[Self], weights: &[f64]) -> Vec<Self> {
        let mut smoothed = vec![];
        let points_len = points.len() as i32;
        let weight_len = weights.len() as i32;
        let weight_offset = weight_len / 2;

        for index in 0..points_len {
            let mut res = Self::origin();
            let initial_pos = index - weight_offset;

            for weight_pos in 0..weight_len {
                let weight = weights[weight_pos as usize];
                let source_pos = initial_pos + weight_pos;

                let source_val = if source_pos < 0 {
                    &points[0]
                } else if source_pos >= points_len {
                    &points[(points_len - 1) as usize]
                } else {
                    &points[source_pos as usize]
                };

                res = res + (*source_val * weight);
            }

            smoothed.push(res);
        }

        smoothed
    }
}

///
/// Represents a coordinate with a 2D position
///
pub trait Coordinate2D {
    fn x(&self) -> f64;
    fn y(&self) -> f64;

    #[inline]
    fn coords(&self) -> (f64, f64) {
        (self.x(), self.y())
    }
}

///
/// Represents a coordinate with a 3D position
///
pub trait Coordinate3D {
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> f64;
}

impl Coordinate for f64 {
    fn from_components(components: &[f64]) -> Self {
        components[0]
    }

    #[inline]
    fn origin() -> Self {
        0.0
    }
    #[inline]
    fn len() -> usize {
        1
    }
    #[inline]
    fn get(&self, _index: usize) -> f64 {
        *self
    }

    #[inline]
    fn from_biggest_components(p1: Self, p2: Self) -> Self {
        if p1 > p2 {
            p1
        } else {
            p2
        }
    }

    #[inline]
    fn from_smallest_components(p1: Self, p2: Self) -> Self {
        if p1 < p2 {
            p1
        } else {
            p2
        }
    }

    #[inline]
    fn distance_to(&self, target: &Self) -> f64 {
        Self::abs(self - target)
    }

    fn dot(&self, target: &Self) -> f64 {
        self * target
    }
}

/// Represents a 2D point
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Coord2(pub f64, pub f64);

impl Coordinate2D for Coord2 {
    ///
    /// X component of this coordinate
    ///
    #[inline]
    fn x(&self) -> f64 {
        self.0
    }

    ///
    /// Y component of this coordinate
    ///
    #[inline]
    fn y(&self) -> f64 {
        self.1
    }
}

impl Add<Self> for Coord2 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub<Self> for Coord2 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Mul<f64> for Coord2 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f64) -> Self {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl From<(f64, f64)> for Coord2 {
    fn from((x, y): (f64, f64)) -> Self {
        Self(x, y)
    }
}

impl From<Coord2> for (f64, f64) {
    fn from(c: Coord2) -> (f64, f64) {
        (c.0, c.1)
    }
}

impl From<(f32, f32)> for Coord2 {
    fn from((x, y): (f32, f32)) -> Self {
        Self(x as _, y as _)
    }
}

impl From<Coord2> for (f32, f32) {
    fn from(c: Coord2) -> (f32, f32) {
        (c.0 as _, c.1 as _)
    }
}

impl Coordinate for Coord2 {
    #[inline]
    fn from_components(components: &[f64]) -> Self {
        Self(components[0], components[1])
    }

    #[inline]
    fn origin() -> Self {
        Self(0.0, 0.0)
    }

    #[inline]
    fn len() -> usize {
        2
    }

    #[inline]
    fn get(&self, index: usize) -> f64 {
        match index {
            0 => self.0,
            1 => self.1,
            _ => panic!("Coord2 only has two components"),
        }
    }

    fn from_biggest_components(p1: Self, p2: Self) -> Self {
        Self(
            f64::from_biggest_components(p1.0, p2.0),
            f64::from_biggest_components(p1.1, p2.1),
        )
    }

    fn from_smallest_components(p1: Self, p2: Self) -> Self {
        Self(
            f64::from_smallest_components(p1.0, p2.0),
            f64::from_smallest_components(p1.1, p2.1),
        )
    }

    #[inline]
    fn distance_to(&self, target: &Self) -> f64 {
        let dist_x = target.0 - self.0;
        let dist_y = target.1 - self.1;

        f64::sqrt(dist_x * dist_x + dist_y * dist_y)
    }

    #[inline]
    fn dot(&self, target: &Self) -> f64 {
        self.0 * target.0 + self.1 * target.1
    }
}
