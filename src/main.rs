mod vec3;
mod color;
mod ray;

use ray::Ray;
use vec3::{Color, Point, Vec3};
use color::write_color;


// Image
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_HEIGHT: u16 = 400;
const IMAGE_WIDTH: u16 = (IMAGE_HEIGHT as f32 * ASPECT_RATIO) as u16;

// Camera
const VIEWPORT_HEIGHT: f32 = 2.0;
const VIEWPORT_WIDTH: f32 = VIEWPORT_HEIGHT * ASPECT_RATIO;
const FOCAL_LENGTH: f32 = 1.0;

fn hit_sphere(center: Point, radius: f32, r: &Ray) -> bool {
    let oc = r.origin - center;
    let a = Vec3::dot(r.dir, r.dir);
    let b = 2.0 * Vec3::dot(oc, r.dir);
    let c = Vec3::dot(oc, oc) - radius * radius;
    let discr = b*b - 4.0*a*c;
    discr > 0.0
}

fn ray_color(r: &Ray) -> Color {
    if hit_sphere(Point::new(0.0, 0.0, -1.0), 0.5, r) {
        Color::new(1.0, 0.0, 0.0)
    } else {
        let unit_direction = r.dir.unit_along();
        let t = (unit_direction.y + 1.0) * 0.5;
        Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
    }
}

fn main() {
    // Scene
    let origin = Point::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

    // Render
    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255");

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let u = (i as f32)/ ((IMAGE_WIDTH - 1) as f32);
            let v = (j as f32)/ ((IMAGE_HEIGHT - 1) as f32);
            let r = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v - origin);
            let pixel_color = ray_color(&r);
            write_color(pixel_color);
        }
    }
}
