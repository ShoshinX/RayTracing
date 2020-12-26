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
mod camera;

fn ray_color(r: &ray::ray, world: &hittable::hittable, depth: i32) -> vec3::color{
    let mut rec = hittable::hit_record {p: vec3::point3{e:[0.0,0.0,0.0]}, normal:vec3::vec3{e:[0.0,0.0,0.0]}, t:0.0, front_face:false};
    if depth <= 0 { return vec3::color{e:[0.0,0.0,0.0]} }
    if world.hit(r,0.001,rtweekend::INFINITY, &mut rec) {
        let target = rec.p + rec.normal + vec3::random_in_hemisphere(&rec.normal);
        return 0.5 * ray_color(&ray::ray{orig:rec.p, dir: target - rec.p}, world, depth - 1)
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
    const SAMPLES_PER_PIXEL : i32 = 100;
    const MAX_DEPTH: i32 = 50;

    // World
    let mut world = hittable_list::hittable_list {objects: Vec::new()};
    world.add(Box::new(sphere::sphere{center: vec3::point3 {e:[0.0,0.0,-1.0]}, radius: 0.5}));
    world.add(Box::new(sphere::sphere{center: vec3::point3 {e:[0.0,-100.5,-1.0]}, radius: 100.0}));

    // Camera
    let cam = camera::camera::new(); 

    // Render
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("\rScanlines remaining: {} ", j);
        io::stdout().flush().unwrap();
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = vec3::color {e:[0.0,0.0,0.0]};
            for s in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + rand::random::<f64>()) / (IMAGE_WIDTH-1) as f64;
                let v = (j as f64 + rand::random::<f64>()) / (IMAGE_HEIGHT-1) as f64;
                let r = cam.get_ray(u,v);
                pixel_color += ray_color(&r,&world, MAX_DEPTH);
            }
            color::write_color(pixel_color, SAMPLES_PER_PIXEL);
        }
    }
    eprintln!("\nDone.\n");
}
