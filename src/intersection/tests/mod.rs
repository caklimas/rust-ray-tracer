use crate::sphere::Sphere;

use super::Intersection;

#[test]
fn intersection_new_test() {
    let sphere = Sphere::new();
    let value = 2.0;
    let intersection = Intersection::new(sphere, value);

    assert_eq!(sphere, intersection.object);
}