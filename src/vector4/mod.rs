mod conv;
mod ops;

use num_traits::{One, Zero};
use std::fmt::Debug;
use std::ops::{Index, IndexMut};

#[derive(Debug, Copy, Clone)]
pub struct Vector4<T>
where
    T: Copy,
{
    raw_data: [T; 4],
}

impl<T> Vector4<T>
where
    T: Copy,
{
    pub fn new(x: T, y: T, z: T, w: T) -> Vector4<T> {
        Vector4 {
            raw_data: [x, y, z, w],
        }
    }

    pub fn from_array(arr: [T; 4]) -> Vector4<T> {
        Vector4 { raw_data: arr }
    }

    pub fn from_tuple(tuple: (T, T, T, T)) -> Vector4<T> {
        let (x, y, z, w) = tuple;
        Vector4 {
            raw_data: [x, y, z, w],
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
    pub fn w(&self) -> T {
        self.raw_data[3]
    }

    pub fn get_components(&self) -> (T, T, T, T) {
        (
            self.raw_data[0],
            self.raw_data[1],
            self.raw_data[2],
            self.raw_data[3],
        )
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
    pub fn set_w(&mut self, value: T) {
        self.raw_data[3] = value;
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
    pub fn w_mut(&mut self) -> &mut T {
        &mut self.raw_data[3]
    }
}

impl<T> Vector4<T>
where
    T: Copy + Zero,
{
    pub fn new_xyz(x: T, y: T, z: T) -> Vector4<T> {
        Vector4 {
            raw_data: [x, y, z, T::zero()],
        }
    }
}

impl<T> Vector4<T>
where
    T: Copy + One,
{
    // Vector which is translates by the translation matrix
    pub fn new_translatable(x: T, y: T, z: T) -> Vector4<T> {
        Vector4 {
            raw_data: [x, y, z, T::one()],
        }
    }
}

impl<T> Index<usize> for Vector4<T>
where
    T: Copy,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        #[cfg(debug_assertions)]
        {
            if index > 3 {
                panic!("Vector index out of bounds")
            }
        }
        &self.raw_data[index]
    }
}

impl<T> IndexMut<usize> for Vector4<T>
where
    T: Copy,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        #[cfg(debug_assertions)]
        {
            if index > 3 {
                panic!("Vector index out of bounds")
            }
        }
        &mut self.raw_data[index]
    }
}

impl<T> PartialEq for Vector4<T>
where
    T: Copy + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        for i in 0..4 {
            if self.raw_data[i] != other.raw_data[i] {
                return false;
            }
        }
        true
    }
}
