use std::io::Result;
//use png;
use png::HasParameters;
use core::ops;
use std::fs::File;

// Chapter 2
#[derive(Debug, Copy, Clone)]
struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        const EPS: f64 = 0.00001;
        (self.r - other.r).abs() < EPS &&
        (self.g - other.g).abs() < EPS &&
        (self.b - other.b).abs() < EPS
    }
}

impl ops::Add<Color> for Color {
    type Output = Color;
    fn add(self, rhs: Color) -> Color {
        Color { 
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b
        }
    }
}

impl ops::Sub<Color> for Color {
    type Output = Color;
    fn sub(self, rhs: Color) -> Color {
        Color { 
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b
        }
    }
}

impl ops::Mul<f64> for Color {
    type Output = Color;
    fn mul(self, rhs: f64) -> Color {
        Color { 
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs
        }
    }
}

impl ops::Mul<Color> for Color {
    type Output = Color;
    fn mul(self, rhs: Color) -> Color {
        Color { 
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b
        }
    }
}

impl Color {
    fn color(r: f64, g: f64, b: f64) -> Color {
        Color {r, g, b}
    }

    fn clamp(num: f64) -> f64 {
        num::clamp(num * 256.0, 0.0, 255.0)
    }

    fn clamp_color(&self) -> Color {
        Color { 
            r: Color::clamp(self.r),
            g: Color::clamp(self.g),
            b: Color::clamp(self.b)
        }
    }
}

struct Canvas {
    pub width: usize,
    pub height: usize,
    canvas: Vec<Vec<Color>>
}

const BLACK: Color = Color { r: 0.0, g: 0.0, b: 0.0 };

use std::collections::LinkedList;
use num::clamp;

impl Canvas {
    fn canvas(width: usize, height: usize) -> Canvas {
        let black_row = vec![BLACK; width];
        Canvas { 
            width, 
            height,
            canvas: vec![black_row; height]
        }
    }

    fn pixel_at(&self, x: usize, y: usize) -> Color {
        self.canvas[y][x]
    }

    fn write_pixel(&mut self, x: usize, y: usize, c: Color) {
        self.canvas[y][x] = c;
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![0u8; &self.width * &self.height * 3];
        let mut index = 0;
        for row in &self.canvas {
            for color in row {
                let clamped_color = color.clamp_color();
                bytes[index] = clamped_color.r as u8;
                bytes[index + 1] = clamped_color.g as u8;
                bytes[index + 2] = clamped_color.b as u8;
                index += 3;
            }
        }
        bytes
    }

    fn ppm_header(&self, file_content: &mut LinkedList<String>) {
        file_content.push_back("P3\n".to_string());
        file_content.push_back(format!("{} {}\n", self.width, self.height));
        file_content.push_back("255\n".to_string());

    }

    fn ppm_color(color_value: f64) -> String {
        format!("{}", num::clamp(color_value * 256.0, 0.0, 255.0) as u8)
    }

    fn ppm_add_string_to_row(row: &mut String, segment: String, file_content: &mut LinkedList<String>) {
        let rowlength = row.len();
        if rowlength + segment.len() >= 70 {
            row.push_str("\n");
            file_content.push_back(row.to_string());
            row.clear();
        }
        else if rowlength > 0 {
            row.push_str(" ");
        }
        row.push_str(&segment);
    }

    fn ppm_row(&self, file_content: &mut LinkedList<String>, row: usize) {
        let mut file_row = String::with_capacity(80);

        for column in 0..self.width {
            let c = self.canvas[row][column];
            Canvas::ppm_add_string_to_row(&mut file_row, Canvas::ppm_color(c.r), file_content);
            Canvas::ppm_add_string_to_row(&mut file_row, Canvas::ppm_color(c.g), file_content);
            Canvas::ppm_add_string_to_row(&mut file_row, Canvas::ppm_color(c.b), file_content);
        }
        if file_row.len() > 0 {
            file_row += "\n";
            file_content.push_back(file_row);
        }
    }

    fn canvas_to_ppm(&self) -> LinkedList<String> {

        let mut file_content: LinkedList<String> = LinkedList::new();
        self.ppm_header(&mut file_content);
        for row in 0..self.height {
            self.ppm_row(&mut file_content, row);
        }
        file_content
    }

    fn save(&self, file_name: &str) {
        let file = File::create(file_name).unwrap();
        let ref mut w = std::io::BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, self.width as u32, self.height as u32);
        encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();

        writer.write_image_data(&self.to_bytes()).unwrap(); // Save
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colors_are_rgb_tuples()
    {
        let c = Color::color(-0.5, 0.4, 1.7);

        assert_eq!(c.r, -0.5);
        assert_eq!(c.g, 0.4);
        assert_eq!(c.b, 1.7);
    }

    #[test]
    fn adding_colors()
    {
        let c1 = Color::color(0.9, 0.6, 0.75);
        let c2 = Color::color(0.7, 0.1, 0.25);

        let actual = c1 + c2;
        let expected = Color::color(1.6, 0.7, 1.0);

        assert_eq!(expected, actual);
    }

    #[test]
    fn subtracting_colors()
    {
        let c1 = Color::color(0.9, 0.6, 0.75);
        let c2 = Color::color(0.7, 0.1, 0.25);

        let actual = c1 - c2;
        let expected = Color::color(0.2, 0.5, 0.5);

        assert_eq!(expected, actual);
    }

    #[test]
    fn multiplying_color_by_scalar()
    {
        let c = Color::color(0.2, 0.3, 0.4);

        let actual = c * 2.0;
        let expected = Color::color(0.4, 0.6, 0.8);

        assert_eq!(expected, actual);
    }

    #[test]
    fn multiplying_colors()
    {
        let c1 = Color::color(1.0, 0.2, 0.4);
        let c2 = Color::color(0.9, 1.0, 0.1);

        let actual = c1 * c2;
        let expected = Color::color(0.9, 0.2, 0.04);

        assert_eq!(expected, actual);
    }

    #[test]
    fn creating_canvas()
    {
        let c = Canvas::canvas(10, 20);

        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);

        for row in c.canvas {
            for color in row {
                assert_eq!(color, BLACK);
            }
        }
    }

    #[test]
    fn writing_pixel_to_canvas() {
        let mut c = Canvas::canvas(10, 20);
        let red = Color::color(1.0, 0.0, 0.0);

        c.write_pixel(2, 3, red);

        assert_eq!(c.pixel_at(2, 3), red);
    }
    
    #[test]
    fn constructing_ppm_header()
    {
        let c = Canvas::canvas(5, 3);
        let mut ppm = c.canvas_to_ppm();

        let marker = ppm.pop_front();
        assert_eq!(marker, Some("P3\n".to_string()));
        let width_height = ppm.pop_front();
        assert_eq!(width_height, Some("5 3\n".to_string()));
        let max_color_value = ppm.pop_front();
        assert_eq!(max_color_value, Some("255\n".to_string()));
    }

    #[test]
    fn construct_pixel_data()
    {
        let mut c = Canvas::canvas(5, 3);
        let c1 = Color::color(1.5, 0.0, 0.0);
        let c2 = Color::color(0.0, 0.5, 0.0);
        let c3 = Color::color(-0.5, 0.0, 1.0);

        c.write_pixel(0, 0, c1);
        c.write_pixel(2, 1, c2);
        c.write_pixel(4, 2, c3);

        let mut ppm = c.canvas_to_ppm();
        let mut color_data = ppm.split_off(3);

        let line4 = color_data.pop_front();
        assert_eq!(line4, Some("255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n".to_string()));
        let line5 = color_data.pop_front();
        assert_eq!(line5, Some("0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n".to_string()));
        let line6 = color_data.pop_front();
        assert_eq!(line6, Some("0 0 0 0 0 0 0 0 0 0 0 0 0 0 255\n".to_string()));
    }

    #[test]
    fn split_long_lines_in_ppm_file()
    {
        let mut c = Canvas::canvas(10, 2);
        let col = Color::color(1.0, 0.8, 0.6);
        for x in 0..10 {
            for y in 0..2 {
                c.write_pixel(x, y, col);
            }
        }

        let mut ppm = c.canvas_to_ppm();
        let mut color_data = ppm.split_off(3);
        
        let line = color_data.pop_front();
        assert_eq!(line, Some("255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n".to_string()));
        let line = color_data.pop_front();
        assert_eq!(line, Some("153 255 204 153 255 204 153 255 204 153 255 204 153\n".to_string()));
        let line = color_data.pop_front();
        assert_eq!(line, Some("255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n".to_string()));
        let line = color_data.pop_front();
        assert_eq!(line, Some("153 255 204 153 255 204 153 255 204 153 255 204 153\n".to_string()));
    }
    #[test]
    fn ppm_file_terminated_with_newline() {
        let c = Canvas::canvas(5, 3);
        let mut ppm = c.canvas_to_ppm();

        let last_line = ppm.pop_back();
        assert_eq!(last_line.unwrap().chars().last(), Some('\n'));
    }

    #[test]
    fn ppm_to_file()
    {
        let mut c = Canvas::canvas(100, 100);
        c.write_pixel(1, 1, Color::color(1.0, 1.0, 1.0));
        c.write_pixel(99, 0, Color::color(1.0, 0.5, 0.5));
        c.save("black.png");
    }
}