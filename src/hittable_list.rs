use crate::{hittable::{Hittable, HitRecord}, ray::Ray};
use std::sync::Arc;

pub struct HittableList {
    pub ls: Vec<Box<dyn Hittable + Send>>,
}

impl HittableList {
    pub fn new(ls: Vec<Box<dyn Hittable + Send>>) -> Self { HittableList { ls }}
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // Returns the Hit record for the closest hit
        let mut closest_so_far = t_max;
        let mut rec: Option<HitRecord> = None;
        for obj in self.ls.iter() {
            match obj.hit(r, t_min, closest_so_far) {
                None => {},
                Some(hit) => {
                    closest_so_far = hit.t;
                    rec = Some(hit);
                }
            };
        }
        rec
    }
}

impl Hittable for Arc<HittableList> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        HittableList::hit(&self, r, t_min, t_max)
    }
}
