use nalgebra::Matrix4;
// Previous iterations of matrix.rs (i.e. pre-nalgebra refactoring) can be
// found in the archive folder. This file only contains tests now.

// Only for tests
pub trait Test {
    fn to_5dp(&self) -> Matrix4<f64>;
}

impl Test for Matrix4<f64> {
    fn to_5dp(&self) -> Self {
        let mut res = Matrix4::zeros();
        for i in 0..4 {
            for j in 0..4 {
                res[(i, j)] = (self[(i, j)] * 100000.0).round() / 100000.0;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::{Matrix2, Matrix3, Vector4};
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn create_2x2_matrix() {
        let m = Matrix2::new(
            -3.0, 5.0,
            1.0, -2.0
        );

        assert_eq!((m.m11, m.m12, m.m21, m.m22), (-3.0, 5.0, 1.0, -2.0));
    }

    #[test]
    fn create_3x3_matrix() {
        let m = Matrix3::new(
            -3.0, 5.0, 0.0,
            1.0, -2.0, -7.0,
            0.0, 1.0, 1.0
        );

        assert_eq!((m.m11, m.m22, m.m33), (-3.0, -2.0, 1.0));
    }

    #[test]
    fn matrix_equality_identical() {
        let m1 = Matrix4::new(
            1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 8.0, 7.0, 6.0,
            5.0, 4.0, 3.0, 2.0
        );
        let m2 = Matrix4::new(
            1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 8.0, 7.0, 6.0,
            5.0, 4.0, 3.0, 2.0
        );

        assert_eq!(m1, m2);
    }

    #[test]
    fn matrix_equality_different() {
        let m1 = Matrix4::new(
            1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 8.0, 7.0, 6.0,
            5.0, 4.0, 3.0, 2.0
        );
        let m2 = Matrix4::new(
            2.0, 3.0, 4.0, 5.0,
            6.0, 7.0, 8.0, 9.0,
            8.0, 7.0, 6.0, 5.0,
            4.0, 3.0, 2.0, 1.0
        );

        assert_ne!(m1, m2);
    }

    #[test]
    fn multiplying_two_matrices() {
        let m1 = Matrix4::new(
            1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 8.0, 7.0, 6.0,
            5.0, 4.0, 3.0, 2.0
        );
        let m2 = Matrix4::new(
            -2.0, 1.0, 2.0, 3.0,
            3.0, 2.0, 1.0, -1.0,
            4.0, 3.0, 6.0, 5.0,
            1.0, 2.0, 7.0, 8.0
        );
        let mdp = Matrix4::new(
            20.0, 22.0, 50.0, 48.0,
            44.0, 54.0, 114.0, 108.0,
            40.0, 58.0, 110.0, 102.0,
            16.0, 26.0, 46.0, 42.0
        );

        assert_eq!(m1 * m2, mdp);
    }

    #[test]
    fn matrix_multiplied_by_tuple1() {
        let m = Matrix4::new(
            1.0, 2.0, 3.0, 4.0,
            2.0, 4.0, 4.0, 2.0,
            8.0, 6.0, 4.0, 1.0,
            0.0, 0.0, 0.0, 1.0
        );
        let v = Vector4::new(
            1.0, 2.0, 3.0, 1.0
        );
        let vdp = Vector4::new(
            18.0, 24.0, 33.0, 1.0
        );

        assert_eq!(m * v, vdp);
    }

    #[test]
    fn multiply_matrix_by_identity() {
        let m = Matrix4::new(
            0.0, 1.0, 2.0, 4.0,
            1.0, 2.0, 4.0, 8.0,
            2.0, 4.0, 8.0, 16.0,
            4.0, 8.0, 16.0, 32.0
        );
        let i = Matrix4::identity();

        // And behold, this uses the Copy trait!
        assert_eq!(m * i, m);
    }

    #[test]
    fn transpose_matrix() {
        let m = Matrix4::new(
            0.0, 9.0, 3.0, 0.0,
            9.0, 8.0, 0.0, 8.0,
            1.0, 8.0, 5.0, 3.0,
            0.0, 0.0, 5.0, 8.0
        );
        let t = Matrix4::new(
            0.0, 9.0, 1.0, 0.0,
            9.0, 8.0, 8.0, 0.0,
            3.0, 0.0, 5.0, 5.0,
            0.0, 8.0, 3.0, 8.0
        );

        assert_eq!(m.transpose(), t);
    }

    #[test]
    fn determinant_of_2x2() {
        let m = Matrix2::new(
            1.0, 5.0,
            -3.0, 2.0
        );

        assert_eq!(m.determinant(), 17.0);
    }

    // These tests are for only for my implementation of matrices which required
    // submatrices, cofactors, and minors. I am now using an optimised matrix
    // library instead.
    /*#[test]
    fn submatrix_of_3x3() {
        let m = Matrix3::new(
            1.0, 5.0, 0.0,
            -3.0, 2.0, 7.0,
            0.0, 6.0, -3.0
        );
        let s = Matrix2::new(
            -3.0, 2.0,
            0.0, 6.0
        );

        assert_eq!(m.subm(0, 2), s);
    }

    #[test]
    fn submatrix_of_4x4() {
        let m = Matrix4::new(
            -6.0, 1.0, 1.0, 6.0,
            -8.0, 5.0, 8.0, 6.0,
            -1.0, 0.0, 8.0, 2.0,
            -7.0, 1.0, -1.0, 1.0
        );
        let s = Matrix3::new(
            -6.0, 1.0, 6.0,
            -8.0, 8.0, 6.0,
            -7.0, -1.0, 1.0
        );

        assert_eq!(m.subm(2, 1), s);
    }

    #[test]
    fn ssm_and_det_of_4x4() {
        let m = Matrix4::new(
            -6.0, 1.0, 1.0, 6.0,
            -8.0, 5.0, 8.0, 6.0,
            -1.0, 0.0, 8.0, 2.0,
            -7.0, 1.0, -1.0, 1.0
        );
        let s = Matrix2::new(
            1.0, 6.0,
            -1.0, 1.0
        );

        assert_eq!(m.subm(2, 1).subm(1, 0), s);
        assert_eq!(m.subm(2, 1).subm(1, 0).determinant(), 7.0);
    }

    #[test]
    fn minor_of_3x3() {
        let m = Matrix3::new(
            3.0, 5.0, 0.0,
            2.0, -1.0, -7.0,
            6.0, -1.0, 5.0
        );

        assert_eq!(m.minor(1, 0), 25.0);
    }

    #[test]
    fn cofactor_of_3x3() {
        let m: Matrix = Matrix3::new(
            3.0, 5.0, 0.0,
            2.0, -1.0, -7.0,
            6.0, -1.0, 5.0
        );

        assert_eq!(m.minor(0, 0), -12.0);
        assert_eq!(m.cofactor(0, 0), -12.0);
        assert_eq!(m.minor(1, 0), 25.0);
        assert_eq!(m.cofactor(1, 0), -25.0);
    }*/

    #[test]
    fn determinant_of_3x3() {
        let m = Matrix3::new(
            1.0, 2.0, 6.0,
            -5.0, 8.0, -4.0,
            2.0, 6.0, 4.0
        );

        //assert_eq!(m.cofactor(0, 0), 56.0);
        //assert_eq!(m.cofactor(0, 1), 12.0);
        //assert_eq!(m.cofactor(0, 2), -46.0);
        assert_eq!(m.determinant(), -196.0);
    }

    #[test]
    fn determinant_of_4x4() {
        let m = Matrix4::new(
            -2.0, -8.0, 3.0, 5.0,
            -3.0, 1.0, 7.0, 3.0,
            1.0, 2.0, -9.0, 6.0,
            -6.0, 7.0, 7.0, -9.0
        );

        //assert_eq!(m.cofactor(0, 0), 690.0);
        //assert_eq!(m.cofactor(0, 1), 447.0);
        //assert_eq!(m.cofactor(0, 2), 210.0);
        //assert_eq!(m.cofactor(0, 3), 51.0);
        //assert_eq!(m.determinant(), -4071.0);
        assert_approx_eq!(m.determinant() as f64, -4071.0);
    }

    #[test]
    fn is_invertible() {
        let m = Matrix4::new(
            6.0, 4.0, 4.0, 4.0,
            5.0, 5.0, 7.0, 6.0,
            4.0, -9.0, 3.0, -7.0,
            9.0, 1.0, 7.0, -6.0
        );

        assert_eq!(m.determinant(), -2120.0);
        assert_eq!(m.is_invertible(), true);
    }

    #[test]
    fn is_not_invertible() {
        let m = Matrix4::new(
            -4.0, 2.0, -2.0, -3.0,
            9.0, 6.0, 2.0, 6.0,
            0.0, -5.0, 1.0, -5.0,
            0.0, 0.0, 0.0, 0.0
        );

        assert_eq!(m.determinant(), 0.0);
        assert_eq!(m.is_invertible(), false);
    }

    // Manually verified. Test result has greater precision than expected result.
    #[test]
    fn invert_matrix1() {
        let m = Matrix4::new(
            -5.0, 2.0, 6.0, -8.0,
            1.0, -5.0, 1.0, 8.0,
            7.0, 7.0, -6.0, -7.0,
            1.0, -3.0, 7.0, 4.0
        );
        let i = Matrix4::new(
            0.21805, 0.45113, 0.24060, -0.04511,
            -0.80827, -1.45677, -0.44361, 0.52068,
            -0.07895, -0.22368, -0.05263, 0.19737,
            -0.52256, -0.81391, -0.30075, 0.30639
        );

        //assert_eq!(m.cofactor(2, 3), -160.0);
        //assert_eq!(m.cofactor(3, 2), 105.0);
        assert_eq!(m.try_inverse().unwrap().to_5dp(), i);
    }

    // Manually verified. Test result has greater precision than expected result.
    #[test]
    fn invert_matrix2() {
        let m = Matrix4::new(
            8.0, -5.0, 9.0, 2.0,
            7.0, 5.0, 6.0, 1.0,
            -6.0, 0.0, 9.0, 6.0,
            -3.0, 0.0, -9.0, -4.0
        );
        let i = Matrix4::new(
            -0.15385, -0.15385, -0.28205, -0.53846,
            -0.07692, 0.12308, 0.02564, 0.03077,
            0.35897, 0.35897, 0.43590, 0.92308,
            -0.69231, -0.69231, -0.76923, -1.92308
        );

        assert_eq!(m.try_inverse().unwrap().to_5dp(), i);
    }

    // Manually verified. Test result has greater precision than expected result.
    #[test]
    fn invert_matrix3() {
        let m = Matrix4::new(
            9.0, 3.0, 0.0, 9.0,
            -5.0, -2.0, -6.0, -3.0,
            -4.0, 9.0, 6.0, 4.0,
            -7.0, 6.0, 6.0, 2.0
        );
        let i = Matrix4::new(
            -0.04074, -0.07778, 0.14444, -0.22222,
            -0.07778, 0.03333, 0.36667, -0.33333,
            -0.02901, -0.14630, -0.10926, 0.12963,
            0.17778, 0.06667, -0.26667, 0.33333
        );

        assert_eq!(m.try_inverse().unwrap().to_5dp(), i);
    }

    // Manually verified. Test result has greater precision than expected result.
    #[test]
    fn multiply_product_by_inverse() {
        let m = Matrix4::new(
            3.0, -9.0, 7.0, 3.0,
            3.0, -8.0, 2.0, -9.0,
            -4.0, 4.0, 4.0, 1.0,
            -6.0, 5.0, -1.0, 1.0
        );
        let n = Matrix4::new(
            8.0, 2.0, 2.0, 2.0,
            3.0, -1.0, 7.0, 0.0,
            7.0, 0.0, 5.0, 4.0,
            6.0, -2.0, 0.0, 5.0
        );

        assert_eq!(((m * n) * n.try_inverse().unwrap()).to_5dp(), m);
    }
}