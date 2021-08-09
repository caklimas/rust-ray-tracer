use crate::color::Color;

#[cfg(test)]
mod tests;

pub struct Canvas {
    pub pixels: Vec<Vec<Color>>
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let mut pixels: Vec<Vec<Color>> = Vec::new();
        for _h in 0..height {
            let mut row: Vec<Color> = Vec::new();
            for _w in 0..width {
                row.push(Color::black())
            }
            pixels.push(row);
        }

        Canvas {
            pixels
        }
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.pixels[y][x] = color;
    }
}