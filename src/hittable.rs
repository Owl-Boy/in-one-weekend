use crate::{ray::{self, Ray}, vec3::{Point, Vec3}};

#[derive(Debug)]
pub struct Hit_Record {
    pub p: Point,
    pub normal: Vec3,
    pub t: f32,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit_Record> { None }
}
