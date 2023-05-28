#![warn(clippy::pedantic)]
use feoray::{
    core::{point, vector, Camera, Colour, Transform, TransformBuilder, World},
    lights::PointLight,
    materials::{Material, Pattern},
    primitives::Object
};
use nalgebra::Matrix4;
use std::f64::consts::PI;

fn main() {
    let floor_pat = Pattern::new_radial(Colour::white(), Colour::blue(), 12);
    let floor_mat = Material::default()
        .with_colour(Colour::new(1.0, 0.9, 0.9))
        .with_specular(0.0)
        .with_reflectivity(0.6)
        .with_pattern(floor_pat);
    let floor = Object::new_plane()
        .with_material(floor_mat);

    let mid_trn = TransformBuilder::new()
        .rot_z(PI / 4.0)
        .translate(-0.5, 1.0, 0.5)
        .build();
    let mid_pat_trn = TransformBuilder::new()
        .uscale(0.25)
        .build();
    let mid_pat = Pattern::new_checkers(Colour::red(), Colour::white())
        .with_transform(mid_pat_trn);
    let mid_mat = Material::default()
        .with_diffuse(0.7)
        .with_specular(0.3)
        .with_reflectivity(0.08)
        .with_pattern(mid_pat);
    let mid = Object::new_sphere()
        .with_transform(mid_trn)
        .with_material(mid_mat)
        .use_manifold();

    let right_trn = TransformBuilder::new()
        .uscale(0.5)
        .translate(1.5, 0.5, -0.5)
        .build();
    let right = Object::glass_orb()
        .with_transform(right_trn);

    let left_trn = TransformBuilder::new()
        .uscale(0.33)
        .translate(-1.5, 0.33, -0.75)
        .build();
    let left_mat = Material::null()
        .with_colour(Colour::grey(192.0/255.0))
        .with_reflectivity(0.95)
        .with_specular(0.9);
    let left = Object::new_sphere()
        .with_transform(left_trn)
        .with_material(left_mat)
        .cast_no_shadow();

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

    canvas.export("test_scene_0005.jpg").unwrap();
}