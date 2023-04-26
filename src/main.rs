#![warn(clippy::pedantic)]
use feoray::*;

fn main() {
    let mut cnvs = canvas(600, 600);
    let mut s = Object::new_sphere();
    let mat = Material::default().with_colour(Colour::new(1.0, 0.2, 1.0));
    s.material = mat;
    let wall_z = 10.0;
    let wall_size = 7.0;
    let ray_origin = point(0.0, 0.0, -5.0);
    let pixel_size = wall_size / cnvs.width as f64;
    let wall_half = wall_size / 2.0;
    let light = PointLight::new(Colour::white(), point(-10.0, 10.0, -10.0));

    for y in 0..cnvs.height {
        let world_y = wall_half - pixel_size * y as f64;
        for x in 0..cnvs.width {
            let world_x = -wall_half + pixel_size * x as f64;
            let position = point(world_x, world_y, wall_z);
            let r = Ray::new(
                ray_origin,
                (position - ray_origin).normalize()
            );
            let xs = s.intersect(r);
            // Well, this works, butit really ought to be using hit()
            if xs.len() > 0 {
                let hit = xs.hit().unwrap();
                let p = r.position(hit.t);
                let n = hit.object.normal_at(p);
                let eyev = -r.direction;
                let clr = hit.object.material.lighting(light, p, eyev, n);
                cnvs.write_pix(x, y, clr);
            }
        }
    }

    cnvs.export("first_light.jpg").unwrap();
}