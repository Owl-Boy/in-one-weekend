use std::f64::consts::PI;

use crate::{vec3::{Point, Vec3}, ray::Ray};

#[derive(Debug)]
pub struct Camera {
    pub origin: Point,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lower_left_corner: Point,
    pub u: Vec3,
    pub v: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(look_from: Point, look_at: Point, view_up: Vec3, vertical_fov: f64, aspect: f64, aperture: f64, focus_dist: f64) -> Self {
        let theta = vertical_fov * PI / 180.0;
        let half_height = focus_dist * f64::tan(theta / 2.0);
        let half_width = aspect * half_height;
        let w = (look_from - look_at).unit_along();
        let u = Vec3::cross(view_up, w);
        let v = Vec3::cross(w, u);
        Camera { 
            origin: look_from , 
            horizontal: u * 2.0 * half_width, 
            vertical: v * 2.0 * half_height, 
            lower_left_corner: look_from - u * half_width - v * half_height - w * focus_dist, 
            u, v, 
            lens_radius: aperture / 2.0
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = Vec3::rand_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        Ray {
            origin: self.origin + offset,
            dir: self.lower_left_corner + self.horizontal*s + self.vertical*t - self.origin - offset,
        }
    }
}
