use super::Matrix;
use crate::{floating_point::FloatingPoint, tuple::Tuple};
use std::ops::Mul;

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
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
}

impl Mul for Matrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        if self.columns != rhs.rows {
            panic!("Matrices cannot be multiplied");
        }

        let mut matrix = Matrix::new(self.rows, self.columns, Option::None);
        for row in 0..self.rows {
            for column in 0..self.columns {
                let mut result = 0.0;
                for i in 0..self.columns {
                    result += self.elements[row][i] * rhs.elements[i][column]
                }

                matrix.elements[row][column] = result;
            }
        }

        matrix
    }
}

impl Mul<Tuple> for Matrix {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        if self.columns != 4 {
            panic!("Matrix must have 4 columns to be multiplied by a Tuple");
        }

        let mut elements = [0.0; 4];
        for (r, row) in self.elements.iter().enumerate() {
            let row_tuple = Tuple::new(row[0], row[1], row[2], row[3]);
            elements[r] = row_tuple.dot(&rhs);
        }

        Tuple::new(elements[0], elements[1], elements[2], elements[3])
    }
}

impl Mul<Tuple> for &Matrix {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        self * &rhs
    }
}

impl Mul<&Tuple> for &Matrix {
    type Output = Tuple;

    fn mul(self, rhs: &Tuple) -> Self::Output {
        if self.columns != 4 {
            panic!("Matrix must have 4 columns to be multiplied by a Tuple");
        }

        let mut elements = [0.0; 4];
        for (r, row) in self.elements.iter().enumerate() {
            let row_tuple = Tuple::new(row[0], row[1], row[2], row[3]);
            elements[r] = row_tuple.dot(&rhs);
        }

        Tuple::new(elements[0], elements[1], elements[2], elements[3])
    }
}
