mod color;
mod vec3;

use color::write_color;
use vec3::Vec3;

fn main() {
    // Image

    let image_width: u16 = 256;
    let image_height: u16 = 256;

    let _x: Vec3 = Vec3(2.0, 3.0, 4.0);

    // Render

    println!("P3");
    println!("{image_width} {image_height}");
    println!("255");

    for j in 0..image_height {
        eprintln!("Scanlines remaining {}", image_height - j);
        for i in 0..image_width {
            let pixel_color = Vec3(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0 as f64,
            );
            write_color(&pixel_color);
        }
    }
    eprintln!("Done")
}
