#![warn(clippy::pedantic)]

mod math;

use crate::math::*;

pub struct Projectile {
    position: Metric, // Point
    velocity: Metric  // Vector
}

impl Projectile {
    pub fn new( position: Metric, velocity: Metric ) -> Projectile {
        Projectile { position, velocity }
    }
}

pub struct Environment {
    gravity: Metric, // Vector
    wind: Metric     // Vector
}

impl Environment {
    pub fn new ( gravity: Metric, wind: Metric ) -> Environment {
        Environment { gravity, wind }
    }
}

pub fn tick(env: &Environment, proj: Projectile) -> Projectile {
    let position = proj.position + proj.velocity;
    let velocity = proj.velocity + env.gravity + env.wind;
    Projectile { position, velocity }
}

fn main() {
    let mut p = Projectile::new(point(0.0, 1.0, 0.0), vector(1.0, 1.0, 0.0).norm());
    let e = Environment::new(vector(0.0, -0.1, 0.0), vector(-0.01, 0.0, 0.0));
    let mut ticks = 0;

    while p.position.y > 0.0 {
        p = tick(&e, p);
        ticks += 1;
    }

    println!("Time to reach the ground is {} ticks.", ticks);
}