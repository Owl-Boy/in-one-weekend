mod vec3;
mod color;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod camera;
mod material;

use hittable::Hittable;
use ray::Ray;
use vec3::{Color, Vec3};
use color::write_color;
use hittable_list::HittableList;
use sphere::Sphere;
use camera::Camera;

use rand::prelude::*;

use crate::material::{Lambertian, Metal, Dielectric};

// Image
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_HEIGHT: u16 = 400;
const IMAGE_WIDTH: u16 = (IMAGE_HEIGHT as f32 * ASPECT_RATIO) as u16;
const SAMPLES_PER_PIXEL: u16 = 100;
const MAX_DEPTH: u16 = 50;

// Camera

fn ray_color<T: Hittable>(r: &Ray, world: &T, depth: u16) -> Color {
    if depth == 0 { return Vec3::new(0.0, 0.0, 0.0); };

    match world.hit(r, 0.00001, f32::INFINITY) {
        None => {
            let unit_direction = r.dir.unit_along();
            let t = (unit_direction.y + 1.0) * 0.5;
            Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
        },
        Some(rec) => {
            match rec.mat.scatter(r, &rec) {
                Some ((scattered, attenuation)) => attenuation * ray_color(&scattered, world, depth-1),
                None => Color::new(0.0, 0.0, 0.0)
            }
        }
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    // Camera
    let camera = Camera::new();

    // World
    let ground_color = Color::new(0.8, 0.8, 0.0);
    let center_color = Color::new(0.1, 0.2, 0.5);
    // let left_color = Color::new(0.8, 0.8, 0.8);
    let right_color = Color::new(0.8, 0.6, 0.2);
    let material_ground = Lambertian::new(ground_color);
    let material_center = Lambertian::new(center_color);
    // let material_left = Metal::new(left_color, 0.1);
    // let material_center = Dielectric::new(1.5);
    let material_left_out = Dielectric::new(1.5);
    let material_left_in = Dielectric::new(1.5);
    let material_right = Metal::new(right_color, 1.0);
    let world = HittableList::new(vec![
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, material_center)),
        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, material_ground)),
        Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, material_right)),
        Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left_out)),
        Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.4, material_left_in)),
    ]);


    // Render
    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255");

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let x: f32 = rng.gen();
                let y: f32 = rng.gen();
                let u = (i as f32 + x)/ ((IMAGE_WIDTH - 1) as f32);
                let v = (j as f32 + y)/ ((IMAGE_HEIGHT - 1) as f32);
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }
            write_color(pixel_color, SAMPLES_PER_PIXEL);
        }
    }
}
