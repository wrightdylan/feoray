// More than meets the eye
use crate::{Matrix, Matrix4};

impl Matrix {
    pub fn translate(x: f64, y: f64, z: f64) -> Matrix {
        let mut trm: Matrix = Matrix4::id();
        trm[(0, 3)] = x;
        trm[(1, 3)] = y;
        trm[(2, 3)] = z;
        trm
    }

    pub fn scale(x: f64, y: f64, z: f64) -> Matrix {
        let mut scm: Matrix = Matrix4::id();
        scm[(0, 0)] = x;
        scm[(1, 1)] = y;
        scm[(2, 2)] = z;
        scm
    }

    pub fn rot_x(rad: f64) -> Matrix {
        let mut rxm: Matrix = Matrix4::id();
        rxm[(1, 1)] = rad.cos();
        rxm[(1, 2)] = -rad.sin();
        rxm[(2, 1)] = rad.sin();
        rxm[(2, 2)] = rad.cos();
        rxm
    }

    pub fn rot_y(rad: f64) -> Matrix {
        let mut rym: Matrix = Matrix4::id();
        rym[(0, 0)] = rad.cos();
        rym[(0, 2)] = rad.sin();
        rym[(2, 0)] = -rad.sin();
        rym[(2, 2)] = rad.cos();
        rym
    }

    pub fn rot_z(rad: f64) -> Matrix {
        let mut rzm: Matrix = Matrix4::id();
        rzm[(0, 0)] = rad.cos();
        rzm[(0, 1)] = -rad.sin();
        rzm[(1, 0)] = rad.sin();
        rzm[(1, 1)] = rad.cos();
        rzm
    }

    pub fn shear(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix {
        let mut shm: Matrix = Matrix4::id();
        shm[(0, 1)] = xy;
        shm[(0, 2)] = xz;
        shm[(1, 0)] = yx;
        shm[(1, 2)] = yz;
        shm[(2, 0)] = zx;
        shm[(2, 1)] = zy;
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
        let m: Matrix = Matrix::translate(5.0, -3.0, 2.0);
        let p = point(-3.0, 4.0, 5.0);
        let r = point(2.0, 1.0,7.0);

        assert_eq!(m * p, r);
    }

    #[test]
    fn mul_by_inv_of_matrix() {
        let m: Matrix = Matrix::translate(5.0, -3.0, 2.0);
        let i = m.inverse();
        let p = point(-3.0, 4.0, 5.0);
        let r = point(-8.0, 7.0, 3.0);

        assert_eq!(i * p, r);
    }

    #[test]
    fn translation_not() {
        let m: Matrix = Matrix::translate(5.0, -3.0, 2.0);
        let v = vector(-3.0, 4.0, 5.0);

        assert_eq!(m * v, v);
    }

    #[test]
    fn scaling_point() {
        let m: Matrix = Matrix::scale(2.0, 3.0, 4.0);
        let p = point(-4.0, 6.0, 8.0);
        let r = point(-8.0, 18.0, 32.0);

        assert_eq!(m * p, r);
    }

    #[test]
    fn scaling_vector() {
        let m: Matrix = Matrix::scale(2.0, 3.0, 4.0);
        let v = vector(-4.0, 6.0, 8.0);
        let r = vector(-8.0, 18.0, 32.0);

        assert_eq!(m * v, r);
    }

    #[test]
    fn mul_inv_of_scaling_matrix() {
        let m: Matrix = Matrix::scale(2.0, 3.0, 4.0);
        let v = vector(-4.0, 6.0, 8.0);
        let r = vector(-2.0, 2.0, 2.0);

        assert_eq!(m.inverse() * v, r);
    }

    #[test]
    fn reflection_is_scaling() {
        let m: Matrix = Matrix::scale(-1.0, 1.0, 1.0);
        let p = point(2.0, 3.0, 4.0);
        let r = point(-2.0, 3.0, 4.0);

        assert_eq!(m * p, r);
    }

    #[test]
    fn rotating_point_around_x() {
        let hq: Matrix = Matrix::rot_x(PI / 4.0);
        let fq: Matrix = Matrix::rot_x(PI / 2.0);
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
        let hq: Matrix = Matrix::rot_x(PI / 4.0);
        let p = point(0.0, 1.0, 0.0);
        let hqpi = hq.inverse() * p;

        assert_approx_eq!(hqpi.x, 0.0);
        assert_approx_eq!(hqpi.y, 2.0_f64.sqrt()/2.0);
        assert_approx_eq!(hqpi.z, -2.0_f64.sqrt()/2.0);
    }

    #[test]
    fn rotating_point_around_y() {
        let hq: Matrix = Matrix::rot_y(PI / 4.0);
        let fq: Matrix = Matrix::rot_y(PI / 2.0);
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
        let hq: Matrix = Matrix::rot_z(PI / 4.0);
        let fq: Matrix = Matrix::rot_z(PI / 2.0);
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
        let m: Matrix = Matrix::shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);
        let r = point(5.0, 3.0, 4.0);

        assert_eq!(m * p, r);
    }

    #[test]
    fn shear_moves_x_ipt_z() {
        let m: Matrix = Matrix::shear(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);
        let r = point(6.0, 3.0, 4.0);

        assert_eq!(m * p, r);
    }

    #[test]
    fn shear_moves_y_ipt_x() {
        let m: Matrix = Matrix::shear(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);
        let r = point(2.0, 5.0, 4.0);

        assert_eq!(m * p, r);
    }

    #[test]
    fn shear_moves_y_ipt_z() {
        let m: Matrix = Matrix::shear(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);
        let r = point(2.0, 7.0, 4.0);

        assert_eq!(m * p, r);
    }

    #[test]
    fn shear_moves_z_ipt_x() {
        let m: Matrix = Matrix::shear(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = point(2.0, 3.0, 4.0);
        let r = point(2.0, 3.0, 6.0);

        assert_eq!(m * p, r);
    }

    #[test]
    fn shear_moves_z_ipt_y() {
        let m: Matrix = Matrix::shear(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = point(2.0, 3.0, 4.0);
        let r = point(2.0, 3.0, 7.0);

        assert_eq!(m * p, r);
    }

    #[test]
    fn individual_transforms_in_sequence() {
        let p = point(1.0, 0.0, 1.0);
        let a: Matrix = Matrix::rot_x(PI / 2.0);
        let b: Matrix = Matrix::scale(5.0, 5.0, 5.0);
        let c: Matrix = Matrix::translate(10.0, 5.0, 7.0);

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
        let a: Matrix = Matrix::rot_x(PI / 2.0);
        let b: Matrix = Matrix::scale(5.0, 5.0, 5.0);
        let c: Matrix = Matrix::translate(10.0, 5.0, 7.0);

        let t = c * b * a;
        let r = t * p;

        assert_approx_eq!(r.x, 15.0);
        assert_approx_eq!(r.y, 0.0);
        assert_approx_eq!(r.z, 7.0);
    }
}