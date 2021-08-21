use crate::sphere::Sphere;

use super::Intersection;

mod intersections;

#[test]
fn intersection_new_test() {
    let sphere = Sphere::new();
    let value = 3.5;
    let intersection = Intersection::new(&sphere, value);

    assert_eq!(&sphere, intersection.object);
    assert_eq!(value, intersection.value);
}