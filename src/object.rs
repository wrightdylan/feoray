use nalgebra::Matrix4;
use crate::{Intersections, Primitive, Ray, Sphere};


#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Object {
    pub shape: Primitive,
    pub transform: Matrix4<f64>,
    pub inverse_transform: Matrix4<f64>
}

impl Object {
    pub fn new_sphere() -> Self {
        let shape = Primitive::Sphere();
        Object { shape, ..Default::default() }
    }

    pub fn intersect(&self, ray: Ray) -> Intersections {
        match self.shape {
            Primitive::Sphere() => Sphere::intersect(ray, self)
        }
    }

    pub fn with_transform(&mut self, transform: Matrix4<f64>) -> Self {
        self.transform = transform;
        self.inverse_transform = transform.try_inverse().unwrap();

        *self
    }
}

// Blender has their default cube, we have a default sphere.
impl Default for Object {
    fn default() -> Self {
        Object {
            shape: Primitive::Sphere(),
            transform: Matrix4::identity(),
            inverse_transform: Matrix4::identity()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Transform;

    #[test]
    fn a_spheres_default_transformation() {
        let s = Object::new_sphere();

        assert_eq!(s.transform, Matrix4::identity());
    }
    
    #[test]
    fn changing_a_spheres_transform() {
        let mut s = Object::new_sphere();
        let t = Matrix4::translate(2.0, 3.0, 4.0);
        s.with_transform(t);

        assert_eq!(s.transform, t);
    }
}