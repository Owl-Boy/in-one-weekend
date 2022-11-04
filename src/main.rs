mod vec3;
mod color;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;

use hittable::Hittable;
use ray::Ray;
use vec3::{Color, Point, Vec3};
use color::write_color;
use hittable_list::Hittable_List;

use crate::sphere::Sphere;


// Image
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_HEIGHT: u16 = 400;
const IMAGE_WIDTH: u16 = (IMAGE_HEIGHT as f32 * ASPECT_RATIO) as u16;

// Camera
const VIEWPORT_HEIGHT: f32 = 2.0;
const VIEWPORT_WIDTH: f32 = VIEWPORT_HEIGHT * ASPECT_RATIO;
const FOCAL_LENGTH: f32 = 1.0;

fn hit_sphere(center: Point, radius: f32, r: &Ray) -> f32 {
    let oc = r.origin - center;
    let a = r.dir.len_squared();
    let half_b = Vec3::dot(oc, r.dir);
    let c = oc.len_squared() - radius * radius;
    let discr = half_b*half_b - a*c;
    if discr >= 0.0 {
        (-half_b - discr.sqrt()) / a
    } else {
        -1.0
    }
}

// fn ray_color(r: &Ray) -> Color {
//     let root = hit_sphere(Point::new(0.0, 0.0, -1.0), 0.5, r);
//     if root > 0.0 {
//         let n = (r.point_at(root) - Vec3::new(0.0, 0.0, -1.0)).unit_along();
//         (n + Vec3::new(1.0, 1.0, 1.0)) * 0.5
//     } else {
        // let unit_direction = r.dir.unit_along();
        // let t = (unit_direction.y + 1.0) * 0.5;
        // Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
//     }
// }

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
    // Camera
    let origin = Point::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

    // World
    let mut world = Hittable_List::new(Vec::new());
    world.add(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0));

    // Render
    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255");

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let u = (i as f32)/ ((IMAGE_WIDTH - 1) as f32);
            let v = (j as f32)/ ((IMAGE_HEIGHT - 1) as f32);
            let r = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v - origin);
            let pixel_color = ray_color(&r, &world);
            write_color(pixel_color);
        }
    }
}
