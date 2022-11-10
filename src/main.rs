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
use indicatif::ProgressBar;

// Image
const ASPECT_RATIO: f64 = 1.5;
const IMAGE_HEIGHT: u16 = 1200;
const IMAGE_WIDTH: u16 = (IMAGE_HEIGHT as f64 * ASPECT_RATIO) as u16;
const SAMPLES_PER_PIXEL: u16 = 500;
const MAX_DEPTH: u16 = 50;

fn ray_color<T: Hittable>(r: &Ray, world: &T, depth: u16) -> Color {
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
    let mut rng = thread_rng();

    // World
    let world = random_scene();

    // Camera
    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let focus_dist = 10.0;
    let aperture = 0.1;
    let cam = Camera::new(look_from, look_at, Vec3::new(0.0, 1.0, 0.0), 20.0, ASPECT_RATIO, aperture, focus_dist);

    // Render
    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255");

    let pb = ProgressBar::new(IMAGE_HEIGHT as u64);
    for j in (0..IMAGE_HEIGHT).rev() {
        pb.inc(1);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let x: f64 = rng.gen();
                let y: f64 = rng.gen();
                let u = (i as f64 + x)/ ((IMAGE_WIDTH - 1) as f64);
                let v = (j as f64 + y)/ ((IMAGE_HEIGHT - 1) as f64);
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }
            write_color(pixel_color, SAMPLES_PER_PIXEL);
        }
    }
    pb.finish_with_message("Done");
}
