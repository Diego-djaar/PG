"""Ponto de entrada principal"""
import numpy as np
import cv2 as cv
import camera
import phong

vec1 = np.array([1,2,3])
vec2 = np.array([4,5,6])



camera_cena = camera.Camera(
    np.array([0,0,0]),
    np.array([1,0,0]),
    100.0,
    np.array([0,1,0]),
    1000,
    1000
)

phong.renderizar_imagem(camera,None,None)

print(vec1+vec2)
print(camera_cena.base_ortonormal_wvu)
