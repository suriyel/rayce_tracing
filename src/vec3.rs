use std::ops::{Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::io::{self, Write};

#[derive(Debug)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {
            e: [x, y, z]
        }
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

    pub fn write_color<W: Write>(&self, stream: &mut W) -> io::Result<()> {
        let num = 255.999;
        let msg = format!("{} {} {}\n",
                          (num * self.e[0]) as i32,
                          (num * self.e[1]) as i32,
                          (num * self.e[2]) as i32);
        stream.write_all(msg.as_bytes())?;
        Ok(())
    }
}
impl<'a,'b> Add<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn add(self, other: &'b Vec3) -> Self::Output {
        Vec3 {
            e: [
                self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2]
            ]
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Self::Output {
        &self + &other
    }
}

impl<'a> Neg for &'a Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            e: [-self.e[0], -self.e[1], -self.e[2]]
        }
    }
}
impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        -&self
    }
}

impl<'a,'b> Sub<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn sub(self, other: &'b Vec3) -> Self::Output {
        Vec3{
            e:[
                self.e[0] - other.e[0],
                self.e[1] - other.e[1],
                self.e[2] - other.e[2]
            ]
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Self::Output {
        &self - &other
    }
}

impl<'a> Div<f64> for &'a Vec3 {
    type Output = Result<Vec3,&'static str>;

    fn div(self, other: f64) -> Self::Output {
        if other ==0.0 {
            return Err("Division by zero is not allowed")
        }

        Ok(
            Vec3 {
                e: [
                    self.e[0] / other,
                    self.e[1] / other,
                    self.e[2] / other
                ]
            }
        )
    }
}

impl Div<f64> for Vec3 {
    type Output = Result<Vec3,&'static str>;

    fn div(self, other: f64) -> Self::Output {
        &self / other
    }
}

impl<'a> Mul<f64> for &'a Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Self::Output {
        Vec3{
            e:[
                self.e[0] * other,
                self.e[1] * other,
                self.e[2] * other
            ]
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Self::Output {
        &self * other
    }
}

impl<'a,'b> Mul<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn mul(self, other:&'b Vec3) -> Self::Output {
        Vec3 {
            e: [
                self.e[0] * other.e[0],
                self.e[1] * other.e[1],
                self.e[2] * other.e[2]
            ]
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Self) -> Self::Output {
        &self * &other
    }
}

impl<'a,'b> Div<&'b Vec3> for &'a Vec3 {
    type Output = Result<Vec3,&'static str>;

    fn div(self, other: &'b Vec3) -> Self::Output {
        let Vec3 { e: [x1, y1, z1] } = *self;
        let Vec3 { e: [x2, y2, z2] } = *other;

        if x2 == 0.0 || y2 == 0.0 || z2 == 0.0 {
            return Err("Division by zero is not allowed")
        }

        Ok(
            Vec3 {
                e: [
                    x1 / x2,
                    y1 / y2,
                    z1 / z2
                ]
            }
        )
    }
}

impl Div for Vec3 {
    type Output = Result<Vec3,&'static str>;

    fn div(self, other: Self) -> Self::Output {
        &self / &other
    }
}

impl<'a> AddAssign<&'a Vec3> for Vec3 {
    fn add_assign(&mut self, other: &'a Vec3) {
        let Vec3 { e } = self;
        let Vec3 { e:[x,y,z]} = *other;

        e[0] += x;
        e[1] += y;
        e[2] += z;
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        (*self) += &other;
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

impl<'a> SubAssign<&'a Vec3> for Vec3 {
    fn sub_assign(&mut self, other: &'a Vec3) {
        let Vec3 { e } = self;
        let Vec3 {e:[x,y,z]} = *other;

        e[0] -= x;
        e[1] -= y;
        e[2] -= z;
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        (*self) -= &other;
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