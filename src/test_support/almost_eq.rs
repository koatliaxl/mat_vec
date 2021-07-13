use crate::{Matrix4x4, Vector3};

pub trait AlmostEq<R = Self, T = Self> {
    fn almost_eq(&self, other: R, tolerance: T) -> bool;
}

impl AlmostEq for f32 {
    fn almost_eq(&self, other: f32, tolerance: f32) -> bool {
        (self - other).abs() < tolerance
    }
}

impl AlmostEq for f64 {
    fn almost_eq(&self, other: f64, tolerance: f64) -> bool {
        (self - other).abs() < tolerance
    }
}

impl AlmostEq<Self, f32> for Matrix4x4<f32> {
    fn almost_eq(&self, other: Self, tolerance: f32) -> bool {
        for r in 0..4 {
            for c in 0..4 {
                if !self[(r, c)].almost_eq(other[(r, c)], tolerance) {
                    return false;
                }
            }
        }
        true
    }
}

impl AlmostEq<Self, f64> for Matrix4x4<f64> {
    fn almost_eq(&self, other: Self, tolerance: f64) -> bool {
        for r in 0..4 {
            for c in 0..4 {
                if !self[(r, c)].almost_eq(other[(r, c)], tolerance) {
                    return false;
                }
            }
        }
        true
    }
}

impl AlmostEq<Self, f32> for Vector3<f32> {
    fn almost_eq(&self, other: Self, tolerance: f32) -> bool {
        let (x, y, z) = self.get_components();
        let (x2, y2, z2) = other.get_components();
        /*if x.almost_eq(x2, tolerance) && y.almost_eq(y2, tolerance) && z.almost_eq(z2, tolerance) {
            true
        } else {
            false
        }*/
        if !x.almost_eq(x2, tolerance) {
            return false;
        }
        if !y.almost_eq(y2, tolerance) {
            return false;
        }
        if !z.almost_eq(z2, tolerance) {
            return false;
        }
        true
    }
}

impl AlmostEq<Self, f64> for Vector3<f64> {
    fn almost_eq(&self, other: Self, tolerance: f64) -> bool {
        let (x, y, z) = self.get_components();
        let (x2, y2, z2) = other.get_components();
        if !x.almost_eq(x2, tolerance) {
            return false;
        }
        if !y.almost_eq(y2, tolerance) {
            return false;
        }
        if !z.almost_eq(z2, tolerance) {
            return false;
        }
        true
    }
}
