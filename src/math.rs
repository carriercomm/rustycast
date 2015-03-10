
use std::ops::{Add, Sub, Mul, Neg, Div};
use std::num::Float;
use std::f32::consts;

// ---------------------------------------------------------------------------

#[derive(Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Copy)]
pub struct Mat3 {
    pub a00: f32, pub a01: f32, pub a02: f32,
    pub a10: f32, pub a11: f32, pub a12: f32,
    pub a20: f32, pub a21: f32, pub a22: f32,
}

#[derive(Copy)]
pub struct LineSeg {
    pub a: Vec2,
    pub b: Vec2,
}

pub const V2_ORIGIN: Vec2 = Vec2 {x:0.0, y:0.0};

pub const M3_IDENTITY: Mat3 = Mat3 {
    a00: 1.0, a01: 0.0, a02: 0.0,
    a10: 0.0, a11: 1.0, a12: 0.0,
    a20: 0.0, a21: 0.0, a22: 1.0,
};

// ----- Operator overloading ------------------------------------------------

impl Add<Vec2> for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2 {x: self.x + rhs.x, y: self.y + rhs.y}
    }
}

impl Sub<Vec2> for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Vec2) -> Vec2 {
        Vec2 {x: self.x - rhs.x, y: self.y - rhs.y}
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: f32) -> Vec2 {
        Vec2 {x: self.x * rhs, y: self.y * rhs}
    }
}

impl Div<f32> for Vec2 {
    type Output = Vec2;
    fn div(self, rhs: f32) -> Vec2 {
        Vec2 {x: self.x / rhs, y: self.y / rhs}
    }
}

impl Neg for Vec2 {
    type Output = Vec2;
    fn neg(self) -> Vec2 {
        Vec2 {x: -self.x, y: -self.y}
    }
}

impl Mul<Mat3> for Mat3 {
    type Output = Mat3;
    fn mul(self, rhs: Mat3) -> Mat3 {
        Mat3 {
            a00: self.a00*rhs.a00 + self.a01*rhs.a10 + self.a02*rhs.a20,
            a01: self.a00*rhs.a01 + self.a01*rhs.a11 + self.a02*rhs.a21,
            a02: self.a00*rhs.a02 + self.a01*rhs.a12 + self.a02*rhs.a22,

            a10: self.a10*rhs.a00 + self.a11*rhs.a10 + self.a12*rhs.a20,
            a11: self.a10*rhs.a01 + self.a11*rhs.a11 + self.a12*rhs.a21,
            a12: self.a10*rhs.a02 + self.a11*rhs.a12 + self.a12*rhs.a22,

            a20: self.a20*rhs.a00 + self.a21*rhs.a10 + self.a22*rhs.a20,
            a21: self.a20*rhs.a01 + self.a21*rhs.a11 + self.a22*rhs.a21,
            a22: self.a20*rhs.a02 + self.a21*rhs.a12 + self.a22*rhs.a22,
        }
    }
}

impl Mul<Vec2> for Mat3 {
    type Output = Vec2;
    fn mul(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.a00*rhs.x + self.a01*rhs.y + self.a02,
            y: self.a10*rhs.x + self.a11*rhs.y + self.a12
        }
    }
}

// ---------------------------------------------------------------------------

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2 {x: x, y: y}
    }

    pub fn get_length_sqr(self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    pub fn get_length(self) -> f32 {
        Float::sqrt(self.get_length_sqr())
    }

    pub fn dot(self, rhs: Vec2) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }

    pub fn cross(self, rhs: Vec2) -> f32 {
        self.x * rhs.y - rhs.x * self.y
    }

    pub fn normalize(self) -> Vec2 {
        self / self.get_length()
    }
}

impl Mat3 {
    pub fn new(a00: f32, a01: f32, a02: f32,
               a10: f32, a11: f32, a12: f32,
               a20: f32, a21: f32, a22: f32) -> Mat3 {
        Mat3 { a00: a00, a01: a01, a02: a02,
               a10: a10, a11: a11, a12: a12,
               a20: a20, a21: a21, a22: a22 }
    }

    pub fn rotation(theta: f32) -> Mat3 {
        Mat3::new(
            Float::cos(theta), -Float::sin(theta), 0.0,
            Float::sin(theta),  Float::cos(theta), 0.0,
                          0.0,                0.0, 1.0,
        )
    }

    pub fn translation(t: Vec2) -> Mat3 {
        Mat3::new(
            1.0, 0.0, t.x,
            0.0, 1.0, t.y,
            0.0, 0.0, 1.0
        )
    }

    pub fn scale(s: Vec2) -> Mat3 {
        Mat3::new(
            s.x, 0.0, 0.0,
            0.0, s.y, 0.0,
            0.0, 0.0, 1.0
        )
    }
}

impl LineSeg {
    pub fn new(x0: f32, y0: f32, x1: f32, y1: f32) -> LineSeg {
        LineSeg {
            a: Vec2 {x: x0, y: y0},
            b: Vec2 {x: x1, y: y1},
        }
    }

    #[allow(unused_variables)]
    pub fn intersects_at(self, rhs: LineSeg) -> Option<f32> {
        None // TODO
    }

    #[allow(unused_variables)]
    pub fn point_at(self, t: f32) -> Vec2 {
        V2_ORIGIN // TODO
    }

    pub fn transform(self, mat: Mat3) -> LineSeg {
        LineSeg {
            a: mat * self.a,
            b: mat * self.b,
        }
    }
}

// ---------------------------------------------------------------------------

fn main() {
    let a = LineSeg::new(-10.0, -10.0, 10.0, -10.0);
    let c = a.transform(Mat3::rotation(consts::FRAC_PI_4));
    println!("{}, {}", c.a.x, c.a.y);
    println!("{}, {}", c.b.x, c.b.y);
}

