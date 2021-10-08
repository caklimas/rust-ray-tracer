use crate::{color::Color, tuple::Tuple};

use super::PointLight;

#[test]
fn new_test() {
    let intensity = Color::new(1.0, 1.0, 1.0);
    let position = Tuple::point(0.0, 0.0, 0.0);
    
    let light = PointLight::new(intensity, position);

    assert_eq!(light.intensity, intensity);
    assert_eq!(light.position, position);
}

#[test]
#[should_panic]
fn new_panic_test() {
    let intensity = Color::new(1.0, 1.0, 1.0);
    let position = Tuple::vector(0.0, 0.0, 0.0);
    
    PointLight::new(intensity, position);
}