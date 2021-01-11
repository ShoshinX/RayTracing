// Question:What does this mean?
use std::io::{self,Write};
use std::ops;
use rand::*;
use crate::rtweekend::*;

// What does derive do? Why must Copy and Clone be together?
#[derive(Copy, Clone)]
pub struct vec3 {
    // [type, size]
    pub e: [f64;3],
}

impl vec3 {
    pub fn x(&self) -> f64 { self.e[0] }
    pub fn y(&self) -> f64 { self.e[1] }
    pub fn z(&self) -> f64 { self.e[2] }
    pub fn length(&self) -> f64 { self.length_squared().sqrt() }
    pub fn length_squared(&self) -> f64 {
            self[0]*self[0] + self[1]*self[1] + self[2]*self[2]
    }
    pub fn random() -> vec3 {
        vec3{e:[random::<f64>(),random::<f64>(),random::<f64>()]}
    }
    pub fn random_min_max(min:f64, max: f64) -> vec3 {
        vec3{e:[random_double(min,max), random_double(min, max), random_double(min,max)]}
    }
    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        (self.e[0].abs() < s) && (self.e[1].abs() < s) && (self.e[2].abs() < s)
    }
}

impl ops::Neg for vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        vec3 {e:[-self.e[0],-self.e[1],-self.e[2]]}
    }
}

impl ops::Index<usize> for vec3 {
    type Output = f64;
    // & here means reference to the object, so its a pass by reference
    fn index(&self, index:usize) -> &Self::Output {
        &self.e[index]
    }
}

impl ops::AddAssign for vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {e: [self[0] + other[0], self[1] + other[1], self[2] + other[2]]}
    }
}

impl ops::MulAssign<f64> for vec3 {
    fn mul_assign(&mut self, other: f64) {
        *self = Self {e: [self[0] * other, self[1] * other, self[2] * other]}
    }
}

impl ops::DivAssign<f64> for vec3 {
    fn div_assign(&mut self, other: f64) {
        *self *= 1.0/other;
    }
}


pub type point3 = vec3; // 3D point
pub type color  = vec3; // RGB color

pub fn print_vec3(v : &vec3) {
   print!("{} {} {}",v.e[0], v.e[1], v.e[2]);
}
impl ops::Add for vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        vec3 {e:[self.e[0] + other.e[0], self.e[1] + other.e[1], self.e[2] + other.e[2]] }
    }
}
impl ops::Sub for vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        vec3 {e:[self.e[0] - other.e[0], self.e[1] - other.e[1], self.e[2] - other.e[2]] }
    }
}
impl ops::Mul<vec3> for vec3 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        vec3 {e:[self.e[0] * other.e[0], self.e[1] * other.e[1], self.e[2] * other.e[2]] }
    }
}
impl ops::Mul<f64> for vec3 {
    type Output = Self;
    fn mul(self, other: f64) -> Self {
        vec3 {e:[self.e[0] * other, self.e[1] * other, self.e[2] * other]}
    }
}
impl ops::Mul<vec3> for f64 {
    type Output = vec3;
    fn mul(self, other: vec3) -> vec3 {
        other * self
    }
}
impl ops::Div<f64> for vec3 {
    type Output = Self;
    fn div(self, other: f64) -> Self {
        (1.0/other) * self
    }
}
pub fn dot_prod_vec3(u: vec3, v: vec3) -> f64 {
    u.e[0] * v.e[0] +
    u.e[1] * v.e[1] +
    u.e[2] * v.e[2]
}
pub fn cross_prod_vec3(u: vec3, v: vec3) -> vec3 {
    vec3 {e:[
        u.e[1] * v.e[2] - u.e[2] * v.e[1],
        u.e[2] * v.e[0] - u.e[0] * v.e[2],
        u.e[0] * v.e[1] - u.e[1] * v.e[0]
    ]}
}
pub fn unit_vector_vec3(v: vec3) -> vec3 {
    let len = v.length();
    v / len
}

pub fn random_in_unit_sphere() -> vec3 {
    let result = loop {
        let p = vec3::random_min_max(-1.0,1.0);
        if p.length_squared() >= 1.0 {continue;}
        break p;
    };
    result
}

pub fn random_unit_vector() -> vec3 {
    unit_vector_vec3(random_in_unit_sphere())
}

pub fn random_in_hemisphere(normal: &vec3) -> vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if dot_prod_vec3(in_unit_sphere, *normal) > 0.0 {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

pub fn reflect(v:&vec3, n: &vec3) -> vec3 {
    *v - 2.0*dot_prod_vec3(*v,*n)*(*n)
}
