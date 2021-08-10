use crate::floating_point::FloatingPoint;
use super::Matrix;

#[test]
fn new_test() {
    let mut elements = Vec::new();
    elements.push(vec![1.0, 2.0, 3.0, 4.0]);
    elements.push(vec![5.5, 6.5, 7.5, 8.5]);
    elements.push(vec![9.0, 10.0, 11.0, 12.0]);
    elements.push(vec![13.5, 14.5, 15.5, 16.5]);

    let matrix = Matrix::new(4, Option::Some(elements));
    assert_eq!(true, FloatingPoint::equals(1.0, matrix.get(0, 0)));
    assert_eq!(true, FloatingPoint::equals(4.0, matrix.get(0, 3)));
    assert_eq!(true, FloatingPoint::equals(5.5, matrix.get(1, 0)));
    assert_eq!(true, FloatingPoint::equals(7.5, matrix.get(1, 2)));
    assert_eq!(true, FloatingPoint::equals(11.0, matrix.get(2, 2)));
    assert_eq!(true, FloatingPoint::equals(13.5, matrix.get(3, 0)));
    assert_eq!(true, FloatingPoint::equals(15.5, matrix.get(3, 2)));
}