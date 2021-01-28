pub use matrix4x4::Matrix4x4;
pub use vector3::Vector3;
pub use vector4::Vector4;

mod matrix4x4;
mod vector3;
mod vector4;

#[cfg(test)]
pub mod tests {
    pub mod mat4x4;
}

mod test_support {
    pub use almost_eq::*;
    pub use decode_bits::*;

    mod almost_eq;
    mod decode_bits;
}
