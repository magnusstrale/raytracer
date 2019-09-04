use super::tuple::Tuple;
use super::sphere::Sphere;

pub struct PrecomputedData {
    pub t: f64,
    pub object: Sphere,
    pub point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub inside: bool
}

impl PrecomputedData {
    pub fn new(t: f64, object: Sphere, point: Tuple, eyev: Tuple, normalv: Tuple, inside: bool) -> Self {
        PrecomputedData { t, object, point, eyev, normalv, inside }
    }
}