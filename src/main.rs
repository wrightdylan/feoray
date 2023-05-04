#![warn(clippy::pedantic)]
use feoray::{
    core::{point, vector, Camera, Colour, Transform, TransformBuilder, World},
    lights::PointLight,
    materials::Material,
    primitives::Object
};
use nalgebra::Matrix4;
use std::f64::consts::PI;

fn main() {
    let floor_mat = Material::default()
        .with_colour(Colour::new(1.0, 0.9, 0.9))
        .with_specular(0.0);
    let floor = Object::new_plane()
        .with_material(floor_mat);

    let mid_trn = Matrix4::translate(-0.5, 1.0, 0.5);
    let mid_mat = Material::default()
        .with_colour(Colour::new(0.1, 1.0, 0.5))
        .with_diffuse(0.7)
        .with_specular(0.3);
    let mid = Object::new_sphere()
        .with_transform(mid_trn)
        .with_material(mid_mat);

    let right_trn = TransformBuilder::new()
        .uscale(0.5)
        .translate(1.5, 0.5, -0.5)
        .build();
    let right_mat = Material::default()
        .with_colour(Colour::new(0.5, 1.0, 0.1))
        .with_diffuse(0.7)
        .with_specular(0.3);
    let right = Object::new_sphere()
        .with_transform(right_trn)
        .with_material(right_mat);

    let left_trn = TransformBuilder::new()
        .uscale(0.33)
        .translate(-1.5, 0.33, -0.75)
        .build();
    let left_mat = Material::default()
        .with_colour(Colour::new(1.0, 0.8, 0.1))
        .with_diffuse(0.7)
        .with_specular(0.3);
    let left = Object::new_sphere()
        .with_transform(left_trn)
        .with_material(left_mat);

    let light_source = PointLight::new(Colour::white(), point(-10.0, 10.0, -10.0));

    let world = World::default()
        .with_light(light_source)
        .with_object(floor)
        .with_object(mid)
        .with_object(left)
        .with_object(right);

    let from = point(0.0, 1.5, -5.0);
    let to = point(0.0, 1.0, 0.0);
    let up = vector(0.0, 1.0, 0.0);
    let cam = Camera::new(700, 350, PI / 3.0)
        .with_transform(Matrix4::view_transform(from, to, up));

    let canvas = cam.render(world);

    canvas.export("test_scene_0003.jpg").unwrap();
}