use vector3::vector3::Vec3;
use vector3::vector3::Color;
use ray::ray::Ray;

mod vector3;
mod ray;

fn hit_sphere(center: Vec3, radius: f64, ray: &Ray) -> bool {
    let oc = ray.origin - center;
    let a = ray.dir.dot_product(ray.dir);
    let b = 2.0 * oc.dot_product(ray.dir);
    let c = oc.dot_product(oc) - radius*radius;
    let discriminant = b*b - 4.0*a*c;
    return discriminant > 0.0;
}

fn ray_color (ray: &Ray) -> Color {
    if hit_sphere(Vec3{x: 0.0, y: 0.0, z: -1.0}, 0.5, &ray) {
        return Color {x: 1.0, y:0.0, z: 0.0};
    }
    let unit_dir = ray.dir.unit_vector();
    let t = 0.5*(unit_dir.y+1.0);
    let mut col1 = Color {x:1.0, y: 1.0, z: 1.0};
    let mut col2 = Color {x: 0.5, y:0.7, z: 1.0};
    col1 *= 1.0-t;
    col2 *=t;
    return col1+col2;
}

// fn main(){
//     let a = Vec3{x: 1.0, y: 2.0, z: 3.0};
//     println!("{}", a.unit_vector().to_string());
// }

fn main() {
    //Image Variables:
    let aspect_ratio = 16.0/9.0;
    let image_width = 800;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    //Base the image height on the aspect ratio and the width, so we get an exact 16:9 image result.

    //Camera Variables:
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    //Get the viewport to also be based on the aspect ratio...
    let focal_length = 1.0;

    let origin = Vec3 { x: 0.0, y: 0.0, z: 0.0};
    let horizontal = Vec3{ x: viewport_width, y: 0.0, z: 0.0};
    let vertical = Vec3{ x: 0.0, y: viewport_height, z: 0.0};
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3{x: 0.0, y: 0.0, z: focal_length};

    println!("P3\n{} {}\n255", image_width, image_height);
    for j in (0..image_height).rev() {
        eprintln!("Scan lines Remaining: {}", j);
        for i in 0..image_width {
            let u:f64 = i as f64 / (image_width as f64 - 1.0) as f64;
            let v:f64 = j as f64 / (image_height as f64 - 1.0) as f64;
            let ray = Ray {origin, dir: lower_left_corner + horizontal*u + vertical*v - origin};
            let color = ray_color(&ray);
            color.print_color();
        }
        print!("\n");
    }
}

fn main2() {
    let mut vec = Vec3 {x: 1.0, y: 1.0, z: 1.0};
    vec /= 2.0;
    println!("{}", vec.to_string())
}

