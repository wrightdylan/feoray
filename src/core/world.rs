use crate::core::{point, Colour, Intersections, PreCompData, Ray, Transform};
use crate::materials::Material;
use crate::primitives::Object;
use crate::lights::PointLight;
use nalgebra::{Matrix4, Vector4};

#[derive(Debug, PartialEq)]
pub struct World {
    pub objects: Vec<Object>,
    pub lights: Vec<PointLight>,
    pub rcrs_lim: u8
}

impl World {
    /// NWO - New World Object.
    pub fn new(objects: Vec<Object>, lights: Vec<PointLight>, rcrs_lim: u8) -> Self {
        World { objects, lights, rcrs_lim }
    }

    /// Calculates the colour of a pixel.
    pub fn colour_at(&self, ray: &Ray, remaining: u8) -> Colour {
        let xs = self.intersect(ray);
        if xs.hit_index().is_some() {
            self.shade_hit(&xs.prepare_computations(xs.hit_index().unwrap(), ray), remaining)
        } else {
            Colour::black()
        }
    }

    /// Not the same as default(). This is only for testing.
    pub fn default_world() -> Self {
        let m = Material::default()
            .with_colour(Colour::new(0.8, 1.0, 0.6))
            .with_diffuse(0.7)
            .with_specular(0.2);
        let s1 = Object::new_sphere().with_material(m);
        let t = Matrix4::uscale(0.5);
        let s2 = Object::new_sphere().with_transform(t);
        World {
            objects: vec![s1, s2],
            lights: vec![PointLight::new(Colour::white(), point(-10.0, 10.0, -10.0))],
            ..Default::default()
            
        }
    }

    /// Intersections of rays and world objects rather than individual objects.
    pub fn intersect(&self, ray: &Ray) -> Intersections {
        let mut intersections = vec![];
        for o in self.objects.iter() {
            intersections.extend(o.intersect(ray).intrsc);
        }

        intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());

        Intersections { intrsc: intersections }
    }

    /// Determines if the point is occulted. Must be calculated for each light source.
    pub fn is_shadowed(&self, light_pos: Vector4<f64>, point: Vector4<f64>) -> bool {
        let v = light_pos - point;
        let distance = v.magnitude();
        let direction = v.normalize();
        let ray = Ray::new(point, direction);
        let intersections = self.intersect(&ray);
        let h = intersections.hit();
        if h != None && h.unwrap().t < distance && h.unwrap().object.umbra {
            true
        } else {
            false
        }
    }

    /// Calculates colour of hit. Support multiple lights right out of the box!
    pub fn shade_hit(&self, comps: &PreCompData, remaining: u8) -> Colour {
        let mut surface = Colour::black();
        let mut reflected = Colour::black();
        let mut refracted = Colour::black();
        for i in 0..self.lights.len() {
            surface += comps.object.material.lighting(
                comps.object,
                self.lights[i],
                comps.over_pos,
                comps.eye_vec,
                comps.normal_vec,
                self.is_shadowed(self.lights[i].position, comps.over_pos)
            );
            reflected += self.reflected_colour(comps, remaining);
            refracted += self.refracted_colour(comps, remaining);
        }

        if comps.object.material.reflectivity > 0.0 && comps.object.material.transparency > 0.0 {
            let reflectance = comps.schlick();

            surface + reflected * reflectance + refracted * (1.0 - reflectance)
        } else {
            surface + reflected + refracted
        }
    }

    /// Calculates colour of reflected light ray.
    pub fn reflected_colour(&self, comps: &PreCompData, remaining: u8) -> Colour {
        if remaining <= 0 || comps.object.material.reflectivity == 0.0 {
            Colour::black()
        } else {
            let ray = Ray::new(comps.over_pos, comps.reflect_vec);
            self.colour_at(&ray, remaining - 1) * comps.object.material.reflectivity
        }
    }

    /// Calculates colour of refracted light ray.
    pub fn refracted_colour(&self, comps: &PreCompData, remaining: u8) -> Colour {
        if remaining <= 0 || comps.object.material.transparency == 0.0 {
            Colour::black()
        } else {
            let n_ratio = (comps.n1 / comps.n2) as f64;
            let cos_i = comps.normal_vec.dot(&comps.eye_vec);
            let sin2_t = n_ratio.powi(2) * (1.0 - cos_i.powi(2));
            if sin2_t > 1.0 {
                Colour::black()
            } else {
                let cos_t = (1.0 - sin2_t).sqrt();
                let direction = comps.normal_vec * (n_ratio * cos_i - cos_t) - comps.eye_vec * n_ratio;
                let refracted_ray = Ray::new(comps.under_pos, direction);
                
                self.colour_at(&refracted_ray, remaining - 1) * comps.object.material.transparency
            }
        }
    }

    /// Applies a light to the world.
    pub fn with_light(mut self, light: PointLight) -> Self {
        self.lights.push(light);

        self
    }

    /// Adds an object to the world.
    pub fn with_object(mut self, object: Object) -> Self {
        self.objects.push(object);

        self
    }

    /// Adjust recursion limit from default (5).
    pub fn with_recursions(mut self, rcrs_lim: u8) -> Self {
        self.rcrs_lim = rcrs_lim;

        self
    }
}

impl Default for World {
    fn default() -> Self {
        World {
            objects: vec![],
            lights: vec![],
            rcrs_lim: 5
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{vector, Intersection};
    use crate::materials::Pattern;

    #[test]
    fn creating_a_world() {
        let w = World::new( vec![], vec![], 0);

        assert_eq!(w.objects.len(), 0);
        assert_eq!(w.lights.len(), 0);
    }

    #[test]
    fn the_default_world() {
        let w = World::default_world();
        let l = PointLight::new(Colour::white(), point(-10.0, 10.0, -10.0));
        let m = Material::default()
            .with_colour(Colour::new(0.8, 1.0, 0.6))
            .with_diffuse(0.7)
            .with_specular(0.2);
        let s1 = Object::new_sphere().with_material(m);
        let t = Matrix4::uscale(0.5);
        let s2 = Object::new_sphere().with_transform(t);

        assert_eq!(w.lights[0], l);
        assert_eq!(w.objects[0], s1);
        assert_eq!(w.objects[1], s2);
    }

    #[test]
    fn intersect_world_with_ray() {
        let w = World::default_world();
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let xs = w.intersect(&r);

        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 4.5);
        assert_eq!(xs[2].t, 5.5);
        assert_eq!(xs[3].t, 6.0);
    }

    #[test]
    fn shading_intersection() {
        let w = World::default_world();
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = w.objects[0];
        let int = Intersection::new(4.0, s);
        let ints = Intersections::new(vec![int]);
        let comps = ints.prepare_computations(0, &r);
        let clr = w.shade_hit(&comps, 1);

        assert_eq!(clr.to_5dp(), Colour::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shading_intersection_from_inside() {
        let mut w = World::default_world();
        w.lights[0] = PointLight::new(Colour::new(1.0, 1.0, 1.0), point(0.0, 0.25, 0.0));
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let s = w.objects[1];
        let int = Intersection::new(0.5, s);
        let ints = Intersections::new(vec![int]);
        let comps = ints.prepare_computations(0, &r);
        let clr = w.shade_hit(&comps, 1);

        assert_eq!(clr.to_5dp(), Colour::new(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn colour_when_ray_misses() {
        let w = World::default_world();
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 1.0, 0.0));
        let clr = w.colour_at(&r, 1);

        assert_eq!(clr, Colour::black());
    }

    #[test]
    fn colour_when_ray_hits() {
        let w = World::default_world();
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let clr = w.colour_at(&r, 1);

        assert_eq!(clr.to_5dp(), Colour::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn colour_with_intersection_behind_ray() {
        let mut w: World = World::default_world();
        let mut inner = w.objects[1];
        inner.material.ambient = 1.0;
        w.objects[0].material.ambient = 1.0;
        w.objects[1].material.ambient = 1.0;
        let r = Ray::new(point(0.0, 0.0, 0.75), vector(0.0, 0.0, -1.0));
        let clr = w.colour_at(&r, 1);

        assert_eq!(clr, inner.material.pattern.pattern_at_object(inner, point(0.0, 0.0, 0.0)));
    }

    #[test]
    fn no_shadow_when_nothing_collinear_with_point_and_light() {
        let w = World::default_world();
        let p = point(0.0, 10.0, 0.0);

        assert!(!w.is_shadowed(w.lights[0].position, p));
    }

    #[test]
    fn shadow_when_object_between_point_and_light() {
        let w = World::default_world();
        let p = point(10.0, -10.0, 10.0);

        assert!(w.is_shadowed(w.lights[0].position, p));
    }

    #[test]
    fn no_shadow_when_object_behind_light() {
        let w = World::default_world();
        let p = point(-20.0, 20.0, -20.0);

        assert!(!w.is_shadowed(w.lights[0].position, p));
    }

    #[test]
    fn no_shadow_when_object_behind_point() {
        let w = World::default_world();
        let p = point(-2.0, 2.0, -2.0);

        assert!(!w.is_shadowed(w.lights[0].position, p));
    }

    #[test]
    fn shade_hit_is_given_intersection_in_shadow() {
        let light = PointLight::new(Colour::white(), point(0.0, 0.0, -10.0));
        let s1 = Object::new_sphere();
        let s2 = Object::new_sphere()
            .with_transform(Matrix4::translate(0.0, 0.0, 10.0));
        let w = World::default()
            .with_light(light)
            .with_object(s1)
            .with_object(s2);
        let r = Ray::new(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
        let int = Intersection::new(4.0, s2);
        let ints = Intersections::new(vec![int]);
        let comps = ints.prepare_computations(0, &r);
        let c = w.shade_hit(&comps, 1);

        assert_eq!(c, Colour::grey(0.1));
    }

    #[test]
    fn reflected_colour_for_nonreflective_material() {
        let w = World::default_world();
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let mut s = w.objects[1];
        s.material.ambient = 1.0;
        let int = Intersection::new(1.0, s);
        let ints = Intersections::new(vec![int]);
        let comps = ints.prepare_computations(0, &r);
        let colour = w.reflected_colour(&comps, 1);

        assert_eq!(colour, Colour::black());
    }

    #[test]
    fn reflected_colour_for_reflective_material() {
        let shape = Object::new_plane()
            .with_material(Material::default().with_reflectivity(0.5))
            .with_transform(Matrix4::translate(0.0, -1.0, 0.0));
        let w = World::default_world()
            .with_object(shape);
        let irr_no = 2.0f64.sqrt() / 2.0;
        let r = Ray::new(point(0.0, 0.0, -3.0), vector(0.0, -irr_no, irr_no));
        let int = Intersection::new(2.0f64.sqrt(), w.objects[2]);
        let ints = Intersections::new(vec![int]);
        let comps = ints.prepare_computations(0, &r);
        let colour = w.reflected_colour(&comps, 1);

        assert_eq!(comps.reflect_vec, vector(0.0, irr_no, irr_no));
        assert_eq!(colour, Colour::new(0.19032, 0.2379, 0.14274));
    }

    /*#[test]
    fn shade_hit_with_reflective_material() {
        let shape = Object::new_plane()
            .with_material(Material::default().with_reflectivity(0.5))
            .with_transform(Matrix4::translate(0.0, -1.0, 0.0));
        let w = World::default_world()
            .with_object(shape);
        let irr_no = 2.0f64.sqrt() / 2.0;
        let r = Ray::new(point(0.0, 0.0, -3.0), vector(0.0, -irr_no, irr_no));
        let int = Intersection::new(2.0f64.sqrt(), w.objects[2]);
        let ints = Intersections::new(vec![int]);
        let comps = ints.prepare_computations(0, &r);
        let colour = w.shade_hit(&comps, 1);

        assert_eq!(colour, Colour::new(0.87677, 0.92436, 0.82918));
    }

    #[test]
    fn colour_at_with_mutually_reflective_surfaces() {
        let light = PointLight::new(Colour::white(), point(0.0, 0.0, 0.0));
        let lower = Object::new_plane()
            .with_material(Material::default().with_reflectivity(1.0))
            .with_transform(Matrix4::translate(0.0, -1.0, 0.0));
        let upper = Object::new_plane()
            .with_material(Material::default().with_reflectivity(1.0))
            .with_transform(Matrix4::translate(0.0, 1.0, 0.0));
        let w = World::default()
            .with_light(light)
            .with_object(lower)
            .with_object(upper);
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 1.0, 0.0));

        assert_eq!(w.colour_at(&r, 1),Colour::white());
    }

    #[test]
    fn reflected_colour_at_max_recursive_depth() {
        let shape = Object::new_plane()
            .with_material(Material::default().with_reflectivity(1.0))
            .with_transform(Matrix4::translate(0.0, -1.0, 0.0));
        let w = World::default_world()
            .with_object(shape);
        let irr_no = 2.0f64.sqrt() / 2.0;
        let r = Ray::new(point(0.0, 0.0, -3.0), vector(0.0, -irr_no, irr_no));
        let int = Intersection::new(2.0f64.sqrt(), w.objects[2]);
        let ints = Intersections::new(vec![int]);
        let comps = ints.prepare_computations(0, &r);
        let colour = w.reflected_colour(&comps, 1);

        assert_eq!(colour, Colour::black());
    }*/

    #[test]
    fn reflected_colour_with_opaque_surface() {
        let w = World::default_world();
        let object = w.objects[0];
        let ray = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let xs = Intersections::new(vec![
            Intersection::new(4.0, object),
            Intersection::new(6.0, object)
        ]);
        let comps = xs.prepare_computations(0, &ray);

        assert_eq!(w.refracted_colour(&comps, 5), Colour::black());
    }

    #[test]
    fn refracted_colour_at_max_recursive_depth() {
        let w = World::default_world();
        let mut object = w.objects[0];
        object.material.transparency = 1.0;
        object.material.ior = 1.5;
        let ray = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let xs = Intersections::new(vec![
            Intersection::new(4.0, object),
            Intersection::new(6.0, object)
        ]);
        let comps = xs.prepare_computations(0, &ray);

        assert_eq!(w.refracted_colour(&comps, 0), Colour::black());
    }

    #[test]
    fn refracted_colour_under_total_interal_reflection() {
        let w = World::default_world();
        let mut object = w.objects[0];
        object.material.transparency = 1.0;
        object.material.ior = 1.5;
        let irr_no = 2.0f64.sqrt() / 2.0;
        let ray = Ray::new(point(0.0, 0.0, irr_no), vector(0.0, 1.0, 0.0));
        let xs = Intersections::new(vec![
            Intersection::new(-irr_no, object),
            Intersection::new(irr_no, object)
        ]);
        let comps = xs.prepare_computations(1, &ray);

        assert_eq!(w.refracted_colour(&comps, 5), Colour::black());
    }

    /*#[test]
    fn refracted_colour_with_refracted_ray() {
        let w = World::default_world();
        let mut a = w.objects[0];
        a.material.ambient = 1.0;
        a.material.pattern = Pattern::new_test();
        let mut b = w.objects[1];
        b.material.transparency = 1.0;
        b.material.ior = 1.5;
        let ray = Ray::new(point(0.0, 0.0, 0.1), vector(0.0, 1.0, 0.0));
        let xs = Intersections::new(vec![
            Intersection::new(-0.9899, a),
            Intersection::new(-0.4899, b),
            Intersection::new(0.4899, b),
            Intersection::new(0.9899, a)
        ]);
        let comps = xs.prepare_computations(2, &ray);

        assert_eq!(w.refracted_colour(&comps, 5), Colour::new(0.0, 0.99888, 0.04725));
    }

    #[test]
    fn shade_hit_with_transparent_material() {
        let floor_mat = Material::default()
            .with_transparency(0.5)
            .with_ior(1.5);
        let floor = Object::new_plane()
            .with_transform(Matrix4::translate(0.0, -1.0, 0.0))
            .with_material(floor_mat);
        let ball_mat = Material::default()
            .with_colour(Colour::red())
            .with_ambient(0.5);
        let ball = Object::new_sphere()
            .with_transform(Matrix4::translate(0.0, -3.5, -0.5))
            .with_material(ball_mat);
        let w = World::default_world()
            .with_object(floor)
            .with_object(ball);
        let irr_no = 2.0f64.sqrt() / 2.0;
        let ray = Ray::new(point(0.0, 0.0, -3.0), vector(0.0, -irr_no, irr_no));
        let xs = Intersections::new(vec![
            Intersection::new(2.0f64.sqrt(), floor)
        ]);
        let comps = xs.prepare_computations(0, &ray);
        let colour = w.shade_hit(&comps, 5);

        assert_eq!(colour, Colour::new(0.93642, 0.68642, 0.68642));
    }

    #[test]
    fn shade_hit_with_reflective_transparent_material() {
        let floor_mat = Material::default()
            .with_reflectivity(0.5)
            .with_transparency(0.5)
            .with_ior(1.5);
        let floor = Object::new_plane()
            .with_transform(Matrix4::translate(0.0, -1.0, 0.0))
            .with_material(floor_mat);
        let ball_mat = Material::default()
            .with_colour(Colour::red())
            .with_ambient(0.5);
        let ball = Object::new_sphere()
            .with_transform(Matrix4::translate(0.0, -3.5, -0.5))
            .with_material(ball_mat);
        let w = World::default_world()
            .with_object(floor)
            .with_object(ball);
        let irr_no = 2.0f64.sqrt() / 2.0;
        let ray = Ray::new(point(0.0, 0.0, -3.0), vector(0.0, -irr_no, irr_no));
        let xs = Intersections::new(vec![
            Intersection::new(2.0f64.sqrt(), floor)
        ]);
        let comps = xs.prepare_computations(0, &ray);
        let colour = w.shade_hit(&comps, 5);

        assert_eq!(colour, Colour::new(0.93642, 0.68642, 0.68642));
    }*/
}