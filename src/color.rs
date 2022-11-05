use crate::vec3::Color;

fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

pub fn write_color(pixel_color: Color, sample_per_pixel: u16) {
    let scale = 1.0 / (sample_per_pixel as f32);
    let new_color = pixel_color * scale;

    let r = new_color.x;
    let g = new_color.y;
    let b = new_color.z;

    let ir = (256 as f32 * clamp(r, 0.0, 0.999)) as u16;
    let ig = (256 as f32 * clamp(g, 0.0, 0.999)) as u16;
    let ib = (256 as f32 * clamp(b, 0.0, 0.999)) as u16;

    println!("{ir} {ig} {ib}");
}
