use super::FloatingPoint;

#[test]
fn equals_success_test() {
    let left = 1.0;
    let right = 1.00000000000;

    assert_eq!(true, FloatingPoint::equals(left, right));
}

#[test]
fn equals_fail_test() {
    let left = 1.0;
    let right = 1.1;

    assert_eq!(false, FloatingPoint::equals(left, right));
}