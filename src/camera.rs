use crate::vec3::*;
use crate::ray::*;

pub struct camera {
    origin :point3,
    lower_left_corner :point3,
    horizontal :vec3,
    vertical : vec3,
}

impl camera {
    pub fn get_ray(&self,u: f64, v: f64) -> ray {
        ray {orig: self.origin, dir: self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin}
    }

    pub fn new() -> camera {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = point3 {e: [0.0,0.0,0.0]};
        let horizontal = point3 {e: [viewport_width,0.0,0.0]};
        let vertical = point3 {e: [0.0, viewport_height, 0.0]};
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - vec3 {e: [0.0,0.0,focal_length]};

        camera {
            origin: origin,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: lower_left_corner,
        }
    }
}
