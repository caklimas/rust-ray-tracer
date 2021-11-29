use crate::{
    matrix::transformation::{scale, translate},
    plane::Plane,
    ray::Ray,
    sphere::Sphere,
    tuple::Tuple,
};

use super::{intersections::Intersections, Intersection};

pub mod intersection_computation;

#[test]
fn prepare_computations_test() {
    let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let shape: Sphere = Default::default();
    let intersection = Intersection::new(&shape, 4.0);

    let computations = intersection.prepare_computations(&ray, Option::None);

    assert_eq!(Tuple::point(0.0, 0.0, -1.0), computations.point);
    assert_eq!(Tuple::vector(0.0, 0.0, -1.0), computations.eye_v);
    assert_eq!(Tuple::vector(0.0, 0.0, -1.0), computations.normal_v);
}

#[test]
fn prepare_computations_intersect_occurs_outside() {
    let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let shape: Sphere = Default::default();
    let intersection = Intersection::new(&shape, 4.0);

    let computations = intersection.prepare_computations(&ray, Option::None);

    assert_eq!(false, computations.inside);
}

#[test]
fn prepare_computations_intersect_occurs_inside() {
    let ray = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
    let shape: Sphere = Default::default();
    let intersection = Intersection::new(&shape, 1.0);

    let computations = intersection.prepare_computations(&ray, Option::None);

    assert_eq!(Tuple::point(0.0, 0.0, 1.0), computations.point);
    assert_eq!(Tuple::vector(0.0, 0.0, -1.0), computations.eye_v);
    assert_eq!(true, computations.inside);
    assert_eq!(Tuple::vector(0.0, 0.0, -1.0), computations.normal_v);
}

#[test]
fn prepare_computations_reflection_vector() {
    let v = (2.0_f64).sqrt() / 2.0;
    let shape = Plane::default();
    let r = Ray::new(Tuple::point(0.0, 1.0, -1.0), Tuple::vector(0.0, -v, v));
    let i = Intersection::new(&shape, (2.0_f64).sqrt());

    let comps = i.prepare_computations(&r, Option::None);

    assert_eq!(Tuple::vector(0.0, v, v), comps.reflect_v);
}

#[test]
fn prepare_computations_finding_n1_and_n2() {
    let mut a = Sphere::glass();
    a.transform = scale(2.0, 2.0, 2.0);
    a.material.refractive_index = 1.5;
    let mut b = Sphere::glass();
    b.transform = translate(0.0, 0.0, -0.25);
    b.material.refractive_index = 2.0;
    let mut c = Sphere::glass();
    c.transform = translate(0.0, 0.0, 0.25);
    c.material.refractive_index = 2.5;
    let r = Ray::new(Tuple::point(0.0, 0.0, -4.0), Tuple::vector(0.0, 0.0, 1.0));
    let xs = Intersections::new(vec![
        Intersection::new(&a, 2.0),
        Intersection::new(&b, 2.75),
        Intersection::new(&c, 3.25),
        Intersection::new(&b, 4.75),
        Intersection::new(&c, 5.25),
        Intersection::new(&a, 6.0),
    ]);
    let n1s = [1.0, 1.5, 2.0, 2.5, 2.5, 1.5];
    let n2s = [1.5, 2.0, 2.5, 2.5, 1.5, 1.0];

    for x in 0..=5 {
        let comps = xs.collection[x].prepare_computations(&r, Option::Some(&xs));
        assert_eq!(n1s[x], comps.n1);
        assert_eq!(n2s[x], comps.n2);
    }
}
