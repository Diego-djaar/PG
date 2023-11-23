"""Testar criar um círculo"""
import numpy as np
import cv2 as cv

img = np.zeros((1000,1000,3), np.uint8)
for i in range(0,1000):
    for j  in range(0,1000):
        if (i-500)*(i-500) + (j-500)*(j-500) < 50000:
            img[i][j] = (60,255,255)
cv.imshow("A New Image", img)
cv.waitKey(0)
