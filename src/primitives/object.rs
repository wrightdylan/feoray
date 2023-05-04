use crate::core::{Intersections, Ray};
use crate::materials::Material;
use crate::primitives::{Plane, Primitive, Sphere, TestShape};
use nalgebra::{Matrix4, Vector4};


#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Object {
    pub shape: Primitive,
    pub material: Material,
    pub transform: Matrix4<f64>,
    pub inverse_transform: Matrix4<f64>
}

impl Object {
    /// Creates a new plane at 0.0, 0.0, 0.0.
    pub fn new_plane() -> Self {
        let shape = Primitive::Plane();
        Object { shape, ..Default::default() }
    }

    /// Creates a new sphere at 0.0, 0.0, 0.0 with radius 1.0.
    pub fn new_sphere() -> Self {
        let shape = Primitive::Sphere();
        Object { shape, ..Default::default() }
    }

    /// Creates a new test shape at 0.0, 0.0, 0.0.
    pub fn new_test_shape() -> Self {
        let shape = Primitive::TestShape(TestShape::new());
        Object { shape, ..Default::default() }
    }

    /// Calculates intersections between a ray and an object, if any.
    pub fn intersect(&self, ray: Ray) -> Intersections {
        match self.shape {
            Primitive::Plane() => Plane::intersect(ray, self),
            Primitive::Sphere() => Sphere::intersect(ray, self),
            Primitive::TestShape(mut t) => t.intersect(ray, self)
        }
    }

    /// Calculates the normal at a specified point on an object.
    pub fn normal_at(&self, object_point: Vector4<f64>) -> Vector4<f64> {
        match self.shape {
            Primitive::Plane() => Plane::normal_at(object_point, self),
            Primitive::Sphere() => Sphere::normal_at(object_point, self),
            Primitive::TestShape(t) => t.normal_at(object_point, self)
        }
    }

    /// Applies a transform directly to an object. For single transforms, use the
    /// trait methods, but for complex transforms use the transform builder.
    pub fn with_transform(&mut self, transform: Matrix4<f64>) -> Self {
        self.transform = transform;
        self.inverse_transform = transform.try_inverse().unwrap();

        *self
    }

    /// Applies a material to an object.
    pub fn with_material(&mut self, material: Material) -> Self {
        self.material = material;

        *self
    }
}

// Blender has their default cube, we have a default sphere.
impl Default for Object {
    fn default() -> Self {
        Object {
            shape: Primitive::Sphere(),
            material: Material::default(),
            transform: Matrix4::identity(),
            inverse_transform: Matrix4::identity()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Transform;

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

    #[test]
    fn the_default_transformation() {
        let s = Object::new_test_shape();

        assert_eq!(s.transform, Matrix4::identity());
    }

    #[test]
    fn assigning_a_transformation() {
        let s = Object::new_test_shape()
            .with_transform(Matrix4::translate(2.0, 3.0, 4.0));

        assert_eq!(s.transform, Matrix4::translate(2.0, 3.0, 4.0));
    }
}