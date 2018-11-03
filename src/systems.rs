use particle;
use particle::{Particle, Spawner, Vec3};
use rand::prelude::*;
use std::f32::consts::PI;

const SPEED: f32 = 0.002;

pub fn spawn_particles(
    delta_time: f32,
    particles: &mut Vec<Option<Particle>>,
    spawners: &mut [Spawner]
) {
    for spawner in spawners.iter_mut() {
        let spawn_delay = 1.0 / spawner.particles_per_second;
        spawner.time_since_spawn += delta_time;
        while spawner.time_since_spawn >= spawn_delay {
            let particle = spawn_particle(spawner.position);
            insert_particle(particles, particle);
            spawner.time_since_spawn -= spawn_delay;
        }
    }
}

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

pub fn update_particles(delta_time: f32, particles: &mut [Option<Particle>]) {
    for p in particles.iter_mut() {
        if p.is_none() {
            continue;
        }

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
