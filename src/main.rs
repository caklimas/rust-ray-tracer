use std::fs;

pub mod canvas;
pub mod color;
pub mod environment;
pub mod floating_point;
pub mod tuple;

fn main() {
}

fn canvas_test() {
    let projectile = environment::projectile::Projectile::new(
        tuple::Tuple::point(0.0, 1.0, 0.0), 
        tuple::Tuple::vector(1.0, 1.8, 0.0).normalize().multiply(11.25)
    );

    let environment = environment::Environment::new(
        tuple::Tuple::vector(0.0, -0.1, 0.0), 
        tuple::Tuple::vector(-0.01, 0.0, 0.0)
    );

    let height = 550;
    let mut canvas = canvas::Canvas::new(900, height);

    let mut result = environment.tick(&projectile);
    while result.position.y() > 0.0 {
        result = environment.tick(&result);
        canvas.write_pixel(result.position.x() as usize, height - (result.position.y() as usize), color::Color::new(1.0, 0.8, 0.6));
    }

    let ppm_string = canvas.to_ppm();
    fs::write("/Users/christopherk/Desktop/Files/sample.ppm", ppm_string.as_str()).unwrap();
}
