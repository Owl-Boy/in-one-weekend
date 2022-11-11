use crate::{ray::Ray, vec3::{Point, Vec3}, material::Material};

pub struct HitRecord<'a> {
    pub p: Point,
    pub normal: Vec3,
    pub t: f64,
    pub mat: &'a dyn Material,
}

pub trait Hittable : Sync {
    fn hit(&self, _r: &Ray, _t_min: f64, _t_max: f64) -> Option<HitRecord> { None }
}
