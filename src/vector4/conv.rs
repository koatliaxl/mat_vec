use crate::{Vector3, Vector4};
use num_traits::One;

// ! The Vector4 created this way is translatable! (4th, 'w' component have value of 1)
impl<T, U> From<&Vector3<T>> for Vector4<U>
where
    U: From<T> + Copy + One,
    T: Copy,
{
    fn from(other: &Vector3<T>) -> Vector4<U> {
        Vector4 {
            raw_data: [
                U::from(other.x()),
                U::from(other.y()),
                U::from(other.z()),
                U::one(),
            ],
        }
    }
}

macro_rules! impl_vec4_from_vec3 {
    ($elem:ty) => {
        impl<T> From<Vector3<T>> for Vector4<$elem>
        where
            T: Copy + One,
            $elem: From<T>,
        {
            fn from(other: Vector3<T>) -> Self {
                Vector4 {
                    raw_data: [
                        <$elem>::from(other.x()),
                        <$elem>::from(other.y()),
                        <$elem>::from(other.z()),
                        <$elem>::from(T::one()),
                    ],
                }
            }
        }
    };
}
impl_vec4_from_vec3!(f32);
impl_vec4_from_vec3!(f64);
