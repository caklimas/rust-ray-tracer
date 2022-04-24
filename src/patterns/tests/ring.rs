use crate::{
    color::Color,
    patterns::{ring::RingPattern, Pattern},
    tuple::Tuple,
};

#[test]
fn ring_should_extend_in_x_z_dimensions() {
    let pattern = Pattern::new(crate::patterns::PatternType::Ring(RingPattern::new(
        Color::white(),
        Color::black(),
    )));

    let a1 = pattern.color_at(&Tuple::point(0.0, 0.0, 0.0));
    let a2 = pattern.color_at(&Tuple::point(1.0, 0.0, 0.0));
    let a3 = pattern.color_at(&Tuple::point(0.0, 0.0, 1.0));
    let a4 = pattern.color_at(&Tuple::point(0.708, 0.0, 0.708));

    assert_eq!(Color::white(), a1);
    assert_eq!(Color::black(), a2);
    assert_eq!(Color::black(), a3);
    assert_eq!(Color::black(), a4);
}
