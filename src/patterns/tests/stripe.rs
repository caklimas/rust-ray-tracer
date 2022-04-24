use crate::{
    color::Color,
    matrix::transformation::{scale, translate},
    patterns::{stripe::StripePattern, Pattern, PatternType},
    shapes::{sphere::Sphere, Shape, ShapeType},
    tuple::Tuple,
};

#[test]
fn stripe_constant_in_y() {
    let pattern = Pattern::new(PatternType::Stripe(StripePattern::new(
        Color::white(),
        Color::black(),
    )));

    let a1 = pattern.color_at(&Tuple::point(0.0, 0.0, 0.0));
    let a2 = pattern.color_at(&Tuple::point(0.0, 1.0, 0.0));
    let a3 = pattern.color_at(&Tuple::point(0.0, 2.0, 0.0));

    assert_eq!(Color::white(), a1);
    assert_eq!(Color::white(), a2);
    assert_eq!(Color::white(), a3);
}

#[test]
fn stripe_constant_in_z() {
    let pattern = Pattern::new(PatternType::Stripe(StripePattern::new(
        Color::white(),
        Color::black(),
    )));

    let a1 = pattern.color_at(&Tuple::point(0.0, 0.0, 0.0));
    let a2 = pattern.color_at(&Tuple::point(0.0, 0.0, 1.0));
    let a3 = pattern.color_at(&Tuple::point(0.0, 0.0, 2.0));

    assert_eq!(Color::white(), a1);
    assert_eq!(Color::white(), a2);
    assert_eq!(Color::white(), a3);
}

#[test]
fn stripe_alternates_in_x() {
    let pattern = Pattern::new(PatternType::Stripe(StripePattern::new(
        Color::white(),
        Color::black(),
    )));

    let a1 = pattern.color_at(&Tuple::point(0.0, 0.0, 0.0));
    let a2 = pattern.color_at(&Tuple::point(0.9, 0.0, 0.0));
    let a3 = pattern.color_at(&Tuple::point(1.0, 0.0, 0.0));
    let a4 = pattern.color_at(&Tuple::point(-0.1, 0.0, 0.0));
    let a5 = pattern.color_at(&Tuple::point(-1.0, 0.0, 0.0));
    let a6 = pattern.color_at(&Tuple::point(-1.1, 0.0, 0.0));

    assert_eq!(Color::white(), a1);
    assert_eq!(Color::white(), a2);
    assert_eq!(Color::black(), a3);
    assert_eq!(Color::black(), a4);
    assert_eq!(Color::black(), a5);
    assert_eq!(Color::white(), a6);
}

#[test]
fn stripe_object_transformation() {
    let pattern = Pattern::new(PatternType::Stripe(StripePattern::new(
        Color::white(),
        Color::black(),
    )));
    let mut object = Sphere::default();
    object.transform = scale(2.0, 2.0, 2.0);

    let shape = Shape::new(ShapeType::Sphere(object));
    let c = pattern.color_at_object(&shape, &Tuple::point(1.5, 0.0, 0.0));

    assert_eq!(Color::white(), c);
}

#[test]
fn stripe_pattern_transformation() {
    let object = Sphere::default();
    let mut pattern = Pattern::new(PatternType::Stripe(StripePattern::new(
        Color::white(),
        Color::black(),
    )));
    pattern.transform = scale(2.0, 2.0, 2.0);

    let shape = Shape::new(ShapeType::Sphere(object));
    let c = pattern.color_at_object(&shape, &Tuple::point(1.5, 0.0, 0.0));

    assert_eq!(Color::white(), c);
}

#[test]
fn stripe_object_pattern_transformation() {
    let mut object = Sphere::default();
    object.transform = scale(2.0, 2.0, 2.0);
    let mut pattern = Pattern::new(PatternType::Stripe(StripePattern::new(
        Color::white(),
        Color::black(),
    )));
    pattern.transform = translate(0.5, 0.0, 0.0);

    let shape = Shape::new(ShapeType::Sphere(object));
    let c = pattern.color_at_object(&shape, &Tuple::point(1.5, 0.0, 0.0));

    assert_eq!(Color::white(), c);
}
