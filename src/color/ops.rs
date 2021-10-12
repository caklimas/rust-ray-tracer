use super::Color;
use std::ops::{Add, Mul, Sub};

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

impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Color {
            red: self.red() * rhs.red(),
            green: self.green() * rhs.green(),
            blue: self.blue() * rhs.blue(),
        }
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        let multiplied = self.to_tuple() * rhs;
        Color::from_tuple(&multiplied)
    }
}
