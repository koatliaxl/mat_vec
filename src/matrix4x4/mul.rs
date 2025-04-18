use super::Matrix4x4;
use std::ops::{Add, AddAssign, Mul, MulAssign};

//todo impl MulAssign

macro_rules! multiply {
    ($L:ty, $R:ty) => {
        impl<T> Mul<$L> for $R
        where
            T: Copy + Add<Output = T> + Mul<Output = T> + AddAssign + Default,
        {
            type Output = Matrix4x4<T>;

            fn mul(self, rhs: $L) -> Self::Output {
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
    };
}
multiply!(Matrix4x4<T>, Matrix4x4<T>);
multiply!(Matrix4x4<T>, &Matrix4x4<T>);
multiply!(&Matrix4x4<T>, Matrix4x4<T>);
multiply!(&Matrix4x4<T>, &Matrix4x4<T>);
multiply!(Matrix4x4<T>, &mut Matrix4x4<T>);
multiply!(&mut Matrix4x4<T>, Matrix4x4<T>);
multiply!(&mut Matrix4x4<T>, &mut Matrix4x4<T>);
multiply!(&mut Matrix4x4<T>, &Matrix4x4<T>);
multiply!(&Matrix4x4<T>, &mut Matrix4x4<T>);

macro_rules! mul_assign {
    ($Rhs: ty) => {
        impl<T> MulAssign<$Rhs> for Matrix4x4<T>
        where
            T: Copy + Add<Output = T> + Mul<Output = T> + AddAssign + Default,
        {
            fn mul_assign(&mut self, rhs: $Rhs) {
                let mut arr = [[T::default(); 4]; 4];
                for r in 0..4 {
                    for c in 0..4 {
                        for i in 0..4 {
                            arr[r][c] += self[(r, i)] * rhs[(i, c)]
                        }
                    }
                }
                *self = Matrix4x4::from_array(arr);
            }
        }
    };
}
mul_assign!(Matrix4x4<T>);
mul_assign!(&Matrix4x4<T>);
mul_assign!(&mut Matrix4x4<T>);
