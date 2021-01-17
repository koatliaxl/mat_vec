mod add;
mod mul;
mod mul_vec4;
mod test;

use crate::Vector3;
use num_traits::{Float, One, Zero};
use std::fmt::{Debug, Display, Formatter, Result};
use std::ops::{Add, AddAssign, Index, Mul};

#[derive(Debug, Clone)]
pub struct Matrix4x4<T>
where
    T: Add<Output = T> + Mul<Output = T> + Copy + AddAssign + Default,
{
    raw_data: [T; 16],
}

impl<T> Matrix4x4<T>
where
    T: Copy + Add<Output = T> + Mul<Output = T> + AddAssign + Default,
{
    pub fn from_array(arr: [[T; 4]; 4]) -> Matrix4x4<T> {
        let mut raw_data = [T::default(); 16];
        for r in 0..4 {
            for c in 0..4 {
                raw_data[r * 4 + c] = arr[r][c];
            }
        }
        Matrix4x4 { raw_data }
    }

    //pub fn from_slice()

    pub fn as_ptr(&self) -> *const T {
        self.raw_data.as_ptr()
    }

    pub fn set(&mut self, row: usize, column: usize, value: T) {
        self.raw_data[row * 4 + column] = value
    }

    pub fn get(&self, row: usize, column: usize) -> T {
        self.raw_data[row * 4 + column]
    }
}

impl<T> Matrix4x4<T>
where
    T: Copy + Mul<Output = T> + AddAssign + Zero + Default,
{
    pub fn zero_matrix() -> Matrix4x4<T> {
        Matrix4x4 {
            raw_data: [T::zero(); 16],
        }
    }
}

impl<T> Matrix4x4<T>
where
    T: Copy + AddAssign + Zero + One + Default,
{
    pub fn new_scaling(sx: T, sy: T, sz: T) -> Matrix4x4<T> {
        let mut mat = Matrix4x4::zero_matrix();
        mat.set(0, 0, sx);
        mat.set(1, 1, sy);
        mat.set(2, 2, sz);
        mat.set(3, 3, T::one());
        mat
    }

    pub fn new_translation(tx: T, ty: T, tz: T) -> Matrix4x4<T> {
        let mut mat = Matrix4x4::identity_matrix();
        mat.set(0, 3, tx);
        mat.set(1, 3, ty);
        mat.set(2, 3, tz);
        mat
    }

    pub fn identity_matrix() -> Matrix4x4<T> {
        let mut mat = Matrix4x4::zero_matrix();
        for i in 0..4 {
            mat.set(i, i, T::one());
        }
        mat
    }
}

impl<T> Matrix4x4<T>
where
    T: AddAssign + Float + Default,
{
    pub fn new_x_rotation(degrees: T) -> Matrix4x4<T> {
        let mut mat = Matrix4x4::identity_matrix();
        let ang_sin = degrees.to_radians().sin();
        let ang_cos = degrees.to_radians().cos();
        mat.set(1, 1, ang_cos);
        mat.set(1, 2, -ang_sin);

        mat.set(2, 1, ang_sin);
        mat.set(2, 2, ang_cos);
        mat
    }

    pub fn new_y_rotation(degrees: T) -> Matrix4x4<T> {
        let mut mat = Matrix4x4::identity_matrix();
        let ang_sin = degrees.to_radians().sin();
        let ang_cos = degrees.to_radians().cos();
        mat.set(0, 0, ang_cos);
        mat.set(0, 2, ang_sin);

        mat.set(2, 0, -ang_sin);
        mat.set(2, 2, ang_cos);
        mat
    }

    pub fn new_z_rotation(degrees: T) -> Matrix4x4<T> {
        let mut mat = Matrix4x4::identity_matrix();
        let ang_sin = degrees.to_radians().sin();
        let ang_cos = degrees.to_radians().cos();
        mat.set(0, 0, ang_cos);
        mat.set(0, 1, -ang_sin);

        mat.set(1, 0, ang_sin);
        mat.set(1, 1, ang_cos);
        mat
    }

    pub fn new_rotation(degrees: T, axis: Vector3<T>) -> Matrix4x4<T> {
        let (rx, ry, rz) = (axis.x(), axis.y(), axis.z());
        let ang_cos = degrees.to_radians().cos();
        let ang_sin = degrees.to_radians().sin();
        let one_minus_cos = T::one() - ang_cos;
        let rx_ry_1_m_cos = rx * ry * one_minus_cos;
        let ry_rz_1_m_cos = ry * rz * one_minus_cos;
        let rz_rx_1_m_cos = rz * rx * one_minus_cos;
        let rx_sin = rx * ang_sin;
        let ry_sin = ry * ang_sin;
        let rz_sin = rz * ang_sin;
        let mut mat = Matrix4x4::zero_matrix();
        mat.set(0, 0, ang_cos + rx * rx * one_minus_cos);
        mat.set(0, 1, rx_ry_1_m_cos - rz_sin);
        mat.set(0, 2, rz_rx_1_m_cos + ry_sin);
        mat.set(1, 0, rx_ry_1_m_cos + rz_sin);
        mat.set(1, 1, ang_cos + ry * ry * one_minus_cos);
        mat.set(1, 2, ry_rz_1_m_cos - rx_sin);
        mat.set(2, 0, rz_rx_1_m_cos - ry_sin);
        mat.set(2, 1, ry_rz_1_m_cos + rx_sin);
        mat.set(2, 2, ang_cos + rz * rz * one_minus_cos);
        mat.set(3, 3, T::one());
        mat
    }

    pub fn new_perspective_projection(
        field_of_view: T,
        aspect_ratio: T,
        z_far: T,
        z_near: T,
    ) -> Matrix4x4<T> {
        let one = T::one();
        let two = one + one;
        let fov_tan = (field_of_view / two).to_radians().tan();
        let mut mat = Matrix4x4::zero_matrix();
        mat.set(0, 0, one / (fov_tan * aspect_ratio));
        mat.set(1, 1, one / fov_tan);
        mat.set(2, 2, -(z_far + z_near) / (z_far - z_near));
        mat.set(2, 3, -two * z_far * z_near / (z_far - z_near));
        mat.set(3, 2, -one);
        mat
    }
}

impl<T> Index<(usize, usize)> for Matrix4x4<T>
where
    T: Add<Output = T> + Mul<Output = T> + Copy + AddAssign + Default,
{
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.raw_data[index.0 * 4 + index.1]
    }
}

impl<T> Display for Matrix4x4<T>
where
    T: Display + Add<Output = T> + Mul<Output = T> + Copy + AddAssign + Default,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f)?;
        for r in 0..4 {
            writeln!(
                f,
                "| {0:}, {1:}, {2:}, {3:} |",
                self[(r, 0)],
                self[(r, 1)],
                self[(r, 2)],
                self[(r, 3)]
            )?;
        }
        Ok(())
    }
}

impl<T> Default for Matrix4x4<T>
where
    T: Add<Output = T> + Mul<Output = T> + Copy + AddAssign + Default,
{
    fn default() -> Self {
        Matrix4x4 {
            raw_data: [T::default(); 16],
        }
    }
}

impl<T> PartialEq for Matrix4x4<T>
where
    T: Add<Output = T> + Mul<Output = T> + Copy + AddAssign + PartialEq + Default,
{
    fn eq(&self, other: &Self) -> bool {
        for r in 0..4 {
            for c in 0..4 {
                if self[(r, c)] != other[(r, c)] {
                    return false;
                }
            }
        }
        true
    }
}
impl<T> Eq for Matrix4x4<T> where
    T: Add<Output = T> + Mul<Output = T> + Copy + AddAssign + PartialEq + Eq + Default
{
}