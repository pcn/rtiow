extern crate approx;
extern crate nalgebra as na; //

use na::{Point3, Vector3};

mod ray;

// use std::mem::Discriminant;

use ray::Ray;

type Color = Vector3<f32>;

fn hit_sphere(center: Point3<f32>, radius: f32, r: &Ray) -> f32 {
    let oc = r.origin - center;
    let a = length_squared(r.direction);
    let half_b = oc.dot(&r.direction);
    let c = length_squared(oc) - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

fn ray_color(r: &Ray) -> Color {
    let t = hit_sphere(Point3::<f32>::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let n = unit_vector(r.at(t) - Vector3::<f32>::new(0.0, 0.0, -1.0));
        0.5 * Color::new(n.x + 1.0, n.y + 1.0, n.z + 1.0)
    } else {
        let unit_direction = unit_vector(r.direction);
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

fn length(v: Vector3<f32>) -> f32 {
    length_squared(v).sqrt()
}

fn length_squared(v: Vector3<f32>) -> f32 {
    v.x * v.x + v.y * v.y + v.z * v.z
}

// This was a bug
fn dot(lhs: Vector3<f32>, rhs: Vector3<f32>) -> f32 {
    lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * lhs.z
}

fn unit_vector(v: Vector3<f32>) -> Vector3<f32> {
    v.clone() / length(v)
}

fn print_color(p: Color) {
    // Write the translated [0,255] value of each color component
    // I'm not clear that I can use println here - I should
    // probably work on how to get this to format somehow?

    println!(
        "{} {} {}",
        (p.x * 255.999) as i32,
        (p.y * 255.999) as i32,
        (p.z * 255.999) as i32,
    )
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as i32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vector3::<f32>::new(0.0, 0.0, 0.0);
    let horizontal = Vector3::<f32>::new(viewport_width, 0.0, 0.0);
    let vertical = Vector3::<f32>::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vector3::<f32>::new(0.0, 0.0, focal_length);

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);
    for j in (0..image_height).rev() {
        eprintln!("\rScanlines remaining: {}", j);
        for i in 0..image_width {
            let u = i as f32 / (image_width - 1) as f32;
            let v = j as f32 / (image_height - 1) as f32;
            let r = Ray::new(
                Point3::from(origin),
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            print_color(ray_color(&r));
        }
    }
    println!("\nDone");
}
