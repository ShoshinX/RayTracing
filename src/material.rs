use crate::rtweekend::*;
use crate::hittable::*;
use crate::ray::*;
use crate::vec3::*;

pub trait material{
    fn scatter(&self,r_in: &ray, rec: &hit_record, attenuation: &mut color, scattered: &mut ray) -> bool;
}

pub struct lambertian {
    pub albedo: color,
}

impl material for lambertian {
    fn scatter(&self,r_in: &ray, rec: &hit_record, attenuation: &mut color, scattered: &mut ray) -> bool {
        let mut scatter_direction = rec.normal + random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }else {}
        *scattered = ray{orig:rec.p, dir:scatter_direction};
        *attenuation = self.albedo;
        true
    }
}

pub struct metal {
    pub albedo: color,
    pub fuzz: f64,
}

impl material for metal {
    fn scatter(&self,r_in: &ray, rec: &hit_record, attenuation: &mut color, scattered: &mut ray) -> bool {
        let reflected = reflect(&unit_vector_vec3(r_in.direction()), &rec.normal);
        *scattered = ray{orig:rec.p, dir:reflected + self.fuzz*random_in_unit_sphere()};
        *attenuation = self.albedo;
        dot_prod_vec3(scattered.direction(),rec.normal) > 0.0
    }
}
