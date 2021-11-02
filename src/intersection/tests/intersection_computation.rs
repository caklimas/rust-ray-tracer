use crate::{
    intersection::intersection_computation::IntersectionComputation, sphere::Sphere, tuple::Tuple,
};

#[test]
#[should_panic]
fn new_point_panic_test() {
    let shape: Sphere = Default::default();
    IntersectionComputation::new(
        &shape,
        0.0,
        Tuple::vector(1.0, 1.0, 1.0),
        Tuple::vector(1.0, 1.0, 1.0),
        Tuple::vector(1.0, 1.0, 1.0),
        true,
        Tuple::point(1.0, 1.0, 1.0),
    );
}

#[test]
#[should_panic]
fn new_eye_v_panic_test() {
    let shape: Sphere = Default::default();
    IntersectionComputation::new(
        &shape,
        0.0,
        Tuple::point(1.0, 1.0, 1.0),
        Tuple::point(1.0, 1.0, 1.0),
        Tuple::vector(1.0, 1.0, 1.0),
        true,
        Tuple::point(1.0, 1.0, 1.0),
    );
}

#[test]
#[should_panic]
fn new_normal_v_panic_test() {
    let shape: Sphere = Default::default();
    IntersectionComputation::new(
        &shape,
        0.0,
        Tuple::point(1.0, 1.0, 1.0),
        Tuple::vector(1.0, 1.0, 1.0),
        Tuple::point(1.0, 1.0, 1.0),
        true,
        Tuple::point(1.0, 1.0, 1.0),
    );
}

#[test]
#[should_panic]
fn new_over_point_panic_test() {
    let shape: Sphere = Default::default();
    IntersectionComputation::new(
        &shape,
        0.0,
        Tuple::point(1.0, 1.0, 1.0),
        Tuple::vector(1.0, 1.0, 1.0),
        Tuple::vector(1.0, 1.0, 1.0),
        true,
        Tuple::vector(1.0, 1.0, 1.0),
    );
}

#[test]
fn new_test() {
    let shape: Sphere = Default::default();
    IntersectionComputation::new(
        &shape,
        0.0,
        Tuple::point(1.0, 1.0, 1.0),
        Tuple::vector(1.0, 1.0, 1.0),
        Tuple::vector(1.0, 1.0, 1.0),
        true,
        Tuple::point(1.0, 1.0, 1.0),
    );
}
