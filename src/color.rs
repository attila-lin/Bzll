use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;

use vector::Vec3D;

#[derive(Debug)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

fn near(a: f64, b: f64) -> bool {
    let delta = 0.000001;
    (a - b).abs() < delta
}

/// close enough
impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        near(self.r, other.r) && near(self.g, other.g) && near(self.b, other.b)
    }
}

impl Add<Color> for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl Div<f64> for Color {
    type Output = Color;

    fn div(self, other: f64) -> Color {
        Color {
            r: self.r / other,
            g: self.g / other,
            b: self.b / other,
        }
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, color: Color) -> Color {
        Color {
            r: self * color.r,
            g: self * color.g,
            b: self * color.b,
        }
    }
}

impl<'a> Mul<&'a Color> for &'a Vec3D {
    type Output = Color;

    fn mul(self, color: &Color) -> Color {
        Color {
            r: self.x * color.r,
            g: self.y * color.g,
            b: self.z * color.b,
        }
    }
}

impl Color {
    pub fn gamma_correct(&self) -> Color {
        Color {
            r: self.r.sqrt(),
            g: self.g.sqrt(),
            b: self.b.sqrt(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Color;
    use vector::Vec3D;

    #[test]
    fn test_add_color() {
        let col1 = Color {
            r: 0.1,
            g: 0.2,
            b: 0.3,
        };
        let col2 = Color {
            r: 0.4,
            g: 0.5,
            b: 0.6,
        };
        assert_eq!(Color {
                       r: 0.5,
                       g: 0.7,
                       b: 0.9,
                   },
                   col1 + col2);
    }

    #[test]
    fn test_div_color() {
        let col1 = Color {
            r: 0.1,
            g: 0.2,
            b: 0.3,
        };
        assert_eq!(Color {
                       r: 0.033333,
                       g: 0.066666,
                       b: 0.1,
                   },
                   col1 / 3.0);
    }

    #[test]
    fn test_mul_color() {
        let col1 = Color {
            r: 0.1,
            g: 0.2,
            b: 0.3,
        };
        assert_eq!(Color {
                       r: 0.2,
                       g: 0.4,
                       b: 0.6,
                   },
                   2.0 * col1);
    }

    #[test]
    fn test_mul_color_by_vector() {
        let vec1 = Vec3D {
            x: 0.4,
            y: 0.3,
            z: 0.2,
        };
        let col1 = Color {
            r: 0.1,
            g: 0.2,
            b: 0.3,
        };
        assert_eq!(Color {
                       r: 0.04,
                       g: 0.06,
                       b: 0.06,
                   },
                   &vec1 * &col1);
    }
}