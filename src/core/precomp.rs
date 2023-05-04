use crate::primitives::Object;
use nalgebra::Vector4;

#[derive(Debug)]
pub struct PreCompData {
    pub t: f64,
    pub object: Object,
    pub pos: Vector4<f64>,
    pub over_pos: Vector4<f64>,
    pub eyev: Vector4<f64>,
    pub normal: Vector4<f64>,
    pub inside: bool
}

impl PreCompData {
    pub fn new(
        t: f64,
        object: Object,
        pos: Vector4<f64>,
        over_pos: Vector4<f64>,
        eyev: Vector4<f64>,
        normal: Vector4<f64>,
        inside: bool
    ) -> Self {
        Self { t, object, pos, over_pos, eyev, normal, inside }
    }
}