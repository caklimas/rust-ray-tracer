use std::f64::consts::FRAC_PI_2;

use crate::floating_point::FloatingPoint;

use super::Camera;

#[test]
fn pixel_size_horizontal_canvas_test() {
    let c = Camera::new(200, 125, FRAC_PI_2);

    assert_eq!(true, FloatingPoint::equals(0.01, *c.pixel_size()));
}

#[test]
fn pixel_size_vertical_canvas_test() {
    let c = Camera::new(125, 200, FRAC_PI_2);

    assert_eq!(true, FloatingPoint::equals(0.01, *c.pixel_size()));
}