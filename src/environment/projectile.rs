use crate::tuple::Tuple;

#[derive(Clone, Copy)]
pub struct Projectile {
    pub position: Tuple,
    pub velocity: Tuple
}

impl Projectile {
    pub fn new(position: Tuple, velocity: Tuple) -> Self {
        if !position.is_point() || !velocity.is_vector() {
            panic!("Position must be a point and velocity must be a vector.");
        }

        Projectile {
            position,
            velocity
        }
    }
}