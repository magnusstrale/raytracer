use super::color::{Color, WHITE};

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
}