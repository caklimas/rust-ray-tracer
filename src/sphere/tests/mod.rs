use crate::{matrix::transformation::{scale, translate}, ray::Ray, tuple::Tuple};
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
    assert_eq!(4.0, xs[0].value);
    assert_eq!(6.0, xs[1].value);
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
    assert_eq!(5.0, xs[0].value);
    assert_eq!(5.0, xs[1].value);
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
    assert_eq!(-1.0, xs[0].value);
    assert_eq!(1.0, xs[1].value);
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
    assert_eq!(-6.0, xs[0].value);
    assert_eq!(-4.0, xs[1].value);
}

#[test]
fn intersect_sets_object() {
    let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let sphere = Sphere::new();
    let xs = sphere.intersect(&ray);

    assert_eq!(2, xs.len());
    assert_eq!(&sphere, xs[0].object);
    assert_eq!(&sphere, xs[1].object);
}

#[test]
fn intersect_scaled_sphere_test() {
    let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let mut sphere = Sphere::new();
    sphere.transform = scale(2.0, 2.0, 2.0);

    let xs = sphere.intersect(&ray);

    assert_eq!(2, xs.len());
    assert_eq!(3.0, xs[0].value);
    assert_eq!(7.0, xs[1].value);
}

#[test]
fn intersect_translated_sphere_test() {
    let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let mut sphere = Sphere::new();
    sphere.transform = translate(2.0, 2.0, 2.0);

    let xs = sphere.intersect(&ray);

    assert_eq!(0, xs.len());
}

#[test]
fn normal_x_axis_test() {
    let sphere = Sphere::new();
    
    let n = sphere.normal_at(Tuple::point(1.0, 0.0, 0.0));

    assert_eq!(Tuple::vector(1.0, 0.0, 0.0), n);
}

#[test]
fn normal_y_axis_test() {
    let sphere = Sphere::new();
    
    let n = sphere.normal_at(Tuple::point(0.0, 1.0, 0.0));

    assert_eq!(Tuple::vector(0.0, 1.0, 0.0), n);
}

#[test]
fn normal_z_axis_test() {
    let sphere = Sphere::new();
    
    let n = sphere.normal_at(Tuple::point(0.0, 0.0, 1.0));

    assert_eq!(Tuple::vector(0.0, 0.0, 1.0), n);
}

#[test]
fn normal_nonaxial_axis_test() {
    let sphere = Sphere::new();
    let value = (3.0_f64).sqrt() / 3.0;
    
    let n = sphere.normal_at(Tuple::point(value, value, value));

    assert_eq!(Tuple::vector(value, value, value), n);
}

#[test]
fn normal_is_normalized_vector_test() {
    let sphere = Sphere::new();
    let value = (3.0_f64).sqrt() / 3.0;
    
    let n = sphere.normal_at(Tuple::point(value, value, value));

    assert_eq!(n.normalize(), n);
}