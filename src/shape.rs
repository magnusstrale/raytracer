use super::tuple::Tuple;
use super::ray::Ray;
use super::intersection::Intersections;
use super::material::Material;

pub trait Shape {
    fn intersect(&self, ray: Ray) -> Intersections;
    fn set_material(&mut self, material: Material);
    fn normal_at(&self, world_point: Tuple) -> Tuple;
}
