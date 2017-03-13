extern crate rand;
use std::fmt;
use ray::Ray;
use vector::Vec3D;


pub trait Material {
    fn scatter<'a>(&self, ray: &Ray, p: &'a Vec3D, n: &'a Vec3D) -> (bool, Vec3D, Ray);
}

impl fmt::Debug for Material {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Material")
    }
}

pub struct Lambertian {
    pub albedo: Vec3D,
}

fn random_in_unit_sphere() -> Vec3D {
    loop {
        let p = &(2.0 *
                  &Vec3D {
            x: rand::random::<f64>(),
            y: rand::random::<f64>(),
            z: rand::random::<f64>(),
        }) - &Vec3D::new_ones_vector();
        if Vec3D::dot(&p, &p) < 1.0 {
            return p;
        }
    }
}

impl Material for Lambertian {
    fn scatter<'a>(&self, _: &Ray, p: &'a Vec3D, n: &'a Vec3D) -> (bool, Vec3D, Ray) {
        let target = &(p + n) + &random_in_unit_sphere();
        let scattered = Ray::new(p, &(&target - p));
        let attenuation = self.albedo.clone();
        (true, attenuation, scattered)
    }
}

pub struct Metal {
    albedo: Vec3D,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: &Vec3D, fuzz: f64) -> Metal {
        let f = if fuzz < 1.0 {
            fuzz
        } else {
            1.0
        };
        Metal {
            albedo: albedo.clone(),
            fuzz: f,
        }
    }
}

fn reflect(v: &Vec3D, n: &Vec3D) -> Vec3D {
    v - &(2.0 * Vec3D::dot(v, n) * n)
}

impl Material for Metal {
    fn scatter<'a>(&self, ray: &Ray, p: &'a Vec3D, n: &'a Vec3D) -> (bool, Vec3D, Ray) {
        let reflected = reflect(&ray.direction().unit_vector(), n);
        let scattered = Ray::new(p, &(&reflected + &(self.fuzz * &random_in_unit_sphere())));
        let attenuation = self.albedo.clone();
        let ok = Vec3D::dot(&scattered.direction(), n) > 0.0;
        (ok, attenuation, scattered)
    }
}

pub struct Dielectric {
    pub refractiveness: f64,
}

fn refract(v: &Vec3D, n: &Vec3D, ni_over_nt: f64) -> (bool, Vec3D) {
    let uv = v.unit_vector();
    let dt = Vec3D::dot(&uv, n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        let refraction = &(ni_over_nt * &(v - &(dt * n))) - &(discriminant.sqrt() * n);
        return (true, refraction);
    }
    (false, Vec3D::new_zero_vector())
}

fn schlick(cosine: f64, refractiveness: f64) -> f64 {
    let r0 = ((1.0 - refractiveness) / (1.0 + refractiveness)).powi(2);
    r0 + ((1.0 - r0) * (1.0 - cosine).powi(5))
}

impl Material for Dielectric {
    fn scatter<'a>(&self, ray: &Ray, p: &'a Vec3D, n: &'a Vec3D) -> (bool, Vec3D, Ray) {
        let reflection = reflect(&ray.direction(), n);
        let attenuation = Vec3D::new_ones_vector();
        let (outward_normal, ni_over_nt, cosine) = if Vec3D::dot(&ray.direction(), n) > 0.0 {
            (-n,
             self.refractiveness,
             self.refractiveness * Vec3D::dot(&ray.direction(), n) / ray.direction().length())
        } else {
            (n.clone(),
             1.0 / self.refractiveness,
             -Vec3D::dot(&ray.direction(), n) / ray.direction().length())
        };
        let (refracted, refraction) = refract(&ray.direction(), &outward_normal, ni_over_nt);
        let reflect_prob = if refracted {
            schlick(cosine, self.refractiveness)
        } else {
            1.0
        };
        let scattered = if rand::random::<f64>() < reflect_prob {
            Ray::new(p, &reflection)
        } else {
            Ray::new(p, &refraction)
        };
        (true, attenuation, scattered)
    }
}