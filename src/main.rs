mod vec3;
mod color;

use vec3::Color;
use color::write_color;

const IMAGE_WIDTH: u16 = 256;
const IMAGE_HEIGHT: u16 = 256;

fn main() {
    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255");

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let r = (i as f32)/((IMAGE_WIDTH - 1) as f32);
            let g = (j as f32)/((IMAGE_HEIGHT - 1) as f32);
            let b = 0.25;
            let pixel_color = Color::new(r, g, b);
            write_color(pixel_color);
        }
    }
}
