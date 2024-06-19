use crate::vec3::Vec3;

pub fn write_color(pixel_color: &Vec3) {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    let rbyte: u8 = (255.999 * r) as u8;
    let gbyte: u8 = (255.999 * g) as u8;
    let bbyte: u8 = (255.999 * b) as u8;

    println!("{rbyte} {gbyte} {bbyte}")
}
