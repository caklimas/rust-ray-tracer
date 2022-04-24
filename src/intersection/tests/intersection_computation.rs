use crate::{
    floating_point::FloatingPoint,
    intersection::{
        intersection_computation::{IntersectionComputation, IntersectionComputationConfig},
        intersections::Intersections,
        prepare_computation::PrepareComputationConfig,
        Intersection,
    },
    ray::Ray,
    shapes::{sphere::Sphere, Shape, ShapeType},
    test::sqrt_2_div_2,
    tuple::Tuple,
};

#[test]
#[should_panic]
fn new_point_panic_test() {
    let shape = Default::default();
    IntersectionComputation::new(
        &Shape::new(ShapeType::Sphere(shape)),
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
    let shape = Default::default();
    IntersectionComputation::new(
        &Shape::new(ShapeType::Sphere(shape)),
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
    let shape = Default::default();
    IntersectionComputation::new(
        &Shape::new(ShapeType::Sphere(shape)),
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
    let shape = Default::default();
    IntersectionComputation::new(
        &Shape::new(ShapeType::Sphere(shape)),
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
    let shape = Sphere::default();
    IntersectionComputation::new(
        &Shape::new(ShapeType::Sphere(shape)),
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
    let shape = Default::default();
    IntersectionComputation::new(
        &Shape::new(ShapeType::Sphere(shape)),
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

#[test]
fn schlick_approx_under_total_internal_reflection() {
    let value = sqrt_2_div_2();
    let shape = Shape::new(ShapeType::Sphere(Sphere::glass()));
    let r = Ray::new(Tuple::point(0.0, 0.0, value), Tuple::vector(0.0, 1.0, 0.0));
    let xs = Intersections::new(vec![
        Intersection::new(&shape, -value),
        Intersection::new(&shape, value),
    ]);
    let comps = xs.collection[1]
        .prepare_computations(&r, Option::Some(&PrepareComputationConfig::new(&xs)));

    let reflectance = comps.schlick();

    assert_eq!(true, FloatingPoint::equals(1.0, reflectance));
}

#[test]
fn schlick_perpendicular_ray() {
    let shape = Shape::new(ShapeType::Sphere(Sphere::glass()));
    let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 1.0, 0.0));
    let xs = Intersections::new(vec![
        Intersection::new(&shape, -1.0),
        Intersection::new(&shape, 1.0),
    ]);
    let comps = xs.collection[1]
        .prepare_computations(&r, Option::Some(&PrepareComputationConfig::new(&xs)));

    let reflectance = comps.schlick();

    assert_eq!(true, FloatingPoint::equals(0.04, reflectance));
}

#[test]
fn schlick_n2_greater_n1() {
    let shape = Shape::new(ShapeType::Sphere(Sphere::glass()));
    let r = Ray::new(Tuple::point(0.0, 0.99, -2.0), Tuple::vector(0.0, 0.0, 1.0));
    let xs = Intersections::new(vec![Intersection::new(&shape, 1.8589)]);
    let comps = xs.collection[0]
        .prepare_computations(&r, Option::Some(&PrepareComputationConfig::new(&xs)));

    let reflectance = comps.schlick();

    assert_eq!(true, FloatingPoint::equals(0.48873, reflectance));
}
