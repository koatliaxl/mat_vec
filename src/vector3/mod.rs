mod ops;
mod scalar_mul;

use num_traits::{AsPrimitive, Float};
use std::ops::{Add, Index, IndexMut, Mul};

#[derive(Copy, Clone, Debug)]
pub struct Vector3<T>
where
    T: Copy,
{
    // todo? change inner structure
    raw_data: [T; 3],
}

impl<T> Vector3<T>
where
    T: Copy,
{
    pub fn new(x: T, y: T, z: T) -> Vector3<T> {
        Vector3 {
            raw_data: [x, y, z],
        }
    }

    //todo new_uniform()

    pub fn from_array(arr: [T; 3]) -> Vector3<T> {
        Vector3 { raw_data: arr }
    }

    pub fn from_tuple(tuple: (T, T, T)) -> Vector3<T> {
        Vector3 {
            raw_data: [tuple.0, tuple.1, tuple.2],
        }
    }

    pub fn x(&self) -> T {
        self.raw_data[0]
    }
    pub fn y(&self) -> T {
        self.raw_data[1]
    }
    pub fn z(&self) -> T {
        self.raw_data[2]
    }

    pub fn get_components(&self) -> (T, T, T) {
        (self.raw_data[0], self.raw_data[1], self.raw_data[2])
    }

    pub fn set_x(&mut self, value: T) {
        self.raw_data[0] = value;
    }
    pub fn set_y(&mut self, value: T) {
        self.raw_data[1] = value;
    }
    pub fn set_z(&mut self, value: T) {
        self.raw_data[2] = value;
    }

    pub fn x_mut(&mut self) -> &mut T {
        &mut self.raw_data[0]
    }
    pub fn y_mut(&mut self) -> &mut T {
        &mut self.raw_data[1]
    }
    pub fn z_mut(&mut self) -> &mut T {
        &mut self.raw_data[2]
    }
}

impl<T> Index<usize> for Vector3<T>
where
    T: Copy,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.raw_data[index]
    }
}

impl<T> IndexMut<usize> for Vector3<T>
where
    T: Copy,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.raw_data[index]
    }
}

impl<T> PartialEq for Vector3<T>
where
    T: Copy + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        for i in 0..3 {
            if self.raw_data[i] != other.raw_data[i] {
                return false;
            }
        }
        true
    }
}

impl<T> Vector3<T>
where
    T: Float,
{
    pub fn length(&self) -> T {
        let (x, y, z) = self.get_components();
        (x * x + y * y + z * z).sqrt()
    }
}
impl<T> Vector3<T>
where
    T: AsPrimitive<f32> + Add<Output = T> + Mul<Output = T>,
{
    pub fn length_as_f32(&self) -> f32 {
        let (x, y, z) = self.get_components();
        let square_len = x * x + y * y + z * z;
        square_len.as_().sqrt()
    }
}
impl<T> Vector3<T>
where
    T: AsPrimitive<f64> + Add<Output = T> + Mul<Output = T>,
{
    pub fn length_as_f64(&self) -> f64 {
        let (x, y, z) = self.get_components();
        let square_len = x * x + y * y + z * z;
        square_len.as_().sqrt()
    }
}

impl<T> Default for Vector3<T>
where
    T: Copy + Default,
{
    fn default() -> Self {
        Vector3 {
            raw_data: [T::default(); 3],
        }
    }
}

// todo Display
