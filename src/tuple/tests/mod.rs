use super::{Tuple, POINT_W, VECTOR_W};

#[test]
fn new_test() {
    let x = 4.3;
    let y = -4.2;
    let z = 3.1;
    let point = Tuple::new(x, y, z, true);

    assert_eq!(x, point.x());
    assert_eq!(y, point.y());
    assert_eq!(z, point.z());
}

#[test]
fn point_test() {
    let point = Tuple::point(0.0, 0.0, 0.0);
    assert_eq!(POINT_W, point.w());
}

#[test]
fn vector_test() {
    let point = Tuple::vector(0.0, 0.0, 0.0);
    assert_eq!(VECTOR_W, point.w());
}