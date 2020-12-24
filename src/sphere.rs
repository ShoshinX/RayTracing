
use crate::vec3::*;
use crate::hittable::*;
use crate::ray::*;

pub struct sphere {
    pub center: point3,
    pub radius: f64,
}

impl hittable for sphere {
    fn hit(&self,r: &ray, t_min:f64, t_max:f64, rec: &mut hit_record) -> bool{
        let oc = r.origin() - self.center;
        let a  = r.direction().length_squared();
        let half_b = dot_prod_vec3(oc, r.direction());
        let c  = oc.length_squared() - self.radius*self.radius;
        let discriminant = half_b*half_b - a*c;
        if discriminant < 0.0 { return false }; 
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root  {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root { return false }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        rec.normal = (rec.p - self.center) / self.radius;
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        true
    }
}
