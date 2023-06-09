use crate::core::Colour;
use crate::primitives::Object;
use nalgebra::{Matrix4, Vector4};
use noise::{NoiseFn, Perlin};
use std::f64::consts::PI;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Pattern {
    pattern: Patterns,
    pub transform: Matrix4<f64>,
    pub inverse_transform: Matrix4<f64>
}

impl Pattern {
    /// Constructs a checker pattern
    pub fn new_checkers(a: Colour, b: Colour) -> Self {
        Pattern {
            pattern: Patterns::Checkers(CheckerPattern { a, b }),
            ..Default::default()
        }
    }

    /// Constructs a gradient pattern
    pub fn new_gradient(a: Colour, b: Colour) -> Self {
        Pattern {
            pattern: Patterns::Gradient(GradientPattern { a, b, jitter: None }),
            ..Default::default()
        }
    }

    /// Constructs a radial pattern
    pub fn new_radial(a: Colour, b: Colour, n: usize ) -> Self {
        Pattern {
            pattern: Patterns::Radial(RadialPattern { a, b, n }),
            ..Default::default()
        }
    }

    /// Constructs a ring pattern
    pub fn new_rings(a: Colour, b: Colour) -> Self {
        Pattern {
            pattern: Patterns::Rings(RingPattern { a, b }),
            ..Default::default()
        }
    }

    /// Constructs a pattern with one solid colour
    pub fn new_solid(colour: Colour) -> Self {
        Pattern {
            pattern: Patterns::Solid(SolidPattern { colour }),
            ..Default::default()
        }
    }

    /// Constructs a stripe pattern
    pub fn new_stripes(a: Colour, b: Colour) -> Self {
        Pattern {
            pattern: Patterns::Stripes(StripePattern { a, b }),
            ..Default::default()
        }
    }

    /// Constructs a pattern only for testing. Not to be used.
    pub fn new_test() -> Self {
        Pattern {
            pattern: Patterns::Test(TestPattern {}),
            ..Default::default()
        }
    }

    fn pattern_at(&self, point: Vector4<f64>) -> Colour {
        match &self.pattern {
            Patterns::Checkers(pattern) => pattern.pattern_at(point),
            Patterns::Gradient(pattern) => pattern.pattern_at(point),
            Patterns::Radial(pattern) => pattern.pattern_at(point),
            Patterns::Rings(pattern) => pattern.pattern_at(point),
            Patterns::Solid(pattern) => pattern.pattern_at(point),
            Patterns::Stripes(pattern) => pattern.pattern_at(point),
            Patterns::Test(pattern) => pattern.pattern_at(point)
        }
    }

    pub fn pattern_at_object(&self, object: Object, pos: Vector4<f64>) -> Colour {
        let object_point = object.inverse_transform * pos;
        let mut point = self.inverse_transform * object_point;

        if object.uv_manifold {
            point = object.uv_at(point);
        }

        self.pattern_at(point)
    }

    pub fn with_transform(&mut self, transform: Matrix4<f64>) -> Self {
        self.transform = transform;
        self.inverse_transform = transform.try_inverse().unwrap();

        *self
    }
}

impl Default for Pattern {
    fn default() -> Self {
        Pattern {
            pattern: Patterns::Solid(SolidPattern { colour: Colour::white() }),
            transform: Matrix4::identity(),
            inverse_transform: Matrix4::identity()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum Patterns {
    Checkers(CheckerPattern),
    Gradient(GradientPattern),
    Radial(RadialPattern),
    Rings(RingPattern),
    Solid(SolidPattern),
    Stripes(StripePattern),
    Test(TestPattern)
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct CheckerPattern {
    a: Colour,
    b: Colour
}

impl CheckerPattern {
    fn pattern_at(&self, point: Vector4<f64>) -> Colour {
        if (point.x.floor() + point.y.floor() + point.z.floor()) % 2.0 == 0.0 {
            self.a
        } else {
            self.b
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct GradientPattern {
    a: Colour,
    b: Colour,
    jitter: Option<Jitter>
}

impl GradientPattern {
    fn pattern_at(&self, point: Vector4<f64>) -> Colour {
        let gradient = self.a + (self.b - self.a) * (point.x - point.x.floor());
        let mut noise_colour = Colour::white();
        if self.jitter.is_some() {
            let perlin = Perlin::new(self.jitter.unwrap().seed);
            let jitter = perlin.get([point.x, point.y, point.z]).abs() as f32;
            noise_colour = Colour::new(jitter, jitter, jitter) * self.jitter.unwrap().amp as f32;
        }
        gradient * noise_colour
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct RadialPattern {
    a: Colour,
    b: Colour,
    n: usize
}

impl RadialPattern {
    fn pattern_at(&self, point: Vector4<f64>) -> Colour {
        let angle = point.z.atan2(point.x);
        let sector_size = PI / (self.n as f64);
        let sector_number = ((angle + PI)/sector_size).floor() as usize;
        if sector_number % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct RingPattern {
    a: Colour,
    b: Colour
}

impl RingPattern {
    fn pattern_at(&self, point: Vector4<f64>) -> Colour {
        if (point.x.powi(2) + point.z.powi(2)).sqrt().floor() % 2.0 == 0.0 {
            self.a
        } else {
            self.b
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct SolidPattern {
    colour: Colour
}

impl SolidPattern {
    fn pattern_at(&self, _point: Vector4<f64>) -> Colour {
        self.colour
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct StripePattern {
    a: Colour,
    b: Colour
}

impl StripePattern {
    fn pattern_at(&self, point: Vector4<f64>) -> Colour {
        if point.x.floor() % 2.0 == 0.0 {
            self.a
        } else {
            self.b
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct TestPattern {}

impl TestPattern {
    fn pattern_at(&self, point: Vector4<f64>) -> Colour {
        Colour::new(point.x as f32, point.y as f32, point.z as f32)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Jitter {
    seed: u32,
    amp: f64
}

impl Jitter {
    pub fn new(seed: u32, amp: f64) -> Self {
        Self { seed, amp }
    }
}

// Can't piggyback off Pattern. Must include it as a Pattern, which gets very messy.
// It's the same issue as nested patterns.
/*pub struct BlendedPattern {
    a: Pattern,
    b: Pattern
}

impl BlendedPattern {
    /// Blend two patterns together. You probably don't want to do more than that.
    pub fn new(a: Pattern, b: Pattern) -> Self {
        BlendedPattern { a, b }
    }

    fn pattern_at(&self, point: Vector4<f64>) -> Colour {
        let colour_a = self.a.pattern_at(point);
        let colour_b = self.b.pattern_at(point);
        (colour_a + colour_b) / 2.0
    }
}*/

#[cfg(test)]
mod tests {
    use crate::core::{point, Transform};

    use super::*;

    #[test]
    fn creating_stripe_pattern() {
        let pattern = StripePattern {
            a: Colour::white(),
            b: Colour::black()
        };

        assert_eq!(pattern.a, Colour::white());
        assert_eq!(pattern.b, Colour::black());
    }

    #[test]
    fn stripe_pattern_is_constant_in_y() {
        let pattern = StripePattern {
            a: Colour::white(),
            b: Colour::black()
        };

        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 0.0)), Colour::white());
        assert_eq!(pattern.pattern_at(point(0.0, 1.0, 0.0)), Colour::white());
        assert_eq!(pattern.pattern_at(point(0.0, 2.0, 0.0)), Colour::white());
    }

    #[test]
    fn stripe_pattern_is_constant_in_z() {
        let pattern = StripePattern {
            a: Colour::white(),
            b: Colour::black()
        };

        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 0.0)), Colour::white());
        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 1.0)), Colour::white());
        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 2.0)), Colour::white());
    }

    #[test]
    fn stripe_pattern_alternates_in_x() {
        let pattern = StripePattern {
            a: Colour::white(),
            b: Colour::black()
        };

        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 0.0)), Colour::white());
        assert_eq!(pattern.pattern_at(point(0.9, 0.0, 0.0)), Colour::white());
        assert_eq!(pattern.pattern_at(point(1.0, 0.0, 0.0)), Colour::black());
        assert_eq!(pattern.pattern_at(point(-0.1, 0.0, 0.0)), Colour::black());
        assert_eq!(pattern.pattern_at(point(-1.0, 0.0, 0.0)), Colour::black());
        assert_eq!(pattern.pattern_at(point(-1.1, 0.0, 0.0)), Colour::white());
    }

    #[test]
    fn stripes_with_object_transformation() {
        let object = Object::default()
            .with_transform(Matrix4::uscale(2.0));
        let pattern = Pattern::new_stripes(Colour::white(), Colour::black());

        assert_eq!(pattern.pattern_at_object(object, point(1.5, 0.0, 0.0)), Colour::white());
    }

    #[test]
    fn stripes_with_pattern_transformation() {
        let object = Object::default();
        let pattern = Pattern::new_stripes(Colour::white(), Colour::black())
            .with_transform(Matrix4::uscale(2.0));

        assert_eq!(pattern.pattern_at_object(object, point(1.5, 0.0, 0.0)), Colour::white())
    }

    #[test]
    fn stripes_with_both_object_and_pattern_transformation() {
        let object = Object::default()
            .with_transform(Matrix4::uscale(2.0));
        let pattern = Pattern::new_stripes(Colour::white(), Colour::black())
            .with_transform(Matrix4::translate(0.5, 0.0, 0.0));

        assert_eq!(pattern.pattern_at_object(object, point(2.5, 0.0, 0.0)), Colour::white());
    }

    #[test]
    fn default_pattern_transformation() {
        let pattern = Pattern::new_test();

        assert_eq!(pattern.transform, Matrix4::identity());
    }

    #[test]
    fn assigning_transform() {
        let pattern = Pattern::new_test()
            .with_transform(Matrix4::translate(1.0, 2.0, 3.0));

        assert_eq!(pattern.transform, Matrix4::translate(1.0, 2.0, 3.0));
    }

    #[test]
    fn pattern_with_object_transformation() {
        let object = Object::default()
            .with_transform(Matrix4::uscale(2.0));
        let pattern = Pattern::new_test();

        assert_eq!(pattern.pattern_at_object(object, point(2.0, 3.0, 4.0)), Colour::new(1.0, 1.5, 2.0));
    }

    #[test]
    fn pattern_with_pattern_transformation() {
        let object = Object::default();
        let pattern = Pattern::new_test()
            .with_transform(Matrix4::uscale(2.0));

        assert_eq!(pattern.pattern_at_object(object, point(2.0, 3.0, 4.0)), Colour::new(1.0, 1.5, 2.0));
    }

    #[test]
    fn pattern_with_both_object_and_pattern_transformations() {
        let object = Object::default()
            .with_transform(Matrix4::uscale(2.0));
        let pattern = Pattern::new_test()
            .with_transform(Matrix4::translate(0.5, 1.0, 1.5));

        assert_eq!(pattern.pattern_at_object(object, point(2.5, 3.0, 3.5)), Colour::new(0.75, 0.5, 0.25));
    }

    #[test]
    fn gradient_linearly_interpolates_between_colours() {
        let pattern = Pattern::new_gradient(Colour::white(), Colour::black());

        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 0.0)), Colour::white());
        assert_eq!(pattern.pattern_at(point(0.25, 0.0, 0.0)), Colour::grey(0.75));
        assert_eq!(pattern.pattern_at(point(0.5, 0.0, 0.0)), Colour::grey(0.5));
        assert_eq!(pattern.pattern_at(point(0.75, 0.0, 0.0)), Colour::grey(0.25));
    }

    #[test]
    fn ring_should_extend_in_both_x_and_z() {
        let pattern = Pattern::new_rings(Colour::white(), Colour::black());

        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 0.0)), Colour::white());
        assert_eq!(pattern.pattern_at(point(1.0, 0.0, 0.0)), Colour::black());
        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 1.0)), Colour::black());
        assert_eq!(pattern.pattern_at(point(0.708, 0.0, 0.708)), Colour::black());
    }

    #[test]
    fn checkers_should_repeat_in_x() {
        let pattern = Pattern::new_checkers(Colour::white(), Colour::black());

        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 0.0)), Colour::white());
        assert_eq!(pattern.pattern_at(point(0.99, 0.0, 0.0)), Colour::white());
        assert_eq!(pattern.pattern_at(point(1.01, 0.0, 0.0)), Colour::black());
    }

    #[test]
    fn checkers_should_repeat_in_y() {
        let pattern = Pattern::new_checkers(Colour::white(), Colour::black());

        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 0.0)), Colour::white());
        assert_eq!(pattern.pattern_at(point(0.0, 0.99, 0.0)), Colour::white());
        assert_eq!(pattern.pattern_at(point(0.0, 1.01, 0.0)), Colour::black());
    }

    #[test]
    fn checkers_should_repeat_in_z() {
        let pattern = Pattern::new_checkers(Colour::white(), Colour::black());

        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 0.0)), Colour::white());
        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 0.99)), Colour::white());
        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 1.01)), Colour::black());
    }
}