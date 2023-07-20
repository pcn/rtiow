use std::cmp::PartialEq;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use std::fmt;

// Section 3.1

#[derive(PartialEq, Clone)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

pub type Point3 = Vec3;
pub type Color = Vec3;

trait Length {
    fn length(&self) -> f64;

    fn length_squared(&self) -> f64;
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
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
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        let retval = self * (1.0 as f64 / rhs);
        retval
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}

impl Length for Vec3 {
    fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    fn length_squared(&self) -> f64 {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }
}

// XXX: Still missing:
//  What/how is double e[3] supposed to be here? Is it extensible at the end of section 3.1

// Section 3.2, translate listing 5
// Need to implement debug and/or something else here for printin the vec3

// Utility functions
impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
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

// I wonder whether it's better to put all of the mul variants
// together, or follow the layout of the original web page.
impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Vec3) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

fn dot(lhs: Vec3, rhs: Vec3) -> Vec3 {
    Vec3 {
        x: lhs.x * rhs.x,
        y: lhs.y * rhs.y,
        z: lhs.z * lhs.z,
    }
}

fn cross(lhs: Vec3, rhs: Vec3) -> Vec3 {
    Vec3 {
        x: (lhs.y * rhs.z) - (lhs.z * rhs.y),
        y: (lhs.z * rhs.x) - (lhs.x * rhs.z),
        z: (lhs.x * rhs.y) - (lhs.y * rhs.x),
    }
}

fn unit_vector(v: Vec3) -> Vec3 {
    v.clone() / v.length()
}

// XXX: in the original c++, this is writing to the output descriptor, and in my
// case I'm copying over the trait implementation of fmt for Vec3, I don't know
// if I need to return the result.

pub trait ColorDisplay {
    fn pixel_color(&self);
}

// Can I put a trait boundary or something else on this to prevent
impl ColorDisplay for Vec3 {
    fn pixel_color(&self) {
        // Write the translated [0,255] value of each color component
        // I'm not clear that I can use println here - I should
        // probably work on how to get this to format somehow?

        println!(
            "{} {} {}",
            (self.x * 255.999) as i32,
            (self.y * 255.999) as i32,
            (self.z * 255.999) as i32,
        )
    }
}
