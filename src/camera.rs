use crate::vec3::*;
use crate::ray::*;

pub struct camera {
    origin :point3,
    lower_left_corner :point3,
    horizontal :vec3,
    vertical : vec3,
    u: vec3,
    v: vec3,
    w: vec3,
    lens_radius: f64,
}

impl camera {
    pub fn get_ray(&self,s: f64, t: f64) -> ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
        ray {orig: self.origin + offset, dir: self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin - offset}
    }

    pub fn new(lookfrom: point3, lookat: point3, vup: vec3, vfov: f64, aspect_ratio: f64, aperture: f64, focus_dist: f64) -> camera {
        let theta = vfov.to_radians();
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        
        let w = unit_vector_vec3(lookfrom - lookat);
        let u = unit_vector_vec3(cross_prod_vec3(vup,w));
        let v = cross_prod_vec3(w,u);

        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - focus_dist * w;

        let lens_radius = aperture / 2.0;
        camera {
            origin: origin,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: lower_left_corner,
            u: u,
            v: v,
            w: w,
            lens_radius: lens_radius,
        }
    }
}
