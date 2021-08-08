use crate::tuple::Tuple;
use super::Environment;

pub mod projectile;

#[test]
#[should_panic]
fn environment_new_gravity_panic_test() {
    Environment::new(
        Tuple::point(0.0, 0.0, 0.0),
        Tuple::vector(0.0, 0.0, 0.0)
    );
}

#[test]
#[should_panic]
fn environment_new_wind_panic_test() {
    Environment::new(
        Tuple::vector(0.0, 0.0, 0.0),
        Tuple::point(0.0, 0.0, 0.0)
    );
}