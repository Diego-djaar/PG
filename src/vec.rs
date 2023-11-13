use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Matrix3x3 {
    a: Vector3,
    b: Vector3,
    c: Vector3,
}

impl Matrix3x3 {
    pub fn from_rows(
        first: (f64, f64, f64),
        second: (f64, f64, f64),
        third: (f64, f64, f64),
    ) -> Self {
        Matrix3x3 {
            a: Vector3::from_tuple(first),
            b: Vector3::from_tuple(second),
            c: Vector3::from_tuple(third),
        }
    }

    pub fn to_rows(self) -> ((f64, f64, f64), (f64, f64, f64), (f64, f64, f64)) {
        (self.a.into(), self.b.into(), self.c.into())
    }
}

/// 3 floats de 64 bits com algumas operações definidas, formando um espaço vetorial com produto escalar padrão.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Vector3 {
        assert!(x.is_finite());
        assert!(y.is_finite());
        assert!(z.is_finite());
        Vector3 { x, y, z }
    }

    pub fn from_tuple(value: (f64, f64, f64)) -> Vector3 {
        Self::new(value.0, value.1, value.2)
    }

    pub fn from_slice(value: [f64; 3]) -> Vector3 {
        Self::new(value[0], value[1], value[2])
    }

    pub fn dot_prod(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn ext_prod(self, other: Self) -> Matrix3x3 {
        todo!()
    }

    pub fn cross_prod(self, other: Self) -> Vector3 {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn mag(self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    pub fn normalize(self) -> Vector3 {
        self / self.mag()
    }

    pub fn get_x(&self) -> f64 {
        self.x
    }

    pub fn get_y(&self) -> f64 {
        self.y
    }

    pub fn get_z(&self) -> f64 {
        self.z
    }
}

impl Into<(f64, f64, f64)> for Vector3 {
    fn into(self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }
}

impl Eq for Vector3 {}

impl Add for Vector3 {
    type Output = Vector3;
    fn add(self, rhs: Self) -> Vector3 {
        Vector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f64) -> Vector3 {
        Vector3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Div<f64> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f64) -> Vector3 {
        Vector3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Self) -> Vector3 {
        self + rhs * (-1.0f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_dot_prod() {
        let vec_zero = Vector3::new(0.0, 0.0, 0.0);
        let vec1 = Vector3::new(1.0, 0.0, 0.0);
        let vec2 = Vector3::new(0.0, 2.0, 0.0);
        let vec3 = Vector3::new(3.0, 4.0, 1.0);
        let vec4 = Vector3::new(0.0, 2.0, 2.0);

        assert_eq!(vec1.dot_prod(vec2), 0.0);
        assert_eq!(vec2.dot_prod(vec_zero), 0.0);
        assert_eq!(vec3.dot_prod(vec4), 10.0);
        assert_eq!(vec4.dot_prod(vec2), 4.0);
        assert_eq!(vec1.dot_prod(vec3), 3.0);
        assert_eq!(vec4.dot_prod(vec3), 10.0);
    }

    #[test]
    fn basic_cross_prod() {
        let vec1 = Vector3::new(3.0, 4.0, 5.0);
        let vec2 = Vector3::new(6.0, 7.0, 8.0);
        assert_eq!(vec1.cross_prod(vec2), Vector3::new(-3.0, 6.0, -3.0));
        assert_eq!(vec2.cross_prod(vec1), Vector3::new(3.0, -6.0, 3.0));
    }

    #[test]
    fn scalar_ops() {
        let vec1 = Vector3::new(3.0, 4.0, 5.0);
        let vec2 = Vector3::new(6.0, 7.0, 8.0);
        assert_eq!(vec1 + vec2, Vector3::new(9.0, 11.0, 13.0));
        assert_eq!(vec1 - vec2, Vector3::new(-3.0, -3.0, -3.0));
        assert_eq!(vec1 * 2.5, Vector3::new(7.5, 10.0, 12.5));
        assert_eq!(
            vec1 * (-3.0) - vec2 * (-0.5),
            Vector3::new(-6.0, -8.5, -11.0)
        );
    }

    #[test]
    fn magnitude() {
        let vec1 = Vector3::new(3.0, 4.0, 12.0);
        assert_eq!(vec1.mag(), 13.0)
    }

    #[test]
    fn normalization() {
        let vec1 = Vector3::new(3.0, 4.0, 12.0);
        assert_eq!(
            vec1.normalize(),
            Vector3::new(3.0 / 13.0, 4.0 / 13.0, 12.0 / 13.0)
        )
    }
}
