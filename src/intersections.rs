use std::cmp::Ordering;
use std::ops::Index;
use crate::{EPSILON, Object};

#[derive(Debug, Clone, PartialOrd)]
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

#[derive(Debug, Clone)]
pub struct Intersections {
    intrsc: Vec<Intersection>
}

impl Intersection {
    pub fn new(t: f64, object: Object) -> Self {
        Intersection {
            t,
            object
        }
    }
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
    use crate::{point, vector, Ray, Sphere};

    #[test]
    fn intersection_encapsulates_t_and_object() {
        let s = Object::new_sphere(Sphere::default());
        let i = Intersection::new(3.5, s);

        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, s);
    }

    #[test]
    fn aggregating_intersections() {
        let s = Object::new_sphere(Sphere::default());
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
        let s = Object::new_sphere(Sphere::default());
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].object, s);
        assert_eq!(xs[1].object, s);
    }

    #[test]
    fn the_hit_when_all_ints_have_pos_t() {
        let s = Object::new_sphere(Sphere::default());
        let i1 = Intersection::new(1.0, s);
        let i2 = Intersection::new(2.0, s);
        let xs = Intersections::new(vec![i1.clone(), i2]);

        assert_eq!(xs.hit().unwrap(), &i1);
    }

    #[test]
    fn the_hit_when_some_ints_have_neg_t() {
        let s = Object::new_sphere(Sphere::default());
        let i1 = Intersection::new(-1.0, s);
        let i2 = Intersection::new(1.0, s);
        let xs = Intersections::new(vec![i1, i2.clone()]);

        assert_eq!(xs.hit().unwrap(), &i2);
    }

    #[test]
    fn the_hit_when_all_ints_have_neg_t() {
        let s = Object::new_sphere(Sphere::default());
        let i1 = Intersection::new(-2.0, s);
        let i2 = Intersection::new(-1.0, s);
        let xs = Intersections::new(vec![i1, i2.clone()]);

        assert_eq!(xs.hit(), None);
    }

    #[test]
    fn the_hit_always_the_lowest_pos_t() {
        let s = Object::new_sphere(Sphere::default());
        let i1 = Intersection::new(5.0, s);
        let i2 = Intersection::new(7.0, s);
        let i3 = Intersection::new(-3.0, s);
        let i4 = Intersection::new(2.0, s);
        let xs = Intersections::new(vec![i1, i2, i3, i4.clone()]);

        assert_eq!(xs.hit().unwrap(), &i4);
    }
}