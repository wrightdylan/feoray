use crate::{Colour, PointLight, Tuple};
use nalgebra::Vector4;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Material {
    pub colour: Colour,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub smoothness: f32 // Called shininess in the book
}
// Look to add transmission, ior, and metallic in the future

impl Material {
    pub fn new(
        colour: Colour,
        ambient: f32,
        diffuse: f32,
        specular: f32,
        smoothness: f32
    ) -> Self {
        Material {
            colour,
            ambient,
            diffuse,
            specular,
            smoothness
        }
    }

    pub fn with_colour(mut self, colour: Colour) -> Material {
        self.colour = colour;

        self
    }

    pub fn with_ambient(mut self, ambient: f32) -> Self {
        self.ambient = ambient;

        self
    }

    pub fn with_diffuse(mut self, diffuse: f32) -> Self {
        self.diffuse = diffuse;

        self
    }

    pub fn with_specular(mut self, specular: f32) -> Self {
        self.specular = specular;

        self
    }

    pub fn with_smoothness(mut self, smoothness: f32) -> Self {
        self.smoothness = smoothness;

        self
    }

    pub fn lighting(
        &self,
        light: PointLight,
        pos: Vector4<f64>,
        eyev: Vector4<f64>,
        normal: Vector4<f64>
    ) -> Colour {
        let eff_colour = self.colour * light.colour;
        let lightv = (light.position - pos).normalize();
        let ambient = eff_colour * self.ambient;
        let light_dot_normal = lightv.dot(&normal);
        let (mut diffuse, mut specular) = (Colour::black(), Colour::black());
        if light_dot_normal >= 0.0 {
            diffuse = eff_colour * self.diffuse * light_dot_normal;
            let reflectv = (-lightv).reflect(normal);
            let reflect_dot_eye = reflectv.dot(&eyev);
            if reflect_dot_eye <= 0.0 {
                specular = Colour::black();
            } else {
                let factor = reflect_dot_eye.powf(self.smoothness.into());
                specular = light.colour * self.specular * factor;
            }
        }
        ambient + diffuse + specular
    }
}

impl Default for Material {
    fn default() -> Self {
        Material {
            colour: Colour::white(),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            smoothness: 200.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{point, vector};

    #[test]
    fn default_material() {
        let m = Material::default();

        assert_eq!(m.colour, Colour::white());
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.smoothness, 200.0);
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface() {
        let m = Material::default();
        let pos = point(0.0, 0.0, 0.0);
        let eyev = vector(0.0, 0.0, -1.0);
        let normal = vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Colour::white(), point(0.0, 0.0, -10.0));
        let res = m.lighting(light, pos, eyev, normal);

        assert_eq!(res, Colour::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface_eye_offset_45d() {
        let m = Material::default();
        let pos =point(0.0, 0.0, 0.0);
        let irr_no = 2.0f64.sqrt() / 2.0;
        let eyev = vector(0.0, irr_no, -irr_no);
        let normal = vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Colour::white(), point(0.0, 0.0, -10.0));
        let res = m.lighting(light, pos, eyev, normal);

        assert_eq!(res, Colour::white());
    }

    #[test]
    fn lighting_with_eye_opposite_surface_light_offset_45d() {
        let m = Material::default();
        let pos = point(0.0, 0.0, 0.0);
        let eyev = vector(0.0, 0.0, -1.0);
        let normal = vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Colour::white(), point(0.0, 10.0, -10.0));
        let res = m.lighting(light, pos, eyev, normal);

        assert_eq!(res.to_5dp(), Colour::new(0.73640, 0.73640, 0.73640));
    }

    #[test]
    fn lighting_with_eye_in_path_of_reflection_vector() {
        let m = Material::default();
        let pos = point(0.0, 0.0, 0.0);
        let irr_no = 2.0f64.sqrt() / 2.0;
        let eyev = vector(0.0, -irr_no, -irr_no);
        let normal = vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Colour::white(), point(0.0, 10.0, -10.0));
        let res = m.lighting(light, pos, eyev, normal);

        assert_eq!(res.to_5dp(), Colour::new(1.63640, 1.63640, 1.63640));
    }

    #[test]
    fn lighting_with_light_behind_the_surface() {
        let m = Material::default();
        let pos = point(0.0, 0.0, 0.0);
        let eyev = vector(0.0, 0.0, -1.0);
        let normal = vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Colour::white(), point(0.0, 0.0, 10.0));
        let res = m.lighting(light, pos, eyev, normal);

        assert_eq!(res, Colour::new(0.1, 0.1, 0.1));
    }
}