use nalgebra::Vector4;

use crate::{Ray, Intersection, Intersections, Object, point};

// Original struct no longer needed as centre and radius is defined by the
// identity matrix anyway.
#[derive(Debug, Clone, Copy)]
pub struct Sphere;

impl Sphere {
    pub fn new() -> Self {
        Sphere {}
    }

    pub fn intersect(ray: Ray, object: &Object) -> Intersections {
        let local_ray = Ray {
            origin: object.inverse_transform * ray.origin,
            direction: object.inverse_transform * ray.direction
        };
        let rosc = local_ray.origin - point(0.0, 0.0, 0.0);
        let a = local_ray.direction.dot(&local_ray.direction);
        let b = 2.0 * rosc.dot(&local_ray.direction);
        let c = rosc.dot(&rosc) - 1.0;
        let d = b * b - 4.0 * a * c;
        if d < 0.0 {
            Intersections::default()
        } else {
            let t1 = (-b - d.sqrt()) / (2.0 * a);
            let t2 = (-b + d.sqrt()) / (2.0 * a);
            let mut intrsc = vec![];
            
            intrsc.push(Intersection {
                t: t1,
                object: object.clone()
            });
            intrsc.push(Intersection {
                t: t2,
                object: object.clone()
            });

            Intersections::new(intrsc)
        }
    }

    pub fn normal_at(object_point: Vector4<f64>, object: &Object) -> Vector4<f64> {
        let object_normal = (object.inverse_transform * object_point) - point(0.0, 0.0, 0.0);
        let mut world_normal = object.inverse_transform.transpose() * object_normal;
        world_normal.w = 0.0;
        world_normal.normalize_mut();
        Vector4::new(world_normal.x, world_normal.y, world_normal.z, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{vector, Material, Transform, Tuple};
    use nalgebra::Matrix4;
    use std::f64::consts::PI;

    #[test]
    fn ray_intersects_sphere_2points() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = Object::new_sphere();
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let r = Ray::new(point(0.0, 1.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = Object::new_sphere();
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0);
    }

    #[test]
    fn ray_misses_sphere() {
        let r = Ray::new(point(0.0, 2.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = Object::new_sphere();
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let s = Object::new_sphere();
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }

    #[test]
    fn sphere_is_behind_ray() {
        let r = Ray::new(
            point(0.0, 0.0, 5.0),
            vector(0.0, 0.0, 1.0)
        );
        let s = Object::new_sphere();
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);
    }

    #[test]
    fn intersecting_scaled_sphere_with_ray() {
        let r = Ray::new(
            point(0.0, 0.0, -5.0),
            vector(0.0, 0.0, 1.0)
        );
        let mut s = Object::new_sphere();
        s.with_transform(Matrix4::uscale(2.0));
        let xs = s.intersect(r);

        println!("{}, {}", xs[0].t, xs[1].t);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.0);
        assert_eq!(xs[1].t, 7.0);
    }

    #[test]
    fn intersecting_translated_sphere_with_ray() {
        let r = Ray::new(
            point(0.0, 0.0, -5.0),
            vector(0.0, 0.0, 1.0)
        );
        let mut s = Object::new_sphere();
        s.with_transform(Matrix4::translate(5.0, 0.0, 0.0));
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn normal_on_sphere_at_point_on_x_axis() {
        let s = Object::new_sphere();
        let n = s.normal_at(point(1.0, 0.0, 0.0));

        assert_eq!(n, vector(1.0, 0.0, 0.0));
    }

    #[test]
    fn normal_on_sphere_at_point_on_y_axis() {
        let s = Object::new_sphere();
        let n = s.normal_at(point(0.0, 1.0, 0.0));

        assert_eq!(n, vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn normal_on_sphere_at_point_on_z_axis() {
        let s = Object::new_sphere();
        let n = s.normal_at(point(0.0, 0.0, 1.0));

        assert_eq!(n, vector(0.0, 0.0, 1.0));
    }

    #[test]
    fn normal_on_sphere_at_nonaxial_point() {
        let s = Object::new_sphere();
        let irr_no = 3.0f64.sqrt() / 3.0;
        let n = s.normal_at(point(irr_no, irr_no, irr_no));

        assert_eq!(n, vector(irr_no, irr_no, irr_no));
    }

    #[test]
    fn normal_is_normalised_vector() {
        let s = Object::new_sphere();
        let irr_no = 3.0f64.sqrt() / 3.0;
        let n = s.normal_at(point(irr_no, irr_no, irr_no));

        assert_eq!(n, n.normalize());
    }

    #[test]
    fn computing_normal_on_translated_sphere() {
        let mut s = Object::new_sphere();
        s.with_transform(Matrix4::translate(0.0, 1.0, 0.0));
        let n = s.normal_at(point(0.0, 1.70711, -0.70711));

        assert_eq!(n.to_5dp(), vector(0.0, 0.70711, -0.70711));
    }

    #[test]
    fn computing_normal_on_transformed_sphere() {
        let mut s = Object::new_sphere();
        s.with_transform(
            Matrix4::nuscale(1.0, 0.5, 1.0) *
            Matrix4::rot_z(PI/5.0)
        );
        let irr_no = 2.0f64.sqrt() / 2.0;
        let n = s.normal_at(point(0.0, irr_no, -irr_no));

        assert_eq!(n.to_5dp(), vector(0.0, 0.97014, -0.24254));
    }

    #[test]
    fn sphere_has_default_material() {
        let s = Object::new_sphere();
        let m = s.material;

        assert_eq!(m, Material::default());
    }

    #[test]
    fn sphere_may_be_assigned_material() {
        let mut s = Object::new_sphere();
        let mut m = Material::default();
        m.ambient = 1.0;
        s.material = m;

        assert_eq!(s.material, m);
    }
}