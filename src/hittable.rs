
use crate::vec3::*;
use crate::ray::*;
use crate::material::*;
use std::rc::Rc;

#[derive(Clone)]
pub struct hit_record {
    pub p: point3,
    pub normal: vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat_ptr: Rc<material>,
}

// TODO: have a default() and new() for initialization
impl hit_record {
    pub fn set_face_normal(&mut self, r: &ray, outward_normal: &vec3) {
        self.front_face = dot_prod_vec3(r.direction(), *outward_normal) < 0.0;
        self.normal = if self.front_face { *outward_normal } else {-*outward_normal};
    }
}

pub trait hittable {
    fn hit(&self,r: &ray, t_min:f64, t_max:f64, rec: &mut hit_record) -> bool;
}
