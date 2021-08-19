use crate::tuple::Tuple;

use super::Ray;

#[test]
fn new_test() {
    let origin = Tuple::point(1.0, 2.0, 3.0);
    let direction = Tuple::vector(4.0, 5.0, 6.0);
    let ray = Ray::new(origin, direction);

    assert_eq!(origin, ray.origin);
    assert_eq!(direction, ray.direction);
}

#[test]
#[should_panic]
fn new_origin_panic_test() {
    Ray::new(Tuple::vector(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 0.0));
}

#[test]
#[should_panic]
fn new_direction_panic_test() {
    Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::point(0.0, 0.0, 0.0));
}

#[test]
fn position_test() {
    let ray = Ray::new(Tuple::point(2.0, 3.0, 4.0), Tuple::vector(1.0, 0.0, 0.0));
    
    assert_eq!(Tuple::point(2.0, 3.0, 4.0), ray.position(0.0));
    assert_eq!(Tuple::point(3.0, 3.0, 4.0), ray.position(1.0));
    assert_eq!(Tuple::point(1.0, 3.0, 4.0), ray.position(-1.0));
    assert_eq!(Tuple::point(4.5, 3.0, 4.0), ray.position(2.5));
}