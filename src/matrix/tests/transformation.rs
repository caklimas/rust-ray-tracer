use std::f64::consts::PI;
use crate::matrix::{Matrix, transformation::*};
use crate::{matrix::{axis::Axis}, tuple::Tuple};

#[test]
fn translation_test() {
    let translation = translate(5.0, -3.0, 2.0);
    let point = Tuple::point(-3.0, 4.0, 5.0);
    let vector = Tuple::vector(-3.0, 4.0, 5.0);
    let inverse = translation.inverse();

    assert_eq!(Tuple::point(2.0, 1.0, 7.0), translation.clone() * point);
    assert_eq!(Tuple::point(-8.0, 7.0, 3.0), inverse * point);
    assert_eq!(vector, translation * vector);
}

#[test]
fn scaling_test() {
    let transform = scale(2.0, 3.0, 4.0);
    let point = Tuple::point(-4.0, 6.0, 8.0);
    let actual = transform * point;

    assert_eq!(Tuple::point(-8.0, 18.0, 32.0), actual);

    let transform = scale(2.0, 3.0, 4.0);
    let vector = Tuple::vector(-4.0, 6.0, 8.0);
    let actual = transform * vector;

    assert_eq!(Tuple::vector(-8.0, 18.0, 32.0), actual);

    let transform = scale(2.0, 3.0, 4.0);
    let inverse = transform.inverse();
    let vector = Tuple::vector(-4.0, 6.0, 8.0);
    let actual = inverse * vector;

    assert_eq!(Tuple::vector(-2.0, 2.0, 2.0), actual);
}

#[test]
fn reflect_test() {
    let transform = reflect(Axis::X);
    let point = Tuple::point(2.0, 3.0, 4.0);
    let actual = transform * point;

    assert_eq!(Tuple::point(-2.0, 3.0, 4.0), actual);
}

#[test]
fn rotate_x_test() {
    let point = Tuple::point(0.0, 1.0, 0.0);
    let half_quarter = rotate_x(PI / 4.0);
    let full_quarter = rotate_x(PI / 2.0);
    let half_quarter_actual = half_quarter.clone() * point;
    let full_quarter_actual = full_quarter * point;
    let half_quarter_result = (2.0f64).sqrt() / 2.0;
    let inverse = half_quarter.inverse();

    assert_eq!(Tuple::point(0.0, half_quarter_result, half_quarter_result), half_quarter_actual);
    assert_eq!(Tuple::point(0.0, 0.0, 1.0), full_quarter_actual);
    assert_eq!(Tuple::point(0.0, half_quarter_result, -half_quarter_result), inverse * point);
}

#[test]
fn rotate_y_test() {
    let point = Tuple::point(0.0, 0.0, 1.0);
    let half_quarter = rotate_y(PI / 4.0);
    let full_quarter = rotate_y(PI / 2.0);
    let half_quarter_actual = half_quarter * point;
    let full_quarter_actual = full_quarter * point;
    let half_quarter_result = (2.0f64).sqrt() / 2.0;

    assert_eq!(Tuple::point(half_quarter_result, 0.0, half_quarter_result), half_quarter_actual);
    assert_eq!(Tuple::point(1.0, 0.0, 0.0), full_quarter_actual);
}

#[test]
fn rotate_z_test() {
    let point = Tuple::point(0.0, 1.0, 0.0);
    let half_quarter = rotate_z(PI / 4.0);
    let full_quarter = rotate_z(PI / 2.0);
    let half_quarter_actual = half_quarter * point;
    let full_quarter_actual = full_quarter * point;
    let half_quarter_result = (2.0f64).sqrt() / 2.0;

    assert_eq!(Tuple::point(-half_quarter_result, half_quarter_result, 0.0), half_quarter_actual);
    assert_eq!(Tuple::point(-1.0, 0.0, 0.0), full_quarter_actual);
}

#[test]
fn shearing_test() {
    let sheared = shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    let point = Tuple::point(2.0, 3.0, 4.0);
    
    assert_eq!(Tuple::point(5.0, 3.0, 4.0), sheared * point);

    let sheared = shear(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
    let point = Tuple::point(2.0, 3.0, 4.0);
    
    assert_eq!(Tuple::point(6.0, 3.0, 4.0), sheared * point);

    let sheared = shear(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
    let point = Tuple::point(2.0, 3.0, 4.0);
    
    assert_eq!(Tuple::point(2.0, 5.0, 4.0), sheared * point);

    let sheared = shear(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
    let point = Tuple::point(2.0, 3.0, 4.0);
    
    assert_eq!(Tuple::point(2.0, 7.0, 4.0), sheared * point);

    let sheared = shear(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
    let point = Tuple::point(2.0, 3.0, 4.0);
    
    assert_eq!(Tuple::point(2.0, 3.0, 6.0), sheared * point);

    let sheared = shear(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
    let point = Tuple::point(2.0, 3.0, 4.0);
    
    assert_eq!(Tuple::point(2.0, 3.0, 7.0), sheared * point);
}

#[test]
fn chaining_transformations() {
    let p = Tuple::point(1.0, 0.0, 1.0);
    let a = rotate_x(PI / 2.0);
    let b = scale(5.0, 5.0, 5.0);
    let c = translate(10.0, 5.0, 7.0);

    let p2 = a * p;
    assert_eq!(Tuple::point(1.0, -1.0, 0.0), p2);

    let p3 = b * p2;
    assert_eq!(Tuple::point(5.0, -5.0, 0.0), p3);

    let p4 = c * p3;
    assert_eq!(Tuple::point(15.0, 0.0, 7.0), p4);
}

#[test]
fn chained_transformation_reverse_order() {
    let p = Tuple::point(1.0, 0.0, 1.0);
    let a = rotate_x(PI / 2.0);
    let b = scale(5.0, 5.0, 5.0);
    let c = translate(10.0, 5.0, 7.0);

    let t = c * b * a;
    
    assert_eq!(Tuple::point(15.0, 0.0, 7.0), t * p);
}

#[test]
fn fluent_api() {
    let p = Tuple::point(1.0, 0.0, 1.0);
    let transform = Matrix::identity(4).rotate_x(PI / 2.0).scale(5.0, 5.0, 5.0).translate(10.0, 5.0, 7.0);
    assert_eq!(Tuple::point(15.0, 0.0, 7.0), transform * p);
}