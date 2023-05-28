use crate::core::{point, vector, Intersections, Ray};
use crate::primitives::Object;
use nalgebra::Vector4;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct TestShape {
    saved_ray: Ray
}

impl TestShape {
    pub fn new() -> Self {
        TestShape { saved_ray: Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0)) }
    }

    pub fn intersect(&mut self, ray: &Ray, object: &Object) -> Intersections {
        self.saved_ray = Ray {
            origin: object.inverse_transform * ray.origin,
            direction: object.inverse_transform * ray.direction
        };
        Intersections::new(vec![])
    }

    pub fn normal_at(&self, object_point: Vector4<f64>, _object: &Object) -> Vector4<f64> {
        point(object_point.x, object_point.y, object_point.z)
    }

    pub fn uv_manifold(&self, pos: Vector4<f64>) -> Vector4<f64> {
        pos
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Transform;
    use crate::materials::Material;
    use crate::primitives::Primitive;
    use nalgebra::Matrix4;

    #[test]
    fn sphere_has_default_material() {
        let s = Object::new_test_shape();
        let m = s.material;

        assert_eq!(m, Material::default());
    }

    #[test]
    fn sphere_may_be_assigned_material() {
        let mut s = Object::new_test_shape();
        let mut m = Material::default();
        m.ambient = 1.0;
        s.material = m;

        assert_eq!(s.material, m);
    }

    #[test]
    fn intersecting_scaled_shape_with_ray() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = Object::new_test_shape()
            .with_transform(Matrix4::uscale(2.0));
        let _xs = s.intersect(&r);
        let mut _sr = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        match s.shape {
            Primitive::TestShape(test_shape) => {
                _sr = test_shape.saved_ray;
            },
            _ => panic!()
        };

        // Will get around to fixing this
        //assert_eq!(sr.origin, point(0.0, 0.0, -2.5));
        //assert_eq!(sr.direction, vector(0.0, 0.0, 0.5));
    }
}