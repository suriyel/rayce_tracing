use crate::ray::Ray;
use crate::sphere::HitRecord;
use crate::vec3::{dot, Vec3};

pub trait Material {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool;
}

pub struct Lambertian {
    albedo: Vec3
}

impl Lambertian {
    pub fn new(albedo: Vec3)->Lambertian {
        Lambertian {
            albedo
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) ->bool {
        let scatter_direction = hit_record.get_normal() + Vec3::random_in_unit_sphere();
        attenuation.copy(self.albedo);
        scattered.copy(Ray::new(hit_record.get_p(), scatter_direction));
        true
    }
}

pub struct Metal {
    albedo: Vec3
}

impl Metal {
    pub fn new(albedo: Vec3) -> Metal {
        Metal {
            albedo
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let reflected = Vec3::reflect(r_in.get_direction().unit_vector(), hit_record.get_normal());
        scattered.copy(Ray::new(hit_record.get_p(), reflected));
        attenuation.copy(self.albedo);
        dot(scattered.get_direction(), hit_record.get_normal()) > 0.0
    }
}