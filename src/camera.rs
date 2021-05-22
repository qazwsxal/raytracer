use crate::vec3::Vec3;
use crate::ray::Ray;

#[derive(Debug, Copy, Clone, PartialEq)] // PartialEq because NaN != NaN
pub struct Camera {
    pub llc: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub origin: Vec3,
}

impl Camera {
    pub fn get_ray(&self, u:f32, v:f32)-> Ray {
        Ray {
            origin: self.origin,
            direction: self.llc + u*self.horizontal + v*self.vertical - self.origin
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            llc: Vec3 { x: -2.0, y: -1.0, z: -1.0 },
            horizontal: Vec3 { x: 4.0, y: 0.0, z: 0.0 },
            vertical: Vec3 { x: 0.0, y: 2.0, z: 0.0 },
            origin: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
        }
    }
}