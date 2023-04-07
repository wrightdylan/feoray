use approx::{AbsDiffEq, abs_diff_eq};
use num_traits::{Zero, One};
use std::ops::{Index, IndexMut, Mul, Add};
use crate::Tuple;


// Screw it! I think I'll just make a generic matrix builder since I don't know what's ahead in the book
#[derive(Debug, Clone, Copy)]
pub struct Matrix<T, const R: usize, const C: usize>([[T; C]; R]);

impl<T: Default + Copy, const R: usize, const C: usize> Matrix<T, R, C> {
    fn new(data: [[T; C]; R]) -> Self {
        Matrix(data)
    }

    fn transpose(&self) -> Matrix<T, C, R> {
        let mut result = [[T::default();R]; C];

        for i in 0..R {
            for j in 0..C {
                result[j][i] = self.0[i][j];
            }
        }

        Matrix::new(result)
    }
}

#[derive(Debug)]
pub struct Matrix2<T>(Matrix<T, 2, 2>);
pub struct Matrix3<T>(Matrix<T, 3, 3>);
pub struct Matrix4<T>(Matrix<T, 4, 4>);

impl<T: Default + Copy> Matrix2<T> {
    pub fn new(
        m11: T, m12: T,
        m21: T, m22: T
    ) -> Matrix<T, 2, 2> {
        Matrix::new([
            [m11, m12],
            [m21, m22]
        ])
    }
}

impl<T: Default + Copy> Matrix3<T> {
    pub fn new(
        m11: T, m12: T, m13: T,
        m21: T, m22: T, m23: T,
        m31: T, m32: T, m33: T
    ) -> Matrix<T, 3, 3> {
        Matrix::new([
            [m11, m12, m13],
            [m21, m22, m23],
            [m31, m32, m33]
        ])
    }
}

impl<T: Default + Copy + Zero + One> Matrix4<T> {
    pub fn new(
        m11: T, m12: T, m13: T, m14: T,
        m21: T, m22: T, m23: T, m24: T,
        m31: T, m32: T, m33: T, m34: T,
        m41: T, m42: T, m43: T, m44: T
    ) -> Matrix<T, 4, 4> {
        Matrix::new([
            [m11, m12, m13, m14],
            [m21, m22, m23, m24],
            [m31, m32, m33, m34],
            [m41, m42, m43, m44]
        ])
    }

    pub fn identity() -> Matrix<T, 4, 4> {
        Matrix::new([
            [T::one(), T::zero(), T::zero(), T::zero()],
            [T::zero(), T::one(), T::zero(), T::zero()],
            [T::zero(), T::zero(), T::one(), T::zero()],
            [T::zero(), T::zero(), T::zero(), T::one()]
        ])
    }
}

// What a dog's breakfast! Why did I try to make this so generic? The amount of time spent doing this...
// I think I may have to refactor and simplify this later. I believe most calculations will be f64 anyway.
impl<T: Mul<Output = T> + Default + Copy +Add<Output = T>> Mul<Matrix<T, 4, 4>> for Matrix<T, 4, 4> {
    type Output = Matrix<T, 4, 4>;

    fn mul(self, rhs: Matrix<T, 4, 4>) -> Self::Output {
        &self * &rhs
    }
}

impl<T, const R: usize, const C: usize> Index<(usize, usize)> for Matrix<T, R, C> {
    type Output = T;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.0[row][col]
    }
}

impl<T, const R: usize, const C: usize> IndexMut<(usize, usize)> for Matrix<T, R, C> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self.0[row][col]
    }
}

impl<T: PartialEq + AbsDiffEq<Epsilon = T>, const R: usize, const C: usize> PartialEq for Matrix<T, R, C> {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..R {
            for j in 0..C {
                if !abs_diff_eq!(self.0[i][j], other.0[i][j]) {
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

impl<T: Eq + AbsDiffEq<Epsilon = T>, const R: usize, const C: usize> Eq for Matrix<T, R, C> {}

impl<T: Mul<Output = T> + Default + Copy + Add<Output = T>, const R: usize, const C: usize, const RC: usize> 
Mul<&Matrix<T, C, RC>> for &Matrix<T, R, C> {
    type Output = Matrix<T, R, RC>;

    fn mul(self, rhs: &Matrix<T, C, RC>) -> Self::Output {
        let mut result = [[T::default(); RC]; R];

        for i in 0..R {
            for j in 0..RC {
                for k in 0..C {
                    result[i][j] = result[i][j] + self.0[i][k] * rhs.0[k][j];
                }
            }
        }

        Matrix::new(result)
    }
}

// Look at this bollocks! I definitely need to remove generics. This is too much of a headache
impl<T: Default + Copy + Mul<Output = T> + Add<Output = T> + PartialEq + PartialOrd +
From<f64>> Mul<Tuple> for Matrix<T, 4, 4> 
where T: Mul<Output = T> + Add<Output = T> + From<f64> + Into<f64>
{
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        let x = self.0[0][0] * rhs.x.into() + self.0[0][1] * rhs.y.into() + self.0[0][2] * rhs.z.into() + self.0[0][3] * rhs.w.into();
        let y = self.0[1][0] * rhs.x.into() + self.0[1][1] * rhs.y.into() + self.0[1][2] * rhs.z.into() + self.0[1][3] * rhs.w.into();
        let z = self.0[2][0] * rhs.x.into() + self.0[2][1] * rhs.y.into() + self.0[2][2] * rhs.z.into() + self.0[2][3] * rhs.w.into();
        let w = self.0[3][0] * rhs.x.into() + self.0[3][1] * rhs.y.into() + self.0[3][2] * rhs.z.into() + self.0[3][3] * rhs.w.into();

        Tuple { x: x.into(), y: y.into(), z: z.into(), w: w.into()}
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
        let i = Matrix4::identity();

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
}