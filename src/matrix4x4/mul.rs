use super::Matrix4x4;
use std::ops::{Add, AddAssign, Mul};

impl<T> Mul for Matrix4x4<T>
where
    T: Copy + Add<Output = T> + Mul<Output = T> + AddAssign + Default,
{
    type Output = Matrix4x4<T>;

    fn mul(self, rhs: Matrix4x4<T>) -> Self::Output {
        let mut arr = [[T::default(); 4]; 4];
        for r in 0..4 {
            for c in 0..4 {
                for i in 0..4 {
                    arr[r][c] += self[(r, i)] * rhs[(i, c)]
                }
            }
        }
        Matrix4x4::from_array(arr)
    }
}

impl<T> Mul<&Matrix4x4<T>> for Matrix4x4<T>
where
    T: Copy + Add<Output = T> + Mul<Output = T> + AddAssign + Default,
{
    type Output = Matrix4x4<T>;

    fn mul(self, rhs: &Matrix4x4<T>) -> Self::Output {
        let mut arr = [[T::default(); 4]; 4];
        for r in 0..4 {
            for c in 0..4 {
                for i in 0..4 {
                    arr[r][c] += self[(r, i)] * rhs[(i, c)]
                }
            }
        }
        Matrix4x4::from_array(arr)
    }
}

impl<T> Mul<Matrix4x4<T>> for &Matrix4x4<T>
where
    T: Copy + Add<Output = T> + Mul<Output = T> + AddAssign + Default,
{
    type Output = Matrix4x4<T>;

    fn mul(self, rhs: Matrix4x4<T>) -> Self::Output {
        let mut arr = [[T::default(); 4]; 4];
        for r in 0..4 {
            for c in 0..4 {
                for i in 0..4 {
                    arr[r][c] += self[(r, i)] * rhs[(i, c)]
                }
            }
        }
        Matrix4x4::from_array(arr)
    }
}

impl<T> Mul for &Matrix4x4<T>
where
    T: Copy + Add<Output = T> + Mul<Output = T> + AddAssign + Default,
{
    type Output = Matrix4x4<T>;

    fn mul(self, rhs: &Matrix4x4<T>) -> Self::Output {
        let mut arr = [[T::default(); 4]; 4];
        for r in 0..4 {
            for c in 0..4 {
                for i in 0..4 {
                    arr[r][c] += self[(r, i)] * rhs[(i, c)]
                }
            }
        }
        Matrix4x4::from_array(arr)
    }
}
