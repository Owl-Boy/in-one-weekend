#![allow(dead_code)]

use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg};
use rand::{prelude::*, distributions::{Distribution, Uniform}};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 {x, y, z}
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let x: f32 = rng.gen();
        let y: f32 = rng.gen();
        let z: f32 = rng.gen();
        Vec3 { x, y, z }
    }
    
    pub fn rand_range(min: f32, max: f32) -> Self {
        let between = Uniform::new(min, max);
        let mut rng = rand::thread_rng();
        let x: f32 = between.sample(&mut rng);
        let y: f32 = between.sample(&mut rng);
        let z: f32 = between.sample(&mut rng);
        Vec3 { x, y, z }
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Vec3::rand_range(-1.0, 1.0);
            if p.len_squared() < 1.0 { return p; }
        }
    }

    pub fn random_in_hemisphere(normal: &Vec3) -> Self {
        let v = Self::random_in_unit_sphere();
        if Self::dot(v, *normal) > 0.0 {
            v
        } else {
            -v
        }
    }

    pub fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().unit_along()
    }

    pub fn dot(v1: Vec3, v2: Vec3) -> f32 {
        v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
    }

    pub fn cross(v1: Vec3, v2: Vec3) -> Vec3 {
        Vec3 {
            x: v1.y * v2.z - v1.z * v2.y,
            y: v1.z * v2.x - v1.x * v2.z,
            z: v1.x * v2.y - v1.y * v2.x,
        }
    }

    pub fn normalize(&mut self) {
        *self /= self.len();
    }
    
    pub fn unit_along(&self) -> Self {
        *self / self.len()
    }

    pub fn len_squared(self) -> f32 {
        Self::dot(self, self)
    }

    pub fn len(self) -> f32 {
        self.len_squared().sqrt()
    } 

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }

    pub fn reflect(v: Vec3, n: Vec3) -> Self {
        v - n*2.0*Self::dot(v, n)
    }

    pub fn refract(v: Vec3, n: Vec3, index: f32) -> Option<Self> {
        let uv = v.unit_along();
        let dt = Vec3::dot(uv, n);
        let disc = 1.0 - index.powi(2) * (1.0 - dt.powi(2));
        if disc > 0.0 {
            let refracted = (uv - n * dt) * index - n * disc.sqrt();
            Some(refracted)
        } else {
            None
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Vec3) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        self * -1.0
    }
}

pub type Point = Vec3;
pub type Color = Vec3;
