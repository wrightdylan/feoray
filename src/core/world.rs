use crate::core::{point, Colour, Intersections, PreCompData, Ray, Transform};
use crate::materials::Material;
use crate::primitives::Object;
use crate::lights::PointLight;
use nalgebra::{Matrix4, Vector4};

#[derive(Debug, PartialEq)]
pub struct World {
    pub objects: Vec<Object>,
    pub lights: Vec<PointLight>
}

impl World {
    /// NWO - New World Object.
    pub fn new(objects: Vec<Object>, lights: Vec<PointLight>) -> Self {
        World { objects, lights }
    }

    /// Calculates the colour of a pixel.
    pub fn colour_at(&self, ray: Ray) -> Colour {
        let xs = self.intersect(ray);
        if xs.hit().is_some() {
            self.shade_hit(xs.hit().unwrap().prepare_computations(ray))
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
            lights: vec![PointLight::new(Colour::white(), point(-10.0, 10.0, -10.0))]
        }
    }

    /// Intersections of rays and world objects rather than individual objects.
    pub fn intersect(&self, ray: Ray) -> Intersections {
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
        let intersections = self.intersect(ray);
        let h = intersections.hit();
        if h != None && h.unwrap().t < distance {
            true
        } else {
            false
        }
    }

    /// Calculates colour of hit. Support multiple lights right out of the box!
    pub fn shade_hit(&self, comps: PreCompData) -> Colour {
        let mut res = Colour::black();
        for i in 0..self.lights.len() {
            res += comps.object.material.lighting(
                comps.object,
                self.lights[i],
                comps.pos,
                comps.eyev,
                comps.normal,
                self.is_shadowed(self.lights[i].position, comps.over_pos)
            );
        }
        res
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
}

impl Default for World {
    fn default() -> Self {
        World {
            objects: vec![],
            lights: vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{vector, Intersection};

    #[test]
    fn creating_a_world() {
        let w = World::new( vec![], vec![]);

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
        let xs = w.intersect(r);

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
        let comps = int.prepare_computations(r);
        let clr = w.shade_hit(comps);

        assert_eq!(clr.to_5dp(), Colour::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shading_intersection_from_inside() {
        let mut w = World::default_world();
        w.lights[0] = PointLight::new(Colour::new(1.0, 1.0, 1.0), point(0.0, 0.25, 0.0));
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let s = w.objects[1];
        let int = Intersection::new(0.5, s);
        let comps = int.prepare_computations(r);
        let clr = w.shade_hit(comps);

        assert_eq!(clr.to_5dp(), Colour::new(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn colour_when_ray_misses() {
        let w = World::default_world();
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 1.0, 0.0));
        let clr = w.colour_at(r);

        assert_eq!(clr, Colour::black());
    }

    #[test]
    fn colour_when_ray_hits() {
        let w = World::default_world();
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let clr = w.colour_at(r);

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
        let clr = w.colour_at(r);

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
        let i = Intersection::new(4.0, s2);
        let comps = i.prepare_computations(r);
        let c = w.shade_hit(comps);

        assert_eq!(c, Colour::grey(0.1));
    }
}