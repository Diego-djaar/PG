use crate::point::Point;
use crate::vec::Vector3;

static UP_VECTOR: Vector3 = Vector3::new(0.0, 1.0, 0.0);
static ALTURA_TELA: i32 = 1080;
static LARGURA_TELA: i32 = 1920;

/// A camera do mundo
#[derive(Debug, Clone)]
pub struct Camera {
    position_c: Point,
    point_view_m: Point,
    base_ortonormal_wvu: (Vector3, Vector3, Vector3),
    distancia_camera_tela: f64,
}

impl Camera {
    pub fn new(pos: Point, view_point: Point, dist: f64) -> Camera {
        let vec1 = (pos - view_point).normalize(); // Vetor convenção

        // Agora, é preciso encontrar mais 2 vetores para fazer a base ortonormal
        let vec2 = Vector3::new(vec1.get_y(), -vec1.get_x(), 0.0).normalize();
        let vec3 = vec1.cross_prod(vec2).normalize();

        Camera {
            position_c: pos,
            point_view_m: view_point,
            base_ortonormal_wvu: (vec1, vec2, vec3),
            distancia_camera_tela: dist,
        }
    }
}
