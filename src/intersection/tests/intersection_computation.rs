use crate::{
    intersection::intersection_computation::{
        IntersectionComputation, IntersectionComputationConfig,
    },
    sphere::Sphere,
    tuple::Tuple,
};

#[test]
#[should_panic]
fn new_point_panic_test() {
    let shape: Sphere = Default::default();
    IntersectionComputation::new(
        &shape,
        0.0,
        IntersectionComputationConfig {
            point: Tuple::vector(1.0, 1.0, 1.0),
            eye_v: Tuple::vector(1.0, 1.0, 1.0),
            normal_v: Tuple::vector(1.0, 1.0, 1.0),
            inside: true,
            over_point: Tuple::point(1.0, 1.0, 1.0),
            reflect_v: Tuple::vector(0.0, 0.0, 0.0),
            n1: 1.0,
            n2: 1.0,
            under_point: Tuple::point(1.0, 1.0, 1.0),
        },
    );
}

#[test]
#[should_panic]
fn new_eye_v_panic_test() {
    let shape: Sphere = Default::default();
    IntersectionComputation::new(
        &shape,
        0.0,
        IntersectionComputationConfig {
            point: Tuple::point(1.0, 1.0, 1.0),
            eye_v: Tuple::point(1.0, 1.0, 1.0),
            normal_v: Tuple::vector(1.0, 1.0, 1.0),
            inside: true,
            over_point: Tuple::point(1.0, 1.0, 1.0),
            reflect_v: Tuple::vector(0.0, 0.0, 0.0),
            n1: 1.0,
            n2: 1.0,
            under_point: Tuple::point(1.0, 1.0, 1.0),
        },
    );
}

#[test]
#[should_panic]
fn new_normal_v_panic_test() {
    let shape: Sphere = Default::default();
    IntersectionComputation::new(
        &shape,
        0.0,
        IntersectionComputationConfig {
            point: Tuple::point(1.0, 1.0, 1.0),
            eye_v: Tuple::vector(1.0, 1.0, 1.0),
            normal_v: Tuple::point(1.0, 1.0, 1.0),
            inside: true,
            over_point: Tuple::point(1.0, 1.0, 1.0),
            reflect_v: Tuple::vector(0.0, 0.0, 0.0),
            n1: 1.0,
            n2: 1.0,
            under_point: Tuple::point(1.0, 1.0, 1.0),
        },
    );
}

#[test]
#[should_panic]
fn new_over_point_panic_test() {
    let shape: Sphere = Default::default();
    IntersectionComputation::new(
        &shape,
        0.0,
        IntersectionComputationConfig {
            point: Tuple::point(1.0, 1.0, 1.0),
            eye_v: Tuple::vector(1.0, 1.0, 1.0),
            normal_v: Tuple::vector(1.0, 1.0, 1.0),
            inside: true,
            over_point: Tuple::vector(1.0, 1.0, 1.0),
            reflect_v: Tuple::vector(0.0, 0.0, 0.0),
            n1: 1.0,
            n2: 1.0,
            under_point: Tuple::point(1.0, 1.0, 1.0),
        },
    );
}

#[test]
#[should_panic]
fn new_reflect_v_panic_test() {
    let shape: Sphere = Default::default();
    IntersectionComputation::new(
        &shape,
        0.0,
        IntersectionComputationConfig {
            point: Tuple::point(1.0, 1.0, 1.0),
            eye_v: Tuple::vector(1.0, 1.0, 1.0),
            normal_v: Tuple::vector(1.0, 1.0, 1.0),
            inside: true,
            over_point: Tuple::point(1.0, 1.0, 1.0),
            reflect_v: Tuple::point(1.0, 1.0, 1.0),
            n1: 1.0,
            n2: 1.0,
            under_point: Tuple::point(1.0, 1.0, 1.0),
        },
    );
}

#[test]
fn new_test() {
    let shape: Sphere = Default::default();
    IntersectionComputation::new(
        &shape,
        0.0,
        IntersectionComputationConfig {
            point: Tuple::point(1.0, 1.0, 1.0),
            eye_v: Tuple::vector(1.0, 1.0, 1.0),
            normal_v: Tuple::vector(1.0, 1.0, 1.0),
            inside: true,
            over_point: Tuple::point(1.0, 1.0, 1.0),
            reflect_v: Tuple::vector(0.0, 0.0, 0.0),
            n1: 1.0,
            n2: 1.0,
            under_point: Tuple::point(1.0, 1.0, 1.0),
        },
    );
}
