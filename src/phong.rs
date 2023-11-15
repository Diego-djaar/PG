use crate::camera::Camera;
use crate::image::Rgb;
use crate::image::RgbImage;
use crate::objetos;
use crate::objetos::Fontes;
use crate::objetos::Objetos;
use crate::point::Point;
use crate::vec::Vector;

use image;
use image::io::Reader as ImageReader;
use image::ImageBuffer;
use std::io::Cursor;

/// Faz a renderização do modelo de Phong
pub fn iluminar(
    mut img: ImageBuffer<Rgb<u8>, Vec<u8>>,
    camera: Camera,
    objetos: Vec<Objetos>,
    fontes: Vec<Fontes>,
    centro_tela: Point,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let w = img.width();
    let h = img.height();
    let base = camera.base_ortonormal_wvu;
    let camera_pos = camera.position_c;

    // Primeiro quadrante
    // Calcular ponto do pixel
    let canto = centro_tela - base.1 - base.2
        + base.1 / (Into::<f64>::into(w))
        + base.2 / (Into::<f64>::into(h));
    let mini_vec_hor = base.1 / Into::<f64>::into(w);
    let mini_vec_vert = base.2 / Into::<f64>::into(h);
    for x in 0..w {
        for y in 0..h {
            let pixel_atual =
                canto + mini_vec_hor * x.into() * 2.0 + mini_vec_vert * y.into() * 2.0;
            let luz = iluminar_pixel(pixel_atual, camera_pos, &objetos, &fontes);
            img.put_pixel(x, y, luz);
        }
    }
    return img;
}

/// Faz o algorítimo em um pixel específico
pub fn iluminar_pixel(
    pixel: Point,
    centro: Point,
    objetos: &Vec<Objetos>,
    fontes: &Vec<Fontes>,
) -> Rgb<u8> {
    for objeto in objetos {
        let cor: Option<Rgb<u8>> = match objeto {
            Objetos::Esfera(esfera) => esfera_interseccao_delta(pixel, centro, esfera),
            Objetos::Plano(_) => todo!(),
        };
        if cor.is_some() {
            return cor.unwrap();
        }
    }
    return Rgb([0, 0, 0]);
}

fn esfera_interseccao_delta(
    pixel: Point,
    centro: Point,
    esfera: &objetos::esfera::Esfera,
) -> Option<Rgb<u8>> {
    // Detectar se a semirreta do centro até o pixel, depois do pixel intersecta a esfera
    let vec = pixel - centro;
    // centro + v*t ~= esfera.centro; Logo:

    // Honestamente, isso tá uma merda
    // Vou fazer do outro jeito
    let e = esfera.centro.0.get_x();
    let x = centro.0.get_x();
    let v = vec.get_x();
    let y = centro.0.get_y();
    let u = vec.get_y();
    let f = esfera.centro.0.get_y();
    let z = centro.0.get_z();
    let w = vec.get_z();
    let g = esfera.centro.0.get_z();
    let r = esfera.raio;

    let delta = 4.0 * (e * v + f * u + g * w - u * y - v * x - w * z).powi(2)
        - 4.0
            * (u.powi(2) + v.powi(2) + w.powi(2))
            * (e.powi(2) - 2.0 * e * x + f.powi(2) - 2.0 * f * y + g.powi(2)
                - 2.0 * g * z
                - r.powi(2)
                + x.powi(2)
                + y.powi(2)
                + z.powi(2));

    match delta {
        d if d > 0.0 => Some(Rgb(esfera.rgb)),
        d if d <= 0.0 => None,
        _ => unreachable!(),
    }
}
