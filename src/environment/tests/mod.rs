use super::{projectile::Projectile, Environment};
use crate::tuple::Tuple;

pub mod projectile;

#[test]
#[should_panic]
fn new_gravity_panic_test() {
    Environment::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 0.0));
}

#[test]
#[should_panic]
fn new_wind_panic_test() {
    Environment::new(Tuple::vector(0.0, 0.0, 0.0), Tuple::point(0.0, 0.0, 0.0));
}

#[test]
fn tick_test() {
    let projectile = Projectile {
        position: Tuple::point(0.0, 1.0, 0.0),
        velocity: Tuple::vector(2.0, 2.0, 0.0).normalize(),
    };

    let environment = Environment {
        gravity: Tuple::vector(0.0, -0.1, 0.0),
        wind: Tuple::vector(-0.01, 0.0, 0.0),
    };

    let mut result = environment.tick(&projectile);
    let mut ticks = 0;
    while result.position.y() > 0.0 {
        result = environment.tick(&result);
        ticks += 1;
    }

    assert_eq!(16, ticks);
}
