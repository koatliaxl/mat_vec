use crate::Vector3;
use num_traits::Float;
use std::ops::{Add, AddAssign, BitXor, Mul, Neg, Not, Rem, Sub, SubAssign};

impl<T> Vector3<T>
where
    T: Float,
{
    pub fn normalize(&self) -> Vector3<T> {
        let (x, y, z) = self.get_components();
        let length = (x * x + y * y + z * z).sqrt();
        Vector3 {
            raw_data: [x / length, y / length, z / length],
        }
    }
}
/// Normalization
impl<T> Not for Vector3<T>
where
    T: Float,
{
    type Output = Vector3<T>;

    fn not(self) -> Self::Output {
        self.normalize()
    }
}

impl<T> Neg for Vector3<T>
where
    T: Copy + Neg<Output = T>,
{
    type Output = Vector3<T>;

    fn neg(self) -> Self::Output {
        let (x, y, z) = self.get_components();
        Vector3 {
            raw_data: [-x, -y, -z],
        }
    }
}

impl<T> Add for Vector3<T>
where
    T: Copy + Add<Output = T>,
{
    type Output = Vector3<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3 {
            raw_data: [self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z()],
        }
    }
}

impl<T> Sub for Vector3<T>
where
    T: Copy + Sub<Output = T>,
{
    type Output = Vector3<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3 {
            raw_data: [self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z()],
        }
    }
}

impl<T> AddAssign for Vector3<T>
where
    T: Copy + Add<Output = T>,
{
    fn add_assign(&mut self, rhs: Self) {
        self.raw_data[0] = self.x() + rhs.x();
        self.raw_data[1] = self.y() + rhs.y();
        self.raw_data[2] = self.z() + rhs.z();
    }
}

impl<T> SubAssign for Vector3<T>
where
    T: Copy + Sub<Output = T>,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.raw_data[0] = self.x() - rhs.x();
        self.raw_data[1] = self.y() - rhs.y();
        self.raw_data[2] = self.z() - rhs.z();
    }
}

/// Scalar Multiplication
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

impl<T> Vector3<T>
where
    T: Copy + Mul<Output = T> + Sub<Output = T>,
{
    pub fn cross_product(&self, other: Vector3<T>) -> Vector3<T> {
        let (x, y, z) = self.get_components();
        let (x2, y2, z2) = (other.x(), other.y(), other.z());
        Vector3 {
            raw_data: [y * z2 - z * y2, z * x2 - x * z2, x * y2 - y * x2],
        }
    }
}
/// Cross product
impl<T> BitXor for Vector3<T>
where
    T: Copy + Mul<Output = T> + Sub<Output = T>,
{
    type Output = Vector3<T>;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.cross_product(rhs)
    }
}

impl<T> Vector3<T>
where
    T: Copy + Mul<Output = T> + Add<Output = T>,
{
    pub fn dot_product(&self, other: Vector3<T>) -> T {
        let (x, y, z) = self.get_components();
        let (x2, y2, z2) = other.get_components();
        x * x2 + y * y2 + z * z2
    }
}
/// Dot product
impl<T> Rem for Vector3<T>
where
    T: Copy + Mul<Output = T> + Add<Output = T>,
{
    type Output = T;

    fn rem(self, rhs: Self) -> Self::Output {
        self.dot_product(rhs)
    }
}
