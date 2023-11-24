"""Define as operações do ray caster"""
import camera
import numpy as np
import objetos as obj
import math
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
    
    img = np.zeros((altura, largura, 3), np.uint8)
    
    # calcula a posição do pixel no canto da tela
    # parte do centro da tela para a largura na direção de V, a altura na direção de U e, então, centraliza no pixel
    # o pixel tem dimensões 2x2, então
    
    deslocamento_horizontal = 2*base[1] / largura
    deslocamento_vertical = 2*base[2] / largura
    
    canto = centro - base[1] - base[2] + base[1]*deslocamento_horizontal/2 + base[2]*deslocamento_vertical/2
    
    for i in range(0, largura):
        for j in range(0, altura):
            pixel_atual = canto + deslocamento_horizontal*i + deslocamento_vertical*j
            luz = colorir_pixel(pixel_atual, camera_pos, esferas, planos)
            img[i][j] = np.array(luz)
    
    return img

def colorir_pixel(pixel_atual: np.ndarray[float], camera_centro: np.ndarray[float], esferas, planos) -> tuple[int, int, int]:
    (distancia_minima, cor_final) = (None, (0,0,0))
    
    for esfera in esferas:
        # testar intersecção, retornar a mais próxima e sua cor
        (dist, cor) = esfera_interseccao(pixel_atual, camera_centro, esfera)
        if dist == None:
            continue
        if distancia_minima == None or dist < distancia_minima:
            distancia_minima=dist
            cor_final = cor
            
    for plano in planos:
        # testar intersecção, retornar a mais próxima e sua cor
        (dist, cor) = plano_interseccao(pixel_atual,camera_centro,plano)
        if dist == None:
            continue
        if distancia_minima == None or dist < distancia_minima:
            distancia_minima=dist
            cor_final = cor
    
    #retornar a cor da intersecção mais próxima
    return cor_final

def projection(self, other: np.ndarray[float]) -> np.ndarray[float]:
        """Projeção de self sobre other"""
        return other*(np.dot(self,other) /  (np.linalg.norm(other)*np.linalg.norm(other)))
    
def is_near_zero(val):
    return val < 0.001 and val > -0.001;

def esfera_interseccao(pixel: np.ndarray[float], camera: np.ndarray[float], esfera:obj.Esfera) -> tuple[float,tuple[int,int,int]]:
    k = esfera.centro - camera
    k_proj = projection(k, pixel - camera)
    d = k_proj - k
    r = esfera.raio
    if np.linalg.norm(d) >= r: # Nesse caso, não há intersecção com a esfera
        return (None, None)
    
    f_len = math.sqrt(r*r - (np.linalg.norm(d)*np.linalg.norm(d)))
    f = (pixel - camera)/(np.linalg.norm(camera-pixel)) * f_len
    x = k_proj - f
    return (np.linalg.norm(x),esfera.rgb)

def plano_interseccao(pixel_atual: np.ndarray[float],centro: np.ndarray[float],plano:obj.Plano) -> tuple[float,tuple[int,int,int]] | None:
    line_unit = (pixel_atual - centro) / (np.linalg.norm(pixel_atual-centro))
    if is_near_zero(np.dot(line_unit, plano.vetor_normal)): # Plano degenerado
        return (None, None)
    d = np.dot((plano.ponto - centro),plano.vetor_normal) / np.dot(line_unit, plano.vetor_normal)
    if d > 0.0:
        return (d,plano.rgb)
    else:
        return (None,None)
    
    
    