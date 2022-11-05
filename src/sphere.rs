use crate::{vec3::{Point, Vec3}, hittable::{Hittable, HitRecord}, ray::Ray};

#[derive(Debug)]
pub struct Sphere {
    pub center: Point,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Point, radius: f32) -> Self {
        Sphere {center, radius}
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.dir.len_squared();
        let half_b = Vec3::dot(oc, r.dir);
        let c = oc.len_squared() - self.radius * self.radius;

        let discr = half_b*half_b - a*c;
        if discr >= 0.0 {
            let sqrtd = discr.sqrt();
            let mut t = (-half_b -sqrtd) / a;
            if t > t_max || t < t_min {
                 t = (-half_b + sqrtd) / a;
                 if t > t_max || t < t_min {
                     None
                 } else {
                     let p = r.point_at(t);
                     let normal = (p - self.center) / self.radius;
                     Some(HitRecord { t, p, normal})
                 }
            } else {
                 let p = r.point_at(t);
                 let normal = (p - self.center) / self.radius;
                 Some(HitRecord { t, p, normal})
            }
        } else {
            None
        }
    }
}
