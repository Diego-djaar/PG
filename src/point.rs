use crate::vec::Vector;
use std::ops::Add;
use std::ops::Sub;
/// Tanto pontos como vetores são os mesmos em sua representação
///
/// Apenas muda como usar nas fórmulas
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point(pub Vector);

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        assert!(x.is_finite());
        assert!(y.is_finite());
        assert!(z.is_finite());
        Point(Vector::new(x, y, z))
    }

    pub fn from_tuple(value: (f64, f64, f64)) -> Point {
        Self::new(value.0, value.1, value.2)
    }

    pub fn from_slice(value: [f64; 3]) -> Point {
        Self::new(value[0], value[1], value[2])
    }
}

impl Into<(f64, f64, f64)> for Point {
    fn into(self) -> (f64, f64, f64) {
        self.0.into()
    }
}

impl Eq for Point {}

impl Add<Vector> for Point {
    type Output = Point;
    fn add(self, rhs: Vector) -> Point {
        Point(self.0 + rhs)
    }
}

impl Sub<Vector> for Point {
    type Output = Point;
    fn sub(self, rhs: Vector) -> Point {
        Point(self.0 - rhs)
    }
}

impl Sub for Point {
    type Output = Vector;
    /// Aka encontra o vetor que representa a distância de dois pontos
    fn sub(self, rhs: Point) -> Vector {
        self.0 - rhs.0
    }
}
