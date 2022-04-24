use std::f64::consts::{FRAC_PI_2, FRAC_PI_4};

use crate::{
    color::Color,
    floating_point::FloatingPoint,
    matrix::transformation::{rotate_y, translate},
    test::sqrt_2_div_2,
    tuple::Tuple,
    world::{view_transform, World},
};

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

#[test]
fn ray_for_pixel_center_of_canvas_test() {
    let c = Camera::new(201, 101, FRAC_PI_2);

    let r = c.ray_for_pixel(100, 50);

    assert_eq!(Tuple::point(0.0, 0.0, 0.0), r.origin);
    assert_eq!(Tuple::vector(0.0, 0.0, -1.0), r.direction);
}

#[test]
fn ray_for_pixel_corner_of_canvas_test() {
    let c = Camera::new(201, 101, FRAC_PI_2);

    let r = c.ray_for_pixel(0, 0);

    assert_eq!(Tuple::point(0.0, 0.0, 0.0), r.origin);
    assert_eq!(Tuple::vector(0.66519, 0.33259, -0.66851), r.direction);
}

#[test]
fn ray_for_pixel_camera_transformed_test() {
    let mut c = Camera::new(201, 101, FRAC_PI_2);
    c.transform = rotate_y(FRAC_PI_4) * translate(0.0, -2.0, 5.0);
    let x_z_result = sqrt_2_div_2();

    let r = c.ray_for_pixel(100, 50);

    assert_eq!(Tuple::point(0.0, 2.0, -5.0), r.origin);
    assert_eq!(Tuple::vector(x_z_result, 0.0, -x_z_result), r.direction);
}

#[test]
fn render_test() {
    let world: World = Default::default();
    let mut camera = Camera::new(11, 11, FRAC_PI_2);
    let from = Tuple::point(0.0, 0.0, -5.0);
    let to = Tuple::point(0.0, 0.0, 0.0);
    let up = Tuple::vector(0.0, 1.0, 0.0);
    camera.transform = view_transform(&from, &to, &up);

    let image = camera.render(&world);

    assert_eq!(Color::new(0.38066, 0.47583, 0.2855), image.pixels[5][5]);
}
