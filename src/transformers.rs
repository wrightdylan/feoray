// More than meets the eye
use nalgebra::{Matrix4, Vector3};

// Previous iterations of transformers.rs (i.e. pre-nalgebra refactoring) can be
// found in the archive folder. This version mostly adapts native nalgebra functionality
// to fit previous version.

pub trait Transform {
    fn translate(x: f64, y: f64, z: f64) -> Matrix4<f64>;
    fn nuscale(x: f64, y: f64, z: f64) -> Matrix4<f64>;
    fn uscale(s: f64) -> Matrix4<f64>;
    fn rot_x(rad: f64) -> Matrix4<f64>;
    fn rot_y(rad: f64) -> Matrix4<f64>;
    fn rot_z(rad: f64) -> Matrix4<f64>;
    fn shear(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix4<f64>;
}

impl Transform for Matrix4<f64> {
    fn translate(x: f64, y: f64, z: f64) -> Matrix4<f64> {
        Matrix4::new_translation(&Vector3::new(x, y, z))
    }

    fn nuscale(x: f64, y: f64, z: f64) -> Matrix4<f64> {
        Matrix4::new_nonuniform_scaling(&Vector3::new(x, y, z))
    }

    fn uscale(s: f64) -> Matrix4<f64> {
        Matrix4::new_scaling(s)
    }

    fn rot_x(rad: f64) -> Matrix4<f64> {
        Matrix4::new_rotation(Vector3::new(rad, 0.0, 0.0))
    }

    fn rot_y(rad: f64) -> Matrix4<f64> {
        Matrix4::new_rotation(Vector3::new(0.0, rad, 0.0))
    }

    fn rot_z(rad: f64) -> Matrix4<f64> {
        Matrix4::new_rotation(Vector3::new(0.0, 0.0, rad))
    }

    fn shear(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix4<f64> {
        let mut shm = Matrix4::identity();
        shm.m12 = xy;
        shm.m13 = xz;
        shm.m21 = yx;
        shm.m23 = yz;
        shm.m31 = zx;
        shm.m32 = zy;
        shm
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use super::*;
    use crate::{point, vector};
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn mul_by_translation_matrix() {
        let m = Matrix4::translate(5.0, -3.0, 2.0);
        let p = point(-3.0, 4.0, 5.0);
        let r = point(2.0, 1.0,7.0);

        assert_eq!(m * p, r);
    }

    #[test]
    fn mul_by_inv_of_matrix() {
        let m = Matrix4::translate(5.0, -3.0, 2.0);
        let i = m.try_inverse().unwrap();
        let p = point(-3.0, 4.0, 5.0);
        let r = point(-8.0, 7.0, 3.0);

        assert_eq!(i * p, r);
    }

    #[test]
    fn translation_not() {
        let m = Matrix4::translate(5.0, -3.0, 2.0);
        let v = vector(-3.0, 4.0, 5.0);

        assert_eq!(m * v, v);
    }

    #[test]
    fn scaling_point() {
        let m = Matrix4::nuscale(2.0, 3.0, 4.0);
        let p = point(-4.0, 6.0, 8.0);
        let r = point(-8.0, 18.0, 32.0);

        assert_eq!(m * p, r);
    }

    #[test]
    fn scaling_vector() {
        let m = Matrix4::nuscale(2.0, 3.0, 4.0);
        let v = vector(-4.0, 6.0, 8.0);
        let r = vector(-8.0, 18.0, 32.0);

        assert_eq!(m * v, r);
    }

    #[test]
    fn mul_inv_of_scaling_matrix() {
        let m = Matrix4::nuscale(2.0, 3.0, 4.0);
        let v = vector(-4.0, 6.0, 8.0);
        let r = vector(-2.0, 2.0, 2.0);

        assert_eq!(m.try_inverse().unwrap() * v, r);
    }

    #[test]
    fn reflection_is_scaling() {
        let m = Matrix4::nuscale(-1.0, 1.0, 1.0);
        let p = point(2.0, 3.0, 4.0);
        let r = point(-2.0, 3.0, 4.0);

        assert_eq!(m * p, r);
    }

    #[test]
    fn rotating_point_around_x() {
        let hq = Matrix4::rot_x(PI / 4.0);
        let fq = Matrix4::rot_x(PI / 2.0);
        let p = point(0.0, 1.0, 0.0);
        let hqp = hq * p;
        let fqp = fq * p;

        assert_approx_eq!(hqp.x, 0.0);
        assert_approx_eq!(hqp.y, 2.0_f64.sqrt()/2.0);
        assert_approx_eq!(hqp.z, 2.0_f64.sqrt()/2.0);
        assert_approx_eq!(fqp.x, 0.0);
        assert_approx_eq!(fqp.y, 0.0);
        assert_approx_eq!(fqp.z, 1.0);
    }

    #[test]
    fn inv_rotate_around_x() {
        let hq = Matrix4::rot_x(PI / 4.0);
        let p = point(0.0, 1.0, 0.0);
        let hqpi = hq.try_inverse().unwrap() * p;

        assert_approx_eq!(hqpi.x, 0.0);
        assert_approx_eq!(hqpi.y, 2.0_f64.sqrt()/2.0);
        assert_approx_eq!(hqpi.z, -2.0_f64.sqrt()/2.0);
    }

    #[test]
    fn rotating_point_around_y() {
        let hq = Matrix4::rot_y(PI / 4.0);
        let fq = Matrix4::rot_y(PI / 2.0);
        let p = point(0.0, 0.0, 1.0);
        let hqp = hq * p;
        let fqp = fq * p;

        assert_approx_eq!(hqp.x, 2.0_f64.sqrt()/2.0);
        assert_approx_eq!(hqp.y, 0.0);
        assert_approx_eq!(hqp.z, 2.0_f64.sqrt()/2.0);
        assert_approx_eq!(fqp.x, 1.0);
        assert_approx_eq!(fqp.y, 0.0);
        assert_approx_eq!(fqp.z, 0.0);
    }

    #[test]
    fn rotating_point_around_z() {
        let hq = Matrix4::rot_z(PI / 4.0);
        let fq = Matrix4::rot_z(PI / 2.0);
        let p = point(0.0, 1.0, 0.0);
        let hqp = hq * p;
        let fqp = fq * p;

        assert_approx_eq!(hqp.x, -2.0_f64.sqrt()/2.0);
        assert_approx_eq!(hqp.y, 2.0_f64.sqrt()/2.0);
        assert_approx_eq!(hqp.z, 0.0);
        assert_approx_eq!(fqp.x, -1.0);
        assert_approx_eq!(fqp.y, 0.0);
        assert_approx_eq!(fqp.z, 0.0);
    }

    #[test]
    fn shear_moves_x_ipt_y() {
        let m = Matrix4::shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);
        let r = point(5.0, 3.0, 4.0);

        assert_eq!(m * p, r);
    }

    #[test]
    fn shear_moves_x_ipt_z() {
        let m = Matrix4::shear(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);
        let r = point(6.0, 3.0, 4.0);

        assert_eq!(m * p, r);
    }

    #[test]
    fn shear_moves_y_ipt_x() {
        let m = Matrix4::shear(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);
        let r = point(2.0, 5.0, 4.0);

        assert_eq!(m * p, r);
    }

    #[test]
    fn shear_moves_y_ipt_z() {
        let m = Matrix4::shear(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);
        let r = point(2.0, 7.0, 4.0);

        assert_eq!(m * p, r);
    }

    #[test]
    fn shear_moves_z_ipt_x() {
        let m = Matrix4::shear(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = point(2.0, 3.0, 4.0);
        let r = point(2.0, 3.0, 6.0);

        assert_eq!(m * p, r);
    }

    #[test]
    fn shear_moves_z_ipt_y() {
        let m = Matrix4::shear(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = point(2.0, 3.0, 4.0);
        let r = point(2.0, 3.0, 7.0);

        assert_eq!(m * p, r);
    }

    #[test]
    fn individual_transforms_in_sequence() {
        let p = point(1.0, 0.0, 1.0);
        let a = Matrix4::rot_x(PI / 2.0);
        let b = Matrix4::nuscale(5.0, 5.0, 5.0);
        let c = Matrix4::translate(10.0, 5.0, 7.0);

        let p2 = a * p;
        assert_approx_eq!(p2.x, 1.0);
        assert_approx_eq!(p2.y, -1.0);
        assert_approx_eq!(p2.z, 0.0);

        let p3 = b * p2;
        assert_approx_eq!(p3.x, 5.0);
        assert_approx_eq!(p3.y, -5.0);
        assert_approx_eq!(p3.z, 0.0);

        let p4 = c * p3;
        assert_approx_eq!(p4.x, 15.0);
        assert_approx_eq!(p4.y, 0.0);
        assert_approx_eq!(p4.z, 7.0);
    }

    #[test]
    fn chained_transforms_ro() {
        let p = point(1.0, 0.0, 1.0);
        let a = Matrix4::rot_x(PI / 2.0);
        let b = Matrix4::nuscale(5.0, 5.0, 5.0);
        let c = Matrix4::translate(10.0, 5.0, 7.0);

        let t = c * b * a;
        let r = t * p;

        assert_approx_eq!(r.x, 15.0);
        assert_approx_eq!(r.y, 0.0);
        assert_approx_eq!(r.z, 7.0);
    }
}