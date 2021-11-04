use crate::{
    color::Color,
    patterns::{stripe::Stripe, Pattern},
    tuple::Tuple,
};

#[test]
fn stripe_constant_in_y() {
    let pattern = Stripe::new(Color::white(), Color::black());

    let a1 = pattern.color_at(&Tuple::point(0.0, 0.0, 0.0));
    let a2 = pattern.color_at(&Tuple::point(0.0, 1.0, 0.0));
    let a3 = pattern.color_at(&Tuple::point(0.0, 2.0, 0.0));

    assert_eq!(Color::white(), a1);
    assert_eq!(Color::white(), a2);
    assert_eq!(Color::white(), a3);
}

#[test]
fn stripe_constant_in_z() {
    let pattern = Stripe::new(Color::white(), Color::black());

    let a1 = pattern.color_at(&Tuple::point(0.0, 0.0, 0.0));
    let a2 = pattern.color_at(&Tuple::point(0.0, 0.0, 1.0));
    let a3 = pattern.color_at(&Tuple::point(0.0, 0.0, 2.0));

    assert_eq!(Color::white(), a1);
    assert_eq!(Color::white(), a2);
    assert_eq!(Color::white(), a3);
}

#[test]
fn stripe_alternates_in_x() {
    let pattern = Stripe::new(Color::white(), Color::black());

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
