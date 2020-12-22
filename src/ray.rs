
use crate::vec3::point3;
use crate::vec3::vec3;

pub struct ray {
    pub orig: point3,
    pub dir: vec3
}

impl ray {
    pub fn origin(&self) -> point3 { self.orig }
    pub fn direction(&self) -> vec3 { self.dir }
    pub fn at(&self, t: f64) -> point3 {
        self.orig + t*self.dir
    }
}

