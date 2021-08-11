use crate::floating_point::FloatingPoint;

#[cfg(test)]
mod tests;

pub struct Matrix {
    pub elements: Vec<Vec<f64>>
}

impl Matrix {
    pub fn new(rows: usize, columns: usize, elements: Option<Vec<Vec<f64>>>) -> Self {
        if elements.is_some() {
            let matrix_elements = elements.unwrap();
            if !Matrix::validate_elements(rows, columns, &matrix_elements) {
                panic!("Incorrect number of rows or columns.");
            }

            return Matrix {
                elements: matrix_elements
            };
        }

        let mut elements = Vec::new();
        for _row in 0..rows {
            let mut row = Vec::new();
            for _column in 0..columns {
                row.push(0.0);
            }
            elements.push(row);
        }

        Matrix {
            elements
        }
    }

    pub fn get(&self, y: usize, x: usize) -> f64 {
        self.elements[y][x]
    }

    pub fn equals(&self, other: &Matrix) -> bool {
        if self.elements.len() != other.elements.len() {
            return false;
        }

        for i in 0..self.elements.len() {
            let row = &self.elements[i];
            let other_row = &other.elements[i];

            if row.len() != other_row.len() {
                return false;
            }

            for j in 0..row.len() {
                if !FloatingPoint::equals(row[j], other_row[j]) {
                    return false;
                }
            }
        }

        true
    }

    fn validate_elements(rows: usize, columns: usize, elements: &Vec<Vec<f64>>) -> bool {
        if elements.len() != rows {
            return false;
        }

        for r in 0..rows {
            let row = &elements[r];
            if row.len() != columns {
                return false;
            }
        }

        true
    }
}