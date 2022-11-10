use crate::{hittable::{Hittable, HitRecord}, ray::Ray};

pub struct HittableList {
    pub ls: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new(ls: Vec<Box<dyn Hittable>>) -> Self { HittableList { ls }}
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
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
