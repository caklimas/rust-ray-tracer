#[cfg(test)]
mod tests;

pub struct Matrix {
    pub elements: Vec<Vec<f64>>
}

impl Matrix {
    pub fn new(size: usize, elements: Option<Vec<Vec<f64>>>) -> Self {
        if elements.is_some() {
            return Matrix {
                elements: elements.unwrap()
            };
        }

        let mut elements = Vec::new();
        for _i in 0..size {
            let mut row = Vec::new();
            for _j in 0..size {
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
}