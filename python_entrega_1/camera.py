"""Define a camera"""
from __future__ import annotations
import numpy as np

class Camera:
    """Define a camera"""
    position_c: np.ndarray[float]
    point_view_m: np.ndarray[float]
    base_ortonormal_wvu: tuple[np.ndarray[float],np.ndarray[float],np.ndarray[float]]
    # up_vector
    distancia_camera_tela: float
    altura_tela: int
    largura_tela: int

    def __init__(
        self: np.ndarray[float],
        pos: np.ndarray[float],
        view_point: np.ndarray[float],
        dist: float,
        vector_up,
        altura_tela: int,
        largura_tela: int,
    ) -> Camera:
        vec_w = (pos - view_point) # Vetor convenção
        vec_w = vec_w / np.linalg.norm(vec_w)

        # vec1 = vec_w.normalize()
        # vec2 = (up_vector - (up_vector.projection(vec1))).normalize()
        # vec3 = vec1.cross_prod(vec2).normalize()
        
        vec_u = np.cross(vector_up, vec_w)
        vec_v = np.cross(vec_w, vec_u)
        base_ortonormal_wvu = (vec_w,vec_u,vec_v) # Base ortonormal

        self.position_c = pos
        self.point_view_m = view_point
        self.base_ortonormal_wvu = base_ortonormal_wvu
        # self.up_vector = vec_v
        self.distancia_camera_tela = dist
        self.altura_tela = altura_tela
        self.largura_tela = largura_tela
