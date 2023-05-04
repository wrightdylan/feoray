use crate::EPSILON;
use crate::core::{vector, Intersection, Intersections, Ray};
use crate::primitives::Object;
use nalgebra::Vector4;

// Object is infinite in size, and has constant normal at all points.
#[derive(Debug, Clone, Copy)]
pub struct Plane;

impl Plane {
    pub fn new() -> Self {
        Plane {}
    }

    pub fn intersect(ray: Ray, object: &Object) -> Intersections {
        if ray.direction.y.abs() < EPSILON {
            Intersections::default()
        } else {
            let t = -ray.origin.y / ray.direction.y;
            Intersections::new(vec![Intersection::new(t, object.clone())])
        }
    }

    pub fn normal_at(_object_point: Vector4<f64>, _object: &Object) -> Vector4<f64> {
        vector(0.0, 1.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::point;

    #[test]
    fn normal_of_plane_constant_everywhere() {
        let p = Object::new_plane();
        let n1 = p.normal_at(point(0.0, 0.0, 0.0));
        let n2 = p.normal_at(point(10.0, 0.0, -10.0));
        let n3 = p.normal_at(point(-5.0, 0.0, 150.0));

        debug_assert_eq!(n1, vector(0.0, 1.0, 0.0));
        debug_assert_eq!(n2, vector(0.0, 1.0, 0.0));
        debug_assert_eq!(n3, vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn intersect_with_ray_parallel_to_plane() {
        let p = Object::new_plane();
        let r = Ray::new(point(0.0, 10.0, 0.0), vector(0.0, 0.0, 1.0));
        let xs = p.intersect(r);

        assert_eq!(xs.intrsc.len(), 0);
    }

    #[test]
    fn intersect_with_coplanar_ray() {
        let p = Object::new_plane();
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let xs = p.intersect(r);

        assert_eq!(xs.intrsc.len(), 0);
    }

    #[test]
    fn ray_intersecting_plane_from_above() {
        let p = Object::new_plane();
        let r = Ray::new(point(0.0, 1.0, 0.0), vector(0.0, -1.0, 0.0));
        let xs = p.intersect(r);

        assert_eq!(xs.intrsc.len(), 1);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[0].object, p);
    }

    #[test]
    fn ray_intersecting_plane_from_below() {
        let p = Object::new_plane();
        let r = Ray::new(point(0.0, -1.0, 0.0), vector(0.0, 1.0, 0.0));
        let xs = p.intersect(r);

        assert_eq!(xs.intrsc.len(), 1);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[0].object, p);
    }
}