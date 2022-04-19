use std::f64::consts::PI;

use super::Sphere;
use crate::{
    matrix::transformation::{rotate_z, scale, translate},
    ray::Ray,
    shape::{Shape, ShapeType},
    tuple::Tuple,
};

#[test]
fn intersect_two_points_test() {
    let sphere = Shape::new(ShapeType::Sphere(Sphere::new()));
    let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));

    let xs = sphere.intersect(&ray);

    assert_eq!(2, xs.len());
    assert_eq!(4.0, xs[0].value);
    assert_eq!(6.0, xs[1].value);
}

#[test]
fn intersect_tangent_test() {
    let sphere = Shape::new(ShapeType::Sphere(Sphere::new()));
    let ray = Ray::new(Tuple::point(0.0, 1.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));

    let xs = sphere.intersect(&ray);

    assert_eq!(2, xs.len());
    assert_eq!(5.0, xs[0].value);
    assert_eq!(5.0, xs[1].value);
}

#[test]
fn intersect_miss_test() {
    let sphere = Shape::new(ShapeType::Sphere(Sphere::new()));
    let ray = Ray::new(Tuple::point(0.0, 2.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));

    let xs = sphere.intersect(&ray);

    assert_eq!(0, xs.len());
}

#[test]
fn intersect_inside_sphere_test() {
    let sphere = Shape::new(ShapeType::Sphere(Sphere::new()));
    let ray = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));

    let xs = sphere.intersect(&ray);

    assert_eq!(2, xs.len());
    assert_eq!(-1.0, xs[0].value);
    assert_eq!(1.0, xs[1].value);
}

#[test]
fn intersect_sphere_behind_ray_test() {
    let sphere = Shape::new(ShapeType::Sphere(Sphere::new()));
    let ray = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));

    let xs = sphere.intersect(&ray);

    assert_eq!(2, xs.len());
    assert_eq!(-6.0, xs[0].value);
    assert_eq!(-4.0, xs[1].value);
}

#[test]
fn intersect_sets_object() {
    let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let sphere = Shape::new(ShapeType::Sphere(Sphere::new()));
    let xs = sphere.intersect(&ray);

    assert_eq!(2, xs.len());
    // assert_eq!(&sphere, xs[0].object);
    // assert_eq!(&sphere, xs[1].object);
}

#[test]
fn intersect_scaled_sphere_test() {
    let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let mut sphere = Sphere::new();
    sphere.transform = scale(2.0, 2.0, 2.0);

    let shape = Shape::new(ShapeType::Sphere(sphere));
    let xs = shape.intersect(&ray);

    assert_eq!(2, xs.len());
    assert_eq!(3.0, xs[0].value);
    assert_eq!(7.0, xs[1].value);
}

#[test]
fn intersect_translated_sphere_test() {
    let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let mut sphere = Sphere::new();
    sphere.transform = translate(2.0, 2.0, 2.0);

    let shape = Shape::new(ShapeType::Sphere(sphere));
    let xs = shape.intersect(&ray);

    assert_eq!(0, xs.len());
}

#[test]
fn normal_x_axis_test() {
    let sphere = Shape::new(ShapeType::Sphere(Sphere::new()));

    let n = sphere.normal_at(Tuple::point(1.0, 0.0, 0.0));

    assert_eq!(Tuple::vector(1.0, 0.0, 0.0), n);
}

#[test]
fn normal_y_axis_test() {
    let sphere = Shape::new(ShapeType::Sphere(Sphere::new()));

    let n = sphere.normal_at(Tuple::point(0.0, 1.0, 0.0));

    assert_eq!(Tuple::vector(0.0, 1.0, 0.0), n);
}

#[test]
fn normal_z_axis_test() {
    let sphere = Shape::new(ShapeType::Sphere(Sphere::new()));

    let n = sphere.normal_at(Tuple::point(0.0, 0.0, 1.0));

    assert_eq!(Tuple::vector(0.0, 0.0, 1.0), n);
}

#[test]
fn normal_nonaxial_axis_test() {
    let sphere = Shape::new(ShapeType::Sphere(Sphere::new()));
    let value = (3.0_f64).sqrt() / 3.0;

    let n = sphere.normal_at(Tuple::point(value, value, value));

    assert_eq!(Tuple::vector(value, value, value), n);
}

#[test]
fn normal_is_normalized_vector_test() {
    let sphere = Shape::new(ShapeType::Sphere(Sphere::new()));
    let value = (3.0_f64).sqrt() / 3.0;

    let n = sphere.normal_at(Tuple::point(value, value, value));

    assert_eq!(n.normalize(), n);
}

#[test]
fn normal_translated_sphere_test() {
    let mut sphere = Sphere::new();
    sphere.transform = translate(0.0, 1.0, 0.0);

    let shape = Shape::new(ShapeType::Sphere(sphere));
    let normal = shape.normal_at(Tuple::point(0.0, 1.70711, -0.70711));

    assert_eq!(Tuple::vector(0.0, 0.70711, -0.70711), normal);
}

#[test]
fn normal_transformed_sphere_test() {
    let mut sphere = Sphere::new();
    let transformation = scale(1.0, 0.5, 1.0) * rotate_z(PI / 5.0);
    let value = (2.0_f64).sqrt() / 2.0;
    sphere.transform = transformation;

    let shape = Shape::new(ShapeType::Sphere(sphere));
    let normal = shape.normal_at(Tuple::point(0.0, value, -value));

    assert_eq!(Tuple::vector(0.0, 0.97014, -0.24254), normal);
}
