use std::ops::{AddAssign, Add};

pub const LIFETIME: f32 = 10.0;

#[derive(Clone, Copy)]
pub struct Vec3(pub [f32; 3]);

pub struct Particle {
    pub position: Vec3,
    pub velocity: Vec3,
    pub life: f32
}

pub struct Spawner {
    pub position: Vec3,
    pub particles_per_second: f32,
    pub time_since_spawn: f32
}

pub struct Simulation {
    pub particles: Vec<Option<Particle>>,
    pub spawners: Vec<Spawner>
}

impl Vec3 {
    pub fn length_squared(&self) -> f32 {
        let v = self.0;
        v[0] * v[0] + v[1] * v[1] + v[2] * v[2]
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        let v = &mut self.0;
        let w = &rhs.0;
        v[0] += w[0];
        v[1] += w[1];
        v[2] += w[2];
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(mut self, rhs: Vec3) -> Vec3 {
        self += rhs;
        self
    }
}

impl Simulation {
    pub fn new(spawners: Vec<Spawner>) -> Simulation {
        let mut capacity = 0;
        {
            let foo = &spawners;
            for s in foo {
                capacity += (s.particles_per_second * LIFETIME).ceil() as usize;
            }
        }
        Simulation {
            particles: Vec::with_capacity(capacity),
            spawners: spawners
        }
    }
}
