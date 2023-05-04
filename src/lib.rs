pub const EPSILON: f64 = 1.0e-5;

pub mod core {
    pub use camera::Camera;
    pub use canvas::{canvas, Canvas};
    pub use colour::Colour;
    pub use intersections::{Intersection, Intersections};
    pub use matrix::Test;
    pub use precomp::PreCompData;
    pub use rays::Ray;
    pub use transformers::{Transform, TransformBuilder};
    pub use tuple::{point, vector, Tuple};
    pub use world::World;

    pub mod camera;
    pub mod canvas;
    pub mod colour;
    pub mod intersections;
    pub mod matrix;
    pub mod precomp;
    pub mod rays;
    pub mod transformers;
    pub mod tuple;
    pub mod world;
}

pub mod materials {
    pub use materials::Material;

    pub mod materials;
}

pub mod primitives {
    pub use object::Object;
    pub use primitives::Primitive;
    pub use plane::Plane;
    pub use sphere::Sphere;
    pub use test_shape::TestShape;

    pub mod object;
    pub mod primitives;
    pub mod plane;
    pub mod sphere;
    pub mod test_shape;
}

pub mod lights {
    pub use point_light::PointLight;

    pub mod point_light;
}