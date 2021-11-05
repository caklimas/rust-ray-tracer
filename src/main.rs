use std::f64::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4};

use canvas::Canvas;
use color::Color;
use material::Material;
use matrix::transformation::{scale, translate};
use point_light::PointLight;
use shape::Shape;
use sphere::Sphere;
use tuple::Tuple;
use world::World;

use crate::{
    camera::Camera,
    patterns::{checker::Checker, ring::Ring, stripe::Stripe, Pattern},
    plane::Plane,
    world::view_transform,
};

pub mod camera;
pub mod canvas;
pub mod color;
pub mod environment;
pub mod floating_point;
pub mod intersection;
pub mod material;
pub mod matrix;
pub mod patterns;
pub mod plane;
pub mod point_light;
pub mod ray;
pub mod shape;
pub mod sphere;
pub mod tuple;
pub mod world;

fn main() {
    canvas_sphere_test();
    camera_world_test();
    camera_plane_test();
}

fn canvas_sphere_test() {
    let ray_origin = tuple::Tuple::point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_pixels = 100;
    let pixel_size: f64 = wall_size / (canvas_pixels as f64);
    let half = wall_size / 2.0;
    let mut canvas = canvas::Canvas::new(canvas_pixels, canvas_pixels);
    let mut shape = sphere::Sphere::new();
    let light = PointLight::new(Color::white(), Tuple::point(-50.0, 10.0, -10.0));
    let material = Material {
        color: Color::new(1.0, 0.0, 0.0),
        ..Default::default()
    };
    shape.material = material;
    shape.transform = matrix::transformation::scale(1.0, 1.0, 1.0);

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * (y as f64);
        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * (x as f64);
            let position = tuple::Tuple::point(world_x, world_y, wall_z);
            let ray = ray::Ray::new(ray_origin, (position - ray_origin).normalize());
            let xs = shape.intersect(&ray);
            let intersections = intersection::intersections::Intersections::new(xs);

            if let Some(hit) = intersections.hit() {
                let position = ray.position(hit.value);
                let normal = hit.object.normal_at(position);
                let eye = ray.direction.negate();
                let color = hit
                    .object
                    .get_material()
                    .lighting(hit.object, &light, &position, &eye, &normal, true);
                canvas.write_pixel(x, y, color);
            }
        }
    }

    write_to_file(
        &canvas,
        r"C:\Users\Christopher\Desktop\Files\Rust\sphere.ppm",
    );
}

fn camera_world_test() {
    let mut floor = Sphere::new();
    floor.transform = scale(10.0, 0.01, 10.0);
    floor.material = Default::default();
    floor.material.color = Color::new(1.0, 0.9, 0.9);
    floor.material.specular = 0.0;

    let mut left_wall = Sphere::new();
    left_wall.transform = scale(10.0, 0.01, 10.0)
        .rotate_x(FRAC_PI_2)
        .rotate_y(-FRAC_PI_4)
        .translate(0.0, 0.0, 5.0);
    left_wall.material = Default::default();
    left_wall.material.color = Color::new(1.0, 0.9, 0.9);
    left_wall.material.specular = 0.0;

    let mut right_wall = Sphere::new();
    right_wall.transform = scale(10.0, 0.01, 10.0)
        .rotate_x(FRAC_PI_2)
        .rotate_y(FRAC_PI_4)
        .translate(0.0, 0.0, 5.0);
    right_wall.material = Default::default();
    right_wall.material.color = Color::new(1.0, 0.9, 0.9);
    right_wall.material.specular = 0.0;

    let mut middle = Sphere::new();
    middle.transform = translate(-0.5, 1.0, 0.5);
    middle.material = Default::default();
    middle.material.color = Color::new(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    let mut right = Sphere::new();
    right.transform = scale(0.5, 0.5, 0.5).translate(1.5, 0.5, -0.5);
    right.material = Default::default();
    right.material.color = Color::new(0.5, 1.0, 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    let mut left = Sphere::new();
    left.transform = scale(0.33, 0.33, 0.33).translate(-1.5, 0.33, -0.75);
    left.material = Default::default();
    left.material.color = Color::new(1.0, 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    let world = World::new(
        PointLight::new(Color::white(), Tuple::point(-10.0, 10.0, -10.0)),
        vec![
            Box::new(floor),
            Box::new(left_wall),
            Box::new(right_wall),
            Box::new(middle),
            Box::new(right),
            Box::new(left),
        ],
    );

    let mut camera = Camera::new(400, 200, FRAC_PI_3);
    camera.transform = view_transform(
        &Tuple::point(0.0, 1.5, -5.0),
        &Tuple::point(0.0, 1.0, 0.0),
        &Tuple::vector(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&world);
    write_to_file(
        &canvas,
        r"C:\Users\Christopher\Desktop\Files\Rust\scene.ppm",
    );
}

fn camera_plane_test() {
    let mut floor = Plane::new();
    floor.transform = scale(10.0, 0.01, 10.0);

    floor.material = Default::default();
    floor.material.color = Color::new(1.0, 0.9, 0.9);
    floor.material.specular = 0.0;

    let mut wall = Plane::new();
    wall.transform = scale(10.0, 0.01, 10.0)
        .rotate_x(FRAC_PI_2)
        .translate(0.0, 0.0, 1.0);

    wall.material = Default::default();
    wall.material.color = Color::new(1.0, 0.9, 0.9);
    wall.material.specular = 0.0;

    let mut middle = Sphere::new();
    middle.transform = translate(-0.5, 1.0, 0.5);
    middle.material = Default::default();
    middle.material.color = Color::new(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    let mut ring = Ring::new(Color::white(), Color::black());
    ring.set_transform(scale(0.1, 0.1, 0.1).rotate_x(FRAC_PI_2));
    middle.material.pattern = Option::Some(Box::new(ring));

    let mut right = Sphere::new();
    right.transform = scale(0.5, 0.5, 0.5).translate(1.5, 0.5, -0.5);
    right.material = Default::default();
    right.material.color = Color::new(0.5, 1.0, 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    let mut stripe = Stripe::new(Color::new(0.0, 0.0, 1.0), Color::new(0.0, 1.0, 0.0));
    stripe.set_transform(scale(0.25, 0.25, 0.25).rotate_z(FRAC_PI_2));
    right.material.pattern = Option::Some(Box::new(stripe));

    let mut left = Sphere::new();
    left.transform = scale(0.33, 0.33, 0.33).translate(-1.5, 0.33, -0.75);
    left.material = Default::default();
    left.material.color = Color::new(1.0, 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    let mut checker = Checker::new(Color::white(), Color::new(0.0, 1.0, 0.0));
    checker.set_transform(scale(0.25, 0.25, 0.25));
    left.material.pattern = Option::Some(Box::new(checker));

    let world = World::new(
        PointLight::new(Color::white(), Tuple::point(-10.0, 10.0, -10.0)),
        vec![
            Box::new(floor),
            Box::new(wall),
            Box::new(middle),
            Box::new(right),
            Box::new(left),
        ],
    );

    let mut camera = Camera::new(400, 200, FRAC_PI_3);
    camera.transform = view_transform(
        &Tuple::point(0.0, 1.5, -5.0),
        &Tuple::point(0.0, 1.0, 0.0),
        &Tuple::vector(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&world);
    write_to_file(
        &canvas,
        r"C:\Users\Christopher\Desktop\Files\Rust\plane.ppm",
    );
}

fn write_to_file(canvas: &Canvas, path: &str) {
    let ppm_string = canvas.to_ppm();
    std::fs::write(path, ppm_string.as_str()).unwrap();
}
