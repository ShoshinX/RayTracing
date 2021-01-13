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
mod material;

fn ray_color(r: &ray::ray, world: &hittable::hittable, depth: i32) -> vec3::color{
    let mut rec = hittable::hit_record {p: vec3::point3{e:[0.0,0.0,0.0]}, normal:vec3::vec3{e:[0.0,0.0,0.0]}, t:0.0, front_face:false, mat_ptr: std::rc::Rc::new(material::lambertian{albedo:vec3::color{e:[0.0,0.0,0.0]}})};
    if depth <= 0 { return vec3::color{e:[0.0,0.0,0.0]} }
    if world.hit(r,0.001,rtweekend::INFINITY, &mut rec) {
        let mut scattered = ray::ray {orig:vec3::point3{e:[0.0,0.0,0.0]},dir:vec3::vec3{e:[0.0,0.0,0.0]}};
        let mut attenuation = vec3::color {e:[0.0,0.0,0.0]};
        if rec.mat_ptr.scatter(r, &rec, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        return vec3::color {e:[0.0,0.0,0.0]};
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
    let R = (rtweekend::PI/4.0).cos();
    let mut world = hittable_list::hittable_list {objects: Vec::new()};
    
    let material_ground = std::rc::Rc::new(material::lambertian{albedo:vec3::color{e:[0.8,0.8,0.0]}});
    let material_center = std::rc::Rc::new(material::lambertian{albedo:vec3::color{e:[0.1,0.2,0.5]}});
    let material_left = std::rc::Rc::new(material::dielectric{ir:1.5});
    let material_right = std::rc::Rc::new(material::metal{albedo:vec3::color{e:[0.8,0.6,0.2]}, fuzz: 0.0});

    world.add(Box::new(sphere::sphere{center: vec3::point3{e:[0.0,-100.5,-1.0]}, radius:100.0, mat_ptr: material_ground.clone()}));
    world.add(Box::new(sphere::sphere{center: vec3::point3{e:[0.0,0.0,-1.0]}, radius:0.5, mat_ptr: material_center.clone()}));
    world.add(Box::new(sphere::sphere{center: vec3::point3{e:[-1.0,0.0,-1.0]}, radius:0.5, mat_ptr: material_left.clone()}));
    world.add(Box::new(sphere::sphere{center: vec3::point3{e:[-1.0,0.0,-1.0]}, radius:-0.4, mat_ptr: material_left.clone()}));
    world.add(Box::new(sphere::sphere{center: vec3::point3{e:[1.0,0.0,-1.0]}, radius:0.5, mat_ptr: material_right.clone()}));

    // Camera
    //let cam = camera::camera::new(vec3::point3{e:[-2.0,2.0,1.0]},vec3::point3{e:[0.0,0.0,-1.0]},vec3::vec3{e:[0.0,1.0,0.0]},90.0, ASPECT_RATIO); 
    let cam = camera::camera::new(vec3::point3{e:[-2.0,2.0,1.0]},vec3::point3{e:[0.0,0.0,-1.0]},vec3::vec3{e:[0.0,1.0,0.0]},20.0, ASPECT_RATIO); 

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
