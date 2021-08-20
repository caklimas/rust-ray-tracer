use crate::floating_point::{FloatingPoint};

use super::Color;

#[test]
fn new_test() {
    let red = 0.1;
    let green = 2.3;
    let blue = 1.2;
    let color = Color::new(red, green, blue);

    assert_eq!(true, FloatingPoint::equals(red, color.red()));
    assert_eq!(true, FloatingPoint::equals(blue, color.blue()));
    assert_eq!(true, FloatingPoint::equals(green, color.green()));
}

#[test]
fn add_test() {
    let color = Color::new(0.9, 0.6, 0.75);
    let other = Color::new(0.7, 0.1, 0.25);

    let result = color + other;
    assert_eq!(true, FloatingPoint::equals(1.6, result.red()));
    assert_eq!(true, FloatingPoint::equals(0.7, result.green()));
    assert_eq!(true, FloatingPoint::equals(1.0, result.blue()));
}

#[test]
fn sub_test() {
    let color = Color::new(0.9, 0.6, 0.75);
    let other = Color::new(0.7, 0.1, 0.25);

    let result = color.sub(&other);
    assert_eq!(true, FloatingPoint::equals(0.2, result.red()));
    assert_eq!(true, FloatingPoint::equals(0.5, result.green()));
    assert_eq!(true, FloatingPoint::equals(0.5, result.blue()));
}

#[test]
fn multiply_test() {
    let color = Color::new(0.2, 0.3, 0.4);

    let result = color.multiply(2.0);
    assert_eq!(true, FloatingPoint::equals(0.4, result.red()));
    assert_eq!(true, FloatingPoint::equals(0.6, result.green()));
    assert_eq!(true, FloatingPoint::equals(0.8, result.blue()));
}

#[test]
fn multiply_color_test() {
    let color = Color::new(1.0, 0.2, 0.4);
    let other = Color::new(0.9, 1.0, 0.1);

    let result = color.multiply_color(&other);
    assert_eq!(true, FloatingPoint::equals(0.9, result.red()));
    assert_eq!(true, FloatingPoint::equals(0.2, result.green()));
    assert_eq!(true, FloatingPoint::equals(0.04, result.blue()));
}