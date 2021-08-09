use crate::floating_point::FloatingPoint;
use super::{Tuple, POINT_W, VECTOR_W};

#[test]
fn new_test() {
    let x = 4.3;
    let y = -4.2;
    let z = 3.1;
    let w = 5.2;
    let tuple = Tuple::new(x, y, z, w);

    assert_eq!(true, FloatingPoint::equals(x, tuple.x()));
    assert_eq!(true, FloatingPoint::equals(y, tuple.y()));
    assert_eq!(true, FloatingPoint::equals(z, tuple.z()));
}

#[test]
fn point_test() {
    let point = Tuple::point(0.0, 0.0, 0.0);
    assert_eq!(true, FloatingPoint::equals(POINT_W, point.w()));
    assert_eq!(true, point.is_point());
}

#[test]
fn vector_test() {
    let point = Tuple::vector(0.0, 0.0, 0.0);
    assert_eq!(true, FloatingPoint::equals(VECTOR_W, point.w()));
    assert_eq!(true, point.is_vector());
}

#[test]
fn add_point_vector_test() {
    let tuple = Tuple::point(1.0, 1.0, 1.0);
    let other = Tuple::vector(1.0, 1.0, 1.0);
    let added = tuple.add(&other);

    assert_eq!(true, FloatingPoint::equals(2.0, added.x()));
    assert_eq!(true, FloatingPoint::equals(2.0, added.y()));
    assert_eq!(true, FloatingPoint::equals(2.0, added.z()));
    assert_eq!(true, FloatingPoint::equals(POINT_W, added.w()));
}

#[test]
fn add_vectors_test() {
    let tuple = Tuple::vector(1.0, 1.0, 1.0);
    let other = Tuple::vector(1.0, 1.0, 1.0);
    let added = tuple.add(&other);

    assert_eq!(true, FloatingPoint::equals(2.0, added.x()));
    assert_eq!(true, FloatingPoint::equals(2.0, added.y()));
    assert_eq!(true, FloatingPoint::equals(2.0, added.z()));
    assert_eq!(true, FloatingPoint::equals(VECTOR_W, added.w()));
}

#[test]
#[should_panic]
fn add_points_panic_test() {
    let tuple = Tuple::point(1.0, 1.0, 1.0);
    let other = Tuple::point(1.0, 1.0, 1.0);
    tuple.add(&other);
}

#[test]
fn sub_points_test() {
    let tuple = Tuple::point(3.0, 2.0, 1.0);
    let other = Tuple::point(5.0, 6.0, 7.0);
    let subtracted = tuple.sub(&other);

    assert_eq!(true, FloatingPoint::equals(-2.0, subtracted.x()));
    assert_eq!(true, FloatingPoint::equals(-4.0, subtracted.y()));
    assert_eq!(true, FloatingPoint::equals(-6.0, subtracted.z()));
    assert_eq!(true, FloatingPoint::equals(VECTOR_W, subtracted.w()));
}

#[test]
fn sub_vector_from_point_test() {
    let tuple = Tuple::point(3.0, 2.0, 1.0);
    let other = Tuple::vector(5.0, 6.0, 7.0);
    let subtracted = tuple.sub(&other);

    assert_eq!(true, FloatingPoint::equals(-2.0, subtracted.x()));
    assert_eq!(true, FloatingPoint::equals(-4.0, subtracted.y()));
    assert_eq!(true, FloatingPoint::equals(-6.0, subtracted.z()));
    assert_eq!(true, FloatingPoint::equals(POINT_W, subtracted.w()));
}

#[test]
fn sub_vectors_test() {
    let tuple = Tuple::vector(3.0, 2.0, 1.0);
    let other = Tuple::vector(5.0, 6.0, 7.0);
    let subtracted = tuple.sub(&other);

    assert_eq!(true, FloatingPoint::equals(-2.0, subtracted.x()));
    assert_eq!(true, FloatingPoint::equals(-4.0, subtracted.y()));
    assert_eq!(true, FloatingPoint::equals(-6.0, subtracted.z()));
    assert_eq!(true, FloatingPoint::equals(VECTOR_W, subtracted.w()));
}

#[test]
#[should_panic]
fn sub_point_from_vector_panic() {
    let tuple = Tuple::vector(3.0, 2.0, 1.0);
    let other = Tuple::point(5.0, 6.0, 7.0);
    tuple.sub(&other);
}

#[test]
fn negate_test() {
    let tuple = Tuple::point(1.0, -2.0, 3.0);
    let negated = tuple.negate();
    assert_eq!(true, FloatingPoint::equals(-1.0, negated.x()));
    assert_eq!(true, FloatingPoint::equals(2.0, negated.y()));
    assert_eq!(true, FloatingPoint::equals(-3.0, negated.z()));
    assert_eq!(true, FloatingPoint::equals(-POINT_W, negated.w()));
}

#[test]
fn scalar_multiply_test() {
    let tuple = Tuple::new(1.0, -2.0, 3.0, -4.0);
    let multiplied = tuple.multiply(3.5);

    assert_eq!(true, multiplied.equals(&Tuple::new(3.5, -7.0, 10.5, -14.0)));
}

#[test]
fn fraction_multiply_test() {
    let tuple = Tuple::new(1.0, -2.0, 3.0, -4.0);
    let multiplied = tuple.multiply(0.5);

    assert_eq!(true, multiplied.equals(&Tuple::new(0.5, -1.0, 1.5, -2.0)));
}

#[test]
fn scalar_divide_test() {
    let tuple = Tuple::new(1.0, -2.0, 3.0, -4.0);
    let divided = tuple.divide(2.0);

    assert_eq!(true, divided.equals(&Tuple::new(0.5, -1.0, 1.5, -2.0)));
}

#[test]
fn magnitude_test() {
    let mut tuple = Tuple::vector(1.0, 0.0, 0.0);
    let mut magnitude = tuple.magnitude();
    assert_eq!(true, FloatingPoint::equals(1.0, magnitude));

    tuple = Tuple::vector(0.0, 1.0, 0.0);
    magnitude = tuple.magnitude();
    assert_eq!(true, FloatingPoint::equals(1.0, magnitude));

    tuple = Tuple::vector(0.0, 0.0, 1.0);
    magnitude = tuple.magnitude();
    assert_eq!(true, FloatingPoint::equals(1.0, magnitude));

    tuple = Tuple::vector(1.0, 2.0, 3.0);
    magnitude = tuple.magnitude();
    assert_eq!(true, FloatingPoint::equals(14.0_f64.sqrt(), magnitude));

    tuple = Tuple::vector(-1.0, -2.0, -3.0);
    magnitude = tuple.magnitude();
    assert_eq!(true, FloatingPoint::equals((14.0_f64).sqrt(), magnitude));
}

#[test]
fn normalize_test() {
    let sqrt = 14.0_f64.sqrt();
    let mut tuple = Tuple::vector(4.0, 0.0, 0.0);
    let mut normalize = tuple.normalize();
    assert_eq!(true, normalize.equals(&Tuple::vector(1.0, 0.0, 0.0)));

    tuple = Tuple::vector(1.0, 2.0, 3.0);
    normalize = tuple.normalize();
    assert_eq!(true, normalize.equals(&Tuple::vector(1.0 / sqrt, 2.0 / sqrt, 3.0 / sqrt)));
    assert_eq!(true, FloatingPoint::equals(1.0, normalize.magnitude()));
}

#[test]
#[should_panic]
fn normalize_panic_test() {
    let point = Tuple::point(0.0, 0.0, 0.0);
    point.normalize();
}

#[test]
fn dot_test() {
    let tuple = Tuple::vector(1.0, 2.0, 3.0);
    let dot = tuple.dot(&Tuple::vector(2.0, 3.0, 4.0));
    assert_eq!(true, FloatingPoint::equals(20.0, dot));
    
}

#[test]
#[should_panic]
fn dot_panic_test() {
    let vector = Tuple::vector(0.0, 0.0, 0.0);
    let point = Tuple::point(0.0, 0.0, 0.0);
    vector.dot(&point);
}

#[test]
fn cross_test() {
    let tuple = Tuple::vector(1.0, 2.0, 3.0);
    let other = Tuple::vector(2.0, 3.0, 4.0);
    
    let mut cross = tuple.cross(&other);
    assert_eq!(true, cross.equals(&Tuple::vector(-1.0, 2.0, -1.0)));

    cross = other.cross(&tuple);
    assert_eq!(true, cross.equals(&Tuple::vector(1.0, -2.0, 1.0)));
}

#[test]
#[should_panic]
fn cross_panic_test() {
    let vector = Tuple::vector(0.0, 0.0, 0.0);
    let point = Tuple::point(0.0, 0.0, 0.0);
    vector.cross(&point);
}

#[test]
fn equals_true_test() {
    let x = 1.0;
    let y = 1.1;
    let z = 1.2;

    let mut tuple = Tuple::point(x, y, z);
    let mut other = Tuple::point(x, y, z);
    assert_eq!(true, tuple.equals(&other));

    tuple = Tuple::vector(x, y, z);
    other = Tuple::vector(x, y, z);
    assert_eq!(true, tuple.equals(&other));
}

#[test]
fn equals_fail_test() {
    let x = 1.0;
    let y = 1.1;
    let z = 1.2;
    let tuple = Tuple::point(x, y, z);

    let mut other = Tuple::point(x + 1.0, y, z);
    assert_eq!(false, tuple.equals(&other));

    other = Tuple::point(x, y + 1.0, z);
    assert_eq!(false, tuple.equals(&other));

    other = Tuple::point(x, y, z + 1.0);
    assert_eq!(false, tuple.equals(&other));

    other = Tuple::vector(x, y, z);
    assert_eq!(false, tuple.equals(&other));
}