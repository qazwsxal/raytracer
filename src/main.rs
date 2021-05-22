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

fn main() {
    let nx = 800;
    let ny = 400;
    let inx_f = 1.0 / (nx as f32);
    let iny_f = 1.0 / (ny as f32);
    let samples = 100;
    let mut array: Array3<u8> = Array3::zeros((ny, nx, 3)); // 250x200 RGB
    let mut world: Vec<Sphere> = vec![];
    world.push(Sphere { center: Vec3 { x: 0.0, y: 0.0, z: -1.0 }, radius: 0.5 });
    world.push(Sphere { center: Vec3 { x: 0.0, y: -100.5, z: -1.0 }, radius: 100.0 });
    let camera = Camera::default();
    let mut rng = rand::thread_rng();
    for (y, mut row) in array.outer_iter_mut().enumerate() {
        for (x, mut pix) in row.outer_iter_mut().enumerate() {
            let mut col = Vec3::default();
            for _ in 0..samples {
                let u_r: f32 = rng.gen();
                let v_r: f32 = rng.gen();
                let u = (u_r + x as f32) * inx_f;
                let v = (v_r + ((ny - y) as f32)) * iny_f;
                let r = camera.get_ray(u, v);
                col += color(r, &world);
            }
            col /= samples as f32;
            pix[0] = (col[0].powf(1.0/2.2)*255.99) as u8;
            pix[1] = (col[1].powf(1.0/2.2)*255.99) as u8;
            pix[2] = (col[2].powf(1.0/2.2)*255.99) as u8;
        }
    }
    assert!(img_out::array_save(array, "img.png".to_string()).is_ok());
}
