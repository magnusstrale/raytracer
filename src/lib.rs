use core::ops;

#[derive(Debug, Copy, Clone)]
struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        const EPS: f64 = 0.00001;
        (self.x - other.x).abs() < EPS &&
        (self.y - other.y).abs() < EPS &&
        (self.z - other.z).abs() < EPS &&
        self.w == other.w
    }
}

impl ops::Add<Tuple> for Tuple {
    type Output = Tuple;
    fn add(self, rhs: Tuple) -> Tuple {
        Tuple { 
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w
        }
    }
}

impl ops::Sub<Tuple> for Tuple {
    type Output = Tuple;
    fn sub(self, rhs: Tuple) -> Tuple {
        Tuple { 
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w
        }
    }
}

impl ops::Neg for Tuple {
    type Output = Tuple;
    fn neg(self) -> Tuple {
        Tuple { 
            x: -self.x ,
            y: -self.y,
            z: -self.z,
            w: -self.w
        }
    }
}

impl ops::Mul<f64> for Tuple {
    type Output = Tuple;
    fn mul(self, rhs: f64) -> Tuple {
        Tuple { 
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs
        }
    }
}

impl ops::Div<f64> for Tuple {
    type Output = Tuple;
    fn div(self, rhs: f64) -> Tuple {
        Tuple { 
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs
        }
    }
}

impl Tuple {
    fn point(x: f64, y: f64, z: f64) -> Tuple {
        Tuple {x, y, z, w: 1.0}
    }

    fn vector(x: f64, y: f64, z: f64) -> Tuple {
        Tuple {x, y, z, w: 0.0}
    }

    fn is_point(&self) -> bool {
        self.w == 1.0
    }

    fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    fn normalize(&self) -> Tuple {
        let m = self.magnitude();
        Tuple::vector(self.x / m, self.y / m, self.z / m)
    }

    fn dot(&self, t: &Tuple) -> f64 {
        self.x * t.x +
        self.y * t.y +
        self.z * t.z +
        self.w * t.w
    }

    fn cross(&self, t: &Tuple) -> Tuple {
        Tuple::vector(
            self.y * t.z - self.z * t.y,
            self.z * t.x - self.x * t.z,
            self.x * t.y - self.y * t.x
            )
    }
}

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
}

struct Canvas {
    pub width: usize,
    pub height: usize,
    canvas: Vec<Color>
}

const BLACK: Color = Color { r: 0.0, g: 0.0, b: 0.0 };

use std::collections::LinkedList;
use num::clamp;

impl Canvas {
    fn canvas(width: usize, height: usize) -> Canvas {
        Canvas { 
            width, 
            height,
            canvas: vec![BLACK; width * height]
        }
    }

    fn pixel_at(&self, x: usize, y: usize) -> Color {
        self.canvas[y * self.width + x]
    }

    fn write_pixel(&mut self, x: usize, y: usize, c: Color) {
        self.canvas[y * self.width + x] = c;
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
        let start_index = row * self.width;
        let mut row = String::with_capacity(80);

        for i in start_index..start_index + self.width {
            let c = self.canvas[i];
            Canvas::ppm_add_string_to_row(&mut row, Canvas::ppm_color(c.r), file_content);
            Canvas::ppm_add_string_to_row(&mut row, Canvas::ppm_color(c.g), file_content);
            Canvas::ppm_add_string_to_row(&mut row, Canvas::ppm_color(c.b), file_content);
        }
        if row.len() > 0 {
            row += "\n";
            file_content.push_back(row);
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

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuple_is_point() {
        let p = Tuple { x: 4.3, y: -4.2, z: 3.1, w: 1.0 };

        assert_eq!(p.x, 4.3);
        assert_eq!(p.y, -4.2);
        assert_eq!(p.z, 3.1);
        assert_eq!(p.w, 1.0);
        assert!(p.is_point());
        assert!(!p.is_vector());
    }

    #[test]
    fn tuple_is_vector() {
        let v = Tuple { x: 4.3, y: -4.2, z: 3.1, w: 0.0 };

        assert_eq!(v.x, 4.3);
        assert_eq!(v.y, -4.2);
        assert_eq!(v.z, 3.1);
        assert_eq!(v.w, 0.0);
        assert!(!v.is_point());
        assert!(v.is_vector());
    }

    #[test]
    fn point_creates_tuple_with_w_1() {
        let p = Tuple::point(4.0, -4.0, 3.0);
        let pt = Tuple { x: 4.0, y: -4.0, z: 3.0, w: 1.0 };

        assert_eq!(p, pt);
    }

    #[test]
    fn vector_creates_tuple_with_w_0() {
        let v = Tuple::vector(4.0, -4.0, 3.0);
        let vt = Tuple { x: 4.0, y: -4.0, z: 3.0, w: 0.0 };

        assert_eq!(v, vt);
    }

    #[test]
    fn adding_two_tuples() {
        let a1 = Tuple { x: 3.0, y: -2.0, z: 5.0, w: 1.0 };
        let a2 = Tuple { x: -2.0, y: 3.0, z: 1.0, w: 0.0 };

        let expected = Tuple { x: 1.0, y: 1.0, z: 6.0, w: 1.0 };
        let actual = a1 + a2;

        assert_eq!(expected, actual);
    }

    #[test]
    fn subtracting_two_points() {
        let p1 = Tuple::point(3.0, 2.0, 1.0);
        let p2 = Tuple::point(5.0, 6.0, 7.0);

        let expected = Tuple::vector(-2.0, -4.0, -6.0);
        let actual = p1 - p2;

        assert_eq!(expected, actual);
    }

    #[test]
    fn subtracting_a_vector_from_a_point() {
        let p = Tuple::point(3.0, 2.0, 1.0);
        let v = Tuple::vector(5.0, 6.0, 7.0);

        let expected = Tuple::point(-2.0, -4.0, -6.0);
        let actual = p - v;

        assert_eq!(expected, actual);
    }

    #[test]
    fn subtracting_a_vector_from_a_vector() {
        let v1 = Tuple::vector(3.0, 2.0, 1.0);
        let v2 = Tuple::vector(5.0, 6.0, 7.0);

        let expected = Tuple::vector(-2.0, -4.0, -6.0);
        let actual = v1 - v2;

        assert_eq!(expected, actual);
    }

    #[test]
    fn subtracting_a_vector_from_the_zero_vector() {
        let zero = Tuple::vector(0.0, 0.0, 0.0);
        let v = Tuple::vector(1.0, -2.0, 3.0);

        let expected = Tuple::vector(-1.0, 2.0, -3.0);
        let actual = zero - v;

        assert_eq!(expected, actual);
    }

    #[test]
    fn negating_a_tuple() {
        let a = Tuple { x: 1.0, y: -2.0, z: 3.0, w: -4.0 };

        let expected = Tuple { x: -1.0, y: 2.0, z: -3.0, w: 4.0 };
        let actual = -a;

        assert_eq!(expected, actual);
    }

    #[test]
    fn multiplying_tuple_by_scalar() {
        let a = Tuple { x: 1.0, y: -2.0, z: 3.0, w: -4.0 };

        let expected = Tuple { x: 3.5, y: -7.0, z: 10.5, w: -14.0 };
        let actual = a * 3.5;

        assert_eq!(expected, actual);
    }

    #[test]
    fn multiplying_tuple_by_fraction() {
        let a = Tuple { x: 1.0, y: -2.0, z: 3.0, w: -4.0 };

        let expected = Tuple { x: 0.5, y: -1.0, z: 1.5, w: -2.0 };
        let actual = a * 0.5;

        assert_eq!(expected, actual);
    }

    #[test]
    fn dividing_tuple_by_scalar() {
        let a = Tuple { x: 1.0, y: -2.0, z: 3.0, w: -4.0 };

        let expected = Tuple { x: 0.5, y: -1.0, z: 1.5, w: -2.0 };
        let actual = a / 2.0;

        assert_eq!(expected, actual);
    }

    #[test]
    fn computing_magnitude_of_vector_1_0_0() {
        let v = Tuple::vector(1.0, 0.0, 0.0);

        assert_eq!(1.0, v.magnitude());
    }

    #[test]
    fn computing_magnitude_of_vector_0_1_0() {
        let v = Tuple::vector(0.0, 1.0, 0.0);

        assert_eq!(1.0, v.magnitude());
    }

    #[test]
    fn computing_magnitude_of_vector_0_0_1() {
        let v = Tuple::vector(0.0, 0.0, 1.0);

        assert_eq!(1.0, v.magnitude());
    }

    #[test]
    fn computing_magnitude_of_vector_1_2_3() {
        let v = Tuple::vector(1.0, 2.0, 3.0);
        let expected = 14_f64.sqrt();
        assert_eq!(expected, v.magnitude());
    }

    #[test]
    fn computing_magnitude_of_vector_1_2_3_neg() {
        let v = Tuple::vector(-1.0, -2.0, -3.0);
        let expected = 14_f64.sqrt();
        assert_eq!(expected, v.magnitude());
    }

    #[test]
    fn normalizing_vector_4_0_0_gives_1_0_0() {
        let v = Tuple::vector(4.0, 0.0, 0.0);
        let expected = Tuple::vector(1.0, 0.0, 0.0);
        assert_eq!(expected, v.normalize());
    }

    #[test]
    fn normalizing_vector_1_2_3() {
        let v = Tuple::vector(1.0, 2.0, 3.0);
        let expected = Tuple::vector(0.26726, 0.53452, 0.80178);
        assert_eq!(expected, v.normalize());
    }

    #[test]
    fn magnitude_of_normalized_vector_is_1() {
        let v = Tuple::vector(1.0, 2.0, 3.0);
        let norm = v.normalize();
        assert_eq!(1.0, norm.magnitude());
    }

    #[test]
    fn dot_product_of_two_tuples() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);
        let actual = a.dot(&b);
        assert_eq!(20.0, actual);
    }

    #[test]
    fn cross_product_of_two_vectors() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);

        let actual_ab = a.cross(&b);
        assert_eq!(Tuple::vector(-1.0, 2.0, -1.0), actual_ab);
        let actual_ba = b.cross(&a);
        assert_eq!(Tuple::vector(1.0, -2.0, 1.0), actual_ba);
    }

    // Chapter 2
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

        for a in c.canvas {
            assert_eq!(a, BLACK);
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
}




