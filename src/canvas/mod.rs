use crate::color::Color;

#[cfg(test)]
mod tests;

pub const MAX_COLOR_VALUE: f64 = 255.0;
pub const PLAIN_PPM_LINE_LENGTH: usize = 69;

pub struct Canvas {
    pub pixels: Vec<Vec<Color>>,
    width: usize,
    height: usize,
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
            height,
        }
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        if x >= self.width || y >= self.height {
            return;
        }

        self.pixels[y][x] = color;
    }

    pub fn to_ppm(&self) -> String {
        let mut ppm_string = String::new();
        ppm_string.push_str(format!("{}\n", "P3").as_str());
        ppm_string.push_str(format!("{} {}\n", self.width, self.height).as_str());
        ppm_string.push_str("255\n");

        for y in 0..self.height {
            let mut row_string = String::new();
            for (x, pixel) in self.pixels[y].iter().enumerate() {
                let rounded_color = Self::get_rounded_ppm_color(pixel);
                if row_string.len() + rounded_color.len() > PLAIN_PPM_LINE_LENGTH {
                    ppm_string.push_str(format!("{}\n", row_string.trim_end()).as_str());
                    row_string.clear();
                }

                row_string.push_str(rounded_color.as_str());

                if x < self.width - 1 {
                    row_string.push(' ');
                }
            }

            ppm_string.push_str(row_string.as_str());
            ppm_string.push('\n');
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
