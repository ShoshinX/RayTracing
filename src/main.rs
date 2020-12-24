// Question:What does this mean?
use std::io::{self,Write};
use std::ops;
mod vec3;
mod color;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod rtweekend;

fn ray_color(r: &ray::ray, world: &hittable::hittable) -> vec3::color{
    let mut rec = hittable::hit_record {p: vec3::point3{e:[0.0,0.0,0.0]}, normal:vec3::vec3{e:[0.0,0.0,0.0]}, t:0.0, front_face:false};
    if world.hit(r,0.0,rtweekend::INFINITY, &mut rec) {
        return 0.5 * (rec.normal + vec3::color {e:[1.0,1.0,1.0]})
    }
    let unit_direction = vec3::unit_vector_vec3(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * vec3::color{e:[1.0,1.0,1.0]} + t*vec3::color{e:[0.5, 0.7, 1.0]}
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;

    // World
    let mut world = hittable_list::hittable_list {objects: Vec::new()};
    world.add(Box::new(sphere::sphere{center: vec3::point3 {e:[0.0,0.0,-1.0]}, radius: 0.5}));
    world.add(Box::new(sphere::sphere{center: vec3::point3 {e:[0.0,-100.5,-1.0]}, radius: 100.0}));

    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = vec3::point3 {e:[0.0,0.0,0.0]};
    let horizontal = vec3::vec3 {e:[viewport_width, 0.0, 0.0]};
    let vertical = vec3::vec3 {e:[0.0, viewport_height, 0.0]};
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - vec3::vec3{e:[0.0,0.0, focal_length]};

    // Render
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("\rScanlines remaining: {} ", j);
        io::stdout().flush().unwrap();
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH-1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT-1) as f64;
            let r = ray::ray {orig: origin, dir: lower_left_corner + u * horizontal + v * vertical};
            let pixel_color = ray_color(&r, &world);
            color::write_color(pixel_color);
        }
    }
    eprintln!("\nDone.\n");
}
