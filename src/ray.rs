use std::ops::*;
use std::fmt::Debug;

use vector::Vec3D;

#[derive(Debug, PartialEq, Clone)]
pub struct Ray
{
	pub a: Vec3D,
	pub b: Vec3D,
}


impl Ray {
    pub fn new(a: &Vec3D, b: &Vec3D) -> Ray {
        Ray {
            a: a.clone(),
            b: b.clone(),
        }
    }

    pub fn origin(&self) -> Vec3D {
        self.a.clone()
    }

    pub fn direction(&self) -> Vec3D {
        self.b.clone()
    }

    pub fn point_at_parameter(&self, t: f64) -> Vec3D {
        &self.a + &(t * &self.b)
    }
}