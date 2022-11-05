use crate::{hittable::{Hittable, HitRecord}, ray::Ray};

pub struct HittableList<T: Hittable> {
    pub ls: Vec<T>,
}

impl<T: Hittable> HittableList<T> {
    pub fn new(ls: Vec<T>) -> Self {
        HittableList { ls }
    }

    pub fn clear(&mut self) {
        self.ls = Vec::new();
    }
    
    pub fn add(&mut self, obj: T) {
        self.ls.push(obj);
    }
}

impl<T: Hittable> Hittable for HittableList<T> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
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
