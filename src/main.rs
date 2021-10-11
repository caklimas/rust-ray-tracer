use color::Color;
use material::Material;
use point_light::PointLight;
use tuple::Tuple;

pub mod canvas;
pub mod color;
pub mod environment;
pub mod floating_point;
pub mod intersection;
pub mod material;
pub mod matrix;
pub mod point_light;
pub mod ray;
pub mod sphere;
pub mod tuple;
pub mod world;

fn main() {
    canvas_sphere_test();
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
    let mut material: Material = Default::default();
    material.color = Color::new(1.0, 0.0, 0.0);
    shape.material = material;
    shape.transform = matrix::transformation::scale(1.0, 1.0, 1.0);

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * (y as f64);
        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * (x as f64);
            let position = tuple::Tuple::point(world_x, world_y, wall_z);
            let ray = ray::Ray::new(ray_origin, (position - ray_origin).normalize());
            let mut xs = shape.intersect(&ray);
            let intersections = intersection::intersections::Intersections::new(&mut xs);

            match intersections.hit() {
                Some(hit) => {
                    let position = ray.position(hit.value);
                    let normal = hit.object.normal_at(position);
                    let eye = ray.direction.negate();
                    let color = hit
                        .object
                        .material
                        .lighting(&light, &position, &eye, &normal);
                    canvas.write_pixel(x, y, color);
                }
                None => (),
            }
        }
    }

    let ppm_string = canvas.to_ppm();
    std::fs::write(
        r"C:\Users\cakli\Desktop\Files\sample.ppm",
        ppm_string.as_str(),
    )
    .unwrap();
}
