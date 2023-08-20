use crate::ray::Ray;
use crate::vec3::*;
use crate::vec3::{dot, Vec3};

struct HitRecord {
    p: Vec3,
    normal: Vec3,
    t: f64,
    front_face: bool
}

impl HitRecord {
    pub fn new(p:Vec3,normal:Vec3,t:f64,front_face:bool)->HitRecord {
        HitRecord {
            p,
            normal,
            t,
            front_face
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = dot(r.get_direction(), &outward_normal) < 0.0;
        let temp  = if self.front_face { outward_normal } else { -outward_normal };
        self.normal = temp
    }

    pub fn copy_from_rec(&mut self, other: HitRecord) {
        self.set_normal(other.normal);
        self.set_t(other.t);
        self.set_p(other.p);
        self.front_face = other.front_face;
    }

    pub fn set_t(&mut self, value: f64) {
        self.t = value
    }

    pub fn set_normal(&mut self, value: Vec3) {
        self.normal = value;
    }

    pub fn set_p(&mut self, value: Vec3) {
        self.p = value;
    }

    pub fn get_normal(&self) -> &Vec3 {
        &self.normal
    }

    pub fn get_p(&self) -> &Vec3 {
        &self.p
    }

    pub fn get_t(&self)->f64 {
        self.t
    }
}

trait Hittable {
    /*
    Sphere是否有交集
     */
    fn hit(&self,r: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool;
}

struct Sphere {
    cen:Vec3,
    r:f64
}

impl Sphere{
    pub fn get_center(&self) ->&Vec3 {
        &self.cen
    }

    pub fn get_radius(&self)->f64 {
        self.r
    }

    fn set_record(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord, temp: f64) -> Option<bool> {
        if temp < t_max && temp > t_min {
            rec.set_t(temp);
            rec.set_p(r.at(temp));
            rec.set_face_normal(r, ((rec.get_p() - self.get_center()) / self.get_radius()).unwrap());
            return Some(true);
        }
        None
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.get_origin() - self.get_center();
        let a = r.get_direction().length_squared();
        let half_b = dot(&oc, r.get_direction());
        let c = oc.length_squared() - self.get_radius() * self.get_radius();
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let temp = (-half_b - root) / a;
            if let Some(value) = self.set_record(r, t_min, t_max, rec, temp) {
                return value;
            }

            let temp = (-half_b + root) / a;
            if let Some(value) = self.set_record(r, t_min, t_max, rec, temp) {
                return value
            }
        }

        return false;
    }
}

struct HittableList {
    objects: Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn add(&mut self,object:Box<dyn Hittable>) {
        self.objects.push(object)
    }

    pub fn get_objects(&self) -> &Vec<Box<dyn Hittable>> {
        &self.objects
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut  closest_so_far = t_max;

        for object in self.get_objects().into_iter() {
            let mut temp_rec = HitRecord::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                0.0,
                false,
            );
            if object.hit(r,t_min,closest_so_far,&mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.get_t();
                rec.copy_from_rec(temp_rec);
            }
        }

        return hit_anything;
    }
}