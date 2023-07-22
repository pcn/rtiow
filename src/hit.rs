extern crate nalgebra as na;
use crate::length_squared;
use crate::ray::Ray;
use na::{Point3, Vector3};

pub struct HitRecord {
    pub p: Point3<f32>,
    pub normal: Vector3<f32>,
    pub t: f32,
}

// pub impl HitRecord {
//     pub fn hit(r: &Ray, t_min: f32, t_max: f32, rec: &hitHrecord) -> bool;
// }

pub struct Sphere {
    pub center: Point3<f32>,
    pub radius: f32,
}

impl Sphere {
    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = r.origin - self.center;
        let a = length_squared(r.direction);
        let half_b = oc.dot(&r.direction);
        let c = length_squared(oc) - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        // I think  match is the idiomatic way I should be doing this
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            } // XXX this is wrong
        }
        rec.t = root;
        rec.p = Point3::from(r.at(rec.t));
        rec.normal = (rec.p - self.center) / self.radius;

        true
    }
}
