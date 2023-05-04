use crate::core::Tuple;
use nalgebra::{Matrix4, Vector4};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Ray {
    pub origin: Vector4<f64>,
    pub direction: Vector4<f64>
}

impl Ray {
    pub fn new(origin: Vector4<f64>, direction: Vector4<f64>) -> Self {
        if !origin.is_point() { panic!("origin should be a point"); }
        if !direction.is_vector() { panic!("direction should be a vector"); }
        Ray { origin, direction }
    }

    pub fn position(&self, t: f64) -> Vector4<f64> {
        self.origin + self.direction * t
    }

    pub fn transform(&self, m: Matrix4<f64>) -> Ray {
        Ray::new(m.clone() * self.origin, m * self.direction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{point, vector, Transform};

    #[test]
    fn create_and_query_ray() {
        let origin = point(1.0, 2.0, 3.0);
        let direction = vector(4.0, 5.0, 6.0);
        let ray = Ray::new(origin, direction);

        assert_eq!(ray.origin, origin);
        assert_eq!(ray.direction, direction);
    }

    #[test]
    fn compute_point_from_dist() {
        let r = Ray::new(point(2.0, 3.0, 4.0), vector(1.0, 0.0, 0.0));

        assert_eq!(r.position(0.0), point(2.0, 3.0, 4.0));
        assert_eq!(r.position(1.0), point(3.0, 3.0, 4.0));
        assert_eq!(r.position(-1.0), point(1.0, 3.0, 4.0));
        assert_eq!(r.position(2.5), point(4.5, 3.0, 4.0));
    }

    #[test]
    fn translating_a_ray() {
        let r = Ray::new(point(1.0, 2.0, 3.0), vector(0.0, 1.0, 0.0));
        let m = Matrix4::translate(3.0, 4.0, 5.0);
        let r2 = r.transform(m);

        assert_eq!(r2.origin, point(4.0, 6.0, 8.0));
        assert_eq!(r2.direction, vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn scaling_a_ray() {
        let r = Ray::new(point(1.0, 2.0, 3.0), vector(0.0, 1.0, 0.0));
        let m = Matrix4::nuscale(2.0, 3.0, 4.0);
        let r2 = r.transform(m);

        assert_eq!(r2.origin, point(2.0, 6.0, 12.0));
        assert_eq!(r2.direction, vector(0.0, 3.0, 0.0));
    }
}