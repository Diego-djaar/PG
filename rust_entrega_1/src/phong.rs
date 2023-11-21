use crate::camera::Camera;
use crate::image::Rgb;
use crate::image::RgbImage;
use crate::objetos;
use crate::objetos::Fontes;
use crate::objetos::Objetos;
use crate::point::Point;
use crate::vec;
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
    let canto = centro_tela - base.1 * w.into() - base.2 * h.into() + base.1 + base.2;
    let mini_vec_hor = base.1;
    let mini_vec_vert = base.2;
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
    let mut max = None;
    for objeto in objetos {
        let cor_dist: Option<(Rgb<u8>, f64)> = match objeto {
            Objetos::Esfera(esfera) => esfera_interseccao(pixel, centro, esfera),
            Objetos::Plano(plano) => plano_interseccao(pixel, centro, plano),
        };
        max = match cor_dist {
            Some(cor_dist) => match max {
                None => Some(cor_dist),
                Some(max) if cor_dist.1 < max.1 => Some(cor_dist),
                _ => max,
            },
            None => max,
        };
    }
    return match max {
        Some(max) => max.0,
        None => Rgb([0, 0, 0]),
    };
}

fn plano_interseccao(
    pixel: Point,
    centro: Point,
    plano: &objetos::plano::Plano,
) -> Option<(Rgb<u8>, f64)> {
    let line_unit = (pixel - centro).normalize();
    if vec::is_near_zero(line_unit.dot_prod(plano.vetor_normal)) {
        // println!("alinhamento perfeito");
        return None;
    }
    let d = ((plano.ponto - centro).dot_prod(plano.vetor_normal))
        / (line_unit.dot_prod(plano.vetor_normal));
    if d > 0.0 {
        return Some((Rgb(plano.rgb), d.powi(2)));
    } else {
        return None;
    }
}

fn esfera_interseccao(
    pixel: Point,
    centro: Point,
    esfera: &objetos::esfera::Esfera,
) -> Option<(Rgb<u8>, f64)> {
    let k = esfera.centro - centro;
    let k_proj = k.projection(pixel - centro);
    let d = k_proj - k;
    let r2 = esfera.raio.powi(2);
    if d.mag_sqrd() >= r2 {
        return None;
    }
    let f_len = r2 - d.mag_sqrd();
    let f = (centro - pixel).normalize() * f_len.sqrt();
    let x = k_proj + f;
    return Some((Rgb(esfera.rgb), x.mag_sqrd()));
}
