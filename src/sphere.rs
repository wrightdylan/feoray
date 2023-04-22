use std::cmp::Ordering;

use crate::{Tuple, Ray, Intersection, Intersections, Object, point};

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub centre: Tuple,
    pub radius: f64
}

fn discriminant(a: f64, b: f64, c: f64) -> f64 {
    b * b - 4.0 * a * c
}

impl Sphere {
    pub fn new(centre: Tuple, radius: f64) -> Self {
        Sphere { centre, radius }
    }

    pub fn intersect(&self, ray: Ray) -> Intersections {
        let rosc = ray.origin - self.centre;
        let a = ray.direction.dprod(ray.direction);
        let b = 2.0* rosc.dprod(ray.direction);
        let c = rosc.dprod(rosc) - self.radius * self.radius;
        let d = discriminant(a, b, c);
        if d < 0.0 {
            Intersections::default()
        } else {
            let t1 = (-b - d.sqrt()) / (2.0 * a);
            let t2 = (-b + d.sqrt()) / (2.0 * a);
            let mut intrsc = vec![];
            
            intrsc.push(Intersection {
                t: t1,
                object: Object::new_sphere(Sphere::default())
            });
            intrsc.push(Intersection {
                t: t2,
                object: Object::new_sphere(Sphere::default())
            });

            Intersections::new(intrsc)
        }
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            centre: point(0.0, 0.0, 0.0),
            radius: 1.0
        }
    }
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self.centre == other.centre && self.radius == other.radius
    }
}

impl PartialOrd for Sphere {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.radius.partial_cmp(&other.radius).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vector;

    #[test]
    fn ray_intersects_sphere_2points() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let r = Ray::new(point(0.0, 1.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0);
    }

    #[test]
    fn ray_misses_sphere() {
        let r = Ray::new(point(0.0, 2.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }

    #[test]
    fn sphere_is_behind_ray() {
        let r = Ray::new(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);
    }
}