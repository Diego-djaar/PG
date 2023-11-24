import numpy as np

class Esfera:
    centro: np.ndarray[float]
    raio: float
    rgb: tuple[int, int, int]
    
    def __init__(self,centro: tuple[float,float,float],raio: float, rgb: tuple[int,int,int]):
        self.centro = np.ndarray(shape=(3), buffer=np.array(centro))
        self.raio = raio
        self.rgb = rgb
    
class Plano:
    ponto: np.ndarray[float]
    vetor_normal: np.ndarray[float]
    rgb: tuple[int, int, int]
    
    def __init__(self,ponto: tuple[float,float,float],vetor_normal: tuple[float,float,float], rgb: tuple[int,int,int]):
        self.ponto = np.ndarray(shape=(3), buffer=np.array(ponto))
        self.vetor_normal = np.ndarray(shape=(3), buffer=np.array(vetor_normal))
        self.rgb = rgb