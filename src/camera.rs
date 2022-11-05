use crate::{vec3::{Point, Vec3}, ray::Ray};

// Image
pub const ASPECT_RATIO: f32 = 16.0 / 9.0;

// Camera
pub const VIEWPORT_HEIGHT: f32 = 2.0;
pub const VIEWPORT_WIDTH: f32 = VIEWPORT_HEIGHT * ASPECT_RATIO;
pub const FOCAL_LENGTH: f32 = 1.0;

#[derive(Debug)]
pub struct Camera {
    pub origin: Point,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lower_left_corner: Point,
}

impl Camera {
    pub fn new() -> Self {
        let origin = Point::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
        let vertical = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
        let lower_left_corner=  origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);
        Camera { origin, horizontal, vertical, lower_left_corner }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.origin,
            dir: self.lower_left_corner + self.horizontal*u + self.vertical*v - self.origin,
        }
    }
}
