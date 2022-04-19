use crate::{
    ray::Ray,
    shapes::{Shape, ShapeType},
    tuple::Tuple,
};

use super::Plane;

#[test]
fn normal_at_test() {
    let plane = Shape::new(ShapeType::Plane(Plane::default()));

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

    assert_eq!(true, xs.is_none());
}

#[test]
fn intersect_ray_coplanar() {
    let plane: Plane = Default::default();
    let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));

    let xs = plane.local_intersect(&r);

    assert_eq!(true, xs.is_none());
}

#[test]
fn intersect_ray_from_above() {
    let plane: Plane = Default::default();
    let r = Ray::new(Tuple::point(0.0, 1.0, 0.0), Tuple::vector(0.0, -1.0, 0.0));

    let xs = plane.local_intersect(&r);

    assert_eq!(true, xs.is_some());

    let xsw = xs.unwrap();
    assert_eq!(1, xsw.len());
    assert_eq!(1.0, xsw[0]);
}

#[test]
fn intersect_ray_from_below() {
    let plane: Plane = Default::default();
    let r = Ray::new(Tuple::point(0.0, -1.0, 0.0), Tuple::vector(0.0, 1.0, 0.0));

    let xs = plane.local_intersect(&r);

    assert_eq!(true, xs.is_some());

    let xsw = xs.unwrap();
    assert_eq!(1, xsw.len());
    assert_eq!(1.0, xsw[0]);
}
