use crate::{interval::Interval, vec3::Vec3};

pub fn write_color(pixel_color: &Vec3) {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    // Translate the [0, 1] component values to the byte range [0,255].
    const INTENSITY: Interval = Interval(0.000, 0.999);
    let rbyte: u8 = (256.0 * INTENSITY.clamp(r)) as u8;
    let gbyte: u8 = (256.0 * INTENSITY.clamp(g)) as u8;
    let bbyte: u8 = (255.0 * INTENSITY.clamp(b)) as u8;

    println!("{rbyte} {gbyte} {bbyte}")
}
