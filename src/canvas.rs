use std::io::Result;
use png::HasParameters;
use std::fs::File;

use super::color::*;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    canvas: Vec<Vec<Color>>
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        let black_row = vec![BLACK; width];
        Canvas { 
            width, 
            height,
            canvas: vec![black_row; height]
        }
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        self.canvas[y][x]
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, c: Color) {
        self.canvas[y][x] = c;
    }

    fn clamp_to_byte(color_component: f64) -> u8 {
        if color_component < 0.0 {
            0u8
        } else if color_component >= 1.0 {
            255u8
        } else {
            (color_component * 256.) as u8
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

    pub fn save(&self, file_name: &str) -> Result<()> {
        let file = File::create(file_name)?;
        let ref mut w = std::io::BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, self.width as u32, self.height as u32);
        encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
        let mut writer = encoder.write_header()?;

        writer.write_image_data(&self.to_rgb_bytes())?; // Save
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_canvas()
    {
        let c = Canvas::new(10, 20);

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
        let mut c = Canvas::new(10, 20);
        let red = Color::new(1., 0., 0.);

        c.write_pixel(2, 3, red);

        assert_eq!(c.pixel_at(2, 3), red);
    }
    
    #[test]
    fn construct_pixel_data()
    {
        const WIDTH: usize = 5;
        const HEIGHT: usize = 3;
        const BYTES_PER_PIXEL: usize = 3;

        let mut c = Canvas::new(WIDTH, HEIGHT);
        let c1 = Color::new(1.5, 0., 0.);
        let c2 = Color::new(0., 0.5, 0.);
        let c3 = Color::new(-0.5, 0., 1.);

        c.write_pixel(0, 0, c1);
        c.write_pixel(2, 1, c2);
        c.write_pixel(4, 2, c3);

        let rgb_bytes = c.to_rgb_bytes();

        assert_eq!(255u8, rgb_bytes[0]);                                        // clamp the 1.5 r value to 255
        assert_eq!(128u8, rgb_bytes[(2 + 1 * WIDTH) * BYTES_PER_PIXEL + 1]);     // the .5 g value should be converted to 128 
        assert_eq!(0u8, rgb_bytes[(4 + 2 * WIDTH) * BYTES_PER_PIXEL + 0]);     // clamp the -.5 r value to 0
        assert_eq!(255u8, rgb_bytes[(4 + 2 * WIDTH) * BYTES_PER_PIXEL + 2]);     // the 1.0 b value should be 255
    }

}