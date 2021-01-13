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

pub struct dielectric {
    pub ir: f64, // Index of refraction
}

impl material for dielectric {
    fn scatter(&self,r_in: &ray, rec: &hit_record, attenuation: &mut color, scattered: &mut ray) -> bool{
        *attenuation = color {e:[1.0,1.0,1.0]};
        let refraction_ratio = if rec.front_face { (1.0/self.ir) } else { self.ir };
        let unit_direction = unit_vector_vec3(r_in.direction());
        let cos_theta = dot_prod_vec3(-unit_direction, rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let mut direction = vec3 {e:[0.0,0.0,0.0]};

        if cannot_refract {
            direction = reflect(&unit_direction, &rec.normal);
        } else {
            direction = refract(&unit_direction, &rec.normal, refraction_ratio);
        }
        *scattered = ray{orig: rec.p, dir: direction};
        true
    }
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0-ref_idx) / (1.0+ref_idx);
    r0 = r0*r0;
    r0 + (1.0 - r0) * (1.0-cosine).powi(5)
}
