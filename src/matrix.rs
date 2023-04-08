use approx::abs_diff_eq;
use std::ops::{Index, IndexMut, Mul};
use crate::Tuple;

#[derive(Debug, Clone, Copy)]
pub struct Matrix<const S: usize> {
    pub data: [[f64; S]; S]
}

// Functions for matrices of all sizes.
impl<const S: usize> Matrix<S> {
    pub fn new(data: [[f64; S]; S]) -> Self {
        Self { data }
    }

    pub fn identity() -> Self {
        let mut data = [[0.0; S]; S];
        for i in 0..S {
            data[i][i] = 1.0;
        }
        Self { data }
    }

    pub fn transpose(&self) -> Matrix<S> {
        let mut result = [[0.0; S]; S];

        for i in 0..S {
            for j in 0..S {
                result[j][i] = self.data[i][j];
            }
        }

        Matrix::new(result)
    }
}

#[derive(Debug)]
pub struct Matrix2(Matrix<2>);
pub struct Matrix3(Matrix<3>);
pub struct Matrix4(Matrix<4>);

// Generate a 2x2 matrix using Matrix<S>
impl Matrix2 {
    pub fn new(
        m11: f64, m12: f64,
        m21: f64, m22: f64
    ) -> Matrix<2> {
        Matrix::new([
            [m11, m12],
            [m21, m22]
        ])
    }
}

// Generate a 3x3 matrix using Matrix<S>
impl Matrix3 {
    pub fn new(
        m11: f64, m12: f64, m13: f64,
        m21: f64, m22: f64, m23: f64,
        m31: f64, m32: f64, m33: f64
    ) -> Matrix<3> {
        Matrix::new([
            [m11, m12, m13],
            [m21, m22, m23],
            [m31, m32, m33]
        ])
    }
}

// Generate a 4x4 matrix using Matrix<S>
impl Matrix4 {
    pub fn new(
        m11: f64, m12: f64, m13: f64, m14: f64,
        m21: f64, m22: f64, m23: f64, m24: f64,
        m31: f64, m32: f64, m33: f64, m34: f64,
        m41: f64, m42: f64, m43: f64, m44: f64
    ) -> Matrix<4> {
        Matrix::new([
            [m11, m12, m13, m14],
            [m21, m22, m23, m24],
            [m31, m32, m33, m34],
            [m41, m42, m43, m44]
        ])
    }
}

// Functions specific to Matrix size 2x2.
impl Matrix<2> {
    pub fn det(&self) -> f64 {
        self.data[0][0] * self.data[1][1] - self.data[0][1] * self.data[1][0]
    }
}

impl<const S: usize> Index<(usize, usize)> for Matrix<S> {
    type Output = f64;
    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.data[row][col]
    }
}

impl<const S: usize> IndexMut<(usize, usize)> for Matrix<S> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self.data[row][col]
    }
}

// Multiply two matrices of same size. Multiplying matrices of different sizes may cause problems.
impl<const S: usize> Mul for Matrix<S> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = Self::new([[0.0; S]; S] );

        for i in 0..S{
            for j in 0..S{
                let mut sum = 0.0;
                for k in 0..S {
                    sum += self[(i, k)] * rhs[(k, j)];
                }
                result[(i, j)] = sum;
            }
        }

        result
    }
}

impl<const S: usize> PartialEq for Matrix<S> {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..S {
            for j in 0..S {
                if !abs_diff_eq!(self.data[i][j], other.data[i][j]) {
                    return false;
                }
            }
        }
        true
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl<const S: usize> Eq for Matrix<S> {}

impl Mul<Tuple> for Matrix<4> 
{
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        let mut result = Tuple::new(0.0, 0.0, 0.0, 0.0);

        for i in 0..4 {
            let mut sum = 0.0;
            for j in 0..4 {
                sum += self.data[i][j] * match j {
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
        let m = Matrix2::new(
            -3.0, 5.0,
            1.0, -2.0
        );

        assert_eq!((m[(0, 0)], m[(0, 1)], m[(1, 0)], m[(1, 1)]), (-3.0, 5.0, 1.0, -2.0))
    }

    #[test]
    fn create_3x3_matrix() {
        let m = Matrix3::new(
            -3.0, 5.0, 0.0,
            1.0, -2.0, -7.0,
            0.0, 1.0, 1.0
        );

        assert_eq!((m[(0, 0)], m[(1, 1)], m[(2, 2)]), (-3.0, -2.0, 1.0));
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
    fn matrix_multiplied_by_tuple() {
        let m = Matrix4::new(
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
        let m = Matrix4::new(
            0.0, 1.0, 2.0, 4.0,
            1.0, 2.0, 4.0, 8.0,
            2.0, 4.0, 8.0, 16.0,
            4.0, 8.0, 16.0, 32.0
        );
        let i = Matrix::<4>::identity();

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
    fn find_determinant() {
        // Function only available for 2x2 matrix
        let m = Matrix2::new(
            1.0, 5.0,
            -3.0, 2.0
        );

        assert_eq!(m.det(), 17.0);
    }
}