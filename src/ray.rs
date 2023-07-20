use crate::vec3::{Point3, Vec3};

pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    fn new(point3: Point3, direction: Vec3) -> Self {
        Self {
            orig: point3,
            dir: direction,
        }
    }

    fn at(&self, t: f64) -> Point3 {
        self.orig + (t * self.dir)
    }
}
