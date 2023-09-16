use crate::common::{ff_max, ff_min, Interval};
use crate::ray::Ray;
use crate::vec3::Point;

#[derive(Default)]
struct AABB {
    x: Interval,
    y: Interval,
    z: Interval
}

impl AABB {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        AABB {
            x,
            y,
            z
        }
    }

    pub fn new_with_bounding_box(a: Point, b: Point) -> Self {
        // Treat the two points a and b as extrema for the bounding box, so we don't require a
        // particular minimum/maximum coordinate order.

        AABB {
            x: Interval::new(ff_min(a.x(), b.x()), ff_max(a.x(), b.x())),
            y: Interval::new(ff_min(a.y(), b.y()), ff_max(a.y(), b.y())),
            z: Interval::new(ff_min(a.z() ,b.z()), ff_max(a.z() ,b.z()))
        }
    }

    pub fn axis(&self, n: i32) -> &Interval {
        if n == 1 { return &self.y }
        if n == 2 { return &self.z }
        &self.x
    }

    pub fn hit_ori(&self, r: &Ray, ray_t:& mut Interval) -> bool {
        for a in 0..3 {
            let t0 = ff_min((self.axis(a).min - r.original().get(a as usize).unwrap()) / r.direction().get(a as usize).unwrap(),
                            (self.axis(a).max - r.original().get(a as usize).unwrap()) / r.direction().get(a as usize).unwrap());
            let t1 = ff_max(self.axis(a).min - r.original().get(a as usize).unwrap() / r.direction().get(a as usize).unwrap(),
                            (self.axis(a).max - r.original().get(a as usize).unwrap()) / r.direction().get(a as usize).unwrap());
            ray_t.min = ff_max(t0, ray_t.min);
            ray_t.max = ff_min(t1, ray_t.max);
            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        true
    }

    pub fn hit(&self, r: &Ray, ray_t:& mut Interval) -> bool {
        for a in 0..3 {
            let invD = 1.0 / r.direction().get(a as usize).unwrap();
            let orig = r.original().get(a as usize).unwrap();

            let mut t0 = (self.axis(a).min - orig) * invD;
            let mut t1 = (self.axis(a).max - orig) * invD;

            if invD < 0.0 {
                let tmp = t0;
                t0 = t1;
                t1 = tmp;
            }

            if t0 > ray_t.min { ray_t.min = t0; }
            if t1 < ray_t.max { ray_t.max = t1; }

            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        true
    }
}