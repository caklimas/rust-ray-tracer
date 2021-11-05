use crate::{
    color::Color,
    patterns::{gradient::Gradient, Pattern},
    tuple::Tuple,
};

#[test]
fn color_at_linearly_interpolates_between_colors() {
    let gradient = Gradient::new(Color::white(), Color::black());

    let c1 = gradient.color_at(&Tuple::point(0.0, 0.0, 0.0));
    let c2 = gradient.color_at(&Tuple::point(0.25, 0.0, 0.0));
    let c3 = gradient.color_at(&Tuple::point(0.5, 0.0, 0.0));
    let c4 = gradient.color_at(&Tuple::point(0.75, 0.0, 0.0));

    assert_eq!(Color::white(), c1);
    assert_eq!(Color::new(0.75, 0.75, 0.75), c2);
    assert_eq!(Color::new(0.5, 0.5, 0.5), c3);
    assert_eq!(Color::new(0.25, 0.25, 0.25), c4);
}
