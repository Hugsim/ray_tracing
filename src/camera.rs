use crate::vec3::*;
use crate::ray::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Camera {
    origin: Pos3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Pos3,
}

impl Camera {
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let origin = self.origin;
        let direction = self.lower_left_corner + u * self.horizontal + v * self.vertical;
        Ray::new(origin, direction)
    }
}

pub fn new() -> Camera {
    let aspect_ratio = 16.0 / 9.0;
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Pos3::new(0.0, 0.0, 0.0);
    let horizontal = Pos3::new(viewport_width, 0.0, 0.0);
    let vertical = Pos3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Pos3::new(0.0, 0.0, focal_length);

    Camera {
        origin, horizontal, vertical, lower_left_corner,
    }
}
