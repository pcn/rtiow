extern crate nalgebra as na;
use crate::length_squared;
use crate::ray::Ray;
use na::{Point3, Vector3};

pub struct HitRecord {
    pub p: Point3<f32>,
    pub normal: Vector3<f32>,
    pub t: f32,
    pub front_face: bool,
}

// pub impl HitRecord {
//     pub fn hit(r: &Ray, t_min: f32, t_max: f32, rec: &hitHrecord) -> bool;
// }

pub struct HittableRecords {
    pub hittables: Vec<HitRecord>,
}

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
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);

        true
    }
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vector3<f32>) {
        let front_face = r.direction.dot(outward_normal) < 0.0;
        *self.normal = if front_face {
            **outward_normal
        } else {
            *(-*outward_normal)
        };
    }
}

impl HittableRecords {
    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &HitRecord) -> Option<HitRecord> {
        let mut temp_rec: HitRecord;
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        // In the original, rec is only being passed in so it can be mofidified.
        // It also gets overridden
        self.hittables.iter().for_each(|h| {
            if h.hit(r, t_min, closest_so_far, temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
            }
        });
        if temp_rec {
            rec = temp_rec;
        }
        hit_anything // I think this should return the temp_rec and hit_anything
                     // (wrapped in an option)
    }
}
