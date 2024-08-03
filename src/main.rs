mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use hittable_list::HittableList;
use ray::Ray;
use sphere::Sphere;
use vec3::Vec3;

fn main() {
    let mut world: HittableList = HittableList::new();

    world.add(Box::new(Sphere::new(Vec3(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3(0.0, -100.5, -1.0), 100.0)));

    let camera: Camera = Camera::new(16.0 / 9.0, 400, 100);
    camera.render(&world);
}
