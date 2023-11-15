use crate::point::Point;
use crate::vec::Vector;

/// A camera do mundo
#[derive(Debug, Clone)]
pub struct Camera {
    pub position_c: Point,
    point_view_m: Point,
    pub base_ortonormal_wvu: (Vector, Vector, Vector),
    up_vector: Vector,
    pub distancia_camera_tela: f64,
    pub altura_tela: u32,
    pub largura_tela: u32,
}

impl Camera {
    pub fn new(
        pos: Point,
        view_point: Point,
        dist: f64,
        up_vector: Vector,
        altura_tela: u32,
        largura_tela: u32,
    ) -> Camera {
        let vec_w = pos - view_point; // Vetor convenção
        let base_ortonormal_wvu = vec_w.create_ortonormal_basis(); // Base ortonormal

        Camera {
            position_c: pos,
            point_view_m: view_point,
            base_ortonormal_wvu,
            up_vector,
            distancia_camera_tela: dist,
            altura_tela,
            largura_tela,
        }
    }
}
