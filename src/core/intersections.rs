use crate::EPSILON;
use crate::core::{PreCompData, Ray, Tuple};
use crate::primitives::Object;
use std::cmp::Ordering;
use std::ops::Index;
use std::slice::Iter;

#[derive(Debug, Clone, Copy, PartialOrd)]
pub struct Intersection {
    pub t: f64,
    pub object: Object
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        (self.t - other.t).abs() < EPSILON
    }
}

impl Eq for Intersection {}

impl Ord for Intersection {
    fn cmp(&self, other: &Self) -> Ordering {
        let diff = self.t - other.t;
        if diff.abs() < EPSILON {
            Ordering::Equal
        } else if diff < 0.0 {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

impl Intersection {
    pub fn new(t: f64, object: Object) -> Self {
        Intersection {
            t,
            object
        }
    }
}

#[derive(Debug, Clone)]
pub struct Intersections {
    pub intrsc: Vec<Intersection>
}

impl Intersections {
    pub fn new(mut intrsc: Vec<Intersection>) -> Self {
        intrsc.sort();
        Intersections { intrsc }
    }

    pub fn hit(&self) -> Option<&Intersection> {
        self.intrsc.iter().find(|i| i.t >= 0.0)
    }

    pub fn len(&self) -> usize {
        self.intrsc.len()
    }

    pub fn hit_index(&self) -> Option<usize> {
        self.intrsc.iter().position(|i| i.t >= 0.0)
    }

    pub fn iter(&self) -> Iter<Intersection> {
        self.intrsc.iter()
    }

    // Reminder to refactor later
    pub fn prepare_computations(&self, index: usize, ray: &Ray) -> PreCompData {
        let mut containers = Vec::<Object>::new();
        let mut n1 = None;
        let mut n2 = None;

        for i in 0..self.len() {
            let is_hit = i == index;
            
            if is_hit {
                if containers.is_empty() {
                    n1 = Some(1.0);
                } else {
                    n1 = Some(containers.last().unwrap().material.ior);
                }
            }

            // Kinda pukey 🤮 but it works
            let mut found = false;
            let mut cnt_idx = 0;
            for (j, obj) in containers.iter().enumerate() {
                if obj == &self[i].object {
                    found = true;
                    cnt_idx = j;
                    break;
                }
            }
            if found {
                containers.remove(cnt_idx);
            } else {
                containers.push(self[i].object);
            }

            if is_hit {
                if containers.is_empty() {
                    n2 = Some(1.0);
                } else {
                    n2 = Some(containers.last().unwrap().material.ior);
                }
                break;
            }
        }

        let intersection = self[index];
        let pos = ray.position(intersection.t);
        let eye_vec = -ray.direction;
        let mut normal_vec = intersection.object.normal_at(pos);
        let inside = if normal_vec.dot(&eye_vec) < 0.0 {
            normal_vec = -normal_vec;
            true
        } else {
            false
        };
        let n1 = n1.unwrap_or(1.0);
        let n2 = n2.unwrap_or(1.0);
        let over_pos = pos + normal_vec * EPSILON;
        let under_pos = pos - normal_vec * EPSILON;
        let reflect_vec = ray.direction.reflect(normal_vec);

        PreCompData::new(
            intersection.t,
            intersection.object,
            pos,
            over_pos,
            under_pos,
            eye_vec,
            n1,
            n2,
            normal_vec,
            reflect_vec,
            inside
        )
    }
}

impl Default for Intersections {
    fn default() -> Self {
        Self::new(vec![])
    }
}

impl Index<usize> for Intersections {
    type Output = Intersection;

    fn index(&self, i: usize) -> &Self::Output {
        &self.intrsc[i]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{point, vector, Ray, Transform};
    use crate::materials::Material;
    use nalgebra::*;

    #[test]
    fn intersection_encapsulates_t_and_object() {
        let s = Object::new_sphere();
        let i = Intersection::new(3.5, s);

        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, s);
    }

    #[test]
    fn aggregating_intersections() {
        let s = Object::new_sphere();
        let i1 = Intersection::new(1.0, s);
        let i2 = Intersection::new(2.0, s);
        let mut intrsc = vec![];
        intrsc.push(i1);
        intrsc.push(i2);
        let xs = Intersections::new(intrsc);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[1].t, 2.0);
    }

    #[test]
    fn intersect_sets_object_on_intersection() {
        let r = Ray::new(
            point(0.0, 0.0, 0.0),
            vector(0.0, 0.0, 1.0)
        );
        let s = Object::new_sphere();
        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].object, s);
        assert_eq!(xs[1].object, s);
    }

    #[test]
    fn the_hit_when_all_ints_have_pos_t() {
        let s = Object::new_sphere();
        let i1 = Intersection::new(1.0, s);
        let i2 = Intersection::new(2.0, s);
        let xs = Intersections::new(vec![i1.clone(), i2]);

        assert_eq!(xs.hit().unwrap(), &i1);
    }

    #[test]
    fn the_hit_when_some_ints_have_neg_t() {
        let s = Object::new_sphere();
        let i1 = Intersection::new(-1.0, s);
        let i2 = Intersection::new(1.0, s);
        let xs = Intersections::new(vec![i1, i2.clone()]);

        assert_eq!(xs.hit().unwrap(), &i2);
    }

    #[test]
    fn the_hit_when_all_ints_have_neg_t() {
        let s = Object::new_sphere();
        let i1 = Intersection::new(-2.0, s);
        let i2 = Intersection::new(-1.0, s);
        let xs = Intersections::new(vec![i1, i2.clone()]);

        assert_eq!(xs.hit(), None);
    }

    #[test]
    fn the_hit_always_the_lowest_pos_t() {
        let s = Object::new_sphere();
        let i1 = Intersection::new(5.0, s);
        let i2 = Intersection::new(7.0, s);
        let i3 = Intersection::new(-3.0, s);
        let i4 = Intersection::new(2.0, s);
        let xs = Intersections::new(vec![i1, i2, i3, i4.clone()]);

        assert_eq!(xs.hit().unwrap(), &i4);
    }

    #[test]
    fn precomputing_state_of_intersection() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = Object::new_sphere();
        let int = Intersection::new(4.0, s);
        let ints = Intersections::new(vec![int]);
        let comps = ints.prepare_computations(0, &r);

        assert_eq!(comps.t, int.t);
        assert_eq!(comps.object, int.object);
        assert_eq!(comps.pos, point(0.0, 0.0, -1.0));
        assert_eq!(comps.eye_vec, vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normal_vec, vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn hit_when_intersection_occurs_outside() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = Object::new_sphere();
        let int = Intersection::new(4.0, s);
        let ints = Intersections::new(vec![int]);
        let comps = ints.prepare_computations(0, &r);

        assert_eq!(comps.inside, false);
    }

    #[test]
    fn hit_when_intersection_occurs_inside() {
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let s = Object::new_sphere();
        let int = Intersection::new(1.0, s);
        let ints = Intersections::new(vec![int]);
        let comps = ints.prepare_computations(0, &r);

        assert_eq!(comps.pos, point(0.0, 0.0, 1.0));
        assert_eq!(comps.eye_vec, vector(0.0, 0.0, -1.0));
        assert_eq!(comps.inside, true);
        assert_eq!(comps.normal_vec, vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn hit_should_offset_the_point() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = Object::new_sphere()
            .with_transform(Matrix4::translate(0.0, 0.0, 1.0));
        let int = Intersection::new(5.0, s);
        let ints = Intersections::new(vec![int]);
        let comps = ints.prepare_computations(0, &r);

        assert!(comps.over_pos.z < -EPSILON / 2.0);
        assert!(comps.pos.z > comps.over_pos.z);
    }

    #[test]
    fn precomputing_reflection_vector() {
        let s = Object::new_plane();
        let irr_no = 2.0f64.sqrt() / 2.0;
        let r = Ray::new(point(0.0, 1.0, -1.0), vector(0.0, -irr_no, irr_no));
        let int = Intersection::new(2.0f64.sqrt(), s);
        let ints = Intersections::new(vec![int]);
        let comps = ints.prepare_computations(0, &r);

        assert_eq!(comps.reflect_vec, vector(0.0, irr_no, irr_no));
    }

    #[test]
    fn finding_n1_and_n2_at_various_intersections() {
        let a = Object::glass_orb()
            .with_transform(Matrix4::uscale(2.0));
        let b = Object::glass_orb()
            .with_transform(Matrix4::translate(0.0, 0.0, -0.25))
            .with_material(Material::default().with_transparency(1.0).with_ior(2.0));
        let c = Object::glass_orb()
            .with_transform(Matrix4::translate(0.0, 0.0, 0.25))
            .with_material(Material::default().with_transparency(1.0).with_ior(2.5));
        let ray = Ray::new(point(0.0, 0.0, -4.0), vector(0.0, 0.0, 1.0));
        let xs = Intersections::new(vec![
            Intersection::new(2.0, a),
            Intersection::new(2.75, b),
            Intersection::new(3.25, c),
            Intersection::new(4.75, b),
            Intersection::new(5.25, c),
            Intersection::new(6.0, a)
        ]);

        assert_eq!(xs.prepare_computations(0, &ray).n1, 1.0);
        assert_eq!(xs.prepare_computations(0, &ray).n2, 1.5);
        assert_eq!(xs.prepare_computations(1, &ray).n1, 1.5);
        assert_eq!(xs.prepare_computations(1, &ray).n2, 2.0);
        assert_eq!(xs.prepare_computations(2, &ray).n1, 2.0);
        assert_eq!(xs.prepare_computations(2, &ray).n2, 2.5);
        assert_eq!(xs.prepare_computations(3, &ray).n1, 2.5);
        assert_eq!(xs.prepare_computations(3, &ray).n2, 2.5);
        assert_eq!(xs.prepare_computations(4, &ray).n1, 2.5);
        assert_eq!(xs.prepare_computations(4, &ray).n2, 1.5);
        assert_eq!(xs.prepare_computations(5, &ray).n1, 1.5);
        assert_eq!(xs.prepare_computations(5, &ray).n2, 1.0);
    }

    #[test]
    fn under_point_is_offset_below_surface() {
        let ray = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape = Object::glass_orb()
            .with_transform(Matrix4::translate(0.0, 0.0, 1.0));
        let xs = Intersections::new(vec![
            Intersection::new(5.0, shape)
        ]);
        let comps = xs.prepare_computations(0, &ray);

        assert!(comps.under_pos.z > EPSILON/2.0);
        assert!(comps.pos.z < comps.under_pos.z);
    }
}