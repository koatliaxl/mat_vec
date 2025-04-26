use crate::{Matrix4x4, Vector4};
use std::ops::{Add, AddAssign, Mul};

macro_rules! impl_vec_mul {
    ($L:ty) => {
        impl<T> Mul<Vector4<T>> for $L
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
    };
}

impl_vec_mul!(Matrix4x4<T>);
impl_vec_mul!(&Matrix4x4<T>);
impl_vec_mul!(&mut Matrix4x4<T>);
