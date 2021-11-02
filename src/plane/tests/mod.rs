use crate::{shape::Shape, tuple::Tuple};

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
