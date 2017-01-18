use std::ops::*;
use std::fmt::Debug;
// extern crate num;
// use num::{Zero, NumCast};

#[derive(Debug, PartialEq, Clone)]
pub struct Vec2
{
	pub x: f64,
	pub y: f64,
}

impl Vec2 {
	pub fn new_zero_vec2() -> Vec2 {
        Vec2 {
            x: 0.0,
            y: 0.0,
        }
    }

    pub fn new_ones_vec2() -> Vec2 {
        Vec2 {
            x: 1.0,
            y: 1.0,
        }
    }

    pub fn length(&self) -> f64 {
    	(self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn length_pow(&self) -> f64 {
    	self.x * self.x + self.y * self.y
    }

    pub fn dot(v1: &Vec2, v2: &Vec2) -> f64 {
        v1.x * v2.x + v1.y * v2.y
    }

    pub fn cross(v1: &Vec2, v2: &Vec2) -> f64 {
        v1.x * v2.y - v2.x * v1.y
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<'a> Add<&'a Vec2> for &'a Vec2 {
    type Output = Vec2;

    fn add(self, other: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

pub struct Vec3
{
	pub x: f64,
	pub y: f64,
	pub z: f64,
}

impl Vec3 {
	pub fn new_zero_vec3() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn new_ones_vec3() -> Vec3 {
        Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }

    pub fn length(&self) -> f64 {
    	(self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn length_pow(&self) -> f64 {
    	self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(v1: &Vec3, v2: &Vec3) -> f64 {
        v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
    }

    pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
        Vec3 {
            x: v1.y * v2.z - v1.z * v2.y,
            y: -(v1.x * v2.z - v1.z * v2.x),
            z: v1.x * v2.y - v1.y * v2.x,
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<'a> Add<&'a Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn add(self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<'a> Div<f64> for &'a Vec3 {
    type Output = Vec3;

    fn div(self, div: f64) -> Vec3 {
        Vec3 {
            x: self.x / div,
            y: self.y / div,
            z: self.z / div,
        }
    }
}

impl<'a> Mul<&'a Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, vector: &Vec3) -> Vec3 {
        Vec3 {
            x: self * vector.x,
            y: self * vector.y,
            z: self * vector.z,
        }
    }
}

impl<'a> Neg for &'a Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<'a, 'b> Sub<&'a Vec3> for &'b Vec3 {
    type Output = Vec3;

    fn sub(self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

#[cfg(test)]
mod test{
	use super::Vec2;
	use super::Vec3;

	#[test]
	fn test_vec2_add()
	{
		let mut v1 = Vec2{ x: 1.0, y: 0.0 };
	    v1 = v1 + Vec2{ x: 2.0, y: 3.0 };
	    format!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!{:?} {:?}", v1.x, v1.y);
	    // assert_eq!(v1, Vec2{ x: 3, y: 3 });
	}

	#[test]
	fn test_vec3_add()
	{
		let mut v1 = Vec3{ x: 1.0, y: 0.0, z: 0.0 };
	    v1 = v1 + Vec3{ x: 2.0, y: 3.0, z: 1.0 };
	    format!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!{:?} {:?} {:?}", v1.x, v1.y, v1.z);
	    // assert_eq!(v1, Vec2{ x: 3, y: 3 });
	}
    
}