use vector3::vector3::Vec3;
use vector3::vector3::Color;
use ray::ray::Ray;

mod vector3;
mod ray;

fn main() {
    let mut ray = Ray{
        origin: Vec3{
            x: 10.0,
            y: 5.0,
            z: 2.5
        },
        dir: Vec3 {
            x: 12.25,
            y: 7.8,
            z: 5.0
        }
    };
    println!("origin: {}", ray.at(5.0).to_string());

}

fn main2() {
    let length = 255;
    let width = 255;
    println!("P3");
    println!("{} {}", length, width);
    println!("255");
    for l in 0..length {
        eprintln!("Scan lines Remaining: {}", (length - l));
        for w in 0..width {
            let color : Color = Color {x: (l as f32 / (length as f32 - 1.0)) * 255.999, y: (w as f32 / (width as f32 - 1.0)) * 255.999, z: 0.25 * 255.999};
            eprintln!("Color R: {}", color.x);
            color.print_color();
        }
        print!("\n");
    }
}
