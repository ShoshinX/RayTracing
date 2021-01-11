use crate::hittable::*;
use crate::ray::*;
use crate::vec3::*;
use crate::material::*;
use std::rc::Rc;

pub struct hittable_list{
    // Box here is a smart pointer
    pub objects: Vec<Box<hittable>>,
}

impl hittable_list {
    pub fn clear(&mut self) { self.objects.clear() }
    pub fn add(&mut self, object: Box<hittable>) { self.objects.push(object) }
}

impl hittable for hittable_list {
    fn hit(&self,r: &ray, t_min:f64, t_max:f64, rec: &mut hit_record) -> bool{
        let mut temp_rec = hit_record {p: point3{e:[0.0,0.0,0.0]}, normal:vec3{e:[0.0,0.0,0.0]}, t:0.0, front_face:false, mat_ptr: Rc::new(lambertian{albedo:color{e:[0.0,0.0,0.0]}})};
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }
}
