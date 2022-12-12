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
use vec3::{Color, Vec3, Point};
use color::write_color;
use hittable_list::HittableList;
use sphere::Sphere;
use camera::Camera;
use material::{Lambertian, Metal, Dielectric};

use rand::prelude::*;
use std::sync::Arc;
use std::thread;
use std::sync::mpsc;
use indicatif::ProgressBar;

// Image
const ASPECT_RATIO: f64 = 1.5;
const IMAGE_HEIGHT: u32 = 1200;
const IMAGE_WIDTH: u32 = (IMAGE_HEIGHT as f64 * ASPECT_RATIO) as u32;
const SAMPLES_PER_PIXEL: u32 = 500;
const MAX_DEPTH: u32 = 50;

fn ray_color<T: Hittable>(r: &Ray, world: &T, depth: u32) -> Color {
    if depth == 0 { return Vec3::new(0.0, 0.0, 0.0); };

    match world.hit(r, 0.00001, f64::INFINITY) {
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

fn random_scene() -> HittableList {
    let mut rng = thread_rng();
    let ground_material = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    let mut world = HittableList::new(vec![Box::new(Sphere::new(Point::new(0.0, -1000.0, 0.0), 1000.0, ground_material))]);

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen();
            let center = Point::new(a as f64 + 0.9 * rng.gen::<f64>(), 0.2, b  as f64+ 0.9 * rng.gen::<f64>());

            if (center - Point::new(4.0, 0.2, 0.0)).len() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Lambertian::new(albedo);
                    world.ls.push(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    let albedo = Color::rand_range(0.5, 1.0);
                    let fuzz = rng.gen::<f64>() * 0.5;
                    let sphere_material = Metal::new(albedo, fuzz);
                    world.ls.push(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    let sphere_material = Dielectric::new(1.5);
                    world.ls.push(Box::new(Sphere::new(center, 0.2, sphere_material)));
                };
            }
        }
    }

    let material1 = Dielectric::new(1.5);
    world.ls.push(Box::new(Sphere::new(Point::new(0.0, 1.0, 0.0), 1.0, material1)));

    let material2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world.ls.push(Box::new(Sphere::new(Point::new(-4.0, 1.0, 0.0), 1.0, material2)));

    let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.ls.push(Box::new(Sphere::new(Point::new(4.0, 1.0, 0.0), 1.0, material3)));

    world
}

fn main() {
    // World
    let world = random_scene();
    let aworld = Arc::new(world);

    // Camera
    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let focus_dist = 10.0;
    let aperture = 0.1;
    let cam = Arc::new(Camera::new(look_from, look_at, Vec3::new(0.0, 1.0, 0.0), 20.0, ASPECT_RATIO, aperture, focus_dist));

    // Render
    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255");
    let pb = Arc::new(ProgressBar::new((IMAGE_HEIGHT * IMAGE_WIDTH) as u64));

    let mut recievers = Vec::with_capacity(IMAGE_HEIGHT as usize);
    for j in (0..IMAGE_HEIGHT).rev() {
        let mut row = Vec::with_capacity(IMAGE_WIDTH as usize);
        for i in 0..IMAGE_WIDTH {
            let (tx, rx) = mpsc::channel();
            let cam1 = cam.clone();
            let world1 = aworld.clone();
            let indic1 = pb.clone();
            thread::spawn(move || {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..SAMPLES_PER_PIXEL {
                    let mut rng = thread_rng();
                    let x: f64 = rng.gen();
                    let y: f64 = rng.gen();
                    let u = (i as f64 + x)/ ((IMAGE_WIDTH - 1) as f64);
                    let v = (j as f64 + y)/ ((IMAGE_HEIGHT - 1) as f64);
                    let r = cam1.get_ray(u, v);
                    pixel_color += ray_color(&r, &world1, MAX_DEPTH);
                }
                tx.send(pixel_color).unwrap();
                indic1.inc(1);
            });
            row.push(rx);
        }
        recievers.push(row);
    }

    for row in recievers {
        for reciever in row {
            let pixel_color = reciever.recv().unwrap();
            write_color(pixel_color, SAMPLES_PER_PIXEL as u64);
        }
    }
    pb.finish_with_message("Done!");
}
