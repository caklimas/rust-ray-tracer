use std::f64::consts::PI;

use crate::{matrix::{Matrix, axis::Axis}, tuple::Tuple};

#[test]
fn translation_test() {
    let translation = Matrix::translation(5.0, -3.0, 2.0);
    let point = Tuple::point(-3.0, 4.0, 5.0);
    let vector = Tuple::vector(-3.0, 4.0, 5.0);
    let inverse = translation.inverse();

    assert_eq!(Tuple::point(2.0, 1.0, 7.0), translation.multiply_tuple(&point));
    assert_eq!(Tuple::point(-8.0, 7.0, 3.0), inverse.multiply_tuple(&point));
    assert_eq!(vector, translation.multiply_tuple(&vector));
}

#[test]
fn scaling_test() {
    let transform = Matrix::scaling(2.0, 3.0, 4.0);
    let point = Tuple::point(-4.0, 6.0, 8.0);
    let actual = transform.multiply_tuple(&point);

    assert_eq!(Tuple::point(-8.0, 18.0, 32.0), actual);

    let transform = Matrix::scaling(2.0, 3.0, 4.0);
    let vector = Tuple::vector(-4.0, 6.0, 8.0);
    let actual = transform.multiply_tuple(&vector);

    assert_eq!(Tuple::vector(-8.0, 18.0, 32.0), actual);

    let transform = Matrix::scaling(2.0, 3.0, 4.0);
    let inverse = transform.inverse();
    let vector = Tuple::vector(-4.0, 6.0, 8.0);
    let actual = inverse.multiply_tuple(&vector);

    assert_eq!(Tuple::vector(-2.0, 2.0, 2.0), actual);
}

#[test]
fn reflect_test() {
    let transform = Matrix::reflect(Axis::X);
    let point = Tuple::point(2.0, 3.0, 4.0);
    let actual = transform.multiply_tuple(&point);

    assert_eq!(Tuple::point(-2.0, 3.0, 4.0), actual);
}

#[test]
fn rotate_x_test() {
    let point = Tuple::point(0.0, 1.0, 0.0);
    let half_quarter = Matrix::rotate_x(PI / 4.0);
    let full_quarter = Matrix::rotate_x(PI / 2.0);
    let half_quarter_actual = half_quarter.multiply_tuple(&point);
    let full_quarter_actual = full_quarter.multiply_tuple(&point);
    let half_quarter_result = (2.0f64).sqrt() / 2.0;
    let inverse = half_quarter.inverse();

    assert_eq!(Tuple::point(0.0, half_quarter_result, half_quarter_result), half_quarter_actual);
    assert_eq!(Tuple::point(0.0, 0.0, 1.0), full_quarter_actual);
    assert_eq!(Tuple::point(0.0, half_quarter_result, -half_quarter_result), inverse.multiply_tuple(&point));
}