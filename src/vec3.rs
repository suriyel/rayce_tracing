use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::io::{self, Write};
use crate::common::*;

#[derive(Debug, Copy, Clone,Default)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {
            e: [x, y, z]
        }
    }

    pub fn random() -> Vec3 {
        Vec3 {
            e: [get_random_double(),
                get_random_double(),
                get_random_double()]
        }
    }

    pub fn random_in_range(min: f64, max: f64) -> Vec3 {
        Vec3 {
            e: [
                random_double(min, max),
                random_double(min, max),
                random_double(min, max)
            ]
        }
    }

    pub fn copy(&mut self, other: Vec3) {
        self.e[0] = other.e[0];
        self.e[1] = other.e[1];
        self.e[2] = other.e[2];
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn get(&self,i:usize) -> Option<f64> {
        if i < self.e.len() {
            Some(self.e[i])
        } else {
            None
        }
    }

    pub fn length_squared(&self) -> f64 {
        let Vec3 { e } = self;
        e[0] * e[0] + e[1] * e[1] + e[2] * e[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn unit_vector(&self)->Vec3 {
        return *self / self.length();
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        // lambertian分布率系数应该为cosφ
        let a = random_double(0.0, PI * 2.0);
        let z = random_double(-1.0, 1.0);
        let r = (1.0 - z * z).sqrt();
        Vec3::new(r * a.cos(), r * a.sin(), z)
    }

    /*
    半球面反射
    */
    pub fn random_in_hemisphere(&self) -> Vec3 {
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        if dot(in_unit_sphere, *self) > 0.0 {
            return in_unit_sphere;
        }
        return -in_unit_sphere;
    }

    pub fn write_color<W: Write>(&self, stream: &mut W, samples_per_pixel: i32) -> io::Result<()> {
        let scala = 1.0 / f64::from(samples_per_pixel);
        // 进行gamma校正，采用最简单的开平方根方式
        let r = (scala * self.e[0]).sqrt();
        let g = (scala * self.e[1]).sqrt();
        let b = (scala * self.e[2]).sqrt();

        let msg = format!("{} {} {}\n",
                          (256.0 * clamp(r,0.0,0.999)) as i32,
                          (256.0 * clamp(g,0.0,0.999)) as i32,
                          (256.0 * clamp(b,0.0,0.999)) as i32);
        stream.write_all(msg.as_bytes())?;
        Ok(())
    }

    pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = dot(-uv, n);
        let r_out_parallel = (uv + n * cos_theta) * etai_over_etat;
        let r_out_perp = n * (-(1.0 - r_out_parallel.length_squared()).sqrt());
        return r_out_parallel + r_out_perp;
    }

    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        return v - n * (dot(v, n) * 2.0);
    }

    pub fn schlick(cosine:f64,ref_idx:f64)->f64 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Self::Output {
        Vec3 {
            e: [
                self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2]
            ]
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            e: [-self.e[0], -self.e[1], -self.e[2]]
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Self::Output {
        Vec3{
            e:[
                self.e[0] - other.e[0],
                self.e[1] - other.e[1],
                self.e[2] - other.e[2]
            ]
        }
    }
}

impl<T> Div<T> for Vec3
    where T: Into<f64> + Copy
{
    type Output = Vec3;

    fn div(self, other: T) -> Self::Output {
        let other_value = other.into();
        Vec3 {
            e: [
                self.e[0] / other_value,
                self.e[1] / other_value,
                self.e[2] / other_value
            ]
        }
    }
}

impl<T> Mul<T> for Vec3
    where T: Into<f64> + Copy{
    type Output = Vec3;

    fn mul(self, other: T) -> Self::Output {
        let other_value = other.into();
        Vec3 {
            e: [
                self.e[0] * other_value,
                self.e[1] * other_value,
                self.e[2] * other_value
            ]
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Self) -> Self::Output {
        Vec3 {
            e: [
                self.e[0] * other.e[0],
                self.e[1] * other.e[1],
                self.e[2] * other.e[2]
            ]
        }
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, other: Self) -> Self::Output {
        let Vec3 { e: [x1, y1, z1] } = self;
        let Vec3 { e: [x2, y2, z2] } = other;

        Vec3 {
            e: [
                x1 / x2,
                y1 / y2,
                z1 / z2
            ]
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        let Vec3 { e } = self;
        let Vec3 { e:[x,y,z]} = other;

        e[0] += x;
        e[1] += y;
        e[2] += z;
    }
}

impl AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, other: f64) {
        let Vec3 { e } = self;
        e[0] += other;
        e[1] += other;
        e[2] += other;
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        let Vec3 { e } = self;
        let Vec3 {e:[x,y,z]} = other;

        e[0] -= x;
        e[1] -= y;
        e[2] -= z;
    }
}

impl SubAssign<f64> for Vec3 {
    fn sub_assign(&mut self, other: f64) {
        let Vec3 { e } = self;
        e[0] -= other;
        e[1] -= other;
        e[2] -= other;
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        let Vec3 { e } = self;
        e[0] *= other.e[0];
        e[1] *= other.e[1];
        e[2] *= other.e[2];
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        let Vec3 { e } = self;
        e[0] *= other;
        e[1] *= other;
        e[2] *= other;
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, other: Self) {
        let Vec3 { e: [x, y, z] } = other;
        if x==0.0 || y == 0.0 || z == 0.0 {
            panic!("Division by zero is not allowed")
        }
        let Vec3 { e } = self;
        e[0] /= x;
        e[1] /= y;
        e[2] /= z;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        if other == 0.0 {
            panic!("Division by zero is not allowed")
        }
        let Vec3 { e } = self;
        e[0] /= other;
        e[1] /= other;
        e[2] /= other;
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        if self.x() == other.x() && self.y() == other.y() && self.z() == other.z() {
            return true
        }
        return  false
    }
}

pub fn dot(u:Vec3,v:Vec3)->f64 {
    let Vec3 { e: [x, y, z] } = u;
    let Vec3 { e: [x1, y1, z1] } = v;

    return x * x1 + y * y1 + z * z1;
}

pub fn cross(u:Vec3,v:Vec3) -> Vec3 {
    let Vec3 { e: [x, y, z] } = u;
    let Vec3 { e: [x1, y1, z1] } = v;

    return Vec3::new(
        y * z1 - z * y1,
        z * x1 - x * z1,
        x * y1 - y * x1);
}