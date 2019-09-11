use std::f64::consts::*;
use super::color::*;
use super::canvas::*;
use super::tuple::*;
use super::ray::*;
use super::shape::*;
use super::sphere::*;
use super::matrix::*;
use super::light::*;
use super::material::*;
use super::world::*;
use super::camera::*;


#[cfg(pics)]
mod tests {
    use super::*;

    #[test]
    fn canvas_to_file()
    {
        let mut c = Canvas::new(100, 100);
        c.write_pixel(1, 1, RED);
        c.write_pixel(99, 0, GREEN);
        c.write_pixel(99, 99, WHITE);
        c.save("black.png").unwrap();
    }

    #[test]
    fn circle_shadow()
    {
        const CANVAS_PIXELS: usize = 1000;
        const WALL_SIZE:usize = 7;
        let ray_origin = Tuple::point(0., 0., -5.);
        let wall_z = 10.0;
        let pixel_size = WALL_SIZE as f64 / CANVAS_PIXELS as f64;
        let half = WALL_SIZE as f64 / 2.0;

        let mut canvas = Canvas::new(CANVAS_PIXELS, CANVAS_PIXELS);
        let color = Color::new(1., 0., 0.);
        let tr = Matrix::shearing(1., 0., 0.5, 0., 0., 0.) * Matrix::scaling(1., 0.5, 1.);
        let shape = Sphere::new(None, Some(tr));
        for y in 0..CANVAS_PIXELS {
            let world_y = half - pixel_size * (y as f64);
            for x in 0..CANVAS_PIXELS {
                let world_x = -half + pixel_size * (x as f64);
                let position = Tuple::point(world_x, world_y, wall_z);
                let r = Ray::new(ray_origin, (position - ray_origin).normalize());
                let xs = shape.intersect(r);
                match xs.hit() {
                    Some(_i) => canvas.write_pixel(x, y, color),
                    None => ()
                }
            }
        }
        canvas.save("shadow.png").unwrap();
    }

    #[test]
    fn rendered_sphere()
    {
        const CANVAS_PIXELS: usize = 1000;
        const WALL_SIZE:usize = 7;
        let ray_origin = Tuple::point(0., 0., -5.);
        let wall_z = 10.0;
        let pixel_size = WALL_SIZE as f64 / CANVAS_PIXELS as f64;
        let half = WALL_SIZE as f64 / 2.0;

        let mut canvas = Canvas::new(CANVAS_PIXELS, CANVAS_PIXELS);
        let mut m = Material::default();
        m.color = Color::new(1., 0.2, 1.);
        let tr = Matrix::shearing(1., 0., 0.5, 0., 0., 0.) * Matrix::scaling(1., 0.5, 1.);
        let shape = Sphere::new(Some(m), Some(tr));
        let light_position = Tuple::point(-10., 10., -10.);
        let light_color = WHITE;
        let light = PointLight::new(light_position, light_color);

        for y in 0..CANVAS_PIXELS {
            let world_y = half - pixel_size * (y as f64);
            for x in 0..CANVAS_PIXELS {
                let world_x = -half + pixel_size * (x as f64);
                let position = Tuple::point(world_x, world_y, wall_z);
                let r = Ray::new(ray_origin, (position - ray_origin).normalize());
                let xs = shape.intersect(r);
                match xs.hit() {
                    Some(h) => {
                        let point = r.position(h.t);
                        let normal = h.object.normal_at(point);
                        let eye = - r.direction;
                        let color = h.object.material().lighting(&light, point, eye, normal, false);
                        canvas.write_pixel(x, y, color);
                    },
                    _ => ()
                }
            }
        }
        canvas.save("sphere.png").unwrap();
    }

    #[test]
    fn camera_render_world() {
        let floor_material = Material::new(Color::new(1., 0.9, 0.9), DEFAULT_AMBIENT, DEFAULT_DIFFUSE, 0., DEFAULT_SHININESS);
        let floor_transform = Matrix::scaling(10., 0.01, 10.);
        let floor = Sphere::new_boxed(Some(floor_material.clone()), Some(floor_transform));

        let left_wall_transform = 
            Matrix::translation(0., 0., 5.) * 
            Matrix::rotation_y(-FRAC_PI_4) * 
            Matrix::rotation_x(FRAC_PI_2) * 
            Matrix::scaling(10., 0.01, 10.);
        let left_wall = Sphere::new_boxed(Some(floor_material.clone()), Some(left_wall_transform));

        let right_wall_transform = 
            Matrix::translation(0., 0., 5.) *
            Matrix::rotation_y(FRAC_PI_4) *
            Matrix::rotation_x(FRAC_PI_2) *
            Matrix::scaling(10., 0.01, 10.);
        let right_wall = Sphere::new_boxed(Some(floor_material), Some(right_wall_transform));

        let middle_transform = Matrix::translation(-0.5, 1., 0.5);
        let middle_material = Material::new(Color::new(0.1, 1., 0.5), DEFAULT_AMBIENT, 0.7, 0.3, DEFAULT_SHININESS);
        let middle = Sphere::new_boxed(Some(middle_material), Some(middle_transform));

        let right_transform = Matrix::translation(1.5, 0.5, -0.5) * Matrix::scaling(0.5, 0.5, 0.5);
        let right_material = Material::new(Color::new(0.5, 1., 0.1), DEFAULT_AMBIENT, 0.7, 0.3, DEFAULT_SHININESS);
        let right = Sphere::new_boxed(Some(right_material), Some(right_transform));

        let left_transform = Matrix::translation(-1.5, 0.33, -0.75) * Matrix::scaling(0.33, 0.33, 0.33);
        let left_material = Material::new(Color::new(1., 0.8, 0.1), DEFAULT_AMBIENT, 0.7, 0.3, DEFAULT_SHININESS);
        let left = Sphere::new_boxed(Some(left_material), Some(left_transform));

        let light = PointLight::new(Tuple::point(-10., 10., -10.), WHITE);
        let world = World::new(light, vec![floor, left_wall, right_wall, middle, right, left]);
        let view_transform = Matrix::view_transform(Tuple::point(0., 1.5, -5.), Tuple::point(0., 1., 0.), Tuple::vector(0., 1., 0.));
        let camera = Camera::new(700, 500, FRAC_PI_3, Some(view_transform));
        let canvas = camera.render(world);

        canvas.save("three_spheres.png").unwrap();


    }
}
