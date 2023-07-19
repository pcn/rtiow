mod color;
mod vec3;

const IMAGE_WIDTH: i32 = 256;
const IMAGE_HEIGHT: i32 = 256;

fn main() {
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("\rScanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let r = i as f32 / (IMAGE_WIDTH as f32 - 1.0);
            let g = j as f32 / (IMAGE_HEIGHT as f32 - 1.0);
            let b = 0.25;

            println!(
                "{} {} {}",
                (r * 255.999) as i32,
                (g * 255.999) as i32,
                (b * 255.999) as i32
            );
        }
    }
    eprintln!("\nDone");
}
