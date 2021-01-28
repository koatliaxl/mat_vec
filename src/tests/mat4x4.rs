use crate::test_support::AlmostEq;
use crate::Matrix4x4;
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

#[test]
fn test_mul_float() {
    // todo? is this right? count operations too?
    // sequential (non-parallel) 1 multiplication and 3 additions per resulting element
    let max_number_of_operations = 1 + 3;
    /* because values are in range of 1..8 (2^0 .. 2^3),
    8 - because max value of element of resulting matrix is (n * n) * 4, which always < 16 for
    n <= 2.0 except when at least one row in first matrix and column in second matrix
    entirely consist of 2.0 */
    let max_binary_magnitude = 4;
    let tolerance =
        f64::EPSILON * (2_u32.pow(max_binary_magnitude) * max_number_of_operations) as f64;
    let mat1 = Matrix4x4::from_array([
        [1.142, 1.673, 1.813, 1.540],
        [1.777, 1.544, 1.309, 1.408],
        [1.381, 1.031, 1.891, 1.074],
        [1.866, 1.286, 1.696, 1.526],
    ]);
    let mat2 = Matrix4x4::from_array([
        [1.460, 1.451, 1.598, 1.985],
        [1.158, 1.180, 1.951, 1.335],
        [1.516, 1.876, 1.185, 1.141],
        [1.734, 1.601, 1.971, 1.009],
    ]);
    let correct_mat1_x_mat2 = Matrix4x4::from_array([
        [9.023522, 9.49791, 10.272684, 8.122818],
        [8.808288, 9.110239, 10.178323, 8.502826],
        [7.93923, 8.487401, 8.576008, 7.358967],
        [9.430768, 9.849868, 10.50836, 8.89569],
    ]);
    let mat1_x_mat2 = mat1 * mat2;
    assert!(mat1_x_mat2.almost_eq(correct_mat1_x_mat2, tolerance));
}
