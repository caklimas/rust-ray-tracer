use crate::tuple::Tuple;

pub mod ops;

#[cfg(test)]
mod tests;

#[derive(Clone, Copy, Debug)]
pub struct Color {
    red: f64,
    green: f64,
    blue: f64,
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Color { red, green, blue }
    }

    pub fn black() -> Self {
        Color::new(0.0, 0.0, 0.0)
    }

    pub fn white() -> Self {
        Color::new(1.0, 1.0, 1.0)
    }

    pub fn red(&self) -> f64 {
        self.red
    }

    pub fn green(&self) -> f64 {
        self.green
    }

    pub fn blue(&self) -> f64 {
        self.blue
    }

    fn to_tuple(self) -> Tuple {
        Tuple::new(self.red(), self.green(), self.blue(), 0.0)
    }

    fn from_tuple(tuple: &Tuple) -> Self {
        Color {
            red: tuple.x(),
            green: tuple.y(),
            blue: tuple.z(),
        }
    }
}
