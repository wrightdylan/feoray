use crate::{point, Colour, Intersections, Material, Object, PointLight, PreCompData, Ray, Transform};
use nalgebra::Matrix4;

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
        if xs.len() > 0 {
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

    /// Calculates colour of hit. Support multiple lights right out of the box!
    pub fn shade_hit(&self, comps: PreCompData) -> Colour {
        let mut res = Colour::black();
        for i in 0..self.lights.len() {
            res += comps.object.material.lighting(
                self.lights[i],
                comps.pos,
                comps.eyev,
                comps.normal
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
    use crate::{vector, Intersection};

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
        let mut w = World::default_world();
        let mut inner = w.objects[1];
        inner.material.ambient = 1.0;
        w.objects[0].material.ambient = 1.0;
        w.objects[1].material.ambient = 1.0;
        let r = Ray::new(point(0.0, 0.0, 0.75), vector(0.0, 0.0, -1.0));
        let clr = w.colour_at(r);

        assert_eq!(clr, inner.material.colour);
    }
}