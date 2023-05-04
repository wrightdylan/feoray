use crate::core::Colour;
use nalgebra::Vector4;

// I use the term colour as that makes more sense than intensity which sounds
// more like a scale of colour.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PointLight {
    pub colour: Colour,
    pub position: Vector4<f64>
}

impl PointLight {
    pub fn new(colour: Colour, position: Vector4<f64>) -> Self {
        PointLight { colour, position }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::point;

    #[test]
    fn point_light_has_position_and_intensity() {
        let c = Colour::white();
        let p = point(0.0, 0.0, 0.0);
        let light = PointLight::new(c, p);

        assert_eq!(light.position, p);
        assert_eq!(light.colour, c);
    }
}