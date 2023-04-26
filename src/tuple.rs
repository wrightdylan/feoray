use nalgebra::Vector4;

// Previous iteration of tuple.rs (i.e. pre-nalgebra refactoring) can be
// found in the archive folder. This file only defines a trait and custom
// methods specific tothis project that either are not available in the
// nalgebra crate, or have slightly different functionality.

pub trait Tuple {
    fn is_point(&self) -> bool;
    fn is_vector(&self) -> bool;
    fn reflect(&self, n: Vector4<f64>) -> Vector4<f64>;
    fn to_5dp(&self) -> Vector4<f64>;
    fn xprod(&self, rhs: &Vector4<f64>) -> Vector4<f64>;
}

impl Tuple for Vector4<f64> {
    /// Tests if a Tuple is a point
    fn is_point(&self) -> bool {
        self.w == 1.0
    }

    /// Tests if a Tuple is a vector
    fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    fn reflect(&self, n: Vector4<f64>) -> Vector4<f64> {
        self - n * 2.0 * self.dot(&n)
    }

    /// Rounds a Tuple to 5dp. Only useful for tests.
    fn to_5dp(&self) -> Self {
        let mut res = Vector4::zeros();
        for i in 0..4 {
            res[i] = (self[i] * 100000.0).round() / 100000.0;
        }
        res
    }

    // Have to use a modification of my xprod method as nalgebra version will
    // not work with my code
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
    fn xprod(&self, rhs: &Vector4<f64>) -> Vector4<f64> {
        let x = (self.y * rhs.z) - (self.z * rhs.y);
        let y = (self.z * rhs.x) - (self.x * rhs.z);
        let z = (self.x * rhs.y) - (self.y * rhs.x);
        let w = 0.0;
        Vector4::new(x, y, z, w)
    }
}

pub fn point(x: f64, y: f64, z: f64) -> Vector4<f64> {
    Vector4::new(x, y, z, 1.0)
}

pub fn vector(x: f64, y: f64, z: f64) -> Vector4<f64> {
    Vector4::new(x, y, z, 0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuple_is_point() {
        let tup = Vector4::new(4.3, -4.2, 3.1, 1.0);

        assert_eq!((tup.x, tup.y, tup.z, tup.w), (4.3, -4.2, 3.1, 1.0));
        assert_eq!(tup.is_point(), true);
    }

    #[test]
    fn tuple_is_vector() {
        let tup = Vector4::new(4.3, -4.2, 3.1, 0.0);

        assert_eq!((tup.x, tup.y, tup.z, tup.w), (4.3, -4.2, 3.1, 0.0));
        assert_eq!(tup.is_vector(), true);
    }

    #[test]
    fn coordinate_to_point() {
        assert_eq!(point(4.0, -4.0, 3.0), Vector4::new(4.0, -4.0, 3.0, 1.0));
    }

    #[test]
    fn coordinate_to_vector() {
        assert_eq!(vector(4.0, -4.0, 3.0), Vector4::new(4.0, -4.0, 3.0, 0.0));
    }

    #[test]
    fn add_tuples() {
        let tup1 = point(3.0, -2.0, 5.0);
        let tup2 = vector(-2.0, 3.0, 1.0);

        assert_eq!(tup1 + tup2, Vector4::new(1.0, 1.0, 6.0, 1.0));
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
        let tup = Vector4::new(1.0, -2.0, 3.0, -4.0);

        assert_eq!(-tup, Vector4::new(-1.0, 2.0, -3.0, 4.0));
    }

    #[test]
    fn multiply_tuple_by_scalar() {
        let tup = Vector4::new(1.0, -2.0, 3.0, -4.0);
        let s = 3.5;

        assert_eq!(tup * s, Vector4::new(3.5, -7.0, 10.5, -14.0));
    }

    #[test]
    fn multiply_tuple_by_fraction() {
        let tup = Vector4::new(1.0, -2.0, 3.0, -4.0);
        let s = 0.5;

        assert_eq!(tup * s, Vector4::new(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn divide_tuple_by_scalar() {
        let tup = Vector4::new(1.0, -2.0, 3.0, -4.0);
        let s = 2.0;

        assert_eq!(tup / s, Vector4::new(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn magnitude_of_vector1() {
        let v = vector(1.0, 0.0, 0.0);

        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn magnitude_of_vector2() {
        let v = vector(0.0, 1.0, 0.0);

        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn magnitude_of_vector3() {
        let v = vector(0.0, 0.0, 1.0);

        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn magnitude_of_vector4() {
        let v = vector(1.0, 2.0, 3.0);

        assert_eq!(v.magnitude(), 14.0_f64.sqrt());
    }

    #[test]
    fn magnitude_of_vector5() {
        let v = vector(-1.0, -2.0, -3.0);

        assert_eq!(v.magnitude(), 14.0_f64.sqrt());
    }

    #[test]
    fn normalise_vector1() {
        let v = vector(4.0, 0.0, 0.0);

        assert_eq!(v.normalize(), vector(1.0, 0.0, 0.0));
    }

    #[test]
    fn normalise_vector2() {
        let v = vector(1.0, 2.0, 3.0);
        let div = 14.0_f64.sqrt();

        assert_eq!(v.normalize(), vector(1.0/div, 2.0/div, 3.0/div));
    }

    #[test]
    fn magnitude_of_normalised_vector() {
        let v = vector(1.0, 2.0, 3.0);

        assert_eq!(v.normalize().magnitude(), 1.0);
    }

    #[test]
    fn dot_product() {
        let v1 = vector(1.0, 2.0, 3.0);
        let v2 = vector(2.0, 3.0, 4.0);

        assert_eq!(v1.dot(&v2), 20.0);
    }

    #[test]
    fn cross_product() {
        let v1 = vector(1.0, 2.0, 3.0);
        let v2 = vector(2.0, 3.0, 4.0);

        assert_eq!(v1.xprod(&v2), vector(-1.0, 2.0, -1.0));
        assert_eq!(v2.xprod(&v1), vector(1.0, -2.0, 1.0));
    }

    #[test]
    fn reflecting_vector_approaching_at_45d() {
        let v = vector(1.0, -1.0, 0.0);
        let n = vector(0.0, 1.0, 0.0);

        assert_eq!(v.reflect(n).to_5dp(), vector(1.0, 1.0, 0.0));
    }

    #[test]
    fn reflecting_vector_off_slanted_surface() {
        let v = vector(0.0, -1.0, 0.0);
        let irr_no = 2.0f64.sqrt() / 2.0;
        let n = vector(irr_no, irr_no, 0.0);

        assert_eq!(v.reflect(n).to_5dp(), vector(1.0, 0.0, 0.0));
    }
}