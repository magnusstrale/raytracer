use core::ops;

#[derive(Debug, Copy, Clone)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64
}

pub const ORIGO: Tuple = Tuple { x: 0., y: 0., z: 0., w: 1. };
pub const VECTOR_Y_UP: Tuple = Tuple { x: 0., y: 1., z: 0., w: 0. };

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        super::approx_eq(self.x, other.x) &&
        super::approx_eq(self.y, other.y) &&
        super::approx_eq(self.z, other.z) &&
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

impl From<[f64; 4]> for Tuple {
    fn from(item: [f64; 4]) -> Self {
        Tuple { x: item[0], y: item[1], z: item[2], w: item[3] }
    }
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Tuple {
        Tuple {x, y, z, w}
    }

    pub fn point(x: f64, y: f64, z: f64) -> Tuple {
        Tuple::new(x, y, z, 1.)
    }

    pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
        Tuple::new(x, y, z, 0.)
    }

    pub fn is_point(&self) -> bool {
        self.w == 1.
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0.
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Tuple {
        let m = self.magnitude();
        Tuple::vector(self.x / m, self.y / m, self.z / m)
    }

    pub fn dot(&self, t: &Tuple) -> f64 {
        self.x * t.x +
        self.y * t.y +
        self.z * t.z +
        self.w * t.w
    }

    pub fn cross(&self, t: &Tuple) -> Tuple {
        Tuple::vector(
            self.y * t.z - self.z * t.y,
            self.z * t.x - self.x * t.z,
            self.x * t.y - self.y * t.x)
    }

    pub fn reflect(&self, normal: Tuple) -> Tuple {
        *self - normal * 2. * self.dot(&normal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuple_is_point() {
        let p = Tuple { x: 4.3, y: -4.2, z: 3.1, w: 1. };

        assert_eq!(p.x, 4.3);
        assert_eq!(p.y, -4.2);
        assert_eq!(p.z, 3.1);
        assert_eq!(p.w, 1.);
        assert!(p.is_point());
        assert!(!p.is_vector());
    }

    #[test]
    fn tuple_is_vector() {
        let v = Tuple { x: 4.3, y: -4.2, z: 3.1, w: 0. };

        assert_eq!(v.x, 4.3);
        assert_eq!(v.y, -4.2);
        assert_eq!(v.z, 3.1);
        assert_eq!(v.w, 0.);
        assert!(!v.is_point());
        assert!(v.is_vector());
    }

    #[test]
    fn origo_is_point() {
        assert!(ORIGO.is_point());
    }
    
    #[test]
    fn point_creates_tuple_with_w_1() {
        let p = Tuple::point(4., -4., 3.);
        let pt = Tuple { x: 4., y: -4., z: 3., w: 1. };

        assert_eq!(p, pt);
    }

    #[test]
    fn vector_creates_tuple_with_w_0() {
        let v = Tuple::vector(4., -4., 3.);
        let vt = Tuple { x: 4., y: -4., z: 3., w: 0. };

        assert_eq!(v, vt);
    }

    #[test]
    fn adding_two_tuples() {
        let a1 = Tuple { x: 3., y: -2., z: 5., w: 1. };
        let a2 = Tuple { x: -2., y: 3., z: 1., w: 0. };

        let expected = Tuple { x: 1., y: 1., z: 6., w: 1. };
        let actual = a1 + a2;

        assert_eq!(expected, actual);
    }

    #[test]
    fn subtracting_two_points() {
        let p1 = Tuple::point(3., 2., 1.);
        let p2 = Tuple::point(5., 6., 7.);

        let expected = Tuple::vector(-2., -4., -6.);
        let actual = p1 - p2;

        assert_eq!(expected, actual);
    }

    #[test]
    fn subtracting_a_vector_from_a_point() {
        let p = Tuple::point(3., 2., 1.);
        let v = Tuple::vector(5., 6., 7.);

        let expected = Tuple::point(-2., -4., -6.);
        let actual = p - v;

        assert_eq!(expected, actual);
    }

    #[test]
    fn subtracting_a_vector_from_a_vector() {
        let v1 = Tuple::vector(3., 2., 1.);
        let v2 = Tuple::vector(5., 6., 7.);

        let expected = Tuple::vector(-2., -4., -6.);
        let actual = v1 - v2;

        assert_eq!(expected, actual);
    }

    #[test]
    fn subtracting_a_vector_from_the_zero_vector() {
        let zero = Tuple::vector(0., 0., 0.);
        let v = Tuple::vector(1., -2., 3.);

        let expected = Tuple::vector(-1., 2., -3.);
        let actual = zero - v;

        assert_eq!(expected, actual);
    }

    #[test]
    fn negating_a_tuple() {
        let a = Tuple { x: 1., y: -2., z: 3., w: -4. };

        let expected = Tuple { x: -1., y: 2., z: -3., w: 4. };
        let actual = -a;

        assert_eq!(expected, actual);
    }

    #[test]
    fn multiplying_tuple_by_scalar() {
        let a = Tuple { x: 1., y: -2., z: 3., w: -4. };

        let expected = Tuple { x: 3.5, y: -7., z: 10.5, w: -14. };
        let actual = a * 3.5;

        assert_eq!(expected, actual);
    }

    #[test]
    fn multiplying_tuple_by_fraction() {
        let a = Tuple { x: 1., y: -2., z: 3., w: -4. };

        let expected = Tuple { x: 0.5, y: -1., z: 1.5, w: -2. };
        let actual = a * 0.5;

        assert_eq!(expected, actual);
    }

    #[test]
    fn dividing_tuple_by_scalar() {
        let a = Tuple { x: 1., y: -2., z: 3., w: -4. };

        let expected = Tuple { x: 0.5, y: -1., z: 1.5, w: -2. };
        let actual = a / 2.0;

        assert_eq!(expected, actual);
    }

    #[test]
    fn computing_magnitude_of_vector_1_0_0() {
        let v = Tuple::vector(1., 0., 0.);

        assert_eq!(1., v.magnitude());
    }

    #[test]
    fn computing_magnitude_of_vector_0_1_0() {
        let v = Tuple::vector(0., 1., 0.);

        assert_eq!(1., v.magnitude());
    }

    #[test]
    fn computing_magnitude_of_vector_0_0_1() {
        let v = Tuple::vector(0., 0., 1.);

        assert_eq!(1., v.magnitude());
    }

    #[test]
    fn computing_magnitude_of_vector_1_2_3() {
        let v = Tuple::vector(1., 2., 3.);
        let expected = 14_f64.sqrt();
        assert_eq!(expected, v.magnitude());
    }

    #[test]
    fn computing_magnitude_of_vector_1_2_3_neg() {
        let v = Tuple::vector(-1., -2., -3.);
        let expected = 14_f64.sqrt();
        assert_eq!(expected, v.magnitude());
    }

    #[test]
    fn normalizing_vector_4_0_0_gives_1_0_0() {
        let v = Tuple::vector(4., 0., 0.);
        let expected = Tuple::vector(1., 0., 0.);
        assert_eq!(expected, v.normalize());
    }

    #[test]
    fn normalizing_vector_1_2_3() {
        let v = Tuple::vector(1., 2., 3.);
        let expected = Tuple::vector(0.26726, 0.53452, 0.80178);
        assert_eq!(expected, v.normalize());
    }

    #[test]
    fn magnitude_of_normalized_vector_is_1() {
        let v = Tuple::vector(1., 2., 3.);
        let norm = v.normalize();
        assert_eq!(1., norm.magnitude());
    }

    #[test]
    fn dot_product_of_two_tuples() {
        let a = Tuple::vector(1., 2., 3.);
        let b = Tuple::vector(2., 3., 4.);
        let actual = a.dot(&b);
        assert_eq!(20., actual);
    }

    #[test]
    fn cross_product_of_two_vectors() {
        let a = Tuple::vector(1., 2., 3.);
        let b = Tuple::vector(2., 3., 4.);

        let actual_ab = a.cross(&b);
        assert_eq!(Tuple::vector(-1., 2., -1.), actual_ab);
        let actual_ba = b.cross(&a);
        assert_eq!(Tuple::vector(1., -2., 1.), actual_ba);
    }

    #[test]
    fn reflecting_vector_approaching_45_degrees() {
        let v = Tuple::vector(1., -1., 0.);
        let n = Tuple::vector(0., 1., 0.);
        let r = v.reflect(n);

        assert_eq!(r, Tuple::vector(1., 1., 0.));
    }

    #[test]
    fn reflecting_vector_off_slanted_surface() {
        let v = Tuple::vector(0., -1., 0.);
        let pv = 2.0f64.sqrt() / 2.0;
        let n = Tuple::vector(pv, pv, 0.);
        let r = v.reflect(n);

        assert_eq!(r, Tuple::vector(1., 0., 0.));
    }
}