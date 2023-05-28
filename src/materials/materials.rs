use super::Pattern;
use crate::core::{Colour, Tuple};
use crate::lights::PointLight;
use crate::primitives::Object;
use nalgebra::Vector4;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Material {
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub smoothness: f32,
    pub reflectivity: f32,
    pub transparency: f32,
    pub ior: f32,
    pub pattern: Pattern
}

impl Material {
    pub fn new(
        ambient: f32,
        diffuse: f32,
        specular: f32,
        smoothness: f32,
        reflectivity: f32,
        transparency: f32,
        ior: f32,
        pattern: Pattern
    ) -> Self {
        Material {
            ambient,
            diffuse,
            specular,
            smoothness,
            reflectivity,
            transparency,
            ior,
            pattern
        }
    }

    pub fn null() -> Self {
        Material {
            ambient: 0.0,
            diffuse: 0.0,
            specular: 0.0,
            smoothness: 255.0,
            reflectivity: 0.0,
            transparency: 0.0,
            ior: 1.0,
            pattern: Pattern::new_solid(Colour::white())
        }
    }

    /// Assigns ambient value
    pub fn with_ambient(mut self, ambient: f32) -> Self {
        self.ambient = ambient;

        self
    }

    /// Assigns solid colour to material
    pub fn with_colour(mut self, colour: Colour) -> Self {
        self.pattern = Pattern::new_solid(colour);

        self
    }

    /// Assigns diffuse value
    pub fn with_diffuse(mut self, diffuse: f32) -> Self {
        self.diffuse = diffuse;

        self
    }

    /// Assigns index of refraction
    pub fn with_ior(mut self, ior: f32) -> Self {
        self.ior = ior;

        self
    }

    /// Applies a pattern (including solid colour)
    pub fn with_pattern(mut self, pattern: Pattern) -> Self {
        self.pattern = pattern;

        self
    }

    /// Assigns reflectivity
    pub fn with_reflectivity(mut self, reflectivity: f32) -> Self {
        self.reflectivity = reflectivity;

        self
    }

    /// Assigns smoothness (aka shininess)
    pub fn with_smoothness(mut self, smoothness: f32) -> Self {
        self.smoothness = smoothness;

        self
    }

    /// Assigns specularity
    pub fn with_specular(mut self, specular: f32) -> Self {
        self.specular = specular;

        self
    }

    /// Assigns transparency
    pub fn with_transparency(mut self, transparency: f32) -> Self {
        self.transparency = transparency;

        self
    }

    pub fn lighting(
        &self,
        object: Object,
        light: PointLight,
        pos: Vector4<f64>,
        eye_vec: Vector4<f64>,
        normal_vec: Vector4<f64>,
        shadow: bool
    ) -> Colour {
        let colour = self.pattern.pattern_at_object(object, pos);
        let eff_colour = colour * light.colour;
        let light_vec = (light.position - pos).normalize();
        let ambient = eff_colour * self.ambient;
        let light_dot_normal = light_vec.dot(&normal_vec);
        let (mut diffuse, mut specular) = (Colour::black(), Colour::black());
        if light_dot_normal >= 0.0 {
            diffuse = eff_colour * self.diffuse * light_dot_normal;
            let reflect_vec = (-light_vec).reflect(normal_vec);
            let reflect_dot_eye = reflect_vec.dot(&eye_vec);
            if reflect_dot_eye <= 0.0 {
                specular = Colour::black();
            } else {
                let factor = reflect_dot_eye.powf(self.smoothness.into());
                specular = light.colour * self.specular * factor;
            }
        }
        
        ambient + if shadow {Colour::black()} else {diffuse + specular}
    }
}

impl Default for Material {
    fn default() -> Self {
        Material {
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            smoothness: 200.0,
            reflectivity: 0.0,
            transparency: 0.0,
            ior: 1.0,
            pattern: Pattern::new_solid(Colour::white())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{point, vector};

    #[test]
    fn default_material() {
        let m = Material::default();

        assert_eq!(m.pattern.pattern_at_object(Object::default(), point(0.0, 0.0, 0.0)), Colour::white());
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
        let shadow = false;
        let res = m.lighting(Object::default(), light, pos, eyev, normal, shadow);

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
        let res = m.lighting(Object::default(), light, pos, eyev, normal, false);

        assert_eq!(res, Colour::white());
    }

    #[test]
    fn lighting_with_eye_opposite_surface_light_offset_45d() {
        let m = Material::default();
        let pos = point(0.0, 0.0, 0.0);
        let eyev = vector(0.0, 0.0, -1.0);
        let normal = vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Colour::white(), point(0.0, 10.0, -10.0));
        let res = m.lighting(Object::default(), light, pos, eyev, normal, false);

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
        let res = m.lighting(Object::default(), light, pos, eyev, normal, false);

        assert_eq!(res.to_5dp(), Colour::new(1.63640, 1.63640, 1.63640));
    }

    #[test]
    fn lighting_with_light_behind_the_surface() {
        let m = Material::default();
        let pos = point(0.0, 0.0, 0.0);
        let eyev = vector(0.0, 0.0, -1.0);
        let normal = vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Colour::white(), point(0.0, 0.0, 10.0));
        let res = m.lighting(Object::default(), light, pos, eyev, normal, false);

        assert_eq!(res, Colour::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn lighting_with_surface_in_shadow() {
        let m = Material::default();
        let pos = point(0.0, 0.0, 0.0);
        let eyev = vector(0.0, 0.0, -1.0);
        let normal = vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Colour::white(), point(0.0, 0.0, -10.0));
        let res = m.lighting(Object::default(), light, pos, eyev, normal, true);

        assert_eq!(res, Colour::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn lightng_with_pattern_applied() {
        let pattern = Pattern::new_stripes(Colour::white(), Colour::black());
        let m = Material::default()
            .with_ambient(1.0)
            .with_diffuse(0.0)
            .with_specular(0.0)
            .with_pattern(pattern);
        let eyev = vector(0.0, 0.0, -1.0);
        let normal = vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Colour::white(), point(0.9, 0.0, 0.0));
        let c1 = m.lighting(Object::default(), light, point(0.9, 0.0, 0.0), eyev, normal, true);
        let c2 = m.lighting(Object::default(), light, point(1.1, 0.0, 0.0), eyev, normal, true);

        assert_eq!(c1, Colour::white());
        assert_eq!(c2, Colour::black());
    }

    #[test]
    fn reflectivity_for_default_material() {
        let m = Material::default();

        assert_eq!(m.reflectivity, 0.0);
    }

    #[test]
    fn transparency_and_ior_for_default_material() {
        let m = Material::default();

        assert_eq!(m.transparency, 0.0);
        assert_eq!(m.ior, 1.0);
    }
}