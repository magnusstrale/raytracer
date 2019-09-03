use super::sphere::Sphere;
use super::shape::Shape;
use super::color::{Color, WHITE};
use super::tuple::Tuple;
use super::matrix::Matrix;
use super::ray::Ray;
use super::material::{Material, DEFAULT_AMBIENT, DEFAULT_SHININESS};
use super::intersection::Intersections;

use super::light::PointLight;

pub struct World {
    pub light: Option<PointLight>,
    pub objects: Vec<Sphere>
}

impl World {
    fn new(light: PointLight, objects: Vec<Sphere>) -> Self {
        World { light: Some(light), objects }
    }

    fn empty() -> Self {
        World { light: None, objects: vec![] }
    }

    fn new_default() -> Self {
        let light = PointLight::new(Tuple::point(-10.0, 10.0, -10.0), WHITE);
        let m = Material::new(Color::new(0.8, 1.0, 0.6), DEFAULT_AMBIENT, 0.7, 0.2, DEFAULT_SHININESS);
        let s1 = Sphere::new(Some(m), None);
        let tr = Matrix::scaling(0.5, 0.5, 0.5);
        let s2 = Sphere::new(None, Some(tr));
        World::new(light, vec![s1, s2])
    }

    fn intersect(&self, ray: Ray) -> Intersections {
        let mut xs = Intersections::new(vec![]);
        for o in self.objects.iter() {
            xs.extend(o.intersect(ray));
        }
        xs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_world()
    {
        let w = World::empty();

        assert_eq!(w.objects.len(), 0);
        assert_eq!(w.light, None);
    }

    #[test]
    fn default_world()
    {
        let light = PointLight::new(Tuple::point(-10.0, 10.0, -10.0), WHITE);
        let w = World::new_default();
        assert_eq!(w.light.unwrap(), light);

        let m = w.objects[0].material;
        assert_eq!(m.color, Color::new(0.8, 1.0, 0.6));
        assert_eq!(m.diffuse, 0.7);
        assert_eq!(m.specular, 0.2);

        let tr = w.objects[1].transform;
        assert_eq!(tr, Matrix::scaling(0.5, 0.5, 0.5));
    }

    #[test]
    fn intersect_world_with_ray() {
        let w = World::new_default();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = w.intersect(r);

        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 4.5);
        assert_eq!(xs[2].t, 5.5);
        assert_eq!(xs[3].t, 6.0);
    }
}