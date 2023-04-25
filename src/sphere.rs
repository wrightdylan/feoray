// use std::cmp::Ordering;
// use nalgebra::Vector4;

use crate::{Ray, Intersection, Intersections, Object, point};

// Original struct no longer needed as centre and radius is defined by the
// identity matrix anyway.
#[derive(Debug, Clone, Copy)]
pub struct Sphere;
/*pub struct Sphere {
    pub centre: Vector4<f64>,
    pub radius: f64
}*/

impl Sphere {
    pub fn new() -> Self {
        Sphere {}
    }

    pub fn intersect(ray: Ray, object: &Object) -> Intersections {
        // let rosc = ray.origin - self.centre;
        let local_ray = Ray {
            origin: object.inverse_transform * ray.origin,
            direction: object.inverse_transform * ray.direction
        };
        let rosc = local_ray.origin - point(0.0, 0.0, 0.0);
        let a = local_ray.direction.dot(&local_ray.direction);
        let b = 2.0 * rosc.dot(&local_ray.direction);
        // let c = rosc.dot(&rosc) - self.radius * self.radius;
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
}

/*impl Default for Sphere {
    fn default() -> Self {
        centre: point(0.0, 0.0, 0.0),
        radius:1.0
    }
}*/

/*impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self.centre == other.centre && self.radius == other.radius
    }
}

impl PartialOrd for Sphere {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.radius.partial_cmp(&other.radius).unwrap())
    }
}*/

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{vector, Transform};
    use nalgebra::Matrix4;

    #[test]
    fn ray_intersects_sphere_2points() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        // let s = Sphere::default();
        let s = Object::new_sphere();
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let r = Ray::new(point(0.0, 1.0, -5.0), vector(0.0, 0.0, 1.0));
        // let s = Sphere::default();
        let s = Object::new_sphere();
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0);
    }

    #[test]
    fn ray_misses_sphere() {
        let r = Ray::new(point(0.0, 2.0, -5.0), vector(0.0, 0.0, 1.0));
        // let s = Sphere::default();
        let s = Object::new_sphere();
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        // let s = Sphere::default();
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
        // let s = Sphere::default();
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
}