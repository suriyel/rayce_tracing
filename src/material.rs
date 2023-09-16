use crate::common::{ff_min, get_random_double, random_double};
use crate::ray::Ray;
use crate::sphere::HitRecord;
use crate::vec3::{Color, dot, Vec3};

pub trait Material {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool;
}

pub struct Lambertian {
    albedo: Color
}

impl Lambertian {
    pub fn new(albedo: Color)->Lambertian {
        Lambertian {
            albedo
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) ->bool {
        let scatter_direction = hit_record.get_normal() + Vec3::random_in_unit_sphere();
        attenuation.copy(self.albedo);
        scattered.copy(Ray::new(hit_record.get_p(), scatter_direction, r_in.get_time()));
        true
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f64
}

impl Metal {
    pub fn new(albedo: Vec3, f: f64) -> Metal {
        Metal {
            albedo,
            fuzz: if f < 1.0 { f } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let reflected = Vec3::reflect(r_in.direction().unit_vector(), hit_record.get_normal());
        scattered.copy(Ray::new(hit_record.get_p(), reflected + Vec3::random_in_unit_sphere() * self.fuzz, r_in.get_time()));
        attenuation.copy(self.albedo);
        dot(scattered.direction(), hit_record.get_normal()) > 0.0
    }
}

pub struct Dielectric {
    ri: f64
}

impl Dielectric {
    pub fn new(ri:f64)->Dielectric {
        Dielectric {
            ri
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        attenuation.copy(Vec3::new(1.0, 1.0, 1.0));
        let etai_over_etat = if hit_record.get_front_face() {
            1.0 / self.ri
        } else {
            self.ri
        };

        let unit_direction = r_in.direction().unit_vector();
        let cos_theta = ff_min(dot(-unit_direction, hit_record.get_normal()), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        if etai_over_etat * sin_theta > 1.0 {
            // 反射
            let reflected = Vec3::reflect(unit_direction, hit_record.get_normal());
            scattered.copy(Ray::new(hit_record.get_p(), reflected, r_in.get_time()));
            return true;
        }

        // Christophe Schlick
        let reflect_prob = Vec3::schlick(cos_theta,etai_over_etat);
        if get_random_double() < reflect_prob {
            let reflected = Vec3::reflect(unit_direction, hit_record.get_normal());
            scattered.copy(Ray::new(hit_record.get_p(), reflected, r_in.get_time()));
        }

        // 折射
        let refracted = Vec3::refract(unit_direction, hit_record.get_normal(), etai_over_etat);
        scattered.copy(Ray::new(hit_record.get_p(), refracted, r_in.get_time()));
        true
    }
}