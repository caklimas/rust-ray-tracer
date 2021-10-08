use crate::{ray::Ray, sphere::Sphere, tuple::Tuple};

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
