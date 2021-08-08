use super::Point;

#[test]
fn new_test() {
    //tuple(4.3, -4.2, 3.1, 1.0)
    let x = 4.3;
    let y = -4.2;
    let z = 3.1;
    let w = 1.0;
    let point = Point::new(x, y, z, w);

    assert_eq!(x, point.x);
    assert_eq!(y, point.y);
    assert_eq!(z, point.z);
    assert_eq!(w, point.w);
}