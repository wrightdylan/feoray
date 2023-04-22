use crate::Sphere;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Primitive {
    Sphere(Sphere)
}