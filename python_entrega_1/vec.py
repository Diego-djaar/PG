"""Declara vetores e operações"""
from __future__ import annotations
import math

def is_near_zero(val: float) -> bool:
    """Retorna se um float é próximo de zero"""
    return val < 0.001 and val > -0.001


class Vector():
    """Representa um vetor de 3 dimensões"""
    x: float
    y: float
    z: float
    def __init__(self, x: float, y: float, z: float):
        """Inicializa um vetor"""
        self.x = x
        self.y = y
        self.z = z

    def dot_prod(self, other: Vector) -> float:
        """Calcula o produto interno"""
        return self.x * other.x + self.y * other.y + self.z * other.z

    def cross_prod(self, other: Vector) -> Vector:
        """Calcula o produto externo"""
        return Vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    def mag(self) -> float:
        """Calcula a magnitude do vetor"""
        return math.sqrt(self.x * self.x + self.y * self.y + self.z * self.z)

    def mag_sqrd(self) -> float:
        """Calcula a magnitude ao quadrado. Ou o produto vetorial do vetor com ele mesmo"""
        return self.x * self.x + self.y * self.y + self.z * self.z

    def normalize(self) -> Vector:
        """Normaliza o vetor, deixando-o com magnitude 1"""
        return self.div(self.mag())

    def get_x(self) -> float:
        """Retorna a componente X do vetor"""
        return self.x

    def get_y(self) -> float:
        """Retorna a componente Y do vetor"""
        return self.y

    def get_z(self) -> float:
        """Retorna a componente Z do vetor"""
        return self.z

    def create_ortonormal_basis(self, top: Vector) -> tuple[Vector, Vector, Vector]:
        """""Cria uma base ortonormal que contém esse vetor"""""
        vec1 = self.normalize()
        vec2 = (top.sub(top.projection(self))).normalize()
        vec3 = vec1.cross_prod(vec2).normalize()
        return (vec1,vec2,vec3)

    def projection(self, other: Vector) -> Vector:
        """Projeção de self sobre other"""
        return other.mul(self.dot_prod(other) / other.mag_sqrd())

    def add(self, rhs: Vector) -> Vector:
        """Adiciona dois vetores"""
        return Vector(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)

    def mul(self, rhs: float) -> Vector:
        """Multiplica vetor por escalar"""
        return Vector(self.x * rhs, self.y * rhs, self.z * rhs)

    def div(self, rhs: float) -> Vector:
        """Divide vetor por escalar"""
        return Vector(self.x / rhs, self.y / rhs, self.z / rhs)

    def sub(self, rhs: Vector) -> Vector:
        """Subtrai rhs de self"""
        return self.add(rhs.mul(-1.0))

def zero_vec() -> Vector:
    """Retorna o vetor nulo"""
    return Vector(0.0, 0.0, 0.0)
