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
    pub fn point(x: f64, y: f64, z: f64) -> Tuple {
        Tuple {x, y, z, w: 1.0}
    }

    pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
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
            self.x * t.y - self.y * t.x)
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
}