"""Define as operações do ray caster"""
import camera
import numpy as np
def renderizar_imagem(
    camera: camera.Camera,
    esferas,
    planos,
    centro: np.ndarray[float]
):
    largura = camera.largura_tela
    altura = camera.altura_tela
    base = camera.base_ortonormal_wvu
    camera_pos = camera.position_c
    
    img = np.zeros((altura, largura), np.uint8)
    
    # calcula a posição do pixel no canto da tela
    # parte do centro da tela para a largura na direção de V, a altura na direção de W e, então, centraliza no pixel
    # o pixel tem dimensões 2x2, então
    
    deslocamento_v = 2*base[1] / largura
    deslocamento_w = 2*base[2] / largura
    
    canto = centro - base[1] - base[2] + base[1]*deslocamento_v/2 + base[2]*deslocamento_w/2
    
    for i in range(0, largura):
        for j in range(0, altura):
            pixel_atual = canto + deslocamento_v*i + deslocamento_w*j
            luz = colorir_pixel(pixel_atual, camera_pos, esferas, planos)
            img[i][j] = luz
    
    return img

def colorir_pixel(pixel_atual: np.ndarray[float], centro: np.ndarray[float], esferas, planos) -> tuple[int, int, int]:
    for esfera in esferas:
        # testar intersecção, retornar a mais próxima e sua cor
        pass
    
    for plano in planos:
        # testar intersecção, retornar a mais próxima e sua cor
        pass
    
    #retornar a cor da intersecção mais próxima
    
    