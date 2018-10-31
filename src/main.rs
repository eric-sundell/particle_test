extern crate glium;

mod particle;
mod systems;

use glium::{glutin, Surface};
use std::time::{Duration, Instant};
use particle::{Simulation, Spawner, Vec3};

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("Particle Test");
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let mut sim = create_simulation();

    let mut closed = false;
    let mut last_frame = Instant::now();
    while !closed {
        // listing the events produced by application and waiting to be received
        events_loop.poll_events(|ev| {
            match ev {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => closed = true,
                    _ => (),
                },
                _ => (),
            }
        });

        let now = Instant::now();
        let delta_time = now.duration_since(last_frame);
        let delta_time = to_delta_seconds(delta_time);
        last_frame = now;

        systems::spawn_particles(delta_time, &mut sim.particles, &mut sim.spawners);
        systems::update_particles(delta_time, &mut sim.particles);

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        target.finish().unwrap();
    }
}

fn create_simulation() -> Simulation {
    let spawner = Spawner {
        position: Vec3([0.0; 3]),
        particles_per_second: 5.0,
        time_since_spawn: 0.0
    };
    Simulation::new(vec![spawner])
}

fn to_delta_seconds(delta_time: Duration) -> f32 {
    let secs = delta_time.as_secs() as f64;
    let nanos = delta_time.subsec_nanos() as f64;
    (secs + (nanos / 1000000000.0)) as f32
}
