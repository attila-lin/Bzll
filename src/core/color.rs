static const black  = Color(0,0,0,1)
static const blue   = Color(0,0,1,0)
static const clear  = Color(0,0,0,0)
static const cyan   = Color(0,1,1,1)
static const gray   = Color(0.5, 0.5, 0.5, 1)
static const magenta= Color(1,0,1,1)
static const red    = Color(1,0,0,1)
static const white  = Color(1,1,1,1)
static const yellow = Color(1, 0.92, 0.016, 1)

#[derive(Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32, // Alpha component of the color (0 is transparent, 1 is opaque).
}

// 重载操作符
impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color { 
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
            a: self.a + other.a 
        }
    }
}

impl Color {
    // public methods
    pub fn new(r: f32, g: f32, b: f32, a: f32) {
        Color {
            r: r,
            g: g,
            b: b,
            a: a
        }
    }

    pub fn to_rgb32() {

    }

    pub fn to_argb32() {

    }

    // static methods
    pub fn HSVToRGB() {

    }

    pub fn Lerp() {

    }

    pub fn LerpUnclamped() {

    }

    pub fn RGBToHSV() {

    }

    //
}