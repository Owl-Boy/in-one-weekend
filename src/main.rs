mod vec3;
mod color;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod camera;

use hittable::Hittable;
use ray::Ray;
use vec3::{Color, Point};
use color::write_color;
use hittable_list::HittableList;
use sphere::Sphere;

use rand::prelude::*;

// Image
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_HEIGHT: u16 = 400;
const IMAGE_WIDTH: u16 = (IMAGE_HEIGHT as f32 * ASPECT_RATIO) as u16;
const SAMPLES_PER_PIXEL: u16 = 100;

// Camera

fn ray_color<T: Hittable>(r: &Ray, world: &T) -> Color {
    match world.hit(r, 0.0, f32::INFINITY) {
        None => {
            let unit_direction = r.dir.unit_along();
            let t = (unit_direction.y + 1.0) * 0.5;
            Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
        },
        Some(rec) => {
            (rec.normal + Color::new(1.0, 1.0, 1.0)) * 0.5
        }
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    // Camera
    let camera = crate::camera::Camera::new();

    // World
    let mut world = HittableList::new(Vec::new());
    world.add(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0));

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
                pixel_color += ray_color(&r, &world);
            }
            write_color(pixel_color, SAMPLES_PER_PIXEL);
        }
    }
}
