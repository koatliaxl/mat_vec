use crate::{Matrix4x4, Vector4};
use std::ops::{Add, AddAssign, Mul};

impl<T> Mul<Vector4<T>> for Matrix4x4<T>
where
    T: Copy + Add<Output = T> + Mul<Output = T> + AddAssign + Default,
{
    type Output = Vector4<T>;

    fn mul(self, rhs: Vector4<T>) -> Self::Output {
        let mut result = [T::default(); 4];
        for r in 0..4 {
            for c in 0..4 {
                result[r] += self[(r, c)] * rhs[c]
            }
        }
        Vector4::from_array(result)
    }
}

impl<T> Mul<Vector4<T>> for &Matrix4x4<T>
where
    T: Copy + Add<Output = T> + Mul<Output = T> + AddAssign + Default,
{
    type Output = Vector4<T>;

    fn mul(self, rhs: Vector4<T>) -> Self::Output {
        let mut result = [T::default(); 4];
        for r in 0..4 {
            for c in 0..4 {
                result[r] += self[(r, c)] * rhs[c]
            }
        }
        Vector4::from_array(result)
    }
}
