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

fn main() {
    //canvas_sphere_test();
}

// fn canvas_test() {
//     let projectile = environment::projectile::Projectile::new(
//         tuple::Tuple::point(0.0, 1.0, 0.0), 
//         tuple::Tuple::vector(1.0, 1.8, 0.0).normalize().multiply(11.25)
//     );

//     let environment = environment::Environment::new(
//         tuple::Tuple::vector(0.0, -0.1, 0.0), 
//         tuple::Tuple::vector(-0.01, 0.0, 0.0)
//     );

//     let height = 550;
//     let mut canvas = canvas::Canvas::new(900, height);

//     let mut result = environment.tick(&projectile);
//     while result.position.y() > 0.0 {
//         result = environment.tick(&result);
//         canvas.write_pixel(result.position.x() as usize, height - (result.position.y() as usize), color::Color::new(1.0, 0.8, 0.6));
//     }

//     let ppm_string = canvas.to_ppm();
//     fs::write("/Users/christopherk/Desktop/Files/sample.ppm", ppm_string.as_str()).unwrap();
// }

// fn canvas_sphere_test() {
//     let ray_origin = tuple::Tuple::point(0.0, 0.0, -5.0);
//     let wall_z = 10.0;
//     let wall_size = 7.0;
//     let canvas_pixels = 100;
//     let pixel_size: f64 = wall_size / (canvas_pixels as f64);
//     let half = wall_size / 2.0;
//     let mut canvas = canvas::Canvas::new(canvas_pixels, canvas_pixels);
//     let color = color::Color::new(1.0, 0.0, 0.0);
//     let mut shape = sphere::Sphere::new();
//     shape.transform = matrix::transformation::shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0).scale(0.5, 1.0, 1.0);

//     for y in 0..canvas_pixels {
//         let world_y = half - pixel_size * (y as f64);
//         for x in 0..canvas_pixels {
//             let world_x = -half + pixel_size * (x as f64);
//             let position = tuple::Tuple::point(world_x, world_y, wall_z);
//             let ray = ray::Ray::new(ray_origin, (position - ray_origin).normalize());
//             let mut xs = shape.intersect(&ray);
//             let intersections = intersection::intersections::Intersections::new(&mut xs);

//             if intersections.hit().is_some() {
//                 canvas.write_pixel(x, y, color);
//             }
//         }
//     }

//     let ppm_string = canvas.to_ppm();
//     std::fs::write(r"C:\Users\Christopher\Desktop\Files/sample.ppm", ppm_string.as_str()).unwrap();
// }
