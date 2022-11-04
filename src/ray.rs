use crate::vec3::{Point, Vec3};

#[derive(Debug)]
pub struct Ray {
    pub origin: Point,
    pub dir: Vec3
}

impl Ray {
    pub fn new(origin: Point, dir: Vec3) -> Self {
        Ray { origin, dir }
    }

    pub fn point_at(&self, t: f32) -> Point {
        self.origin + self.dir * t
    }
}
