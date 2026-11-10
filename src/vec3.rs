use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

use rand::{random, random_range};

#[derive(Copy, Clone, Default, PartialEq, Debug)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }

    pub fn get(&self, n: usize) -> f64 {
        self.e[n]
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

    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2] 
    }

    pub fn near_zero(&self) -> bool {
        const EPSILON: f64 = 1e-8;
        self.e[0].abs() < EPSILON && self.e[1].abs() < EPSILON && self.e[2].abs() < EPSILON
    }

    // Returns the vector to a random point in the [-.5, -.5]-[+.5,+.5] unit square.
    pub fn sample_square() -> Vec3 {
        Vec3::new(
            random::<f64>() - 0.5, 
            random::<f64>() - 0.5, 
            0.0
        )
    }

    pub fn random() -> Vec3 {
        Vec3::new(random(), random(), random())
    }

    pub fn random_range(min: f64, max: f64) -> Vec3 {
        Vec3::new(
            random_range(min..max),
            random_range(min..max),
            random_range(min..max)
        )
    }

    pub fn random_unit_vector() -> Vec3 {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            if 1e-160 < p.length_squared() && p.length_squared() < 1.0 {
                return p / p.length();
            }
        }
    }

    pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
        let on_unit_sphere = Vec3::random_unit_vector();
        if dot(&on_unit_sphere, normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    pub fn random_in_unit_disc() -> Vec3 {
        loop {
            let p = Vec3::new(random_range(-1.0..1.0), random_range(-1.0..1.0), 0.0);
            if p.length_squared() < 1.0 {
                return p
            }
        }
    }

    pub fn zero() -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }
}

pub type Point3 = Vec3;

// Output formatting
impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} {} {}", self.x(), self.y(), self.z())
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self * rhs.x(), self * rhs.y(), self * rhs.z())
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        rhs * self
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x() / rhs, self.y() / rhs, self.z() / rhs)
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u.x() * v.x() + u.y() * v.y() + u.z() * v.z()
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3::new(
        u.y() * v.z() - u.z() * v.y(), 
        u.z() * v.x() - u.x() * v.z(), 
        u.x() * v.y() - u.y() * v.x()
    )
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * dot(&v, &n) * n
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = f64::min(dot(&-uv, &n), 1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * n;
    r_out_perp + r_out_parallel
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_eq() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v1, v2)
    }

    #[test]
    fn vec_display() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);

        assert_eq!("1 2 3", v1.to_string())
    }


    #[test]
    fn vec_add() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(1.0, 2.0, 3.0);

        let v_expected = Vec3::new(2.0, 4.0, 6.0);
        assert_eq!(v_expected, v1 + v2)
    }

    #[test]
    fn vec_add_assign() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(1.0, 2.0, 3.0);

        v1 += v2;

        let v_expected = Vec3::new(2.0, 4.0, 6.0);
        assert_eq!(v_expected, v1)
    }

    #[test]
    fn vec_sub() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(1.0, 2.0, 3.0);

        let v_expected = Vec3::new(0.0, 0.0, 0.0);
        assert_eq!(v_expected, v1 - v2)
    }

    #[test]
    fn vec_mult_vec() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(1.0, 2.0, 3.0);

        let v_expected = Vec3::new(1.0, 4.0, 9.0);
        assert_eq!(v_expected, v1 * v2)
    }

    #[test]
    fn vec_mult_f64() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);

        let v_expected = Vec3::new(3.0, 6.0, 9.0);
        assert_eq!(v_expected, v1 * 3.0)
    }

    #[test]
    fn vec_mult_assign_f64() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);

        v1 *= 3.0;

        let v_expected = Vec3::new(3.0, 6.0, 9.0);
        assert_eq!(v_expected, v1)
    }

    #[test]
    fn f64_mult_vec() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);

        let v_expected = Vec3::new(3.0, 6.0, 9.0);
        assert_eq!(v_expected, 3.0 * v1)
    }

    #[test]
    fn vec_div() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);

        let v_expected = Vec3::new(0.5, 1.0, 1.5);
        assert_eq!(v_expected, v1 / 2.0)
    }

    #[test]
    fn vec_div_assign() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);

        v1 /= 2.0;

        let v_expected = Vec3::new(0.5, 1.0, 1.5);
        assert_eq!(v_expected, v1)
    }

    #[test]
    fn vec_neg() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);

        let v_expected = Vec3::new(-1.0, -2.0, -3.0);
        assert_eq!(v_expected, -v1)
    }

    #[test]
    fn dot_prod() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(14.0, dot(&v1,  &v2))
    }

    #[test]
    fn cross_prod() {
        let v1 = Vec3::new(2.0, 3.0, 4.0);
        let v2 = Vec3::new(5.0, 6.0, 7.0);

        let v_expected = Vec3::new(-3.0, 6.0, -3.0);
        assert_eq!(v_expected, cross(&v1, &v2))
    }

    #[test]
    fn unit_vec() {
        let v = Vec3::new(5.0, 0.0, 0.0);

        let v_expected = Vec3::new(1.0, 0.0, 0.0);
        assert_eq!(v_expected, unit_vector(v))
    }
}
