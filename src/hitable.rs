use ray::Ray;
use vector::Vec3D;

pub trait Hitable {
	fn hit(&self,
		   ray: &Ray,
		   t_min: f64,
		   t_max: f64)
		   -> (bool, f64, Vec3D, Vec3D, bool, Vec3D, Ray);
}

pub struct Hitables<'a> {
	pub objects: &'a [&'a Hitable],
}

impl<'a> Hitable for Hitables<'a> {
	fn hit(&self,
		   ray: &Ray,
		   t_min: f64,
		   t_max: f64)
		   -> (bool, f64, Vec3D, Vec3D, bool, Vec3D, Ray) {
		let mut hit = false;
		let mut p = Vec3D::new_zero_vector();
		let mut n = Vec3D::new_zero_vector();
		let mut closest = t_max;
		let mut scatter_ok = false;
		let mut attenuation = Vec3D::new_zero_vector();
		let mut scattered = Ray::new(&Vec3D::new_zero_vector(), &Vec3D::new_zero_vector());
		for obj in self.objects.iter() {
			let (thishit, t, thisp, thisn, thisscatok, thisat, thissc) = obj.hit(ray,
																				 t_min,
																				 closest);
			if thishit {
				hit = true;
				closest = t;
				p = thisp;
				n = thisn;
				scatter_ok = thisscatok;
				attenuation = thisat;
				scattered = thissc;
			}
		}
		(hit, closest, p, n, scatter_ok, attenuation, scattered)
	}
}