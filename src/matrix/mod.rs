use crate::{floating_point::FloatingPoint, tuple::Tuple};

#[cfg(test)]
mod tests;

pub struct Matrix {
    pub rows: usize,
    pub columns: usize,
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

    pub fn get(&self, y: usize, x: usize) -> f64 {
        self.elements[y][x]
    }

    pub fn equals(&self, other: &Matrix) -> bool {
        if self.rows != other.rows || self.columns != other.columns {
            return false;
        }

        for i in 0..self.elements.len() {
            let row = &self.elements[i];
            let other_row = &other.elements[i];

            for j in 0..row.len() {
                if !FloatingPoint::equals(row[j], other_row[j]) {
                    return false;
                }
            }
        }

        true
    }

    pub fn multiply(&self, other: &Matrix) -> Matrix {
        if self.columns != other.rows {
            panic!("Matrices cannot be multiplied");
        }

        let mut matrix = Matrix::new(self.rows, self.columns, Option::None);
        for row in 0..self.rows {
            for column in 0..self.columns {
                let mut result = 0.0;
                for i in 0..self.columns {
                    result += self.elements[row][i] * other.elements[i][column]
                }

                matrix.elements[row][column] = result;
            }
        }

        matrix
    }

    pub fn multiply_tuple(&self, tuple: &Tuple) -> Tuple {
        if self.columns != 4 {
            panic!("Matrix must have 4 columns to be multiplied by a Tuple");
        }

        let mut elements = [0.0; 4];
        for r in 0..4 {
            let row = &self.elements[r];
            let row_tuple = Tuple::new(row[0], row[1], row[2], row[3]);
            elements[r] = row_tuple.dot(tuple);
        }

        Tuple::new(elements[0], elements[1], elements[2], elements[3])
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