use super::color::{Color, WHITE};
use super::tuple::Tuple;

struct PointLight {
    position: Tuple,
    intensity: Color
}

impl PointLight {
    fn new(position: Tuple, intensity: Color) -> PointLight {
        PointLight { position, intensity }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_light_has_position_and_intensity() {
        let intensity = WHITE;
        let position = Tuple::point(0.0, 0.0, 0.0);
        let light = PointLight::new(position, intensity);

        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
    }

}