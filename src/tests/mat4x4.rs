use crate::test_support::AlmostEq;
use crate::Vector4;
use crate::{Matrix4x4, Vector3};

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
    println!("{}\n{}", correct_m1_x_m2, correct_m4_x_m5);
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
    // todo? is this right? count operations too (and not only results)?
    // sequential (non-parallel) 1 multiplication and 3 additions per resulting element
    let max_num_of_operations = 1 + 3;
    /* because values in resulting matrix can only be at range of [1..16), which corresponds to
    binary orders of magnitude [2^0..2^4), maximum possible magnitude of resulting element is 3;
    smaller than 16 - because maximum value of element of resulting matrix is (n * n) * 4, which
    always < 16 for n <= 2.0 except when at least one row in first matrix and column in second
    matrix entirely consist of 2.0 */
    let max_binary_magnitude = 3; // (order of magnitude)
    let tolerance = f64::EPSILON * (2_u32.pow(max_binary_magnitude) * max_num_of_operations) as f64;
    assert!((mat1 * mat2).almost_eq(correct_mat1_x_mat2, tolerance));
}

#[test]
fn test_transpose() {
    let mat = Matrix4x4::from_array([
        [0, 5, 3, 8],
        [7, 27, 9, 4],
        [93, 52, 40, 35],
        [89, 2, 6, 1], /* Rustfmt force vertical formatting */
    ]);
    let correct_transpose = Matrix4x4::from_array([
        [0, 7, 93, 89],
        [5, 27, 52, 2],
        [3, 9, 40, 6],
        [8, 4, 35, 1], /* Rustfmt force vertical formatting */
    ]);
    assert_eq!(mat.transpose(), correct_transpose);
}

#[test]
fn test_formatting() {
    let mat1 = Matrix4x4::from_array([
        [1, 0, 5, 9], /* Rustfmt force vertical formatting */
        [-23, 325, 24, 4],
        [7, 8, -5, 1],
        [-4, 2, 3, 6],
    ]);
    let mat1_align_rows = "\
        |   1    0   5  9 |\n\
        | -23  325  24  4 |\n\
        |   7    8  -5  1 |\n\
        |  -4    2   3  6 |\n\
    ";
    let mat1_uniform_cols = "\
        |   1    0    5    9 |\n\
        | -23  325   24    4 |\n\
        |   7    8   -5    1 |\n\
        |  -4    2    3    6 |\n\
    ";
    assert_eq!(mat1.format_uniform_columns(), mat1_uniform_cols);
    assert_eq!(mat1.format_align_rows(), mat1_align_rows);
    let mat2 = Matrix4x4::from_array([
        [2.71, 3.14, 1.61, 8.0],
        [-1.4, 2426.85, 7.0, 5.6],
        [7.3, 8.134, 0.14329, 1.2],
        [0.0, 3.2, 4.6, 9.5],
    ]);
    let mat2_align_magnitudes = "\
        |  2.71     3.14   1.61     8   |\n\
        | -1.4   2426.85   7        5.6 |\n\
        |  7.3      8.134  0.14329  1.2 |\n\
        |  0        3.2    4.6      9.5 |\n\
    ";
    let mat2_with_precision_2 = "\
        |  2.71     3.14  1.61  8   |\n\
        | -1.4   2426.85  7     5.6 |\n\
        |  7.3      8.13  0.14  1.2 |\n\
        |  0        3.2   4.6   9.5 |\n\
    ";
    assert_eq!(mat2.format_align_magnitudes(), mat2_align_magnitudes);
    assert_eq!(mat2.format_with_precision(2), mat2_with_precision_2);
}

#[test]
// (test macro expansion)
fn test_macro_const() {
    let identity_f32 = Matrix4x4::<f32>::identity_matrix();
    assert_eq!(identity_f32, Matrix4x4::<f32>::IDENTITY_MATRIX);
    let identity_i32 = Matrix4x4::<i32>::identity_matrix();
    assert_eq!(identity_i32, Matrix4x4::<i32>::IDENTITY_MATRIX);
    let zero_f64 = Matrix4x4::<f64>::zero_matrix();
    assert_eq!(zero_f64, Matrix4x4::<f64>::ZERO_MATRIX);
    let zero_u32 = Matrix4x4::<u32>::zero_matrix();
    assert_eq!(zero_u32, Matrix4x4::<u32>::ZERO_MATRIX);
}

#[test]
fn test_inverted_matrices() {
    let a = Matrix4x4::new_LookAt_matrix(
        Vector3::new(2.6, -3.1, 5.2),
        Vector3::new(0.0, 0.0, -1.0),
        Vector3::new(0.0, 1.0, 0.0),
    );
    let b = Matrix4x4::inv_LookAt_matrix(
        Vector3::new(2.6, -3.1, 5.2),
        Vector3::new(0.0, 0.0, -1.0),
        Vector3::new(0.0, 1.0, 0.0),
    );
    let cor = Matrix4x4::<f64>::IDENTITY_MATRIX;
    assert_eq!(a * b, cor);
    let a = Matrix4x4::new_orthographic_projection(5.0, 5.0, 13.0, 0.1);
    let b = Matrix4x4::inv_orthographic_projection(5.0, 5.0, 13.0, 0.1);
    assert_eq!(a * b, cor)
}
