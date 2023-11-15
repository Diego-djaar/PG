use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

/// Verifica se um valor é próximo o suficiente de zero
fn is_near_zero(val: f64) -> bool {
    return val < 0.001 && val > -0.001;
}

fn vec_near_null(val: Vector) -> bool {
    is_near_zero(val.get_x()) && is_near_zero(val.get_y()) && is_near_zero(val.get_z())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Matrix3x3 {
    a: Vector,
    b: Vector,
    c: Vector,
}

impl Matrix3x3 {
    pub fn from_rows(
        first: (f64, f64, f64),
        second: (f64, f64, f64),
        third: (f64, f64, f64),
    ) -> Self {
        Matrix3x3 {
            a: Vector::from_tuple(first),
            b: Vector::from_tuple(second),
            c: Vector::from_tuple(third),
        }
    }

    pub fn to_rows(self) -> ((f64, f64, f64), (f64, f64, f64), (f64, f64, f64)) {
        (self.a.into(), self.b.into(), self.c.into())
    }
}

/// 3 floats de 64 bits com algumas operações definidas, formando um espaço vetorial com produto escalar padrão.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector {
    pub const fn new(x: f64, y: f64, z: f64) -> Vector {
        assert!(x.is_finite());
        assert!(y.is_finite());
        assert!(z.is_finite());
        Vector { x, y, z }
    }

    pub fn from_tuple(value: (f64, f64, f64)) -> Vector {
        Self::new(value.0, value.1, value.2)
    }

    pub fn from_slice(value: [f64; 3]) -> Vector {
        Self::new(value[0], value[1], value[2])
    }

    pub fn dot_prod(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn ext_prod(self, other: Self) -> Matrix3x3 {
        todo!()
    }

    pub fn cross_prod(self, other: Self) -> Vector {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn mag(self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    pub fn mag_sqrd(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn normalize(self) -> Vector {
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

    pub fn zero() -> Vector {
        Vector::new(0.0, 0.0, 0.0)
    }

    /// Cria uma base ortonormal que contém esse vetor
    pub fn create_ortonormal_basis(self) -> (Vector, Vector, Vector) {
        let vec1 = self.normalize();

        // Agora, é preciso encontrar mais 2 vetores para fazer a base ortonormal
        let vec2 = match Vector::new(vec1.get_y(), -vec1.get_x(), 0.0) {
            x if x == Vector::zero() => Vector::new(0.0, vec1.get_z(), -vec1.get_y()),
            x => x,
        }
        .normalize();
        let vec3 = vec1.cross_prod(vec2).normalize();

        (vec1, vec2, vec3)
    }

    pub fn projection(self, other: Vector) -> Vector {
        other * self.dot_prod(other) / other.mag_sqrd()
    }
}

impl Into<(f64, f64, f64)> for Vector {
    fn into(self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }
}

impl Eq for Vector {}

impl Add for Vector {
    type Output = Vector;
    fn add(self, rhs: Self) -> Vector {
        Vector::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Vector {
        Vector::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Div<f64> for Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Vector {
        Vector::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Vector {
        self + rhs * (-1.0f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_dot_prod() {
        let vec_zero = Vector::new(0.0, 0.0, 0.0);
        let vec1 = Vector::new(1.0, 0.0, 0.0);
        let vec2 = Vector::new(0.0, 2.0, 0.0);
        let vec3 = Vector::new(3.0, 4.0, 1.0);
        let vec4 = Vector::new(0.0, 2.0, 2.0);

        assert_eq!(vec1.dot_prod(vec2), 0.0);
        assert_eq!(vec2.dot_prod(vec_zero), 0.0);
        assert_eq!(vec3.dot_prod(vec4), 10.0);
        assert_eq!(vec4.dot_prod(vec2), 4.0);
        assert_eq!(vec1.dot_prod(vec3), 3.0);
        assert_eq!(vec4.dot_prod(vec3), 10.0);
    }

    #[test]
    fn basic_cross_prod() {
        let vec1 = Vector::new(3.0, 4.0, 5.0);
        let vec2 = Vector::new(6.0, 7.0, 8.0);
        assert_eq!(vec1.cross_prod(vec2), Vector::new(-3.0, 6.0, -3.0));
        assert_eq!(vec2.cross_prod(vec1), Vector::new(3.0, -6.0, 3.0));
    }

    #[test]
    fn scalar_ops() {
        let vec1 = Vector::new(3.0, 4.0, 5.0);
        let vec2 = Vector::new(6.0, 7.0, 8.0);
        assert_eq!(vec1 + vec2, Vector::new(9.0, 11.0, 13.0));
        assert_eq!(vec1 - vec2, Vector::new(-3.0, -3.0, -3.0));
        assert_eq!(vec1 * 2.5, Vector::new(7.5, 10.0, 12.5));
        assert_eq!(
            vec1 * (-3.0) - vec2 * (-0.5),
            Vector::new(-6.0, -8.5, -11.0)
        );
    }

    #[test]
    fn magnitude() {
        let vec1 = Vector::new(3.0, 4.0, 12.0);
        assert_eq!(vec1.mag(), 13.0)
    }

    #[test]
    fn normalization() {
        let vec1 = Vector::new(3.0, 4.0, 12.0);
        assert_eq!(
            vec1.normalize(),
            Vector::new(3.0 / 13.0, 4.0 / 13.0, 12.0 / 13.0)
        )
    }

    #[test]
    fn test_ortonormality() {
        ortonormality(4.0, 5.0, 6.0);
        ortonormality(0.0, 1.0, 0.0);
        ortonormality(0.0, 0.0, 0.1);
        ortonormality(0.0, 0.0, 100000.0);
        ortonormality(27.0, 0.0, 00000.0);
    }
    fn ortonormality(x: f64, y: f64, z: f64) {
        // std::env::set_var("RUST_BACKTRACE", "1");
        let vecs_orto = Vector::new(x, y, z).create_ortonormal_basis();
        // println!("{:?}", vecs_orto);
        assert!(is_near_zero(vecs_orto.0.dot_prod(vecs_orto.1)));
        assert!(is_near_zero(vecs_orto.1.dot_prod(vecs_orto.2)));
        assert!(is_near_zero(vecs_orto.2.dot_prod(vecs_orto.0)));
    }

    #[test]
    fn projection() {
        let vec1 = Vector::new(1.0, 1.0, 1.0);
        let vec2 = Vector::new(1.0, 0.0, 0.0);
        let vec3 = Vector::new(137.0, -4.0, 666.0);
        let vec4 = Vector::new(420.0, 69.0, 3.14);
        assert!(vec1.projection(vec2) == vec2);
        assert!(vec2.projection(vec1) == Vector::new(1.0 / 3.0, 1.0 / 3.0, 1.0 / 3.0));
        assert!(vec_near_null(
            vec3.projection(vec4)
                - Vector::new(137.600499633551443, 22.605796368369166, 1.028727544879408)
        ));
    }
}
