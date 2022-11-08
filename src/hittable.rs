use crate::{ray::Ray, vec3::{Point, Vec3}, material::Material};

pub struct HitRecord<'a> {
    pub p: Point,
    pub normal: Vec3,
    pub t: f32,
    pub mat: &'a dyn Material,
}

pub trait Hittable {
    fn hit(&self, _r: &Ray, _t_min: f32, _t_max: f32) -> Option<HitRecord> { None }
}
