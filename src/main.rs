#![warn(clippy::pedantic)]
use feoray::*;
use std::f64::consts::PI;

fn main() {
    let mut cnvs = canvas(80, 80);

    for i in 0..12 {
        let transform: Matrix = Matrix::translate(40.0, 0.0, 40.0)
            * Matrix::rot_y((PI * i as f64) / 6.0)
            * Matrix::scale(0.0, 0.0, 30.0);
        let p = transform * point(0.0, 0.0, 1.0);
        if p.x < cnvs.width as f64 && p.z < cnvs.height as f64 {
            cnvs.write_pix(p.x as usize, cnvs.height - p.z as usize - 1, Colour::red());
        }
    }

    cnvs.export("clock.jpg").unwrap();
}