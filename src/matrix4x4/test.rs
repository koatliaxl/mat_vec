#![cfg(test)]

use super::Matrix4x4;
use crate::Vector4;

#[test]
fn test_mul() {
    let m1 = Matrix4x4::from_array([
        [4, 2, 0, 0], /* Rustfmt force vertical formatting */
        [0, 8, 1, 0],
        [0, 1, 0, 0],
        [0, 0, 0, 0],
    ]);
    let m2 = Matrix4x4::from_array([
        [4, 2, 1, 0], /* Rustfmt force vertical formatting */
        [2, 0, 4, 0],
        [9, 4, 2, 0],
        [0, 0, 0, 0],
    ]);
    /*let m3 = &m1 * &m2;
    println!("{}\n{}\n{}", m1, m2, m3);*/
    let correct_m1_x_m2 = Matrix4x4::from_array([
        [20, 8, 12, 0], /* Rustfmt force vertical formatting */
        [25, 4, 34, 0],
        [2, 0, 4, 0],
        [0, 0, 0, 0],
    ]);
    assert_eq!(&m1 * &m2, correct_m1_x_m2);
    let m4 = Matrix4x4::from_array([
        [5, 9, 6, 8], /* Rustfmt force vertical formatting */
        [7, 4, 2, 7],
        [0, 3, 1, 0],
        [8, 2, 6, 9],
    ]);
    let m5 = Matrix4x4::from_array([
        [5, 9, 5, 2], /* Rustfmt force vertical formatting */
        [1, 8, 0, 8],
        [5, 3, 9, 0],
        [4, 2, 8, 9],
    ]);
    //println!("{}", m4 * m5);
    let correct_m4_x_m5 = Matrix4x4::from_array([
        [96_, 151, 143, 154], /* Rustfmt force vertical formatting */
        [77_, 115, 109, 109],
        [8__, 27_, 9__, 24_],
        [108, 124, 166, 113],
    ]);
    assert_eq!(m4 * m5, correct_m4_x_m5);
    let m4 = Matrix4x4::from_array([
        [-2, 4, -1, -4], /* Rustfmt force vertical formatting */
        [7, 2, 8, -1],
        [8, -9, -6, 3],
        [2, 5, 3, 4],
    ]);
    let m5 = Matrix4x4::from_array([
        [-7, 3, 7, 7], /* Rustfmt force vertical formatting */
        [-8, 1, -3, -1],
        [3, 3, 7, 8],
        [-7, -1, -9, 4],
    ]);
    let correct_m4_x_m5 = Matrix4x4::from_array([
        [7, -1, 3, -42], /* Rustfmt force vertical formatting */
        [-34, 48, 108, 107],
        [-23, -6, 14, 29],
        [-73, 16, -16, 49],
    ]);
    assert_eq!(m4 * m5, correct_m4_x_m5);
}

#[test]
fn test_mul_vec4() {
    let mat = Matrix4x4::from_array([
        [3, 5, 8, 0], /* Rustfmt force vertical formatting */
        [7, 2, 9, 0],
        [7, 3, 6, 0],
        [0, 0, 0, 1],
    ]);
    let vec4 = Vector4::new_xyz(9, 3, 4);
    let correct_mat_x_vec4 = Vector4::from_array([74, 105, 96, 0]);
    assert_eq!(&mat * vec4, correct_mat_x_vec4);
    let vec4 = Vector4::new(4, 8, 3, 1);
    let correct_mat_x_vec4 = Vector4::from_array([76, 71, 70, 1]);
    assert_eq!(mat * vec4, correct_mat_x_vec4);
}
