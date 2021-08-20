use crate::{ray::Ray, tuple::Tuple};
use super::Sphere;

#[test]
fn intersect_two_points_test() {
    let sphere = Sphere::new();
    let ray = Ray::new(
        Tuple::point(0.0, 0.0, -5.0),
        Tuple::vector(0.0, 0.0, 1.0)
    );

    let xs = sphere.intersect(&ray);

    assert_eq!(2, xs.len());
    assert_eq!(4.0, xs[0]);
    assert_eq!(6.0, xs[1]);
}

#[test]
fn intersect_tangent_test() {
    let sphere = Sphere::new();
    let ray = Ray::new(
        Tuple::point(0.0, 1.0, -5.0),
        Tuple::vector(0.0, 0.0, 1.0)
    );

    let xs = sphere.intersect(&ray);

    assert_eq!(2, xs.len());
    assert_eq!(5.0, xs[0]);
    assert_eq!(5.0, xs[1]);
}

#[test]
fn intersect_miss_test() {
    let sphere = Sphere::new();
    let ray = Ray::new(
        Tuple::point(0.0, 2.0, -5.0),
        Tuple::vector(0.0, 0.0, 1.0)
    );

    let xs = sphere.intersect(&ray);

    assert_eq!(0, xs.len());
}

#[test]
fn intersect_inside_sphere_test() {
    let sphere = Sphere::new();
    let ray = Ray::new(
        Tuple::point(0.0, 0.0, 0.0),
        Tuple::vector(0.0, 0.0, 1.0)
    );

    let xs = sphere.intersect(&ray);

    assert_eq!(2, xs.len());
    assert_eq!(-1.0, xs[0]);
    assert_eq!(1.0, xs[1]);
}

#[test]
fn intersect_sphere_behind_ray_test() {
    let sphere = Sphere::new();
    let ray = Ray::new(
        Tuple::point(0.0, 0.0, 5.0),
        Tuple::vector(0.0, 0.0, 1.0)
    );

    let xs = sphere.intersect(&ray);

    assert_eq!(2, xs.len());
    assert_eq!(-6.0, xs[0]);
    assert_eq!(-4.0, xs[1]);
}