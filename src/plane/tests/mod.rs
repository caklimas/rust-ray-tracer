use crate::{ray::Ray, shape::Shape, tuple::Tuple};

use super::Plane;

#[test]
fn normal_at_test() {
    let plane: Plane = Default::default();

    let n1 = plane.normal_at(Tuple::point(0.0, 0.0, 0.0));
    let n2 = plane.normal_at(Tuple::point(10.0, 0.0, -10.0));
    let n3 = plane.normal_at(Tuple::point(-5.0, 0.0, 150.0));

    assert_eq!(Tuple::vector(0.0, 1.0, 0.0), n1);
    assert_eq!(Tuple::vector(0.0, 1.0, 0.0), n2);
    assert_eq!(Tuple::vector(0.0, 1.0, 0.0), n3);
}

#[test]
fn intersect_ray_parallel() {
    let plane: Plane = Default::default();
    let r = Ray::new(Tuple::point(0.0, 10.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));

    let xs = plane.local_intersect(&r);

    assert_eq!(0, xs.len());
}

#[test]
fn intersect_ray_coplanar() {
    let plane: Plane = Default::default();
    let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));

    let xs = plane.local_intersect(&r);

    assert_eq!(0, xs.len());
}

#[test]
fn intersect_ray_from_above() {
    let plane: Plane = Default::default();
    let r = Ray::new(Tuple::point(0.0, 1.0, 0.0), Tuple::vector(0.0, -1.0, 0.0));

    let xs = plane.local_intersect(&r);

    assert_eq!(1, xs.len());
    assert_eq!(1.0, xs[0].value);
}

#[test]
fn intersect_ray_from_below() {
    let plane: Plane = Default::default();
    let r = Ray::new(Tuple::point(0.0, -1.0, 0.0), Tuple::vector(0.0, 1.0, 0.0));

    let xs = plane.local_intersect(&r);

    assert_eq!(1, xs.len());
    assert_eq!(1.0, xs[0].value);
}
