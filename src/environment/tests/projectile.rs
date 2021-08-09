use crate::tuple::Tuple;
use crate::environment::projectile::Projectile;

#[test]
#[should_panic]
fn new_position_panic_test() {
    Projectile::new(
        Tuple::vector(0.0, 0.0, 0.0),
        Tuple::vector(0.0, 0.0, 0.0)
    );
}

#[test]
#[should_panic]
fn new_wind_panic_test() {
    Projectile::new(
        Tuple::point(0.0, 0.0, 0.0),
        Tuple::point(0.0, 0.0, 0.0)
    );
}