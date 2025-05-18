use crate::Vector3;
use std::ops::{Div, Mul, MulAssign};

/// Scalar Multiplication
///
impl<T> Mul<T> for Vector3<T>
where
    T: Copy + Mul<Output = T>,
{
    type Output = Vector3<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vector3 {
            raw_data: [self.x() * rhs, self.y() * rhs, self.z() * rhs],
        }
    }
}
macro_rules! impl_scalar_mul {
    ($Scalar:ty, $VecElem:ty) => {
        impl Mul<$Scalar> for Vector3<$VecElem> {
            type Output = Vector3<$VecElem>;

            fn mul(self, rhs: $Scalar) -> Self::Output {
                self * (rhs as $VecElem)
            }
        }
    };
}
impl_scalar_mul!(i32, f32);
impl_scalar_mul!(i64, f32);
impl_scalar_mul!(u32, f32);
impl_scalar_mul!(u64, f32);
impl_scalar_mul!(f64, f32);
impl_scalar_mul!(usize, f32);
impl_scalar_mul!(isize, f32);
impl_scalar_mul!(i32, f64);
impl_scalar_mul!(i64, f64);
impl_scalar_mul!(u32, f64);
impl_scalar_mul!(u64, f64);
impl_scalar_mul!(f32, f64);
impl_scalar_mul!(usize, f64);
impl_scalar_mul!(isize, f64);

macro_rules! impl_scalar_mul_vec {
    ($Scalar:ty, $VecElem:ty) => {
        impl Mul<Vector3<$VecElem>> for $Scalar {
            type Output = Vector3<$VecElem>;

            fn mul(self, rhs: Vector3<$VecElem>) -> Self::Output {
                rhs * (self as $VecElem)
            }
        }
    };
    ($Scalar:ty) => {
        impl Mul<Vector3<$Scalar>> for $Scalar {
            type Output = Vector3<$Scalar>;

            fn mul(self, rhs: Vector3<$Scalar>) -> Self::Output {
                rhs * self
            }
        }
    };
}
impl_scalar_mul_vec!(f32);
impl_scalar_mul_vec!(f64, f32);
impl_scalar_mul_vec!(i32, f32);
impl_scalar_mul_vec!(i64, f32);
impl_scalar_mul_vec!(u32, f32);
impl_scalar_mul_vec!(u64, f32);
impl_scalar_mul_vec!(usize, f32);
impl_scalar_mul_vec!(isize, f32);
impl_scalar_mul_vec!(f64);
impl_scalar_mul_vec!(f32, f64);
impl_scalar_mul_vec!(i32, f64);
impl_scalar_mul_vec!(i64, f64);
impl_scalar_mul_vec!(u32, f64);
impl_scalar_mul_vec!(u64, f64);
impl_scalar_mul_vec!(usize, f64);
impl_scalar_mul_vec!(isize, f64);

impl<T> MulAssign<T> for Vector3<T>
where
    T: Copy + Mul<Output = T>,
{
    fn mul_assign(&mut self, rhs: T) {
        self.raw_data = [self.x() * rhs, self.y() * rhs, self.z() * rhs];
    }
}
macro_rules! impl_mul_assign {
    ($vec:ty, $scalar:ty) => {
        impl MulAssign<$scalar> for Vector3<$vec> {
            fn mul_assign(&mut self, rhs: $scalar) {
                let m = rhs as $vec;
                self.raw_data = [self.x() * m, self.y() * m, self.z() * m];
            }
        }
    };
}

impl_mul_assign!(f32, i32);
impl_mul_assign!(f32, i64);
impl_mul_assign!(f32, u32);
impl_mul_assign!(f32, u64);
impl_mul_assign!(f32, usize);
impl_mul_assign!(f32, isize);
impl_mul_assign!(f32, f64);
impl_mul_assign!(f64, i32);
impl_mul_assign!(f64, i64);
impl_mul_assign!(f64, u32);
impl_mul_assign!(f64, u64);
impl_mul_assign!(f64, usize);
impl_mul_assign!(f64, isize);
impl_mul_assign!(f64, f32);

/// Division by scalar
///
impl<T> Div<T> for Vector3<T>
where
    T: Copy + Div<Output = T>,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Vector3 {
            raw_data: [self.x() / rhs, self.y() / rhs, self.z() / rhs],
        }
    }
}

macro_rules! impl_scalar_div {
    ($elem:ty, $scalar:ty) => {
        impl Div<$scalar> for Vector3<$elem> {
            type Output = Self;

            fn div(self, rhs: $scalar) -> Self::Output {
                self / rhs as $elem
                /*let d = rhs as $elem;
                Vector3 {
                    raw_data: [self.x() / d, self.y() / d, self.z() / d],
                }*/
            }
        }
    };
}

impl_scalar_div!(f32, f64);
impl_scalar_div!(f32, i32);
impl_scalar_div!(f32, u32);
impl_scalar_div!(f32, i64);
impl_scalar_div!(f32, u64);
impl_scalar_div!(f32, isize);
impl_scalar_div!(f32, usize);
impl_scalar_div!(f64, f32);
impl_scalar_div!(f64, i32);
impl_scalar_div!(f64, u32);
impl_scalar_div!(f64, i64);
impl_scalar_div!(f64, u64);
impl_scalar_div!(f64, isize);
impl_scalar_div!(f64, usize);
