use crate::common::random_double;
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

    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        return v - n * (dot(v, n) * 2.0);
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let reflected = Metal::reflect(r_in.get_direction().unit_vector(), hit_record.get_normal());
        scattered.copy(Ray::new(hit_record.get_p(), reflected + Vec3::random_in_unit_sphere() * self.fuzz));
        attenuation.copy(self.albedo);
        dot(scattered.get_direction(), hit_record.get_normal()) > 0.0
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

    pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = dot(-uv, n);
        let r_out_parallel = (uv + n * cos_theta) * etai_over_etat;
        let r_out_perp = n * (-(1.0 - r_out_parallel.length_squared()).sqrt());
        return r_out_parallel + r_out_perp;
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

        let unit_direction = r_in.get_direction().unit_vector();
        let refracted = Dielectric::refract(unit_direction, hit_record.get_normal(), etai_over_etat);
        scattered.copy(Ray::new(hit_record.get_p(), refracted));
        true
    }
}