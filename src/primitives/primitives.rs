use super::TestShape;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Primitive {
    Plane(),
    Sphere(),
    TestShape(TestShape)
}