"""Define as operações do ray caster"""
import camera
import numpy as np
def renderizar_imagem(
    camera: camera.Camera,
    esferas,
    planos,
    centro
):
    img = np.zeros((camera.altura_tela, camera.largura_tela), np.uint8)
    