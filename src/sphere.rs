use std::cell::RefCell;
use std::rc::Rc;
use crate::material::{Lambertian, Material};
use crate::ray::Ray;
use crate::vec3::*;
use crate::vec3::{dot, Vec3};

/*
表面材质信息
 */
pub struct HitRecord {
    // 入射点
    p: Vec3,
    // 入射点指向圆心的法向量
    normal: Vec3,
    // 材质引用
    material: Rc<dyn Material>,
    // 相交解(此处为圆和Ray二元一次 x0、x1)
    t: f64,
    // 入射角方向判断
    front_face: bool
}

impl HitRecord {
    pub fn new(p: Vec3, normal: Vec3, t: f64, front_face: bool, material: Rc<dyn Material>) -> HitRecord {
        HitRecord {
            p,
            normal,
            t,
            front_face,
            material,
        }
    }

    pub fn new_default()->HitRecord {
        HitRecord::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 0.0),
            0.0,
            false,
            Rc::new(Lambertian::new(Vec3::new(0.0, 0.0, 0.0)))
        )
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = dot(r.direction(), outward_normal) < 0.0;
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

    pub fn get_normal(&self) -> Vec3 {
        self.normal
    }

    pub fn get_p(&self) -> Vec3 {
        self.p
    }

    pub fn get_t(&self)->f64 {
        self.t
    }

    pub fn get_material(&self)-> &Rc<dyn Material> {
        &self.material
    }

    pub fn get_front_face(&self)->bool {
        self.front_face
    }
}

pub trait Hittable {
    /*
    Sphere是否有交集
     */
    fn hit(&self,r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

pub struct Sphere {
    cen: Point,
    cen_vec: Point,
    is_moving: bool,
    r: f64,
    material: Rc<dyn Material>,
}

impl Sphere{
    pub fn new_moving_sphere(center:Point,center2:Point,radius:f64,material:Rc<dyn Material>)->Sphere {
        Sphere {
            cen: center,
            cen_vec: center2 - center,
            is_moving: true,
            r: radius,
            material
        }
    }

    pub fn new(center:Point,radius:f64,material:Rc<dyn Material>)->Sphere {
        Sphere {
            cen: center,
            cen_vec: Point::default(),
            is_moving: false,
            r: radius,
            material
        }
    }

    pub fn get_center(&self) ->Point {
        self.cen
    }

    pub fn get_moving_center(&self,time:f64) -> Point {
        self.cen + self.cen_vec * time
    }

    pub fn get_radius(&self)->f64 {
        self.r
    }

    fn set_record(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord, temp: f64,center:Point) -> Option<bool> {
        if temp < t_max && temp > t_min {
            rec.set_t(temp);
            rec.set_p(r.at(temp));
            rec.set_face_normal(r, (rec.get_p() - center) / self.get_radius());
            rec.material = self.material.clone();
            return Some(true);
        }
        None
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let center = if self.is_moving { self.get_moving_center(r.get_time()) } else { self.cen };
        let oc = r.original() - center;
        let a = r.direction().length_squared();
        let half_b = dot(oc, r.direction());
        let c = oc.length_squared() - self.get_radius() * self.get_radius();
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            // 圆和射线交叠判定，二元一次方程两组解
            let temp = (-half_b - root) / a;
            if let Some(value) = self.set_record(r, t_min, t_max, rec, temp,center) {
                return value;
            }

            let temp = (-half_b + root) / a;
            if let Some(value) = self.set_record(r, t_min, t_max, rec, temp,center) {
                return value
            }
        }

        return false;
    }
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub fn new()->HittableList {
        HittableList {
            objects: Vec::new()
        }
    }

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
        let mut closest_so_far = t_max;

        for object in self.get_objects().into_iter() {
            if object.hit(r,t_min,closest_so_far,rec) {
                hit_anything = true;
                closest_so_far = rec.get_t();
            }
        }

        return hit_anything;
    }
}