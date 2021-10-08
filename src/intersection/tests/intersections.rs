use crate::{
    intersection::{intersections::Intersections, Intersection},
    sphere::Sphere,
};

#[test]
fn new_test() {
    let sphere = Sphere::new();
    let v1 = 1.0;
    let v2 = 2.0;
    let i1 = Intersection::new(&sphere, v1);
    let i2 = Intersection::new(&sphere, v2);
    let mut v = vec![i1, i2];
    let xs = Intersections::new(&mut v);

    assert_eq!(2, xs.collection.len());
    assert_eq!(v1, xs.collection[0].value);
    assert_eq!(v2, xs.collection[1].value);
}

#[test]
fn hit_all_positive_test() {
    let sphere = Sphere::new();
    let i1 = Intersection::new(&sphere, 1.0);
    let i2 = Intersection::new(&sphere, 2.0);
    let expected = i1.clone();
    let intersections = Intersections::new(&mut vec![i1, i2]);

    let hit = intersections.hit();

    assert_eq!(true, hit.is_some());
    assert_eq!(&expected, hit.unwrap());
}

#[test]
fn hit_some_negative_test() {
    let sphere = Sphere::new();
    let i1 = Intersection::new(&sphere, 1.0);
    let i2 = Intersection::new(&sphere, -1.0);
    let expected = i1.clone();
    let intersections = Intersections::new(&mut vec![i1, i2]);

    let hit = intersections.hit();

    assert_eq!(true, hit.is_some());
    assert_eq!(&expected, hit.unwrap());
}

#[test]
fn hit_all_negative_test() {
    let sphere = Sphere::new();
    let i1 = Intersection::new(&sphere, -2.0);
    let i2 = Intersection::new(&sphere, -1.0);
    let intersections = Intersections::new(&mut vec![i1, i2]);

    let hit = intersections.hit();

    assert_eq!(true, hit.is_none());
}

#[test]
fn hit_lowest_non_negative_test() {
    let sphere = Sphere::new();
    let i1 = Intersection::new(&sphere, 5.0);
    let i2 = Intersection::new(&sphere, 7.0);
    let i3 = Intersection::new(&sphere, -3.0);
    let i4 = Intersection::new(&sphere, 2.0);
    let expected = i4.clone();
    let intersections = Intersections::new(&mut vec![i1, i2, i3, i4]);

    let hit = intersections.hit();

    assert_eq!(true, hit.is_some());
    assert_eq!(&expected, hit.unwrap());
}
