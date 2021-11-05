use crate::{
    color::Color,
    patterns::{checker::Checker, Pattern},
    tuple::Tuple,
};

#[test]
fn color_at_should_repeat_in_x() {
    let pattern = Checker::new(Color::white(), Color::black());

    let a1 = pattern.color_at(&Tuple::point(0.0, 0.0, 0.0));
    let a2 = pattern.color_at(&Tuple::point(0.99, 0.0, 0.0));
    let a3 = pattern.color_at(&Tuple::point(1.01, 0.0, 0.0));

    assert_eq!(Color::white(), a1);
    assert_eq!(Color::white(), a2);
    assert_eq!(Color::black(), a3);
}

#[test]
fn color_at_should_repeat_in_y() {
    let pattern = Checker::new(Color::white(), Color::black());

    let a1 = pattern.color_at(&Tuple::point(0.0, 0.0, 0.0));
    let a2 = pattern.color_at(&Tuple::point(0.0, 0.99, 0.0));
    let a3 = pattern.color_at(&Tuple::point(0.0, 1.01, 0.0));

    assert_eq!(Color::white(), a1);
    assert_eq!(Color::white(), a2);
    assert_eq!(Color::black(), a3);
}

#[test]
fn color_at_should_repeat_in_z() {
    let pattern = Checker::new(Color::white(), Color::black());

    let a1 = pattern.color_at(&Tuple::point(0.0, 0.0, 0.0));
    let a2 = pattern.color_at(&Tuple::point(0.0, 0.0, 0.99));
    let a3 = pattern.color_at(&Tuple::point(0.0, 0.0, 1.01));

    assert_eq!(Color::white(), a1);
    assert_eq!(Color::white(), a2);
    assert_eq!(Color::black(), a3);
}
