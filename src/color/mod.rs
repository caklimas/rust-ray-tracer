use std::ops::{Add, Sub};

use crate::tuple::Tuple;

#[cfg(test)]
mod tests;

#[derive(Clone, Copy, Debug)]
pub struct Color {
    red: f64,
    green: f64,
    blue: f64
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Color { red, green, blue }
    }

    pub fn black() -> Self {
        Color::new(0.0, 0.0, 0.0)
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

    pub fn multiply(&self, scalar: f64) -> Color {
        let multiplied = self.to_tuple().multiply(scalar);
        Color::from_tuple(&multiplied)
    }

    pub fn multiply_color(&self, other: &Color) -> Color {
        Color {
            red: self.red() * other.red(),
            green: self.green() * other.green(),
            blue: self.blue() * other.blue()
        }
    }

    fn to_tuple(self) -> Tuple {
        Tuple::new(self.red(), self.green(), self.blue(), 0.0)
    }

    fn from_tuple(tuple: &Tuple) -> Self {
        Color {
            red: tuple.x(),
            green: tuple.y(),
            blue: tuple.z()
        }
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.to_tuple() == other.to_tuple()
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let added = self.to_tuple() + rhs.to_tuple();
        Color::from_tuple(&added)
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let subtracted = self.to_tuple() - rhs.to_tuple();
        Color::from_tuple(&subtracted)
    }
}