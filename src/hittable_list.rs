use crate::{hittable::{Hittable, Hit_Record}, ray::Ray};

pub struct Hittable_List<T: Hittable> {
    pub ls: Vec<T>,
}

impl<T: Hittable> Hittable_List<T> {
    pub fn new(ls: Vec<T>) -> Self {
        Hittable_List { ls }
    }

    pub fn clear(&mut self) {
        self.ls = Vec::new();
    }
    
    pub fn add(&mut self, obj: T) {
        self.ls.push(obj);
    }
}

impl<T: Hittable> Hittable for Hittable_List<T> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit_Record> {
        let mut closest_so_far = t_max;
        let mut rec: Option<Hit_Record> = None;
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
