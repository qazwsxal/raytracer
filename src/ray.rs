use crate::vec3::Vec3;
use rand::Rng;


#[derive(Debug, Copy, Clone, PartialEq, Default)] // PartialEq because NaN != NaN
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}

pub fn rand_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = 2.0 * Vec3 { x: rng.gen(), y: rng.gen(), z: rng.gen() } - Vec3 { x: 1.0, y: 1.0, z: 1.0 };
        if p.squared_length() < 1.0 { return p; }
    }

}