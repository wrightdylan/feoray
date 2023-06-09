#![allow(unused)]
use std::ops::{Add, Div, Mul, Neg, Sub};
use crate::EPSILON;

// A struct that handles both points and vectors.
#[derive(Debug, Clone, Copy)]
pub struct Tuple{
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64
}

impl Tuple {
    /// Constructs a new Tuple container.
    /// Can be either a point or vector depending on value of w.
    /// 
    /// When w = 0.0 it is a vector.
    /// When w = 1.0 it is a point.
    /// 
    /// # Example
    /// 
    /// ```ignore
    /// let tup = Tuple::new(4.3, -4.2, 3.1, 0.0);
    /// 
    /// assert_eq!((tup.x, tup.y, tup.z, tup.w), (4.3, -4.2, 3.1, 0.0));
    /// ```
    pub fn new( x: f64, y: f64, z:f64, w: f64) -> Self {
        Tuple { x, y, z, w }
    }

    /// Tests if a Tuple is a point
    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    /// Tests if a Tuple is a vector
    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    /// Returns the length/magnitude of a vector.
    /// 
    /// # Example
    /// 
    /// ```ignore
    /// let v = vector(1.0, 2.0, 3.0);
    /// 
    /// assert_eq!(v.length(), 14.0_f64.sqrt());
    /// ```
    pub fn length(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }

    /// Normalises a vector.
    /// Normalising a vector scales all components so the length of the vector = 1.0, also called a unit vector.
    /// This is useful for describing a vector's direction without regard to its length.
    /// 
    /// # Example
    /// 
    /// ```ignore
    /// let v = vector(4.0, 0.0, 0.0);
    /// 
    /// assert_eq!(v.norm(), vector(1.0, 0.0, 0.0));
    /// ```
    pub fn norm(&self) -> Tuple {
        let len = self.length();

        Tuple {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
            w: self.w / len
        }
    }

    /// Returns the dot product of two vectors.
    /// 
    /// # Example
    /// 
    /// ```ignore
    /// let v1 = vector(1.0, 2.0, 3.0);
    /// let v2 = vector(2.0, 3.0, 4.0);
    ///
    /// assert_eq!(v1.dprod(v2), 20.0);
    /// ```
    pub fn dprod(&self, rhs: Tuple) -> f64 {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z) + (self.w * rhs.w)
    }

    /// Returns the cross product of two vectors.
    /// 
    /// # Example
    /// 
    /// ```ignore
    /// let v1 = vector(1.0, 2.0, 3.0);
    /// let v2 = vector(2.0, 3.0, 4.0);
    ///
    /// assert_eq!(v1.xprod(v2), vector(-1.0, 2.0, -1.0));
    /// assert_eq!(v2.xprod(v1), vector(1.0, -2.0, 1.0));
    /// ```
    pub fn xprod(&self, rhs: Tuple) -> Tuple {
        Tuple {
            x: (self.y * rhs.z) - (self.z * rhs.y),
            y: (self.z * rhs.x) - (self.x * rhs.z),
            z: (self.x * rhs.y) - (self.y * rhs.x),
            w: 0.0
        }
    }

    /// Converts a Tuple struct to a vector type
    pub fn to_vector(&self) -> Tuple {
        Tuple {
            x: self.x,
            y: self.y,
            z: self.z,
            w: 0.0
        }
    }

    /// Converts a Tuple struct to a point type
    pub fn to_point(&self) -> Tuple {
        Tuple {
            x: self.x,
            y: self.y,
            z: self.z,
            w: 1.0
        }
    }
}

impl Add for Tuple {
    type Output = Tuple;

    fn add(self, rhs: Tuple) -> Self::Output {
        Tuple {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w
        }
    }
}

impl Sub for Tuple {
    type Output = Tuple;

    fn sub(self, rhs: Tuple) -> Self::Output {
        Tuple {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w
        }
    }
}

impl Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Self::Output {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z, 
            w: -self.w
        }
    }
}

impl Mul<Tuple> for Tuple {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        Tuple {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
            w: self.w * rhs.w
        }
    }
}

impl Mul<f64> for Tuple {
    type Output = Tuple;

    fn mul(self, rhs: f64) -> Self::Output {
        Tuple {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs
        }
    }
}

impl Div<Tuple> for Tuple {
    type Output = Tuple;

    fn div(self, rhs: Tuple) -> Self::Output {
        Tuple {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
            w: self.w / rhs.w
        }
    }
}

impl Div<f64> for Tuple {
    type Output = Tuple;

    fn div(self, rhs: f64) -> Self::Output {
        Tuple {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs
        }
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Tuple) -> bool {
        self.x == other.x
            && self.y == other.y
            && self.z == other.z
            && self.w == other.w
    }
}

pub fn point(x: f64, y: f64, z: f64) -> Tuple {
    Tuple::new(x, y, z, 1.0)
}

pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
    Tuple::new(x, y, z, 0.0)
}

pub fn float_equal(a: f64, b: f64) -> bool {
    if (a - b).abs() < EPSILON {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuple_is_point() {
        let tup = Tuple::new(4.3, -4.2, 3.1, 1.0);

        assert_eq!((tup.x, tup.y, tup.z, tup.w), (4.3, -4.2, 3.1, 1.0));
    }

    #[test]
    fn tuple_is_vector() {
        let tup = Tuple::new(4.3, -4.2, 3.1, 0.0);

        assert_eq!((tup.x, tup.y, tup.z, tup.w), (4.3, -4.2, 3.1, 0.0));
    }

    #[test]
    fn coordinate_to_point() {
        assert_eq!(point(4.0, -4.0, 3.0), Tuple::new(4.0, -4.0, 3.0, 1.0));
    }

    #[test]
    fn coordinate_to_vector() {
        assert_eq!(vector(4.0, -4.0, 3.0), Tuple::new(4.0, -4.0, 3.0, 0.0));
    }

    #[test]
    fn add_tuples() {
        let tup1 = point(3.0, -2.0, 5.0);
        let tup2 = vector(-2.0, 3.0, 1.0);

        assert_eq!(tup1 + tup2, Tuple::new(1.0, 1.0, 6.0, 1.0));
    }

    #[test]
    fn sub_two_points() {
        let p1 = point(3.0, 2.0, 1.0);
        let p2 = point(5.0, 6.0, 7.0);

        assert_eq!(p1 - p2, vector(-2.0, -4.0, -6.0))
    }

    #[test]
    fn sub_vec_from_point() {
        let p = point(3.0, 2.0, 1.0);
        let v = vector(5.0, 6.0, 7.0);

        assert_eq!(p - v, point(-2.0, -4.0, -6.0));
    }

    #[test]
    fn sub_vec_from_vec() {
        let v1 = vector(3.0, 2.0, 1.0);
        let v2 = vector(5.0, 6.0, 7.0);

        assert_eq!(v1 - v2, vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn negate_tuple() {
        let tup = Tuple::new(1.0, -2.0, 3.0, -4.0);

        assert_eq!(-tup, Tuple::new(-1.0, 2.0, -3.0, 4.0));
    }

    #[test]
    fn multiply_tuple_by_scalar() {
        let tup = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let s = 3.5;

        assert_eq!(tup * s, Tuple::new(3.5, -7.0, 10.5, -14.0));
    }

    #[test]
    fn multiply_tuple_by_fraction() {
        let tup = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let s = 0.5;

        assert_eq!(tup * s, Tuple::new(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn divide_tuple_by_scalar() {
        let tup = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let s = 2.0;

        assert_eq!(tup / s, Tuple::new(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn magnitude_of_vector1() {
        let v = vector(1.0, 0.0, 0.0);

        assert_eq!(v.length(), 1.0);
    }

    #[test]
    fn magnitude_of_vector2() {
        let v = vector(0.0, 1.0, 0.0);

        assert_eq!(v.length(), 1.0);
    }

    #[test]
    fn magnitude_of_vector3() {
        let v = vector(0.0, 0.0, 1.0);

        assert_eq!(v.length(), 1.0);
    }

    #[test]
    fn magnitude_of_vector4() {
        let v = vector(1.0, 2.0, 3.0);

        assert_eq!(v.length(), 14.0_f64.sqrt());
    }

    #[test]
    fn magnitude_of_vector5() {
        let v = vector(-1.0, -2.0, -3.0);

        assert_eq!(v.length(), 14.0_f64.sqrt());
    }

    #[test]
    fn normalise_vector1() {
        let v = vector(4.0, 0.0, 0.0);

        assert_eq!(v.norm(), vector(1.0, 0.0, 0.0));
    }

    #[test]
    fn normalise_vector2() {
        let v = vector(1.0, 2.0, 3.0);
        let div = 14.0_f64.sqrt();

        assert_eq!(v.norm(), vector(1.0/div, 2.0/div, 3.0/div));
    }

    #[test]
    fn magnitude_of_normalised_vector() {
        let v = vector(1.0, 2.0, 3.0);

        assert_eq!(v.norm().length(), 1.0);
    }

    #[test]
    fn dot_product() {
        let v1 = vector(1.0, 2.0, 3.0);
        let v2 = vector(2.0, 3.0, 4.0);

        assert_eq!(v1.dprod(v2), 20.0);
    }

    #[test]
    fn cross_product() {
        let v1 = vector(1.0, 2.0, 3.0);
        let v2 = vector(2.0, 3.0, 4.0);

        assert_eq!(v1.xprod(v2), vector(-1.0, 2.0, -1.0));
        assert_eq!(v2.xprod(v1), vector(1.0, -2.0, 1.0));
    }
}