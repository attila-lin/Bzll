use std::ops::*;
use std::fmt::Debug;
// extern crate num;
// use num::{Zero, NumCast};

#[derive(Debug, PartialEq, Clone)]
pub struct Point2D
{
	pub x: f64,
	pub y: f64,
}

impl Point2D {
	pub fn new_zero_Point2D() -> Point2D {
        Point2D {
            x: 0.0,
            y: 0.0,
        }
    }

    pub fn new_ones_Point2D() -> Point2D {
        Point2D {
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

    pub fn dot(v1: &Point2D, v2: &Point2D) -> f64 {
        v1.x * v2.x + v1.y * v2.y
    }

    pub fn cross(v1: &Point2D, v2: &Point2D) -> f64 {
        v1.x * v2.y - v2.x * v1.y
    }
}

impl Add for Point2D {
    type Output = Point2D;

    fn add(self, other: Point2D) -> Point2D {
        Point2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<'a> Add<&'a Point2D> for &'a Point2D {
    type Output = Point2D;

    fn add(self, other: &Point2D) -> Point2D {
        Point2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

pub struct Point3D
{
	pub x: f64,
	pub y: f64,
	pub z: f64,
}

impl Point3D {
	pub fn new_zero_Point3D() -> Point3D {
        Point3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn new_ones_Point3D() -> Point3D {
        Point3D {
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

    pub fn dot(v1: &Point3D, v2: &Point3D) -> f64 {
        v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
    }

    pub fn cross(v1: &Point3D, v2: &Point3D) -> Point3D {
        Point3D {
            x: v1.y * v2.z - v1.z * v2.y,
            y: -(v1.x * v2.z - v1.z * v2.x),
            z: v1.x * v2.y - v1.y * v2.x,
        }
    }
}

impl Add for Point3D {
    type Output = Point3D;

    fn add(self, other: Point3D) -> Point3D {
        Point3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<'a> Add<&'a Point3D> for &'a Point3D {
    type Output = Point3D;

    fn add(self, other: &Point3D) -> Point3D {
        Point3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<'a> Div<f64> for &'a Point3D {
    type Output = Point3D;

    fn div(self, div: f64) -> Point3D {
        Point3D {
            x: self.x / div,
            y: self.y / div,
            z: self.z / div,
        }
    }
}

impl<'a> Mul<&'a Point3D> for f64 {
    type Output = Point3D;

    fn mul(self, vector: &Point3D) -> Point3D {
        Point3D {
            x: self * vector.x,
            y: self * vector.y,
            z: self * vector.z,
        }
    }
}

impl<'a> Neg for &'a Point3D {
    type Output = Point3D;

    fn neg(self) -> Point3D {
        Point3D {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<'a, 'b> Sub<&'a Point3D> for &'b Point3D {
    type Output = Point3D;

    fn sub(self, other: &Point3D) -> Point3D {
        Point3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

#[cfg(test)]
mod test{
	use super::Point2D;
	use super::Point3D;

	#[test]
	fn test_Point2D_add()
	{
		let mut v1 = Point2D{ x: 1.0, y: 0.0 };
	    v1 = v1 + Point2D{ x: 2.0, y: 3.0 };
	    format!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!{:?} {:?}", v1.x, v1.y);
	    // assert_eq!(v1, Point2D{ x: 3, y: 3 });
	}

	#[test]
	fn test_Point3D_add()
	{
		let mut v1 = Point3D{ x: 1.0, y: 0.0, z: 0.0 };
	    v1 = v1 + Point3D{ x: 2.0, y: 3.0, z: 1.0 };
	    format!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!{:?} {:?} {:?}", v1.x, v1.y, v1.z);
	    // assert_eq!(v1, Point2D{ x: 3, y: 3 });
	}
    
}