use crate::{Vector3, Vector4};

#[test]
fn test_vec3_conversions() {
    /*let a = Vector3::new(2.0, 1.3, 0.7);
    let b = Vector3::<f32>::from(a); // the editor shows that this is an error, but
                                     // it's compiles and pass tests, i.e. conversion somehow works.
    let cor = Vector3::<f32>::new(2.0, 1.3, 0.7);
    assert_eq!(b, cor)*/
    let a = Vector3::new(2, 1_i32, 7);
    let b = Vector3::<f32>::from(a);
    let cor = Vector3::<f32>::new(2.0, 1.0, 7.0);
    assert_eq!(b, cor);
    let a = Vector3::new(2, 1_u32, 7);
    let b = Vector3::<f64>::from(a);
    let cor = Vector3::<f64>::new(2.0, 1.0, 7.0);
    assert_eq!(b, cor);
    /*let a = Vector3::new(2.7, -1.3, 7.5);
    let b = Vector3::<i32>::from(a);
    let cor = Vector3::new(2, -1, 7);
    assert_eq!(b, cor);*/
    let a = Vector4::new_translatable(2, -1, 7);
    let b = Vector3::from(a);
    let cor = Vector3::new(2.0, -1.0, 7.0);
    assert_eq!(b, cor);
    let a = Vector3::new(1.7, -3.05, 2.4);
    let b = Vector4::<f64>::from(a);
    let cor = Vector4::new(1.7, -3.05, 2.4, 1.0);
    assert_eq!(b, cor);
}
