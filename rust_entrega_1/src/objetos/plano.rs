use crate::point::Point;
use crate::vec::Vector;

pub struct Plano {
    pub ponto: Point,
    pub vetor_normal: Vector,
    pub rgb: [u8; 3],
}
