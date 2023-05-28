use crate::primitives::Object;
use nalgebra::Vector4;

#[derive(Debug)]
pub struct PreCompData {
    pub t: f64,
    pub object: Object,
    pub pos: Vector4<f64>,
    pub over_pos: Vector4<f64>,
    pub under_pos: Vector4<f64>,
    pub eye_vec: Vector4<f64>,
    pub n1: f32,
    pub n2: f32,
    pub normal_vec: Vector4<f64>,
    pub reflect_vec: Vector4<f64>,
    pub inside: bool
}

impl PreCompData {
    pub fn new(
        t: f64,
        object: Object,
        pos: Vector4<f64>,
        over_pos: Vector4<f64>,
        under_pos: Vector4<f64>,
        eye_vec: Vector4<f64>,
        n1: f32,
        n2: f32,
        normal_vec: Vector4<f64>,
        reflect_vec: Vector4<f64>,
        inside: bool
    ) -> Self {
        Self {
            t,
            object,
            pos,
            over_pos,
            under_pos,
            eye_vec,
            n1,
            n2,
            normal_vec,
            reflect_vec,
            inside
        }
    }

    /// Schlick approximation of the Fresnel effect.
    pub fn schlick(&self) -> f64 {
        let mut cos = self.normal_vec.dot(&self.eye_vec);

        if self.n1 > self.n2 {
            let n_ratio = (self.n1 / self.n2) as f64;
            let sin2_t = n_ratio.powi(2) * (1.0 - cos.powi(2));
            if sin2_t > 1.0 {
                return 1.0;
            }

            cos = (1.0 - sin2_t).sqrt();
        }

        let r0 = ((self.n1 - self.n2) / (self.n1 + self.n2)).powi(2) as f64;
        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }
}

#[cfg(test)]
mod tests {
    use crate::core::{point, vector, Intersection, Intersections, Ray};
    use crate::primitives::Object;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn schlick_approximation_under_total_internal_reflection() {
        let object = Object::glass_orb();
        let irr_no = 2.0f64.sqrt() / 2.0;
        let ray = Ray::new(point(0.0, 0.0, irr_no), vector(0.0, 1.0, 0.0));
        let xs = Intersections::new(vec![
            Intersection::new(-irr_no, object),
            Intersection::new(irr_no, object)
        ]);
        let comps = xs.prepare_computations(1, &ray);

        assert_eq!(comps.schlick(), 1.0);
    }

    #[test]
    fn schlick_approximation_with_perpendicular_viewing_angle() {
        let object = Object::glass_orb();
        let ray = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 1.0, 0.0));
        let xs = Intersections::new(vec![
            Intersection::new(-1.0, object),
            Intersection::new(1.0, object)
        ]);
        let comps = xs.prepare_computations(1, &ray);

        assert_approx_eq!(comps.schlick(), 0.04);
    }

    #[test]
    fn schlick_approximation_with_small_angle_and_n2_gt_n1() {
        let object = Object::glass_orb();
        let ray = Ray::new(point(0.0, 0.99, -2.0), vector(0.0, 0.0, 1.0));
        let xs = Intersections::new(vec![
            Intersection::new(1.8589, object)
        ]);
        let comps = xs.prepare_computations(0, &ray);

        assert_approx_eq!(comps.schlick(), 0.48873);
    }
}