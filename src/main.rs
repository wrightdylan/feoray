#![warn(clippy::pedantic)]
use feoray::*;
use nalgebra::Matrix4;
use std::f64::consts::PI;

fn main() {
    let floor_trn = Matrix4::nuscale(10.0, 0.01, 10.0);
    let floor_mat = Material::default()
        .with_colour(Colour::new(1.0, 0.9, 0.9))
        .with_specular(0.0);
    let floor = Object::new_sphere()
        .with_transform(floor_trn)
        .with_material(floor_mat);

    let left_wall_trn = TransformBuilder::new()
        .nuscale(10.0, 0.01, 10.0)
        .rot_x(PI / 2.0)
        .rot_y(-PI / 4.0)
        .translate(0.0, 0.0, 5.0)
        .build();
    let left_wall_mat = floor_mat.clone();
    let left_wall = Object::new_sphere()
        .with_transform(left_wall_trn)
        .with_material(left_wall_mat);

    let right_wall_trn = TransformBuilder::new()
        .nuscale(10.0, 0.01, 10.0)
        .rot_x(PI / 2.0)
        .rot_y(PI / 4.0)
        .translate(0.0, 0.0, 5.0)
        .build();
    let right_wall_mat = floor_mat.clone();
    let right_wall = Object::new_sphere()
        .with_transform(right_wall_trn)
        .with_material(right_wall_mat);

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

    let light1 = PointLight::new(Colour::grey(0.25), point(-11.0, 11.0, -10.0));
    let light2 = PointLight::new(Colour::grey(0.25), point(-9.0, 11.0, -10.0));
    let light3 = PointLight::new(Colour::grey(0.25), point(-11.0, 9.0, -10.0));
    let light4 = PointLight::new(Colour::grey(0.25), point(-9.0, 9.0, -10.0));

    let world = World::default()
        .with_light(light1)
        .with_light(light2)
        .with_light(light3)
        .with_light(light4)
        .with_object(floor)
        .with_object(left_wall)
        .with_object(right_wall)
        .with_object(mid)
        .with_object(left)
        .with_object(right);

    let from = point(0.0, 1.5, -5.0);
    let to = point(0.0, 1.0, 0.0);
    let up = vector(0.0, 1.0, 0.0);
    let cam = Camera::new(700, 350, PI / 3.0)
        .with_transform(Matrix4::view_transform(from, to, up));

    let canvas = cam.render(world);

    canvas.export("scene_shadows.jpg").unwrap();
}