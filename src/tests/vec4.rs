use crate::Vector4;

#[test]
fn test_conditional_compilation() {
    let _a = Vector4::new_translatable(2.1, 3.4, -1.6);
    /*println!(
        "1:{}, 2: {},_ ,3: {}, 4 (out of bounds): {}",
        a[0], a[1], a[3], a[4],
    )*/
}
