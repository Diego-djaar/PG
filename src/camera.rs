use crate::point::Point;
use crate::vec::Vector;

static UP_VECTOR: Vector = Vector::new(0.0, 1.0, 0.0);
static ALTURA_TELA: i32 = 1080;
static LARGURA_TELA: i32 = 1920;

/// A camera do mundo
#[derive(Debug, Clone)]
pub struct Camera {
    position_c: Point,
    point_view_m: Point,
    base_ortonormal_wvu: (Vector, Vector, Vector),
    distancia_camera_tela: f64,
}

impl Camera {
    pub fn new(pos: Point, view_point: Point, dist: f64) -> Camera {
        let vec_w = pos - view_point; // Vetor convenção
        let base_ortonormal_wvu = vec_w.create_ortonormal_basis(); // Vetor convenção

        Camera {
            position_c: pos,
            point_view_m: view_point,
            base_ortonormal_wvu,
            distancia_camera_tela: dist,
        }
    }
}
