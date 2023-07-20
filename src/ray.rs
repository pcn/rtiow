use crate::vec3::{Point3, Vec3};

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3, // In the source, the accessor method is direction, the var members name is dir, but let's just use direction everywhere.
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + (t * self.direction)
    }
}
