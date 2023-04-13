#![allow(unused)]
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Colour {
    pub r: f32,
    pub g: f32,
    pub b: f32
}

impl Colour {
    /// Constructs a new Colour container.
    /// Values cover red, green, blue.
    /// 
    /// # Example
    /// 
    /// ```ignore
    /// let c = Colour::new(-0.5, 0.4, 1.7);
    /// 
    /// assert_eq!((c.r, c.g, c.b), (-0.5, 0.4, 1.7));
    /// ```
    pub fn new( r: f32, g: f32, b:f32 ) -> Self {
        Colour { r, g, b }
    }

    /// Scales and converts f32 colours to u8.
    /// Clamps min and max values between 0.0 and 1.0 before converting.
    /// Always gives values between 0 and 255.
    /// 
    /// # Example
    /// 
    /// ```ignore
    /// let c = Colour::new(-0.5, 0.4, 1.7);
    /// 
    /// assert_eq!(c.scale(), (0, 102, 255));
    /// ```
    pub fn scale(&self) -> (u8, u8, u8) {
        (
            scale_channel(self.r),
            scale_channel(self.g),
            scale_channel(self.b),
        )
    }

    /// Predefined screen colour
    pub fn red() -> Self {
        Colour { r: 1.0, g: 0.0, b: 0.0 }
    }

    /// Predefined screen colour
    pub fn green() -> Self {
        Colour { r: 0.0, g: 1.0, b: 0.0 }
    }

    /// Predefined screen colour
    pub fn blue() -> Self {
        Colour { r: 0.0, g: 0.0, b: 0.1 }
    }

    /// Predefined screen colour
    pub fn white() -> Self {
        Colour { r: 1.0, g: 1.0, b: 1.0 }
    }

    /// Predefined print colour
    pub fn cyan() -> Self {
        Colour { r: 0.0, g: 1.0, b: 1.0 }
    }

    /// Predefined print colour
    pub fn magenta() -> Self {
        Colour { r: 1.0, g: 0.0, b: 1.0 }
    }

    /// Predefined print colour
    pub fn yellow() -> Self {
        Colour { r: 1.0, g: 1.0, b: 0.0 }
    }

    /// Predefined print colour
    pub fn black() -> Self {
        Colour { r: 0.0, g: 0.0, b: 0.0 }
    }
}

impl Add for Colour {
    type Output = Colour;

    fn add(self, rhs: Colour) -> Self::Output {
        Colour {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b
        }
    }
}

impl Sub for Colour {
    type Output = Colour;

    fn sub(self, rhs: Colour) -> Self::Output {
        Colour {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b
        }
    }
}

impl Mul<Colour> for Colour {
    type Output = Colour;

    fn mul(self, rhs: Colour) -> Self::Output {
        Colour {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b
        }
    }
}

impl Mul<f32> for Colour {
    type Output = Colour;

    fn mul(self, rhs: f32) -> Self::Output {
        Colour {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs
        }
    }
}

impl PartialEq for Colour {
    fn eq(&self, other: &Colour) -> bool {
        self.r == other.r
            && self.g == other.g
            && self.b == other.b
    }
}

pub fn colour(r: f32, g: f32, b: f32) -> Colour {
    Colour::new(r, g, b)
}

fn scale_channel(channel: f32) -> u8 {
    let channel = if channel < 0.0 {
        0.0
    } else if channel > 1.0 {
        1.0
    } else {
        channel
    };

    (channel * 255.0) as u8
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_colour_struct() {
        let c = Colour::new(-0.5, 0.4, 1.7);
        assert_approx_eq!(c.r, -0.5);
        assert_approx_eq!(c.g, 0.4);
        assert_approx_eq!(c.b, 1.7);
    }

    #[test]
    fn add_colours() {
        let c1 = colour(0.9, 0.6, 0.75);
        let c2 = colour(0.7, 0.1, 0.25);
        let c = c1 + c2;

        assert_approx_eq!(c.r, 1.6);
        assert_approx_eq!(c.g, 0.7);
        assert_approx_eq!(c.b, 1.0);
    }

    #[test]
    fn sub_colours() {
        let c1 = colour(0.9, 0.6, 0.75);
        let c2 = colour(0.7, 0.1, 0.25);
        let c = c1 - c2;

        assert_approx_eq!(c.r, 0.2);
        assert_approx_eq!(c.g, 0.5);
        assert_approx_eq!(c.b, 0.5);
    }

    #[test]
    fn multiply_colour_by_scalar() {
        let c = colour(0.2, 0.3, 0.4);

        assert_eq!(c * 2.0, colour(0.4, 0.6, 0.8));
    }

    #[test]
    fn multiply_colour_by_colour() {
        let c1 = colour(1.0, 0.2, 0.4);
        let c2 = colour(0.9, 1.0, 0.1);
        let c = c1 * c2;

        assert_approx_eq!(c.r, 0.9);
        assert_approx_eq!(c.g, 0.2);
        assert_approx_eq!(c.b, 0.04);
    }

    #[test]
    fn scale_colour() {
        let c = Colour::new(-0.5, 0.4, 1.7);

        assert_eq!(c.scale(), (0, 102, 255));
    }
}