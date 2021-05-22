use crate::ray::Ray;
use crate::vec3::*;

// pub enum Materials {
//     Lambertian(Vec3)
//
// }
//
// pub struct Lambertian {
//     albedo: Vec3
// }

#[derive(Debug, Copy, Clone, PartialEq, Default)] // PartialEq because NaN != NaN
pub struct HitRecord {
    pub t: f32,
    pub loc: Vec3,
    pub normal: Vec3,
//    pub mat: &'a Materials
}

const T_MIN: f32 = 0.001;
const T_MAX: f32 = f32::MAX;


pub trait Hitable {
    fn hit(&self, r: &Ray) -> Option<f32>;
    fn norm(&self, point: Vec3) -> Vec3;
    fn hit_rec(&self, r: &Ray) -> Option<HitRecord>;
}

#[derive(Debug, Copy, Clone, PartialEq, Default)] // PartialEq because NaN != NaN
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray) -> Option<f32> {
        let oc = r.origin - self.center;
        let a = dot(r.direction, r.direction);
        let b = 2.0 * dot(oc, r.direction);
        let c = dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        // If it doesn't hit
        if discriminant < 0.0 {
            return None;
        } else {
            let t = (-b - discriminant.sqrt()) / (2.0 * a);
            // If it hits *behind* the current position
            if t <= T_MIN {
                return None;
            }
            // Else it hits in front of the current position
            else {
                return Some(t);
            }
        }
    }
    fn norm(&self, point: Vec3) -> Vec3 {
        (point - self.center) / self.radius
    }
    fn hit_rec(&self, r: &Ray) -> Option<HitRecord> {
        let hit_dist = self.hit(r);
        if hit_dist == None {
            return None;
        } else {
            let t = hit_dist.unwrap();
            let loc = r.point_at_parameter(t);
            let normal = self.norm(loc);
            return Some(HitRecord { t, loc, normal });
        }
    }
}

impl<T> Hitable for &Vec<T> where
    T: Hitable,
{
    fn hit(&self, r: &Ray) -> Option<f32> {
        let res = self
            .into_iter()
            .filter_map(|x| x.hit(&r))
            .collect::<Vec<f32>>();
        if res.len() == 0 {
            return None;
        } else {
            // res.min() doesn't work here as f32 isn't Ordered (NaNs)
            //return Some(res.min())
            return Some(res.into_iter().fold(f32::MAX, f32::min));
        }
    }

    fn norm(&self, _point: Vec3) -> Vec3 {
        todo!()
    }

    fn hit_rec(&self, r: &Ray) -> Option<HitRecord> {
        let res = self
            .iter().enumerate()
            .filter_map(|(i, x)|
                match x.hit(r) {
                    None => None,
                    Some(t) => Some((i, t)),
                })
            .collect::<Vec<_>>();
        if res.len() == 0 {
            return None;
        } else {
            let out = res
                .iter()
                .min_by(|(_, xt), (_, yt)|
                    xt.partial_cmp(&yt)
                        .unwrap());
            if out == None {
                return None;
            } else {
                let (i, t): (usize, f32) = *out.unwrap();
                let loc = r.point_at_parameter(t);
                let normal = self[i].norm(loc);
                return Some(HitRecord { t: t, loc, normal });
            }
        }
    }
}