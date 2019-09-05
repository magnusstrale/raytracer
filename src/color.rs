use core::ops;

// Chapter 2
#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64
}

pub const BLACK: Color = Color { r: 0.0, g: 0.0, b: 0.0 };
pub const WHITE: Color = Color { r: 1.0, g: 1.0, b: 1.0 };
pub const RED: Color = Color { r: 1.0, g: 0.0, b: 0.0 };
pub const GREEN: Color = Color { r: 0.0, g: 1.0, b: 0.0 };
pub const BLUE: Color = Color { r: 0.0, g: 0.0, b: 1.0 };

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        super::approx_eq(self.r, other.r) &&
        super::approx_eq(self.g, other.g) &&
        super::approx_eq(self.b, other.b)
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
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color {r, g, b}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colors_are_rgb_tuples()
    {
        let c = Color::new(-0.5, 0.4, 1.7);

        assert_eq!(c.r, -0.5);
        assert_eq!(c.g, 0.4);
        assert_eq!(c.b, 1.7);
    }

    #[test]
    fn adding_colors()
    {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        let actual = c1 + c2;
        let expected = Color::new(1.6, 0.7, 1.0);

        assert_eq!(expected, actual);
    }

    #[test]
    fn subtracting_colors()
    {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        let actual = c1 - c2;
        let expected = Color::new(0.2, 0.5, 0.5);

        assert_eq!(expected, actual);
    }

    #[test]
    fn multiplying_color_by_scalar()
    {
        let c = Color::new(0.2, 0.3, 0.4);

        let actual = c * 2.0;
        let expected = Color::new(0.4, 0.6, 0.8);

        assert_eq!(expected, actual);
    }

    #[test]
    fn multiplying_colors()
    {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);

        let actual = c1 * c2;
        let expected = Color::new(0.9, 0.2, 0.04);

        assert_eq!(expected, actual);
    }

}