use crate::{ray::Ray, hittable::HitRecord, vec3::{Color, Vec3}};
use rand::{thread_rng, prelude::*};

pub trait Material {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<(Ray, Color)> { None }
}

#[derive(Debug)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.p, scatter_direction);
        Some((scattered, self.albedo))
    } 
}

#[derive(Debug)]
pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzziness: f32) -> Self {
        let fuzz = if fuzziness < 1.0 { fuzziness } else { 1.0 };
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = Vec3::reflect(r_in.dir.unit_along(), rec.normal);
        let scattered = Ray::new(rec.p, reflected + Vec3::random_in_unit_sphere() * self.fuzz);
        if Vec3::dot(scattered.dir, rec.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Dielectric {
    pub ref_idx: f32,
}

impl Dielectric {
    pub fn new(ref_idx: f32) -> Self {
        Dielectric { ref_idx }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let (outward_normal, ni_by_nt) = if Vec3::dot(r_in.dir, rec.normal) > 0.0 {
            (-rec.normal, self.ref_idx)
        } else {
            (rec.normal, 1.0 / self.ref_idx)
        };
    
        let cos_theta = f32::min(Vec3::dot(-r_in.dir.unit_along(), outward_normal), 1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();
        let cannot_refract = ni_by_nt * sin_theta > 1.0;
        let mut rng = thread_rng();
        let val: f32 = rng.gen();

        let direction = if cannot_refract || reflectance(cos_theta, ni_by_nt) > val {
            Vec3::reflect(r_in.dir.unit_along(), outward_normal)
        } else {
            match Vec3::refract(r_in.dir.unit_along(), outward_normal, ni_by_nt) {
                Some (ray) => ray,
                None => Vec3::new(0.0, 0.0, 0.0)
            }
        };

        let scattered = Ray::new(rec.p, direction);
        Some((scattered, attenuation))
    }
}

fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
