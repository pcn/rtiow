// use crate::vec3::{Point3, Vec3};

extern crate nalgebra as na;
use na::{Point3, Vector3};

pub struct Ray {
    pub origin: Point3<f32>,
    pub direction: Vector3<f32>, // In the source, the accessor method is direction, the var members name is dir, but let's just use direction everywhere.
}

impl Ray {
    pub fn new(origin: Point3<f32>, direction: Vector3<f32>) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f32) -> Vector3<f32> {
        Vector3::from(self.origin.coords) + (t * self.direction)
    }
}
