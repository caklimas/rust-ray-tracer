use crate::tuple::Tuple;
use self::projectile::Projectile;

pub mod projectile;

#[cfg(test)]
mod tests;

pub struct Environment {
    pub gravity: Tuple,
    pub wind: Tuple
}

impl Environment {
    pub fn new(gravity: Tuple, wind: Tuple) -> Self {
        if !gravity.is_vector() || !wind.is_vector() {
            panic!("Gravity and wind must both be vectors");
        }

        Environment {
            gravity,
            wind
        }
    }

    pub fn tick(&self, projectile: &Projectile) -> Projectile {
        Projectile {
            position: projectile.position + projectile.velocity,
            velocity: projectile.velocity + self.gravity + self.wind
        }
    }
}