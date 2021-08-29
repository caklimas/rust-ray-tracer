use crate::color::Color;

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