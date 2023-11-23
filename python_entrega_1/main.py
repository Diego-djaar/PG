"""Ponto de entrada principal"""
import numpy as np
import cv2 as cv
import camera
import phong

vec1 = np.array([1,2,3])
vec2 = np.array([4,5,6])

img = np.zeros((1000,1000,3), np.uint8)
for i in range(300,600):
    for j  in range(300,900):
        img[i][j] = (255,255,255)
cv.imshow("A New Image", img)
cv.waitKey(0)

camera_cena = camera.Camera(
    np.array([0,0,0]),
    np.array([1,0,0]),
    100.0,
    np.array([0,1,0]),
    1000,
    1000
)

print(vec1+vec2)
print(camera_cena)
