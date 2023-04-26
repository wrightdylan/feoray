pub const EPSILON: f64 = 1.0e-5;

pub use tuple::*;
pub use canvas::*;
pub use colour::*;
pub use intersections::*;
pub use materials::*;
pub use matrix::*;
pub use object::*;
pub use point_light::*;
pub use primitives::*;
pub use rays::*;
pub use sphere::*;
pub use transformers::*;

pub mod tuple;
pub mod canvas;
pub mod colour;
pub mod intersections;
pub mod materials;
pub mod matrix;
pub mod object;
pub mod point_light;
pub mod primitives;
pub mod rays;
pub mod sphere;
pub mod transformers;