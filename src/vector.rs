use std::ops::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Vec2D
{
	pub x: f64,
	pub y: f64,
}

impl Vec2D {
	pub fn new_zero_vector() -> Vec2D {
        Vec2D {
            x: 0.0,
            y: 0.0,
        }
    }

    pub fn new_ones_vector() -> Vec2D {
        Vec2D {
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

    pub fn unit_vector(&self) -> Vec2D {
        self / self.length()
    }

    pub fn dot(v1: &Vec2D, v2: &Vec2D) -> f64 {
        v1.x * v2.x + v1.y * v2.y
    }

    pub fn cross(v1: &Vec2D, v2: &Vec2D) -> f64 {
        v1.x * v2.y - v2.x * v1.y
    }
}

impl Add for Vec2D {
    type Output = Vec2D;

    fn add(self, other: Vec2D) -> Vec2D {
        Vec2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<'a> Add<&'a Vec2D> for &'a Vec2D {
    type Output = Vec2D;

    fn add(self, other: &Vec2D) -> Vec2D {
        Vec2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<'a> Div<f64> for &'a Vec2D {
    type Output = Vec2D;

    fn div(self, div: f64) -> Vec2D {
        Vec2D {
            x: self.x / div,
            y: self.y / div,
        }
    }
}


#[derive(Debug, PartialEq, Clone)]
pub struct Vec3D
{
	pub x: f64,
	pub y: f64,
	pub z: f64,
}

impl Vec3D {
	pub fn new_zero_vector() -> Vec3D {
        Vec3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn new_ones_vector() -> Vec3D {
        Vec3D {
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

    pub fn unit_vector(&self) -> Vec3D {
        self / self.length()
    }

    pub fn dot(v1: &Vec3D, v2: &Vec3D) -> f64 {
        v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
    }

    pub fn cross(v1: &Vec3D, v2: &Vec3D) -> Vec3D {
        Vec3D {
            x: v1.y * v2.z - v1.z * v2.y,
            y: -(v1.x * v2.z - v1.z * v2.x),
            z: v1.x * v2.y - v1.y * v2.x,
        }
    }
}

impl Add for Vec3D {
    type Output = Vec3D;

    fn add(self, other: Vec3D) -> Vec3D {
        Vec3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<'a> Add<&'a Vec3D> for &'a Vec3D {
    type Output = Vec3D;

    fn add(self, other: &Vec3D) -> Vec3D {
        Vec3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<'a> Div<f64> for &'a Vec3D {
    type Output = Vec3D;

    fn div(self, div: f64) -> Vec3D {
        Vec3D {
            x: self.x / div,
            y: self.y / div,
            z: self.z / div,
        }
    }
}

impl<'a> Mul<&'a Vec3D> for f64 {
    type Output = Vec3D;

    fn mul(self, vector: &Vec3D) -> Vec3D {
        Vec3D {
            x: self * vector.x,
            y: self * vector.y,
            z: self * vector.z,
        }
    }
}

impl<'a> Neg for &'a Vec3D {
    type Output = Vec3D;

    fn neg(self) -> Vec3D {
        Vec3D {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<'a, 'b> Sub<&'a Vec3D> for &'b Vec3D {
    type Output = Vec3D;

    fn sub(self, other: &Vec3D) -> Vec3D {
        Vec3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

#[cfg(test)]
mod test{
	use super::Vec2D;
	use super::Vec3D;

	#[test]
	fn test_vec2D_add()
	{
		let mut v1 = Vec2D{ x: 1.0, y: 0.0 };
	    v1 = v1 + Vec2D{ x: 2.0, y: 3.0 };
	    format!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!{:?} {:?}", v1.x, v1.y);
	    // assert_eq!(v1, Vec2D{ x: 3, y: 3 });
	}

	#[test]
	fn test_vec3D_add()
	{
		let mut v1 = Vec3D{ x: 1.0, y: 0.0, z: 0.0 };
	    v1 = v1 + Vec3D{ x: 2.0, y: 3.0, z: 1.0 };
	    format!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!{:?} {:?} {:?}", v1.x, v1.y, v1.z);
	    // assert_eq!(v1, Vec2D{ x: 3, y: 3 });
	}
    
}