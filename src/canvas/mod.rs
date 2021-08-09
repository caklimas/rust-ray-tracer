use crate::color::Color;

#[cfg(test)]
mod tests;

pub const MAX_COLOR_VALUE: f64 = 255.0;

pub struct Canvas {
    pub pixels: Vec<Vec<Color>>,
    width: usize,
    height: usize
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
            pixels,
            width,
            height
        }
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.pixels[y][x] = color;
    }

    pub fn to_ppm(&self) -> String {
        let mut ppm_string = String::new();
        ppm_string.push_str(format!("{}\n", "P3").as_str());
        ppm_string.push_str(format!("{} {}\n", self.width, self.height).as_str());
        ppm_string.push_str("255\n");
        
        for y in 0..self.height {
            let row = &self.pixels[y];
            let mut row_string = String::new();
            for x in 0..self.width {
                let pixel = row[x];
                row_string.push_str(Self::get_rounded_ppm_color(&pixel).as_str());
                if x < self.width - 1 {
                    row_string.push_str(" ");
                }
            }

            ppm_string.push_str(row_string.as_str());
            if y < self.height - 1 {
                ppm_string.push('\n');
            }
            
        }

        ppm_string
    }

    fn get_rounded_ppm_color(pixel: &Color) -> String {
        // (color * MAX_COLOR_VALUE).round()
        format!(
            "{} {} {}",
            Self::get_rounded_color(pixel.red()),
            Self::get_rounded_color(pixel.green()),
            Self::get_rounded_color(pixel.blue())
        )
    }

    fn get_rounded_color(color: f64) -> f64 {
        let mut rounded = (color * MAX_COLOR_VALUE).round();
        if rounded > MAX_COLOR_VALUE {
            rounded = MAX_COLOR_VALUE;
        } else if rounded < 0.0 {
            rounded = 0.0;
        }

        rounded
    }
}