use crate::{ray::Ray, tuple::Tuple};

use super::World;

#[test]
fn intersect_test() {
    let world: World = Default::default();
    let xs = world.intersect(Ray::new(
        Tuple::point(0.0, 0.0, -5.0),
        Tuple::vector(0.0, 0.0, 1.0),
    ));

    assert_eq!(4, xs.len());
    assert_eq!(4.0, xs[0].value);
    assert_eq!(4.5, xs[1].value);
    assert_eq!(5.5, xs[2].value);
    assert_eq!(6.0, xs[3].value);
}
