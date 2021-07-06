use crate::Vector3;

// (test macro expansion)
#[test]
fn test_scalar_mul() {
    let vec_f32 = Vector3::<f32>::new(2.75, -1.5, 3.25);
    // explicitly set to "<f64>" to avoid possible assignation of <f32> in some cases.
    let vec_f64 = Vector3::<f64>::new(2.75, -1.5, 3.25);

    assert_eq!(vec_f32 * -2_i32, Vector3::new(-5.5, 3.0, -6.5));
    assert_eq!(vec_f64 * -2_i32, Vector3::new(-5.5, 3.0, -6.5));
    assert_eq!(vec_f32 * 3_i64, Vector3::new(8.25, -4.5, 9.75));
    assert_eq!(vec_f64 * 3_i64, Vector3::new(8.25, -4.5, 9.75));
    assert_eq!(vec_f32 * 4_u32, Vector3::new(11.0, -6.0, 13.0));
    assert_eq!(vec_f64 * 4_u32, Vector3::new(11.0, -6.0, 13.0));
    assert_eq!(vec_f32 * 5_u64, Vector3::new(13.75, -7.5, 16.25));
    assert_eq!(vec_f64 * 5_u64, Vector3::new(13.75, -7.5, 16.25));
    assert_eq!(vec_f32 * -1.75_f64, Vector3::new(-4.8125, 2.625, -5.6875));
    assert_eq!(vec_f64 * -1.75_f32, Vector3::new(-4.8125, 2.625, -5.6875));

    assert_eq!(-1.75_f32 * vec_f32, Vector3::new(-4.8125, 2.625, -5.6875));
    assert_eq!(-1.75_f64 * vec_f32, Vector3::new(-4.8125, 2.625, -5.6875));
    assert_eq!(-1.75 * vec_f64, Vector3::new(-4.8125, 2.625, -5.6875));
    assert_eq!(-1.75_f32 * vec_f64, Vector3::new(-4.8125, 2.625, -5.6875));
    assert_eq!(-2_i32 * vec_f32, Vector3::new(-5.5, 3.0, -6.5));
    assert_eq!(-2_i32 * vec_f64, Vector3::new(-5.5, 3.0, -6.5));
    assert_eq!(3_i64 * vec_f32, Vector3::new(8.25, -4.5, 9.75));
    assert_eq!(3_i64 * vec_f64, Vector3::new(8.25, -4.5, 9.75));
    assert_eq!(4_u32 * vec_f32, Vector3::new(11.0, -6.0, 13.0));
    assert_eq!(4_u32 * vec_f64, Vector3::new(11.0, -6.0, 13.0));
    assert_eq!(5_u64 * vec_f32, Vector3::new(13.75, -7.5, 16.25));
    assert_eq!(5_u64 * vec_f64, Vector3::new(13.75, -7.5, 16.25));
}
