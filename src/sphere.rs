use super::intersection::{Intersection, Intersections};
use super::material::Material;
use super::matrix::{Matrix, IDENTITY_MATRIX};
use super::ray::Ray;
use super::shape::{inverse_transform_parameter, Shape, BoxShape};
use super::tuple::{Tuple, ORIGO};
use std::any::Any;
use std::f64::consts::*;

#[derive(Debug, Clone)]
pub struct Sphere {
    inverse_transform: Matrix,
    transform: Matrix,
    material: Material,
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self.transform == other.transform && self.material == other.material
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            transform: IDENTITY_MATRIX,
            inverse_transform: IDENTITY_MATRIX,
            material: Material::default(),
        }
    }
}

impl Shape for Sphere {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn box_eq(&self, other: &dyn Any) -> bool {
        other.downcast_ref::<Self>().map_or(false, |a| self == a)
    }

    fn box_clone(&self) -> BoxShape {
        Box::new((*self).clone())
    }

    fn inner_intersect(&self, object_ray: Ray) -> Intersections {
        let sphere_to_ray = object_ray.origin - ORIGO;
        let a = object_ray.direction.dot(&object_ray.direction);
        let b = 2.0 * object_ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return Intersections::new(vec![]);
        }

        let i1 = Intersection::new(
            (-b - discriminant.sqrt()) / (2.0 * a),
            Box::new(self.clone()),
        );
        let i2 = Intersection::new(
            (-b + discriminant.sqrt()) / (2.0 * a),
            Box::new(self.clone()),
        );
        Intersections::new(vec![i2, i1])
    }

    fn inner_normal_at(&self, object_point: Tuple) -> Tuple {
        object_point - ORIGO
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn transformation(&self) -> Matrix {
        self.transform
    }

    fn inverse_transformation(&self) -> Matrix {
        self.inverse_transform
    }
}

impl Sphere {
    pub fn new(material: Option<Material>, transform: Option<Matrix>) -> Self {
        Self {
            transform: transform.unwrap_or(IDENTITY_MATRIX),
            inverse_transform: inverse_transform_parameter(transform),
            material: material.unwrap_or(Material::default()),
        }
    }

    pub fn new_boxed(material: Option<Material>, transform: Option<Matrix>) -> BoxShape {
        Box::new(Sphere::new(material, transform))
    }

    pub fn default_boxed() -> BoxShape {
        Box::new(Sphere::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ray_intersect_sphere_at_two_points() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let s = Sphere::default();
        let xs = s.inner_intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.);
        assert_eq!(xs[1].t, 6.);
    }

    #[test]
    fn ray_intersect_sphere_at_tangent() {
        let r = Ray::new(Tuple::point(0., 1., -5.), Tuple::vector(0., 0., 1.));
        let s = Sphere::default();
        let xs = s.inner_intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.);
        assert_eq!(xs[1].t, 5.);
    }

    #[test]
    fn ray_miss_sphere() {
        let r = Ray::new(Tuple::point(0., 2., -5.), Tuple::vector(0., 0., 1.));
        let s = Sphere::default();
        let xs = s.inner_intersect(r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_originates_insider_sphere() {
        let r = Ray::new(Tuple::point(0., 0., 0.), Tuple::vector(0., 0., 1.));
        let s = Sphere::default();
        let xs = s.inner_intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.);
        assert_eq!(xs[1].t, 1.);
    }

    #[test]
    fn sphere_behind_ray() {
        let r = Ray::new(Tuple::point(0., 0., 5.), Tuple::vector(0., 0., 1.));
        let s = Sphere::default();
        let xs = s.inner_intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.);
        assert_eq!(xs[1].t, -4.);
    }

    #[test]
    fn normal_on_sphere_on_x_axis() {
        let s = Sphere::default();
        let n = s.inner_normal_at(Tuple::point(1., 0., 0.));

        assert_eq!(n, Tuple::vector(1., 0., 0.));
    }

    #[test]
    fn normal_on_sphere_on_y_axis() {
        let s = Sphere::default();
        let n = s.inner_normal_at(Tuple::point(0., 1., 0.));

        assert_eq!(n, Tuple::vector(0., 1., 0.));
    }

    #[test]
    fn normal_on_sphere_on_z_axis() {
        let s = Sphere::default();
        let n = s.inner_normal_at(Tuple::point(0., 0., 1.));

        assert_eq!(n, Tuple::vector(0., 0., 1.));
    }

    #[test]
    fn normal_on_sphere_on_non_axial_point() {
        let s = Sphere::default();
        let pv = 3.0f64.sqrt() / 3.0;
        let n = s.inner_normal_at(Tuple::point(pv, pv, pv));

        assert_eq!(n, Tuple::vector(pv, pv, pv));
    }

    #[test]
    fn normal_is_normalized_vector() {
        let s = Sphere::default();
        let pv = 3.0f64.sqrt() / 3.0;
        let n = s.inner_normal_at(Tuple::point(pv, pv, pv));

        assert_eq!(n, n.normalize());
    }
}
