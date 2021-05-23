mod vec3;
mod ray;
mod hitables;
mod img_out;
mod camera;

use ndarray::Array3;
use vec3::Vec3;
use ray::*;
use hitables::*;
use camera::Camera;
use rand::Rng;
use rand::prelude::ThreadRng;

const NX: usize = 800;
const NY: usize = 400;
const INX_F: f32 = 1.0 / (NX as f32);
const INY_F: f32 = 1.0 / (NY as f32);
const SAMPLES: i32 = 100;


fn main() {
    let mut array: Array3<u8> = Array3::zeros((NY, NX, 3)); // 250x200 RGB
    let mut world: Vec<Sphere> = vec![];
    world.push(Sphere { center: Vec3 { x: 0.0, y: 0.0, z: -1.0 }, radius: 0.5 });
    world.push(Sphere { center: Vec3 { x: 0.0, y: -100.5, z: -1.0 }, radius: 100.0 });
    let camera = Camera::default();
    let mut rng = rand::thread_rng();
    for (y, mut row) in array.outer_iter_mut().enumerate() {
        for (x, mut pix) in row.outer_iter_mut().enumerate() {
            let mut col: Vec3 = (0..SAMPLES)
                .map(|_| sample(&world, camera, &mut rng, y, x))
                .sum();
            col /= SAMPLES as f32;
            pix[0] = (col[0].powf(1.0 / 2.2) * 255.99) as u8;
            pix[1] = (col[1].powf(1.0 / 2.2) * 255.99) as u8;
            pix[2] = (col[2].powf(1.0 / 2.2) * 255.99) as u8;
        }
    }
    assert!(img_out::array_save(array, "img.png".to_string()).is_ok());
}

fn sample<T: Hitable>(world: T, camera: Camera, rng: &mut ThreadRng, y: usize, x: usize) -> Vec3 {
    let u_r: f32 = rng.gen();
    let v_r: f32 = rng.gen();
    let u = (u_r + x as f32) * INX_F;
    let v = (v_r + ((NY - y) as f32)) * INY_F;
    let r = camera.get_ray(u, v);
    let samp_col = color(r, world);
    samp_col
}

fn color<T: Hitable>(ray: Ray, hitable: T) -> Vec3 {
    let t = hitable.hit_rec(&ray);

    if t == None {
        let unit_direction = ray.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Vec3 { x: 1.0, y: 1.0, z: 1.0 } + t * Vec3 { x: 0.5, y: 0.7, z: 1.0 }
    } else {
        let record = t.unwrap();
        let newdir = record.loc + record.normal + rand_unit_sphere();
        let newray = Ray { origin: record.loc, direction: newdir };
        0.5 * color(newray, hitable)
    }
}
