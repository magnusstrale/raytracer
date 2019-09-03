use super::color::{Color, BLACK, WHITE};
use super::tuple::{Tuple, ORIGO};
use super::pointlight::PointLight;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64
}

pub const DEFAULT_AMBIENT: f64 = 0.1;
pub const DEFAULT_DIFFUSE: f64 = 0.9;
pub const DEFAULT_SPECULAR: f64 = 0.9;
pub const DEFAULT_SHININESS: f64 = 200.0;

impl Material {
    pub fn new(color: Option<Color>, ambient: Option<f64>, diffuse: Option<f64>, specular: Option<f64>, shininess: Option<f64>) -> Material {
        Material { 
            color: color.unwrap_or(WHITE), 
            ambient: ambient.unwrap_or(DEFAULT_AMBIENT), 
            diffuse: diffuse.unwrap_or(DEFAULT_DIFFUSE), 
            specular: specular.unwrap_or(DEFAULT_SPECULAR), 
            shininess: shininess.unwrap_or(DEFAULT_SHININESS)
        }
    }

    pub fn new_default() -> Material {
        Material::new(None, None, None, None, None)
    }

    pub fn lighting(&self, light: &PointLight, point: Tuple, eyev: Tuple, normalv: Tuple) -> Color {
        let effective_color = self.color * light.intensity;
        let lightv = (light.position - point).normalize();
        let ambient = effective_color * self.ambient;
        let light_dot_normal = lightv.dot(&normalv);
        let (diffuse, specular) = if light_dot_normal < 0.0 {
            (BLACK, BLACK)
        }
        else {
            let reflectv = (-lightv).reflect(normalv);
            let reflect_dot_eye = reflectv.dot(&eyev);
            (effective_color * self.diffuse * light_dot_normal, 
            if reflect_dot_eye <= 0.0 { 
                BLACK
            }
            else {
                let factor = reflect_dot_eye.powf(self.shininess);
                light.intensity * self.specular * factor
            })
        };
        ambient + diffuse + specular
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_material() {
        let m = Material::new_default();
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }

    #[test]
    fn lighing_eye_between_light_and_surface() {
        let m = Material::new_default();
        let position = ORIGO;

        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 0.0, -10.0), WHITE);

        let result = m.lighting(&light, position, eyev, normalv);

        assert_eq!(result, Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighing_eye_between_light_and_surface_eye_offset_45_degrees() {
        let m = Material::new_default();
        let position = ORIGO;

        let pv = 2.0f64.sqrt() / 2.0;
        let eyev = Tuple::vector(0.0, pv, -pv);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 0.0, -10.0), WHITE);

        let result = m.lighting(&light, position, eyev, normalv);

        assert_eq!(result, Color::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn lighing_eye_opposite_surface_light_offset_45_degrees() {
        let m = Material::new_default();
        let position = ORIGO;

        let eyev = Tuple::vector(0.0, 0.0, -1.0 );
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 10.0, -10.0), WHITE);

        let result = m.lighting(&light, position, eyev, normalv);

        assert_eq!(result, Color::new(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn lighing_eye_in_path_of_reflection_vector() {
        let m = Material::new_default();
        let position = ORIGO;

        let pv = -2.0f64.sqrt() / 2.0;
        let eyev = Tuple::vector(0.0, pv, pv);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 10.0, -10.0), WHITE);

        let result = m.lighting(&light, position, eyev, normalv);

        assert_eq!(result, Color::new(1.6364, 1.6364, 1.6364));
    }

    #[test]
    fn lighing_light_behind_surface() {
        let m = Material::new_default();
        let position = ORIGO;

        let eyev = Tuple::vector(0.0, 0.0, -1.0 );
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 0.0, 10.0), WHITE);

        let result = m.lighting(&light, position, eyev, normalv);

        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }
}