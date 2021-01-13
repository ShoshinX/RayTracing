use crate::vec3::color;
use crate::rtweekend::*;
use rand::Rng;

pub fn write_color(pixel_color: color, samples_per_pixel: i32) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    // Divide the color by the number of samples.
    let scale = 1.0 / samples_per_pixel as f64;
    r = (r*scale).sqrt();
    g = (g*scale).sqrt();
    b = (b*scale).sqrt();
    // Why use 255.999?
    print!("{} {} {}\n", 256.0 * clamp(r, 0.0, 0.999), 256.0 * clamp(g, 0.0, 0.999), 256.0 * clamp(b, 0.0,0.999));
}

pub fn random_color() -> color {
    color {e:[rand::random::<f64>(),rand::random::<f64>(),rand::random::<f64>()]}
}

pub fn random_color_2(a:f64, b:f64) -> color {
    let mut rng = rand::thread_rng();
    color {e:[rng.gen_range(a..b),rng.gen_range(a..b),rng.gen_range(a..b)]}
}
