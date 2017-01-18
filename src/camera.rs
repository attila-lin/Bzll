extern crate rand;

use std::f64;

use ray::Ray;
use vector::Vec3D;

pub struct Camera {
    lower_left_corner: Vec3D,
    horizontal: Vec3D,
    vertical: Vec3D,
    origin: Vec3D,
    u: Vec3D,
    v: Vec3D,
    lens_radius: f64,
}

impl Camera {
    pub fn new(from: &Vec3D,
               at: &Vec3D,
               up: &Vec3D,
               vfov: f64,
               aspect: f64,
               aperture: f64,
               focus_dist: f64)
               -> Camera {
        let lens_radius = aperture / 2.0;
        let theta = vfov * f64::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = (from - at).unit_vector();
        let u = Vec3D::cross(up, &w).unit_vector();
        let v = Vec3D::cross(&w, &u);
        Camera {
            lower_left_corner: &(&(from - &(half_width * focus_dist * &u)) +
                                 &(half_height * focus_dist * &v)) -
                               &(focus_dist * &w),
            horizontal: 2.0 * half_width * focus_dist * &u,
            vertical: -2.0 * half_height * focus_dist * &v,
            origin: from.clone(),
            u: u,
            v: v,
            lens_radius: lens_radius,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * &random_in_unit_disk();
        let offset = &(rd.x * &self.u) + &(rd.y * &self.v);
        Ray::new(&(&self.origin + &offset),
                 &(&(&(&self.lower_left_corner + &(s * &self.horizontal)) +
                     &(t * &self.vertical)) - &(&self.origin + &offset)))
    }
}

fn random_in_unit_disk() -> Vec3D {
    loop {
        let p = &(2.0 *
                  &Vec3D {
            x: rand::random::<f64>(),
            y: rand::random::<f64>(),
            z: 0.0,
        }) -
                &Vec3D {
            x: 1.0,
            y: 1.0,
            z: 0.0,
        };
        if Vec3D::dot(&p, &p) < 1.0 {
            return p;
        }
    }
}