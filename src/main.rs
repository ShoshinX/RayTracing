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

fn random_scene() -> hittable_list::hittable_list {
    let mut world = hittable_list::hittable_list {objects: Vec::new()};

    let ground_material = std::rc::Rc::new(material::lambertian{albedo:vec3::color{e:[0.5,0.5,0.5]}});
    world.add(Box::new(sphere::sphere{center: vec3::point3{e:[0.0,-1000.0,0.0]}, radius:1000.0, mat_ptr: ground_material.clone()}));

    let mut a = -11.0;
    while a < 11.0 {
        let mut b = -11.0;
        while b < 11.0 {
            let choose_mat = rand::random::<f64>();
            let center = vec3::point3{e:[a+0.9* rand::random::<f64>(), 0.2, b+0.9 * rand::random::<f64>()]};

            if (center - vec3::point3{e:[4.0,0.2,0.0]}).length() > 0.9 {
                let mut sphere_material : std::rc::Rc<material::material> = std::rc::Rc::new(material::dielectric{ir:1.5});
                if choose_mat < 0.8 {
                    let albedo = color::random_color() * color::random_color();
                    sphere_material = std::rc::Rc::new(material::lambertian{albedo:albedo});
                    world.add(Box::new(sphere::sphere{center: center, radius:0.2, mat_ptr: sphere_material.clone()}));
                } else if choose_mat < 0.95 {
                    let albedo = color::random_color_2(0.5,1.0);
                    let fuzz = rtweekend::random_double(0.0,0.5);
                    sphere_material = std::rc::Rc::new(material::metal{albedo:albedo, fuzz: fuzz});
                    world.add(Box::new(sphere::sphere{center: center, radius:0.2, mat_ptr: sphere_material.clone()}));
                } else {
                    sphere_material = std::rc::Rc::new(material::dielectric{ir:1.5});
                    world.add(Box::new(sphere::sphere{center: center, radius:0.2, mat_ptr: sphere_material.clone()}));
                }
            }
            b+=1.0;
        }
        a+= 1.0;
    }
    let material1 = std::rc::Rc::new(material::dielectric{ir:1.5});
    world.add(Box::new(sphere::sphere{center: vec3::point3{e:[0.0,1.0,0.0]}, radius:1.0, mat_ptr: material1.clone()}));
    let material2 = std::rc::Rc::new(material::lambertian{albedo:vec3::color{e:[0.4,0.2,0.1]}});
    world.add(Box::new(sphere::sphere{center: vec3::point3{e:[-4.0,1.0,0.0]}, radius:1.0, mat_ptr: material2.clone()}));
    let material3 = std::rc::Rc::new(material::metal{albedo:vec3::color{e:[0.7,0.6,0.5]}, fuzz: 0.0});
    world.add(Box::new(sphere::sphere{center: vec3::point3{e:[4.0,1.0,0.0]}, radius:1.0, mat_ptr: material3.clone()}));

    world
}

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
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: i32 = 1200;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL : i32 = 500;
    const MAX_DEPTH: i32 = 50;

    // World
    let mut world = random_scene();

    // Camera
    let lookfrom = vec3::point3{e:[13.0,2.0,3.0]};
    let lookat = vec3::point3{e:[0.0,0.0,0.0]};
    let vup = vec3::vec3{e:[0.0,1.0,0.0]};
    let dist_to_focus = 10.0; 
    let aperture = 0.1;
    let cam = camera::camera::new(lookfrom,lookat,vup,20.0, ASPECT_RATIO, aperture, dist_to_focus); 

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
