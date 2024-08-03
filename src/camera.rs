use crate::color::write_color;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::{Ray, Vec3};
use rand::rngs::ThreadRng;
use rand::Rng;

#[derive(Clone, Copy)]
pub struct Camera {
    //    aspect_ratio: f64,
    samples_per_pixel: u16,
    image_width: u16,
    image_height: u16,
    pixel_samples_scale: f64,
    center: Vec3,
    pixel100_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn render(&self, world: &dyn Hittable) {
        println!("P3");
        println!("{} {}", self.image_width, self.image_height);
        println!("255");

        let mut rng: ThreadRng = rand::thread_rng();

        for j in 0..self.image_height {
            eprintln!("Scanlines remaining {}", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = Vec3(0.0, 0.0, 0.0);
                for _sample in 0..self.samples_per_pixel {
                    let r: Ray = self.get_ray(i, j, &mut rng);
                    pixel_color += Self::ray_color(&r, world);
                }
                write_color(&(self.pixel_samples_scale * pixel_color));
            }
        }
        eprintln!("Done")
    }

    pub fn new(aspect_ratio: f64, image_width: u16, samples_per_pixel: u16) -> Self {
        let image_height: u16 = ((image_width as f64 / aspect_ratio) as u16).max(1);

        let center: Vec3 = Vec3(0.0, 0.0, 0.0);

        let focal_length: f64 = 1.0;
        let viewport_height: f64 = 2.0;
        let viewport_width: f64 = viewport_height * (image_width as f64) / image_height as f64;

        // Calculate the vectors across the horizontal and down the vertucal viewport edges.
        let viewport_u = Vec3(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and verical delta vectors from pixel to pixel.
        let pixel_delta_u: Vec3 = viewport_u / image_width as f64;
        let pixel_delta_v: Vec3 = viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            center - Vec3(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel100_loc: Vec3 = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Camera {
            //    aspect_ratio,
            samples_per_pixel,
            image_width,
            image_height,
            pixel_samples_scale: 1.0 / samples_per_pixel as f64,
            center,
            pixel100_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    fn get_ray(self, i: u16, j: u16, rng: &mut ThreadRng) -> Ray {
        let offset = Self::sample_square(rng);
        let pixel_sample = self.pixel100_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square(rng: &mut ThreadRng) -> Vec3 {
        Vec3(rng.gen::<f64>() - 0.5, rng.gen::<f64>() - 0.5, 0.0)
    }

    fn ray_color(r: &Ray, world: &dyn Hittable) -> Vec3 {
        let rec: &mut HitRecord = &mut HitRecord::default();
        if world.hit(r, &Interval(0.0, f64::INFINITY), rec) {
            return 0.5 * (rec.normal + Vec3(1.0, 1.0, 1.0));
        }

        let unit_direction = Vec3::unit_vector(&r.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Vec3(1.0, 1.0, 1.0) + a * (Vec3(0.5, 0.7, 1.0))
    }
}
