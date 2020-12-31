use crate::Vector4;
use num_traits::Float;

impl<T> Vector4<T>
where
    T: Float,
{
    pub fn normalize_3_elements(&self) -> Vector4<T> {
        let (x, y, z, w) = self.get_components();
        let length = (x * x + y * y + z * z).sqrt();
        Vector4 {
            raw_data: [x / length, y / length, z / length, w],
        }
    }
}
