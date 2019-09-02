use super::tuple::Tuple;
use super::ray::Ray;
use super::intersection::{Intersection, Intersections};
use super::matrix::Matrix;
use std::sync::atomic::{AtomicUsize, Ordering};

static SPHERE_COUNT: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    index: usize,
    pub transform: Matrix,
    inverse_transform: Matrix
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl Sphere {
    pub fn new() -> Self {
        let im = Matrix::identity_matrix();
        Sphere { index: SPHERE_COUNT.fetch_add(1, Ordering::SeqCst), transform: im, inverse_transform: im }
    }

    pub fn intersect(&self, ray: Ray) -> Intersections {
        let ray2 = ray.transform(self.inverse_transform);
        let sphere_to_ray = ray2.origin - Tuple::point(0.0, 0.0, 0.0);
        let a = ray2.direction.dot(&ray2.direction);
        let b = 2.0 * ray2.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 { return Intersections::new(vec![]) }

        let i1 = Intersection::new((-b - discriminant.sqrt()) / (2.0 * a), *self);
        let i2 = Intersection::new((-b + discriminant.sqrt()) / (2.0 * a), *self);
        Intersections::new(vec![i2, i1])
    }

    pub fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
        self.inverse_transform = transform.inverse().unwrap();  // Will blow up if transformation matrix is not invertible, which is a good thing
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ray_intersect_sphere_at_two_points()
    {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
    }    

    #[test]
    fn ray_intersect_sphere_at_tangent()
    {
        let r = Ray::new(Tuple::point(0.0, 1.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0);
    }    

    #[test]
    fn ray_miss_sphere()
    {
        let r = Ray::new(Tuple::point(0.0, 2.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_originates_insider_sphere() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }

    #[test]
    fn sphere_behind_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);
    }

    #[test]
    fn sphere_default_transform() {
        let s = Sphere::new();

        assert_eq!(s.transform, Matrix::identity_matrix());
    }

    #[test]
    fn change_sphere_transform() {
        let mut s = Sphere::new();
        let t = Matrix::translation(2.0, 3.0, 4.0);
        s.set_transform(t);

        assert_eq!(s.transform, t);
    }

    #[test]
    fn intersect_scaled_sphere_with_ray()
    {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        s.set_transform(Matrix::scaling(2.0, 2.0, 2.0));
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.0);
        assert_eq!(xs[1].t, 7.0);
    }

    #[test]
    fn intersect_translated_sphere_with_ray()
    {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        s.set_transform(Matrix::translation(5.0, 0.0, 0.0));
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 0);
    }
}
