use crate::vec3::Color;

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

pub fn write_color(pixel_color: Color, sample_per_pixel: u16) {
    let r = pixel_color.x;
    let g = pixel_color.y;
    let b = pixel_color.z;

    let scale = 1.0 / (sample_per_pixel as f64);

    let r = (scale * r).sqrt();
    let g = (scale * g).sqrt();
    let b = (scale * b).sqrt();

    let ir = (256 as f64 * clamp(r, 0.0, 0.999)) as u16;
    let ig = (256 as f64 * clamp(g, 0.0, 0.999)) as u16;
    let ib = (256 as f64 * clamp(b, 0.0, 0.999)) as u16;

    println!("{ir} {ig} {ib}");
}
