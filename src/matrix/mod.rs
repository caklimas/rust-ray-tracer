use crate::{floating_point::FloatingPoint};

pub mod axis;
pub mod ops;
pub mod transformation;

#[cfg(test)]
mod tests;

#[derive(Clone, Debug)]
pub struct Matrix {
    pub rows: usize,
    pub columns: usize,
    pub elements: Vec<Vec<f64>>
}

impl Matrix {
    pub fn new(rows: usize, columns: usize, elements: Option<Vec<Vec<f64>>>) -> Self {
        if let Some(matrix_elements) = elements {
            if !Matrix::validate_elements(rows, columns, &matrix_elements) {
                panic!("Incorrect number of rows or columns.");
            }

            return Matrix {
                rows,
                columns,
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
            rows,
            columns,
            elements
        }
    }

    pub fn identity(size: usize) -> Self {
        let mut elements = Vec::new();
        for s in 0..size {
            let mut row = Vec::new();
            for s2 in 0..size {
                row.push(if s == s2 { 1.0 } else { 0.0 });
            }

            elements.push(row);
        }

        Matrix::new(size, size, Option::Some(elements))
    }

    pub fn transpose(&self) -> Matrix {
        let mut elements = Vec::new();
        for i in 0..self.columns {
            let mut row = Vec::new();
            for j in 0..self.rows {
                row.push(self.elements[j][i]);
            }
            elements.push(row);
        }

        Matrix::new(self.columns, self.rows, Option::Some(elements))
    }

    pub fn determinant(&self) -> f64 {
        if self.rows == 2 && self.columns == 2 {
            return (self.elements[0][0] * self.elements[1][1]) - (self.elements[0][1] * self.elements[1][0]);
        }

        let mut determinant = 0.0;
        for column in 0..self.columns {
            determinant += self.elements[0][column] * self.cofactor(0, column)
        }

        determinant
    }

    pub fn submatrix(&self, row_index: usize, column_index: usize) -> Matrix {
        if row_index >= self.rows || column_index >= self.columns {
            panic!("Row or column exceeds index");
        }

        let mut submatrix_elements = Vec::new();
        for r in 0..self.rows {
            if r == row_index {
                continue;
            }

            let row = &self.elements[r];
            let mut sub_elements = Vec::new();
            for (column, item) in row.iter().enumerate() {
                if column == column_index {
                    continue;
                }

                sub_elements.push(*item);
            }
            submatrix_elements.push(sub_elements);
        }

        Matrix::new(self.rows - 1, self.columns - 1, Option::Some(submatrix_elements))
    }

    pub fn minor(&self, row: usize, column: usize) -> f64 {
        let submatrix = self.submatrix(row, column);
        submatrix.determinant()
    }

    pub fn cofactor(&self, row: usize, column: usize) -> f64 {
        let minor = self.minor(row, column);
        if (row + column) % 2 == 0 { minor } else { minor * -1.0 }
    }

    pub fn inverse(&self) -> Matrix {
        if FloatingPoint::equals(self.determinant(), 0.0) {
            panic!("Matrix is not invertible");
        }

        let mut matrix = Matrix::new(self.rows, self.columns, Option::None);
        let determinant = self.determinant();
        for r in 0..self.rows {
            for c in 0..self.columns {
                let cofactor = self.cofactor(r, c);
                matrix.elements[c][r] = cofactor / determinant;
            }
        }

        matrix
    }

    fn validate_elements(rows: usize, columns: usize, elements: &[Vec<f64>]) -> bool {
        if elements.len() != rows {
            return false;
        }

        for row in elements.iter() {
            if row.len() != columns {
                return false;
            }
        }

        true
    }
}