use std::io::Result;
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
const WHITE: Color = Color { r: 1.0, g: 1.0, b: 1.0 };
const RED: Color = Color { r: 1.0, g: 0.0, b: 0.0 };
const GREEN: Color = Color { r: 0.0, g: 1.0, b: 0.0 };
const BLUE: Color = Color { r: 0.0, g: 0.0, b: 1.0 };

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

    fn clamp_to_byte(color_component: f64) -> u8 {
        if color_component < 0.0 {
            0u8
        } else if color_component >= 1.0 {
            255u8
        } else {
            (color_component * 256.0) as u8
        }
    }

    fn to_rgb_bytes(&self) -> Vec<u8> {
        const BYTES_PER_PIXEL: usize = 3;
        let mut bytes = vec![0u8; &self.width * &self.height * BYTES_PER_PIXEL];
        let mut index = 0;
        for row in &self.canvas {
            for color in row {
                bytes[index] = Canvas::clamp_to_byte(color.r);
                bytes[index + 1] = Canvas::clamp_to_byte(color.g);
                bytes[index + 2] = Canvas::clamp_to_byte(color.b);
                index += 3;
            }
        }
        bytes
    }

    fn save(&self, file_name: &str) {
        let file = File::create(file_name).unwrap();
        let ref mut w = std::io::BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, self.width as u32, self.height as u32);
        encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();

        writer.write_image_data(&self.to_rgb_bytes()).unwrap(); // Save
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
    fn construct_pixel_data()
    {
        const width: usize = 5;
        const height: usize = 3;
        const bytes_per_pixel: usize = 3;

        let mut c = Canvas::canvas(width, height);
        let c1 = Color::color(1.5, 0.0, 0.0);
        let c2 = Color::color(0.0, 0.5, 0.0);
        let c3 = Color::color(-0.5, 0.0, 1.0);

        c.write_pixel(0, 0, c1);
        c.write_pixel(2, 1, c2);
        c.write_pixel(4, 2, c3);

        let rgb_bytes = c.to_rgb_bytes();

        assert_eq!(255u8, rgb_bytes[0]);                                        // clamp the 1.5 r value to 255
        assert_eq!(128u8, rgb_bytes[(2 + 1 * width) * bytes_per_pixel + 1]);     // the .5 g value should be converted to 128 
        assert_eq!(0u8, rgb_bytes[(4 + 2 * width) * bytes_per_pixel + 0]);     // clamp the -.5 r value to 0
        assert_eq!(255u8, rgb_bytes[(4 + 2 * width) * bytes_per_pixel + 2]);     // the 1.0 b value should be 255
    }

    #[test]
    fn canvas_to_file()
    {
        let mut c = Canvas::canvas(100, 100);
        c.write_pixel(1, 1, Color::color(1.0, 1.0, 1.0));
        c.write_pixel(99, 0, Color::color(1.0, 0.5, 0.5));
        c.save("black.png");
    }
}