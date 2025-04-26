use crate::{Vector3, Vector4};
//use num_traits::AsPrimitive;

macro_rules! impl_primitive_conv {
    ($from: ty, $to: ty) => {
        impl From<Vector3<$from>> for Vector3<$to> {
            fn from(other: Vector3<$from>) -> Self {
                Vector3 {
                    raw_data: [other.x() as $to, other.y() as $to, other.z() as $to],
                }
            }
        }
    };
}

impl_primitive_conv!(f32, f64);
impl_primitive_conv!(f64, f32);

impl_primitive_conv!(i64, f32);
impl_primitive_conv!(i32, f32);
impl_primitive_conv!(isize, f32);
impl_primitive_conv!(u64, f32);
impl_primitive_conv!(u32, f32);
impl_primitive_conv!(usize, f32);

impl_primitive_conv!(i64, f64);
impl_primitive_conv!(i32, f64);
impl_primitive_conv!(isize, f64);
impl_primitive_conv!(u64, f64);
impl_primitive_conv!(u32, f64);
impl_primitive_conv!(usize, f64);

impl_primitive_conv!(i16, f32);
impl_primitive_conv!(i8, f32);
impl_primitive_conv!(u16, f32);
impl_primitive_conv!(u8, f32);
impl_primitive_conv!(i16, f64);
impl_primitive_conv!(i8, f64);
impl_primitive_conv!(u16, f64);
impl_primitive_conv!(u8, f64);

/* This can't be implemented
impl<T, U> From<Vector3<T>> for Vector3<U> where U: From<T>, T: Copy {
    fn from(other: Vector3<T>) -> Vector3<U> {
        Vector3 {
            raw_data: [U::from(other.x()) , U::from(other.y()), U::from(other.z())]
        }
    }
}*/

// commented out because this is conflicting implementation to every impl. of 'From'
impl<T, U> From<&Vector4<T>> for Vector3<U>
where
    U: From<T> + Copy,
    T: Copy,
{
    fn from(other: &Vector4<T>) -> Vector3<U> {
        Vector3 {
            raw_data: [U::from(other.x()), U::from(other.y()), U::from(other.z())],
        }
    }
}

macro_rules! impl_vec3_from_vec4 {
    ($elem:ty) => {
        impl<T> From<Vector4<T>> for Vector3<$elem>
        where
            T: Copy,
            $elem: From<T>,
        {
            fn from(other: Vector4<T>) -> Vector3<$elem> {
                Vector3 {
                    raw_data: [other.x().into(), other.y().into(), other.z().into()],
                }
            }
        }
    };
}
impl_vec3_from_vec4!(f32);
impl_vec3_from_vec4!(f64);
