mod shaders;

use glium::{index, program, Display, Surface, VertexBuffer, implement_vertex};
use glium::draw_parameters::{DrawParameters};
use glium::uniforms::{UniformsStorage};
use crate::particle::{Particle};

/// The maximum number of rendered particles.
const MAX_PARTICLES: usize = 50000;

/// Represents vertex data for a particle.
#[derive(Copy, Clone)]
struct ParticleVertex {
    pub position: [f32; 3],
    pub velocity: [f32; 3],
    pub life: f32
}

implement_vertex!(ParticleVertex, position, velocity, life);

/// Contains the state required to render the scene.
pub struct Renderer {
    display: Display,
    vertex_count: usize,
    buffer: VertexBuffer<ParticleVertex>,
    program: program::Program
}

impl Renderer {
    /// Initializes a new `Renderer`.
    pub fn new(display: Display) -> Renderer {
        let buffer = VertexBuffer::empty_dynamic(&display, MAX_PARTICLES).unwrap();
        let program = shaders::create_program(&display);
        Renderer {
            display: display,
            vertex_count: 0,
            buffer: buffer,
            program: program
        }
    }

    /// Borrows the `Renderer`'s `Display`.
    pub fn display(&self) -> &Display {
        &self.display
    }

    /// Updates the particle vertex data.
    pub fn fill_vertices<'a, I>(&mut self, particles: I)
    where I: Iterator<Item=&'a Particle> {
        self.buffer.invalidate();
        let mut mapping = self.buffer.map_write();
        let vertices = particles
            .take(MAX_PARTICLES)
            .map(|p| ParticleVertex {
                position: p.position.0,
                velocity: p.velocity.0,
                life: p.life
            });
        let mut index = 0;
        for v in vertices {
            mapping.set(index, v);
            index += 1;
        }
        self.vertex_count = index;
    }

    /// Draws the scene.
    pub fn render(&mut self) {
        let mut target = self.display.draw();
        target.clear_color(0.01, 0.01, 0.25, 0.0);

        let buffer_slice = self.buffer.slice(0..self.vertex_count);
        let indices = index::NoIndices(index::PrimitiveType::Points);
        let mvp_matrix = identity_matrix();
        let uniforms = UniformsStorage::new("mvpMatrix", mvp_matrix);
        let draw_params = DrawParameters {
            blend: glium::Blend::alpha_blending(),
            line_width: Some(2.0),
            .. Default::default()
        };

        if buffer_slice.is_some() {
            let buffer_slice = buffer_slice.unwrap();
            target.draw(
                buffer_slice,
                &indices,
                &self.program,
                &uniforms,
                &draw_params
            ).unwrap();
        }

        target.finish().unwrap();
    }
}

/// Creates a 4x4 identity matrix.
fn identity_matrix() -> [[f32; 4]; 4] {
    let mut matrix = [[0.0; 4]; 4];
    for i in 0..4 {
        matrix[i][i] = 1.0;
    }
    matrix
}
