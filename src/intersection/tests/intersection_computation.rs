use crate::{intersection::intersection_computation::IntersectionComputation, tuple::Tuple};

#[test]
#[should_panic]
fn new_point_panic_test() {
    IntersectionComputation::new(
        &Default::default(),
        0.0,
        Tuple::vector(1.0, 1.0, 1.0),
        Tuple::vector(1.0, 1.0, 1.0),
        Tuple::vector(1.0, 1.0, 1.0),
        true,
    );
}

#[test]
#[should_panic]
fn new_eye_v_panic_test() {
    IntersectionComputation::new(
        &Default::default(),
        0.0,
        Tuple::point(1.0, 1.0, 1.0),
        Tuple::point(1.0, 1.0, 1.0),
        Tuple::vector(1.0, 1.0, 1.0),
        true,
    );
}

#[test]
#[should_panic]
fn new_normal_v_panic_test() {
    IntersectionComputation::new(
        &Default::default(),
        0.0,
        Tuple::point(1.0, 1.0, 1.0),
        Tuple::vector(1.0, 1.0, 1.0),
        Tuple::point(1.0, 1.0, 1.0),
        true,
    );
}

#[test]
fn new_panic_test() {
    IntersectionComputation::new(
        &Default::default(),
        0.0,
        Tuple::point(1.0, 1.0, 1.0),
        Tuple::vector(1.0, 1.0, 1.0),
        Tuple::vector(1.0, 1.0, 1.0),
        true,
    );
}
