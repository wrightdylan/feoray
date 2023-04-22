use crate::{Intersections, Primitive, Ray, Sphere};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Object {
    pub shape: Primitive
}

impl Object {
    pub fn new_sphere(sphere: Sphere) -> Self {
        Object { shape: Primitive::Sphere(sphere) }
    }

    pub fn intersect(&self, ray: Ray) -> Intersections {
        match self.shape {
            Primitive::Sphere(ref sphere) => sphere.intersect(ray)
        }
    }
}