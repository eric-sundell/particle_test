use std::ops::{AddAssign, Add, MulAssign, Mul, SubAssign, Sub};

/// How long, in seconds, a single particle lives.
pub const LIFETIME: f32 = 10.0;

/// A 3-dimensional vector.
#[derive(Debug, Clone, Copy)]
pub struct Vec3(pub [f32; 3]);

/// An individual particle.
#[derive(Debug, Clone, Copy)]
pub struct Particle {
    pub position: Vec3,
    pub velocity: Vec3,
    pub life: f32
}

/// A source of particles.
pub struct Spawner {
    pub position: Vec3,
    pub particles_per_second: f32,
    pub time_since_spawn: f32,
    pub active: bool
}

/// A point that draws paticles to itself.
pub struct Attractor {
    pub position: Vec3,
    pub mass: f32
}

/// Represents the state of the particle simulation.
pub struct Simulation {
    pub particles: Vec<Option<Particle>>,
    pub spawners: Vec<Spawner>,
    pub attractors: Vec<Attractor>
}

impl Vec3 {
    /// Computes the squared length of the vector.
    pub fn length_squared(&self) -> f32 {
        let v = self.0;
        v[0] * v[0] + v[1] * v[1] + v[2] * v[2]
    }

    /// Computes the length of the vector.
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    /// Creates a unit length vector from this one.
    pub fn normalize(&self) -> Vec3 {
        let len = self.length();
        let v = &self.0;
        Vec3([
            v[0] / len,
            v[1] / len,
            v[2] / len
        ])
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

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        let v = &mut self.0;
        v[0] *= rhs;
        v[1] *= rhs;
        v[2] *= rhs;
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(mut self, rhs: f32) -> Vec3 {
        self *= rhs;
        self
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        let v = &mut self.0;
        let w = rhs.0;
        v[0] -= w[0];
        v[1] -= w[1];
        v[2] -= w[2];
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(mut self, rhs: Vec3) -> Vec3 {
        self -= rhs;
        self
    }
}

impl Simulation {
    /// Creates a new simulation.
    pub fn new(spawners: Vec<Spawner>, attractors: Vec<Attractor>) -> Simulation {
        let mut capacity = 0;
        {
            let foo = &spawners;
            for s in foo {
                capacity += (s.particles_per_second * LIFETIME).ceil() as usize;
            }
        }
        Simulation {
            particles: Vec::with_capacity(capacity),
            spawners: spawners,
            attractors: attractors
        }
    }
}
