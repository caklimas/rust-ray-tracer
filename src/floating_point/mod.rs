#[cfg(test)]
mod tests;

pub struct FloatingPoint {}

impl FloatingPoint {
    pub fn equals(left: f64, right: f64) -> bool {
        (left - right).abs() < 0.00001
    }
}
