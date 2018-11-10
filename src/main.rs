#[macro_use]
extern crate glium;
extern crate rand;

mod particle;
mod renderer;
mod systems;

use glium::{glutin};
use std::time::{Duration, Instant};
use particle::{Attractor, Simulation, Spawner, Vec3};

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("Particle Test");
    let context = glutin::ContextBuilder::new()
        .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 3)))
        .with_vsync(true);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let mut sim = create_simulation();
    let mut renderer = renderer::Renderer::new(display);

    let mut closed = false;
    let mut last_frame = Instant::now();
    while !closed {
        let mut dump_verts = false;
        
        events_loop.poll_events(|ev| {
            match ev {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => closed = true,
                    glutin::WindowEvent::KeyboardInput{input, ..} => match input.virtual_keycode {
                        Some(key) => match key {
                            glium::glutin::VirtualKeyCode::D => dump_verts = true,
                            _ => ()
                        },
                        _ => ()
                    },
                    glutin::WindowEvent::CursorMoved{position, ..} => {
                        let coords = convert_mouse_coords(position, &renderer.display().gl_window());
                        let coords = Vec3([coords.0, coords.1, 0.0]);
                        sim.spawners[0].position = coords;
                    },
                    glutin::WindowEvent::MouseInput{button, state, ..} => match button {
                        glutin::MouseButton::Left => match state {
                            glutin::ElementState::Pressed => sim.spawners[0].active = true,
                            glutin::ElementState::Released => sim.spawners[0].active = false
                        },
                        _ => (),
                    },
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
        systems::apply_attractors(
            delta_time,
            sim.particles.iter_mut().filter_map(|p| p.as_mut()),
            &sim.attractors
        );
        systems::update_particles(delta_time, &mut sim.particles);

        if dump_verts {
            println!("{:#?}", sim.particles);
        }

        let live_particles = sim.particles.iter().filter_map(|p| p.as_ref());
        renderer.fill_vertices(live_particles);

        renderer.render();
    }
}

/// Initializes the simulation.
fn create_simulation() -> Simulation {
    const SPAWN_RATE: f32 = 20.0;
    let spawners = vec![
        Spawner {
            position: Vec3([0.0, 0.0, 0.0]),
            particles_per_second: SPAWN_RATE * 2.0,
            time_since_spawn: 0.0,
            active: false
        },
        Spawner {
            position: Vec3([0.5, 0.0, 0.0]),
            particles_per_second: SPAWN_RATE,
            time_since_spawn: 0.0,
            active: true
        },
        Spawner {
            position: Vec3([-0.5, 0.0, 0.0]),
            particles_per_second: SPAWN_RATE,
            time_since_spawn: 0.0,
            active: true
        }
    ];
    let attractors = vec![
        Attractor {
            position: Vec3([0.5, 0.5, 0.0]),
            mass: 1.0
        },
        Attractor {
            position: Vec3([-0.1, -0.5, 0.0]),
            mass: 1.0
        }
    ];
    Simulation::new(spawners, attractors)
}

/// Converts a `Duration` into fractional seconds.
fn to_delta_seconds(delta_time: Duration) -> f32 {
    let secs = delta_time.as_secs() as f64;
    let nanos = delta_time.subsec_nanos() as f64;
    (secs + (nanos / 1000000000.0)) as f32
}

/// Transforms the mouse's `LogicalPosition` into NDC coordinates.
fn convert_mouse_coords(mouse: glutin::dpi::LogicalPosition, window: &glutin::Window) -> (f32, f32) {
    let window_size = window.get_inner_size().unwrap();
    let x = (mouse.x / window_size.width) * 2.0 - 1.0;
    let y = -((mouse.y / window_size.height) * 2.0 - 1.0);
    (x as f32, y as f32)
}
