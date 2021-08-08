#[cfg(test)]
mod tests;

pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        if w != 0.0 && w != 1.0 {
            panic!("w must be 0 or 1");
        }

        Point {
            x,
            y,
            z,
            w
        }
    }

    pub fn is_vector(&self) -> bool {
        return self.w == 1.0;
    }
}