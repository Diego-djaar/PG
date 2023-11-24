"""Ponto de entrada principal"""
import numpy as np
import cv2 as cv
import camera
import phong
import objetos as obj

esfera = obj.Esfera (
        (160.0, -10.0, 20),
        20.0,
        (255, 126, 255)
)

esfera2 = obj.Esfera (
        (160.0, 0.0, 0.0),
        20.0,
        (0, 126, 255)
)
esfera3 = obj.Esfera (
        (160.0, -15.0, 0.0),
        20.0,
        (255, 126, 126)
)

plano = obj.Plano (
        (0.0,-1.0,0.0),
        (0.0,1.0,0.0),
        (255,255,255)
)

camera_cena = camera.Camera(
    np.array([0,0,0]),
    np.array([1,0,0]),
    2.0,
    np.array([0,1,0]),
    1000,
    1000
)

print(camera_cena.base_ortonormal_wvu)

centro_tela = camera_cena.position_c - camera_cena.base_ortonormal_wvu[0] * camera_cena.distancia_camera_tela
img = phong.renderizar_imagem(
        camera_cena,
        [esfera,esfera2, esfera3],
        [plano], 
        centro_tela
)

cv.imshow("A New Image", img)
cv.waitKey(0)

