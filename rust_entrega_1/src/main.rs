#![feature(const_float_classify)]
use camera::Camera;
use image::RgbImage;
use objetos::esfera::Esfera;
use objetos::plano::Plano;
use point::Point;
use vec::Vector;

use image;

mod camera;
mod objetos;
mod phong;
mod point;
mod vec;

fn main() {
    let h = 1080;
    let v = 1920;

    let camera = Camera::new(
        Point::new(0.0, 0.0, 0.0),
        Point::new(10.0, 0.0, 0.0),
        4000.0,
        Vector::new(0.0, 1.0, 0.0),
        h,
        v,
    );

    let esfera = Esfera {
        centro: Point::new(1600.0, 150.0, 0.0),
        raio: 200.0,
        rgb: [255, 126, 255],
    };

    let esfera2 = Esfera {
        centro: Point::new(1600.0, 0.0, 0.0),
        raio: 200.0,
        rgb: [0, 126, 255],
    };

    let esfera3 = Esfera {
        centro: Point::new(1700.0, 0.0, 200.0),
        raio: 300.0,
        rgb: [255, 126, 126],
    };

    let plano = Plano {
        ponto: Point::new(1600.0, 0.0, 0.0),
        vetor_normal: Vector::new(1.0, 1.0, 0.0),
        rgb: [255, 255, 255],
    };

    // Agora, testar intersecções câmera-pixel-esfera com base no algorítimo

    // Encontrar centro da tela
    let centro_tela =
        camera.position_c - camera.base_ortonormal_wvu.0 * camera.distancia_camera_tela;
    let img = RgbImage::new(v, h);

    println!("{:?}", camera);

    // iluminação de Phong
    let img_renderizada = phong::iluminar(
        img,
        camera,
        vec![
            objetos::Objetos::Esfera(esfera),
            objetos::Objetos::Esfera(esfera2),
            objetos::Objetos::Esfera(esfera3),
            objetos::Objetos::Plano(plano),
        ],
        vec![],
        centro_tela,
    );

    // Por fim, gerar a imagem 1080X1920
    img_renderizada.save("render.png").unwrap();
}

#[cfg(test)]
mod tests {
    use image::Rgb;

    use super::*;

    #[test]
    fn create_image() {
        // É útil imprimir a imagem de alguma forma, vou testar isso antes.
        let mut img = RgbImage::new(1920, 1080);
        for x in 0..1920 {
            for y in 0..540 {
                img.put_pixel(x, y, Rgb([255, 0, 0]));
            }
            for y in 541..1080 {
                img.put_pixel(x, y, Rgb([255, 255, 255]));
            }
        }
        img.save("teste.png").unwrap();
        // Ae deu certo
    }

    #[test]
    fn imagem_circulo() {
        let mut img = RgbImage::new(1920, 1080);
        for x in 0..1920 {
            for y in 0..1080 {
                let i = x as i32 - 500;
                let j = y as i32 - 500;
                if i * i + j * j < 40000 {
                    img.put_pixel(x, y, Rgb([126, 126, 126]));
                }
            }
        }
        img.save("circulo.png").unwrap();
    }
}
