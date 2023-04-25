use approx::abs_diff_eq;
use std::ops::{Index, IndexMut, Mul};
use crate::Tuple;

#[derive(Debug, Clone)]
pub struct Matrix {
    pub data: Vec<f64>,
    pub size: usize
}

// Functions for matrices of all sizes.
impl Matrix {
    /// Returns the cofactor of a given matrix element.
    /// Intended for use on 3x3 and 4x4 matrices.
    /// 
    /// # Example
    /// 
    /// ```ignore
    /// let m: Matrix = Matrix4::new(
    ///     -5.0, 2.0, 6.0, -8.0,
    ///     1.0, -5.0, 1.0, 8.0,
    ///     7.0, 7.0, -6.0, -7.0,
    ///     1.0, -3.0, 7.0, 4.0
    /// );
    ///
    /// assert_eq!(m.cofactor(2, 3), -160.0);
    /// assert_eq!(m.cofactor(3, 2), 105.0);
    /// ```
    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        let cf;
        if self.size == 4 {
            cf = self.subm(row, col).det3();
        } else {
            cf = self.subm(row, col).det2();
        }
        if (row + col) %2 == 0 {
            cf
        } else {
            -cf
        }
    }

    /// Fills a matrix of any size with the `fill` along the matrix's diagonal.
    /// Intended to be used on an empty matrix. Mostly useful for creating an identity.
    fn fill_diagonal(mut self, fill: f64) -> Self {
        for i in 0..self.size {
            self[(i, i)] = fill;
        }
        self
    }

    /// Returns the submatrix of a given matrix by subtracting the given row and column.
    /// Mostly useful for calculating cofactors.
    fn subm(&self, row: usize, col: usize) -> Self {
        let sub_size = self.size - 1;
        let mut sub_data = Vec::new();

        for i in 0..self.size {
            if i == row {
                continue;
            }
            for j in 0..self.size {
                if j == col {
                    continue;
                }
                sub_data.push(self.data[i * self.size + j]);
            }
        }
        
        Self { data: sub_data, size: sub_size }
    }

    // Only used in tests
    #[allow(dead_code)]
    fn to_5dp(&self) -> Self {
        let mut res: Vec<f64> = Vec::new();
        for i in 0..self.data.len() {
            res.push(((self.data[i] * 100000.0).round()) / 100000.0);
        }
        Matrix { data: res, size: self.size }
    }

    /// Transpose transposes.
    /// A waste of time applying to an identity.
    pub fn transpose(&self) -> Self {
        let mut result = vec![0.0; self.size * self.size];

        for i in 0..self.size {
            for j in 0..self.size {
                result[j * self.size + i] = self.data[i * self.size + j];
            }
        }
        Self { data: result, size: self.size }
    }
}

trait Matrix2 {
    fn new(m11: f64, m12: f64, m21: f64, m22: f64) -> Self;
    fn det2(&self) -> f64;
    fn empty() -> Self;
}
trait Matrix3 {
    fn new(
        m11: f64, m12: f64, m13: f64,
        m21: f64, m22: f64, m23: f64,
        m31: f64, m32: f64, m33: f64
    ) -> Self;
    fn det3(&self) -> f64;
    fn empty() -> Self;
    fn minor(&self, row: usize, col: usize) -> f64;
}
pub trait Matrix4 {
    fn new(
        m11: f64, m12: f64, m13: f64, m14: f64,
        m21: f64, m22: f64, m23: f64, m24: f64,
        m31: f64, m32: f64, m33: f64, m34: f64,
        m41: f64, m42: f64, m43: f64, m44: f64
    ) -> Self;
    fn det4(&self) -> f64;
    fn empty() -> Self;
    fn id() -> Self;
    fn inverse(&self) -> Self;
    fn is_invertible(&self) -> bool;
}

// Functions specific to Matrix size 2x2.
impl Matrix2 for Matrix {
    fn new(m11: f64, m12: f64, m21: f64, m22: f64) -> Self {
        Self {
            data: vec![m11, m12, m21, m22],
            size: 2
        }
    }

    fn det2(&self) -> f64 {
        assert_eq!(self.size, 2, "Matrix must be 2x2");
        self.data[0] * self.data[3] - self.data[1] * self.data[2]
    }

    fn empty() -> Self {
        Self { data: vec![0.0; 4], size: 2 }
    }
}

// Functions specific to Matrix size 3x3.
impl Matrix3 for Matrix {
    fn new(
        m11: f64, m12: f64, m13: f64,
        m21: f64, m22: f64, m23: f64,
        m31: f64, m32: f64, m33: f64
    ) -> Self {
        Self {
            data: vec![
                m11, m12, m13,
                m21, m22, m23,
                m31, m32, m33
            ],
            size: 3
        }
    }

    fn det3(&self) -> f64 {
        self[(0, 0)] * self.cofactor(0, 0) +
        self[(0, 1)] * self.cofactor(0, 1) +
        self[(0, 2)] * self.cofactor(0, 2)
    }

    fn empty() -> Self {
        Self { data: vec![0.0; 9], size: 3 }
    }

    fn minor(&self, row: usize, col: usize) -> f64 {
        self.subm(row, col).det2()
    }
}

// Functions specific to Matrix size 4x4.
impl Matrix4 for Matrix {
    fn new(
        m11: f64, m12: f64, m13: f64, m14: f64,
        m21: f64, m22: f64, m23: f64, m24: f64,
        m31: f64, m32: f64, m33: f64, m34: f64,
        m41: f64, m42: f64, m43: f64, m44: f64
    ) -> Self {
        Self {
            data: vec![
                m11, m12, m13, m14,
                m21, m22, m23, m24,
                m31, m32, m33, m34,
                m41, m42, m43, m44
            ],
            size: 4
        }
    }

    // Computationally expensive.
    // subm().det3() is the minor of a 4x4 matrix, not the cofactor
    fn det4(&self) -> f64 {
        self[(0, 0)] * self.subm(0, 0).det3() -
        self[(0, 1)] * self.subm(0, 1).det3() + 
        self[(0, 2)] * self.subm(0, 2).det3() -
        self[(0, 3)] * self.subm(0, 3).det3()
    }

    fn empty() -> Self {
        Self { data: vec![0.0; 16], size: 4 }
    }

    fn id() -> Self {
        <Matrix as Matrix4>::empty().fill_diagonal(1.0)
    }

    fn inverse(&self) -> Self {
        let det = self.det4();
        if det == 0.0 {
            panic!("Not inversible.")
        } else {
            let mut invm: Matrix = Matrix4::empty();
            for row in 0..4 {
                for col in 0..4 {
                    invm[(col, row)] = self.cofactor(row, col) / det;
                }
            }
            invm
        }
    }

    // Useful only for tests
    fn is_invertible(&self) -> bool {
        self.det4() != 0.0
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = f64;
    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.data[row * self.size + col]
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self.data[row * self.size + col]
    }
}

// Multiply two matrices of same size.
impl Mul for Matrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.size, rhs.size, "Columns in first matrix must equal rows in second matrix");
        let mut result = Self {
            data: vec![0.0; self.size * self.size],
            size: self.size
        };

        for i in 0..self.size {
            for j in 0..self.size {
                let mut sum = 0.0;
                for k in 0..self.size {
                    sum += self[(i, k)] * rhs[(k, j)];
                }
                result[(i, j)] = sum;
            }
        }

        result
    }
}

// Only useful to pass tests. Bit of a problem somewhere, but this is likely due
// to the inability to derive the Copy trait on the Matrix struct
impl Mul for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.size, rhs.size, "Columns in first matrix must equal rows in second matrix");
        let mut result = Matrix {
            data: vec![0.0; self.size * self.size],
            size: self.size
        };

        for i in 0..self.size {
            for j in 0..self.size {
                let mut sum = 0.0;
                for k in 0..self.size {
                    sum += self[(i, k)] * rhs[(k, j)];
                }
                result[(i, j)] = sum;
            }
        }

        result
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        if self.size != other.size {
            return false;
        }

        for i in 0..self.data.len() {
            if !abs_diff_eq!(self.data[i], other.data[i]) {
                return false;
            }
        }

        true
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl Mul<Tuple> for Matrix
{
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        let mut result = Tuple::new(0.0, 0.0, 0.0, 0.0);

        for i in 0..4 {
            let mut sum = 0.0;
            for j in 0..4 {
                sum += self[(i,j)] * match j {
                    0 => rhs.x,
                    1 => rhs.y,
                    2 => rhs.z,
                    3 => rhs.w,
                    _ => unreachable!()
                };
            }
            match i {
                0 => result.x = sum,
                1 => result.y = sum,
                2 => result.z = sum,
                3 => result.w = sum,
                _ => unreachable!()
            };
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_2x2_matrix() {
        let m: Matrix = Matrix2::new(
            -3.0, 5.0,
            1.0, -2.0
        );

        assert_eq!((m[(0, 0)], m[(0, 1)], m[(1, 0)], m[(1, 1)]), (-3.0, 5.0, 1.0, -2.0))
    }

    #[test]
    fn create_3x3_matrix() {
        let m: Matrix = Matrix3::new(
            -3.0, 5.0, 0.0,
            1.0, -2.0, -7.0,
            0.0, 1.0, 1.0
        );

        assert_eq!((m[(0, 0)], m[(1, 1)], m[(2, 2)]), (-3.0, -2.0, 1.0));
    }

    #[test]
    fn matrix_equality_identical() {
        let m1: Matrix = Matrix4::new(
            1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 8.0, 7.0, 6.0,
            5.0, 4.0, 3.0, 2.0
        );
        let m2: Matrix = Matrix4::new(
            1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 8.0, 7.0, 6.0,
            5.0, 4.0, 3.0, 2.0
        );

        assert_eq!(m1, m2);
    }

    #[test]
    fn matrix_equality_different() {
        let m1: Matrix = Matrix4::new(
            1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 8.0, 7.0, 6.0,
            5.0, 4.0, 3.0, 2.0
        );
        let m2: Matrix = Matrix4::new(
            2.0, 3.0, 4.0, 5.0,
            6.0, 7.0, 8.0, 9.0,
            8.0, 7.0, 6.0, 5.0,
            4.0, 3.0, 2.0, 1.0
        );

        assert_ne!(m1, m2);
    }

    #[test]
    fn multiplying_two_matrices() {
        let m1: Matrix = Matrix4::new(
            1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 8.0, 7.0, 6.0,
            5.0, 4.0, 3.0, 2.0
        );
        let m2: Matrix = Matrix4::new(
            -2.0, 1.0, 2.0, 3.0,
            3.0, 2.0, 1.0, -1.0,
            4.0, 3.0, 6.0, 5.0,
            1.0, 2.0, 7.0, 8.0
        );
        let mdp: Matrix = Matrix4::new(
            20.0, 22.0, 50.0, 48.0,
            44.0, 54.0, 114.0, 108.0,
            40.0, 58.0, 110.0, 102.0,
            16.0, 26.0, 46.0, 42.0
        );

        assert_eq!(m1 * m2, mdp);
    }

    #[test]
    fn matrix_multiplied_by_tuple() {
        let m: Matrix = Matrix4::new(
            1.0, 2.0, 3.0, 4.0,
            2.0, 4.0, 4.0, 2.0,
            8.0, 6.0, 4.0, 1.0,
            0.0, 0.0, 0.0, 1.0
        );
        let t = Tuple::new(
            1.0, 2.0, 3.0, 1.0
        );
        let tdp = Tuple::new(
            18.0, 24.0, 33.0, 1.0
        );

        assert_eq!(m * t, tdp);
    }

    #[test]
    fn multiply_matrix_by_identity() {
        let m: Matrix = Matrix4::new(
            0.0, 1.0, 2.0, 4.0,
            1.0, 2.0, 4.0, 8.0,
            2.0, 4.0, 8.0, 16.0,
            4.0, 8.0, 16.0, 32.0
        );
        let i: Matrix = Matrix4::id();

        assert_eq!(&m * &i, m);
    }

    #[test]
    fn transpose_matrix() {
        let m: Matrix = Matrix4::new(
            0.0, 9.0, 3.0, 0.0,
            9.0, 8.0, 0.0, 8.0,
            1.0, 8.0, 5.0, 3.0,
            0.0, 0.0, 5.0, 8.0
        );
        let t: Matrix = Matrix4::new(
            0.0, 9.0, 1.0, 0.0,
            9.0, 8.0, 8.0, 0.0,
            3.0, 0.0, 5.0, 5.0,
            0.0, 8.0, 3.0, 8.0
        );

        assert_eq!(m.transpose(), t);
    }

    #[test]
    fn determinant_of_2x2() {
        // Function only available for 2x2 matrix
        let m: Matrix = Matrix2::new(
            1.0, 5.0,
            -3.0, 2.0
        );

        assert_eq!(m.det2(), 17.0);
    }

    #[test]
    fn submatrix_of_3x3() {
        let m: Matrix = Matrix3::new(
            1.0, 5.0, 0.0,
            -3.0, 2.0, 7.0,
            0.0, 6.0, -3.0
        );
        let s: Matrix = Matrix2::new(
            -3.0, 2.0,
            0.0, 6.0
        );

        assert_eq!(m.subm(0, 2), s);
    }

    #[test]
    fn submatrix_of_4x4() {
        let m: Matrix = Matrix4::new(
            -6.0, 1.0, 1.0, 6.0,
            -8.0, 5.0, 8.0, 6.0,
            -1.0, 0.0, 8.0, 2.0,
            -7.0, 1.0, -1.0, 1.0
        );
        let s: Matrix = Matrix3::new(
            -6.0, 1.0, 6.0,
            -8.0, 8.0, 6.0,
            -7.0, -1.0, 1.0
        );

        assert_eq!(m.subm(2, 1), s);
    }

    #[test]
    fn ssm_and_det_of_4x4() {
        let m: Matrix = Matrix4::new(
            -6.0, 1.0, 1.0, 6.0,
            -8.0, 5.0, 8.0, 6.0,
            -1.0, 0.0, 8.0, 2.0,
            -7.0, 1.0, -1.0, 1.0
        );
        let s: Matrix = Matrix2::new(
            1.0, 6.0,
            -1.0, 1.0
        );

        assert_eq!(m.subm(2, 1).subm(1, 0), s);
        assert_eq!(m.subm(2, 1).subm(1, 0).det2(), 7.0);
    }

    #[test]
    fn minor_of_3x3() {
        let m: Matrix = Matrix3::new(
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
    }

    #[test]
    fn determinant_of_3x3() {
        let m: Matrix = Matrix3::new(
            1.0, 2.0, 6.0,
            -5.0, 8.0, -4.0,
            2.0, 6.0, 4.0
        );

        assert_eq!(m.cofactor(0, 0), 56.0);
        assert_eq!(m.cofactor(0, 1), 12.0);
        assert_eq!(m.cofactor(0, 2), -46.0);
        assert_eq!(m.det3(), -196.0);
    }

    #[test]
    fn determinant_of_4x4() {
        let m: Matrix = Matrix4::new(
            -2.0, -8.0, 3.0, 5.0,
            -3.0, 1.0, 7.0, 3.0,
            1.0, 2.0, -9.0, 6.0,
            -6.0, 7.0, 7.0, -9.0
        );

        assert_eq!(m.cofactor(0, 0), 690.0);
        assert_eq!(m.cofactor(0, 1), 447.0);
        assert_eq!(m.cofactor(0, 2), 210.0);
        assert_eq!(m.cofactor(0, 3), 51.0);
        assert_eq!(m.det4(), -4071.0);
    }

    #[test]
    fn is_invertible() {
        let m: Matrix = Matrix4::new(
            6.0, 4.0, 4.0, 4.0,
            5.0, 5.0, 7.0, 6.0,
            4.0, -9.0, 3.0, -7.0,
            9.0, 1.0, 7.0, -6.0
        );

        assert_eq!(m.det4(), -2120.0);
        assert_eq!(m.is_invertible(), true);
    }

    #[test]
    fn is_not_invertible() {
        let m: Matrix = Matrix4::new(
            -4.0, 2.0, -2.0, -3.0,
            9.0, 6.0, 2.0, 6.0,
            0.0, -5.0, 1.0, -5.0,
            0.0, 0.0, 0.0, 0.0
        );

        assert_eq!(m.det4(), 0.0);
        assert_eq!(m.is_invertible(), false);
    }

    // Manually verified. Test result has greater precision than expected result.
    #[test]
    fn invert_matrix1() {
        let m: Matrix = Matrix4::new(
            -5.0, 2.0, 6.0, -8.0,
            1.0, -5.0, 1.0, 8.0,
            7.0, 7.0, -6.0, -7.0,
            1.0, -3.0, 7.0, 4.0
        );
        let i: Matrix = Matrix4::new(
            0.21805, 0.45113, 0.24060, -0.04511,
            -0.80827, -1.45677, -0.44361, 0.52068,
            -0.07895, -0.22368, -0.05263, 0.19737,
            -0.52256, -0.81391, -0.30075, 0.30639
        );

        assert_eq!(m.cofactor(2, 3), -160.0);
        assert_eq!(m.cofactor(3, 2), 105.0);
        assert_eq!(m.inverse().to_5dp(), i);
    }

    // Manually verified. Test result has greater precision than expected result.
    #[test]
    fn invert_matrix2() {
        let m: Matrix = Matrix4::new(
            8.0, -5.0, 9.0, 2.0,
            7.0, 5.0, 6.0, 1.0,
            -6.0, 0.0, 9.0, 6.0,
            -3.0, 0.0, -9.0, -4.0
        );
        let i: Matrix = Matrix4::new(
            -0.15385, -0.15385, -0.28205, -0.53846,
            -0.07692, 0.12308, 0.02564, 0.03077,
            0.35897, 0.35897, 0.43590, 0.92308,
            -0.69231, -0.69231, -0.76923, -1.92308
        );

        assert_eq!(m.inverse().to_5dp(), i);
    }

    // Manually verified. Test result has greater precision than expected result.
    #[test]
    fn invert_matrix3() {
        let m: Matrix = Matrix4::new(
            9.0, 3.0, 0.0, 9.0,
            -5.0, -2.0, -6.0, -3.0,
            -4.0, 9.0, 6.0, 4.0,
            -7.0, 6.0, 6.0, 2.0
        );
        let i: Matrix = Matrix4::new(
            -0.04074, -0.07778, 0.14444, -0.22222,
            -0.07778, 0.03333, 0.36667, -0.33333,
            -0.02901, -0.14630, -0.10926, 0.12963,
            0.17778, 0.06667, -0.26667, 0.33333
        );

        assert_eq!(m.inverse().to_5dp(), i);
    }

    // Manually verified. Test result has greater precision than expected result.
    #[test]
    fn multiply_product_by_inverse() {
        let m: Matrix = Matrix4::new(
            3.0, -9.0, 7.0, 3.0,
            3.0, -8.0, 2.0, -9.0,
            -4.0, 4.0, 4.0, 1.0,
            -6.0, 5.0, -1.0, 1.0
        );
        let n: Matrix = Matrix4::new(
            8.0, 2.0, 2.0, 2.0,
            3.0, -1.0, 7.0, 0.0,
            7.0, 0.0, 5.0, 4.0,
            6.0, -2.0, 0.0, 5.0
        );

        assert_eq!(((&m * &n) * n.inverse()).to_5dp(), m);
    }
}