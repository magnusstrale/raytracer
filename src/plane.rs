use super::intersection::{Intersection, Intersections};
use super::material::{Material, DEFAULT_MATERIAL};
use super::matrix::{Matrix, IDENTITY_MATRIX};
use super::ray::Ray;
use super::shape::{BoxShape, Shape};
use super::tuple::{Tuple, VECTOR_Y_UP};
use std::any::Any;

#[derive(Debug, Clone, PartialEq)]
pub struct Plane {}

impl Shape for Plane {
    fn box_clone(&self) -> BoxShape {
        Box::new((*self).clone())
    }

    fn box_eq(&self, other: &dyn Any) -> bool {
        other.downcast_ref::<Self>().map_or(false, |a| self == a)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn inner_intersect(&self, object_ray: Ray) -> Intersections {
        if super::approx_eq(0., object_ray.direction.y) {
            return Intersections::new(vec![]);
        }
        let t = -object_ray.origin.y / object_ray.direction.y;

        Intersections::new(vec![Intersection::new(t, Box::new(self.clone()))])
    }

    fn inner_normal_at(&self, _object_point: Tuple) -> Tuple {
        VECTOR_Y_UP
    }

    fn material(&self) -> &Material {
        &DEFAULT_MATERIAL
    }

    fn transformation(&self) -> Matrix {
        IDENTITY_MATRIX
    }

    fn inverse_transformation(&self) -> Matrix {
        IDENTITY_MATRIX
    }
}

impl Plane {
    pub fn new() -> Self {
        Plane {}
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_of_plane_is_constant() {
        let p = Plane::new();
        let n1 = p.inner_normal_at(Tuple::point(0., 0., 0.));
        let n2 = p.inner_normal_at(Tuple::point(10., 0., -10.));
        let n3 = p.inner_normal_at(Tuple::point(-5., 0., 150.));

        assert_eq!(n1, Tuple::vector(0., 1., 0.));
        assert_eq!(n2, Tuple::vector(0., 1., 0.));
        assert_eq!(n3, Tuple::vector(0., 1., 0.));
    }

    #[test]
    fn intersect_with_ray_parallel_to_plane() {
        let p = Plane::new();
        let r = Ray::new(Tuple::point(0., 10., 0.), Tuple::vector(0., 0., 1.));
        let xs = p.inner_intersect(r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn intersect_with_coplanar_ray() {
        let p = Plane::new();
        let r = Ray::new(Tuple::point(0., 0., 0.), Tuple::vector(0., 0., 1.));
        let xs = p.inner_intersect(r);

        assert_eq!(xs.len(), 0);
    }

    fn box_plane(p: Plane) -> BoxShape {
        Box::new(p)
    }
    
    #[test]
    fn ray_intersecting_plane_from_above() {
        let p = Plane::new();
        let r = Ray::new(Tuple::point(0., 1., 0.), Tuple::vector(0., -1., 0.));
        let xs = p.inner_intersect(r);

        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.);
        assert_eq!(&xs[0].object, &box_plane(p));
    }
    
    #[test]
    fn below() {
        let p = Plane::new();
        let r = Ray::new(Tuple::point(0., -1., 0.), Tuple::vector(0., 1., 0.));
        let xs = p.inner_intersect(r);

        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.);
        assert_eq!(&xs[0].object, &box_plane(p));
    }
}
