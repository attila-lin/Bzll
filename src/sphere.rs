use hitable::Hitable;
use material::Material;
use ray::Ray;
use vector::Vec3D;

pub struct Sphere<'a> {
	pub center: &'a Vec3D,
    pub radius: f64,
    pub material: &'a Material,
}

impl<'a> Hitable for Sphere<'a> {
    fn hit(&self,
           ray: &Ray,
           t_min: f64,
           t_max: f64)
           -> (bool, f64, Vec3D, Vec3D, bool, Vec3D, Ray) {
        let oc = &ray.origin() - self.center;
        let a = Vec3D::dot(&ray.direction(), &ray.direction());
        let b = Vec3D::dot(&oc, &ray.direction());
        let c = Vec3D::dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = ray.point_at_parameter(temp);
                let normal = &(&p - self.center) / self.radius;
                let (scatter_ok, attenuation, scattered) = scatter(self.material, ray, &p, &normal);
                return (true, temp, p, normal, scatter_ok, attenuation, scattered);
            }
            let temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = ray.point_at_parameter(temp);
                let normal = &(&p - self.center) / self.radius;
                let (scatter_ok, attenuation, scattered) = scatter(self.material, ray, &p, &normal);
                return (true, temp, p, normal, scatter_ok, attenuation, scattered);
            }
        }
        (false,
         0.0,
         Vec3D::new_zero_vector(),
         Vec3D::new_zero_vector(),
         false,
         Vec3D::new_zero_vector(),
         Ray::new(&Vec3D::new_zero_vector(), &Vec3D::new_zero_vector()))
    }
}

fn scatter(material: &Material, ray: &Ray, p: &Vec3D, n: &Vec3D) -> (bool, Vec3D, Ray) {
    material.scatter(ray, p, n)
}