use crate::{color::Color, patterns::stripe::Stripe, point_light::PointLight, tuple::Tuple};

use super::Material;

#[test]
fn new_test() {
    let color = Color::new(0.0, 0.0, 0.0);
    let ambient = 0.1;
    let diffuse = 0.2;
    let specular = 0.3;
    let shininess = 100.0;

    let material = Material::new(color, ambient, diffuse, specular, shininess);

    assert_eq!(color, material.color);
    assert_eq!(ambient, material.ambient);
    assert_eq!(diffuse, material.diffuse);
    assert_eq!(specular, material.specular);
    assert_eq!(shininess, material.shininess);
}

#[test]
#[should_panic]
fn new_ambient_panic() {
    let color = Color::new(0.0, 0.0, 0.0);
    let ambient = -0.1;
    let diffuse = 0.2;
    let specular = 0.3;
    let shininess = 100.0;

    Material::new(color, ambient, diffuse, specular, shininess);
}

#[test]
#[should_panic]
fn new_diffuse_panic() {
    let color = Color::new(0.0, 0.0, 0.0);
    let ambient = 0.1;
    let diffuse = 5.2;
    let specular = 0.3;
    let shininess = 100.0;

    Material::new(color, ambient, diffuse, specular, shininess);
}

#[test]
#[should_panic]
fn new_specular_panic() {
    let color = Color::new(0.0, 0.0, 0.0);
    let ambient = 0.1;
    let diffuse = 0.2;
    let specular = 4.3;
    let shininess = 100.0;

    Material::new(color, ambient, diffuse, specular, shininess);
}

#[test]
fn default_test() {
    let material: Material = Default::default();

    assert_eq!(Color::new(1.0, 1.0, 1.0), material.color);
    assert_eq!(0.1, material.ambient);
    assert_eq!(0.9, material.diffuse);
    assert_eq!(0.9, material.specular);

    assert_eq!(200.0, material.shininess);
}

#[test]
fn lighting_eye_between_light_and_surface_test() {
    let material: Material = Default::default();
    let position = Tuple::point(0.0, 0.0, 0.0);
    let eye = Tuple::vector(0.0, 0.0, -1.0);
    let normal = Tuple::vector(0.0, 0.0, -1.0);
    let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Tuple::point(0.0, 0.0, -10.0));

    let result = material.lighting(&light, &position, &eye, &normal, false);

    assert_eq!(Color::new(1.9, 1.9, 1.9), result);
}

#[test]
fn lighting_eye_between_light_and_surface_45_offset_test() {
    let value = 2.0_f64.sqrt() / 2.0;
    let material: Material = Default::default();
    let position = Tuple::point(0.0, 0.0, 0.0);
    let eye = Tuple::vector(0.0, value, -value);
    let normal = Tuple::vector(0.0, 0.0, -1.0);
    let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Tuple::point(0.0, 0.0, -10.0));

    let result = material.lighting(&light, &position, &eye, &normal, false);

    assert_eq!(Color::new(1.0, 1.0, 1.0), result);
}

#[test]
fn lighting_eye_opposite_surface_light_45_offset_test() {
    let material: Material = Default::default();
    let position = Tuple::point(0.0, 0.0, 0.0);
    let eye = Tuple::vector(0.0, 0.0, -1.0);
    let normal = Tuple::vector(0.0, 0.0, -1.0);
    let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Tuple::point(0.0, 10.0, -10.0));

    let result = material.lighting(&light, &position, &eye, &normal, false);

    assert_eq!(Color::new(0.7364, 0.7364, 0.7364), result);
}

#[test]
fn lighting_eye_path_reflection_vector() {
    let value = 2.0_f64.sqrt() / 2.0;
    let material: Material = Default::default();
    let position = Tuple::point(0.0, 0.0, 0.0);
    let eye = Tuple::vector(0.0, -value, -value);
    let normal = Tuple::vector(0.0, 0.0, -1.0);
    let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Tuple::point(0.0, 10.0, -10.0));

    let result = material.lighting(&light, &position, &eye, &normal, false);

    assert_eq!(Color::new(1.6364, 1.6364, 1.6364), result);
}

#[test]
fn lighting_light_behind_surface() {
    let material: Material = Default::default();
    let position = Tuple::point(0.0, 0.0, 0.0);
    let eye = Tuple::vector(0.0, 0.0, -1.0);
    let normal = Tuple::vector(0.0, 0.0, -1.0);
    let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Tuple::point(0.0, 0.0, 10.0));

    let result = material.lighting(&light, &position, &eye, &normal, false);

    assert_eq!(Color::new(0.1, 0.1, 0.1), result);
}

#[test]
fn lighting_surface_in_shadow_test() {
    let material: Material = Default::default();
    let position = Tuple::point(0.0, 0.0, 0.0);
    let eye = Tuple::vector(0.0, 0.0, -1.0);
    let normal = Tuple::vector(0.0, 0.0, -1.0);
    let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Tuple::point(0.0, 0.0, -10.0));

    let result = material.lighting(&light, &position, &eye, &normal, true);

    assert_eq!(Color::new(0.1, 0.1, 0.1), result);
}

#[test]
fn lighting_pattern_applied() {
    let mut m: Material = Default::default();
    m.pattern = Option::Some(Box::new(Stripe::new(Color::white(), Color::black())));
    m.ambient = 1.0;
    m.diffuse = 0.0;
    m.specular = 0.0;

    let eye_v = Tuple::vector(0.0, 0.0, -1.0);
    let normal_v = Tuple::vector(0.0, 0.0, -1.0);
    let light = PointLight::new(Color::white(), Tuple::point(0.0, 0.0, -10.0));

    let c1 = m.lighting(
        &light,
        &Tuple::point(0.9, 0.0, 0.0),
        &eye_v,
        &normal_v,
        false,
    );

    let c2 = m.lighting(
        &light,
        &Tuple::point(1.1, 0.0, 0.0),
        &eye_v,
        &normal_v,
        false,
    );

    assert_eq!(Color::white(), c1);
    assert_eq!(Color::black(), c2);
}
