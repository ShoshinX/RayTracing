use crate::vec3::color;
use crate::rtweekend::*;

pub fn write_color(pixel_color: color, samples_per_pixel: i32) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    // Divide the color by the number of samples.
    let scale = 1.0 / samples_per_pixel as f64;
    r *= scale;
    g *= scale;
    b *= scale;
    // Why use 255.999?
    print!("{} {} {}\n", 256.0 * clamp(r, 0.0, 0.999), 256.0 * clamp(g, 0.0, 0.999), 256.0 * clamp(b, 0.0,0.999));
}
