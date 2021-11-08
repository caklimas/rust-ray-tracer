use crate::{plane::Plane, ray::Ray, sphere::Sphere, tuple::Tuple};

use super::Intersection;

pub mod intersection_computation;

#[test]
fn prepare_computations_test() {
    let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let shape: Sphere = Default::default();
    let intersection = Intersection::new(&shape, 4.0);

    let computations = intersection.prepare_computations(&ray);

    assert_eq!(Tuple::point(0.0, 0.0, -1.0), computations.point);
    assert_eq!(Tuple::vector(0.0, 0.0, -1.0), computations.eye_v);
    assert_eq!(Tuple::vector(0.0, 0.0, -1.0), computations.normal_v);
}

#[test]
fn prepare_computations_intersect_occurs_outside() {
    let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let shape: Sphere = Default::default();
    let intersection = Intersection::new(&shape, 4.0);

    let computations = intersection.prepare_computations(&ray);

    assert_eq!(false, computations.inside);
}

#[test]
fn prepare_computations_intersect_occurs_inside() {
    let ray = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
    let shape: Sphere = Default::default();
    let intersection = Intersection::new(&shape, 1.0);

    let computations = intersection.prepare_computations(&ray);

    assert_eq!(Tuple::point(0.0, 0.0, 1.0), computations.point);
    assert_eq!(Tuple::vector(0.0, 0.0, -1.0), computations.eye_v);
    assert_eq!(true, computations.inside);
    assert_eq!(Tuple::vector(0.0, 0.0, -1.0), computations.normal_v);
}

#[test]
fn prepare_computating_reflection_vector() {
    let v = (2.0_f64).sqrt() / 2.0;
    let shape = Plane::default();
    let r = Ray::new(Tuple::point(0.0, 1.0, -1.0), Tuple::vector(0.0, -v, v));
    let i = Intersection::new(&shape, (2.0_f64).sqrt());

    let comps = i.prepare_computations(&r);

    assert_eq!(Tuple::vector(0.0, v, v), comps.reflect_v);
}
