use crate::vec3::Color;

pub fn write_color(pixel_color: Color) {
    let x = (pixel_color.x * 255.999) as u16;
    let y = (pixel_color.y * 255.999) as u16;
    let z = (pixel_color.z * 255.999) as u16;
    println!("{x} {y} {z}");
}
