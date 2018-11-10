use particle;
use particle::{Attractor, Particle, Spawner, Vec3};
use rand::prelude::*;
use std::f32::consts::PI;

const SPEED: f32 = 0.002;
const GRAVITATION: f32 = 0.0005;

/// Spawns particles from active `Spawner`s.
pub fn spawn_particles(
    delta_time: f32,
    particles: &mut Vec<Option<Particle>>,
    spawners: &mut [Spawner]
) {
    for spawner in spawners.iter_mut().filter(|s| s.active) {
        let spawn_delay = 1.0 / spawner.particles_per_second;
        spawner.time_since_spawn += delta_time;
        while spawner.time_since_spawn >= spawn_delay {
            let particle = spawn_particle(spawner.position);
            insert_particle(particles, particle);
            spawner.time_since_spawn -= spawn_delay;
        }
    }
}

/// Creates a new particle with a random direction.
fn spawn_particle(position: Vec3) -> Particle {
    let mut rng = thread_rng();
    let angle = rng.gen_range(0.0, 2.0 * PI);
    let x = angle.cos() * SPEED;
    let y = angle.sin() * SPEED;
    let velocity = Vec3([x, y, 0.0]);
    Particle {
        position: position,
        velocity: velocity,
        life: particle::LIFETIME
    }
}

/// Inserts a particle into the particles vector, using an existing free slot
/// if possible.
fn insert_particle(particles: &mut Vec<Option<Particle>>, particle: Particle) {
    for p in particles.iter_mut() {
        if p.is_none() {
            p.take();
            p.get_or_insert(particle);
            return;
        }
    }

    particles.push(Some(particle));
}

/// Applies `Attractor`s' gravity to particles.
pub fn apply_attractors<'a, I>(delta_time: f32, particles: I, attractors: &[Attractor])
where I: Iterator<Item=&'a mut Particle> {
    for p in particles {
        for a in attractors.iter().filter(|a| a.active) {
            let dist = p.position - a.position;
            let gravity = calc_gravity(a.mass, &dist) * delta_time;
            p.velocity += gravity;
        }
    }
}

/// Computes the velocity to apply based on mass and distance.
fn calc_gravity(mass: f32, distance: &Vec3) -> Vec3 {
    let radius = distance.length();
    let dir = distance.normalize();
    let scale = -(GRAVITATION * mass) / radius;
    dir * scale
}

/// Update active particles' positions based on their velocities.
pub fn update_particles(delta_time: f32, particles: &mut [Option<Particle>]) {
    let iter = particles.iter_mut()
        .filter(|p| p.is_some());
    for p in iter {
        let mut is_dead = false;
        {
            let p = p.as_mut().unwrap();
            p.position += p.velocity;
            p.life -= delta_time;
            if p.life < 0.0 {
                is_dead = true;
            }
        }

        if is_dead {
            p.take();
        }
    }
}
