use std::f64::consts::{PI, SQRT_2};

use super::tuple::{Tuple, ORIGO, VECTOR_Y_UP};
use super::color::GREEN;
use super::ray::Ray;
use super::intersection::Intersections;
use super::material::{Material, DEFAULT_MATERIAL};
use super::matrix::{Matrix, IDENTITY_MATRIX};

pub trait Shape {
    fn inner_intersect(&self, object_ray: Ray) -> Intersections;
    fn inner_normal_at(&self, object_point: Tuple) -> Tuple;
    fn material(&self) -> &Material;
    fn transformation(&self) -> Matrix;
    fn inverse_transformation(&self) -> Matrix;

    fn intersect(&self, world_ray: Ray) -> Intersections {
        self.inner_intersect(world_ray.transform(self.inverse_transformation()))
    }

    fn normal_at(&self, world_point: Tuple) -> Tuple {
        let object_normal = self.inner_normal_at(self.inverse_transformation() * world_point);
        let mut world_normal = self.inverse_transformation().transpose() * object_normal;
        world_normal.w = 0.;

        world_normal.normalize()
    }

    fn inverse_transform_parameter(transform: Option<Matrix>) -> Matrix {
        match transform {
            None => IDENTITY_MATRIX,
            Some(t) => t.inverse().unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static mut saved_ray: Ray = Ray { origin: ORIGO, direction: VECTOR_Y_UP };

    struct TestShape {
        material: Material,
        inverse_transform: Matrix,
        transform: Matrix
    }

    impl Shape for TestShape {
        fn inner_intersect(&self, object_ray: Ray) -> Intersections {
            unsafe {
                saved_ray = object_ray;
            }
            Intersections::new(vec![])
        }

        fn inner_normal_at(&self, object_point: Tuple) -> Tuple {
            Tuple::vector(object_point.x, object_point.y, object_point.z)
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

    impl TestShape {
        fn new(material: Option<Material>, transform: Option<Matrix>) -> Self {
            Self { 
                material: material.unwrap_or(DEFAULT_MATERIAL), 
                transform: transform.unwrap_or(IDENTITY_MATRIX),
                inverse_transform: TestShape::inverse_transform_parameter(transform)
            }
        }
    }

    #[test]
    fn default_transformation() {
        let s = TestShape::new(None, None);

        assert_eq!(s.transformation(), IDENTITY_MATRIX);
    }

    #[test]
    fn assign_transformation() {
        let tr = Matrix::translation(2., 3., 4.);
        let s = TestShape::new(None, Some(tr));

        assert_eq!(s.transformation(), tr);
    }

    #[test]
    fn default_material() {
        let s = TestShape::new(None, None);
        let m = s.material();

        assert_eq!(*m, DEFAULT_MATERIAL);
    }

    #[test]
    fn assign_material() {
        let m = Material::new(GREEN, 0.1, 0.2, 0.3, 0.4);
        let s = TestShape::new(Some(m.clone()), None);

        assert_eq!(*s.material(), m);
    }

    #[test]
    fn intersect_scaled_shape_with_ray()
    {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let tr = Matrix::scaling(2., 2., 2.);
        let s = TestShape::new(None, Some(tr));
        s.intersect(r);

        unsafe {
            assert_eq!(saved_ray.origin, Tuple::point(0., 0., -2.5));
            assert_eq!(saved_ray.direction, Tuple::vector(0., 0., 0.5));
        }
    }

    #[test]
    fn intersect_translated_shape_with_ray()
    {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let tr = Matrix::translation(5., 0., 0.);
        let s = TestShape::new(None, Some(tr));
        s.intersect(r);

        unsafe {
            assert_eq!(saved_ray.origin, Tuple::point(-5., 0., -5.));
            assert_eq!(saved_ray.direction, Tuple::vector(0., 0., 1.));
        }
    }

    #[test]
    fn compute_normal_on_translated_shape() {
        let tr = Matrix::translation(0., 1., 0.);
        let s = TestShape::new(None, Some(tr));
        let n = s.normal_at(Tuple::point(0., 1.70711, -0.70711));

        assert_eq!(n, Tuple::vector(0., 0.70711, -0.70711));
    }

    #[test]
    fn compute_normal_on_transformed_shape() {
        let tr = Matrix::scaling(1., 0.5, 1.) * Matrix::rotation_z(PI / 5.);
        let s = TestShape::new(None, Some(tr));
        let n = s.normal_at(Tuple::point(0., SQRT_2 / 2., -SQRT_2 / 2.));

        assert_eq!(n, Tuple::vector(0., 0.97014, -0.24254));
    }

}