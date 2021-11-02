#[cfg(test)]
mod tests;

pub const EPSILON: f64 = 0.00001;

pub struct FloatingPoint {}

impl FloatingPoint {
    pub fn equals(left: f64, right: f64) -> bool {
        (left - right).abs() < EPSILON
    }
}
