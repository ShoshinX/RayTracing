use crate::vec3::color;

pub fn write_color(pixel_color: color) {
    // Why use 255.999?
    print!("{} {} {}\n", 255.999 * pixel_color.x(), 255.999 * pixel_color.y(), 255.999 * pixel_color.z());
}
