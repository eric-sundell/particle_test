mod shaders;

use glium::{index, program, Display, Surface, VertexBuffer};
use glium::draw_parameters::{DrawParameters};
use glium::uniforms::{UniformsStorage};
use particle::{Particle};

#[derive(Copy, Clone)]
struct ParticleVertex {
    pub position: [f32; 3],
    pub velocity: [f32; 3],
    pub life: f32
}

implement_vertex!(ParticleVertex, position, velocity, life);

pub struct Renderer {
    display: Display,
    vertices: Vec<ParticleVertex>,
    buffer: VertexBuffer<ParticleVertex>,
    program: program::Program
}

impl Renderer {
    pub fn new(display: Display) -> Renderer {
        let buffer = VertexBuffer::empty_dynamic(&display, 100).unwrap();
        let program = shaders::create_program(&display);
        Renderer {
            display: display,
            vertices: Vec::new(),
            buffer: buffer,
            program: program
        }
    }

    pub fn fill_vertices<'a, I>(&mut self, particles: I)
    where I: Iterator<Item=&'a Particle> {
        self.vertices.clear();
        let new_verts = particles
            .map(|p| ParticleVertex {
                position: p.position.0,
                velocity: p.velocity.0,
                life: p.life
            });
        self.vertices.extend(new_verts);
    }

    pub fn render(&mut self) {
        let mut target = self.display.draw();
        target.clear_color(0.01, 0.01, 0.25, 0.0);

        self.update_buffer();

        let indices = index::NoIndices(index::PrimitiveType::Points);
        let mvp_matrix = identity_matrix();
        let uniforms = UniformsStorage::new("mvpMatrix", mvp_matrix);
        let draw_params = DrawParameters {
            blend: glium::Blend::alpha_blending(),
            line_width: Some(2.0),
            .. Default::default()
        };

        target.draw(
            &self.buffer,
            &indices,
            &self.program,
            &uniforms,
            &draw_params
        ).unwrap();

        target.finish().unwrap();
    }

    fn update_buffer(&mut self) {
        self.buffer = VertexBuffer::dynamic(&self.display, &self.vertices).unwrap();
    }
}

fn identity_matrix() -> [[f32; 4]; 4] {
    let mut matrix = [[0.0; 4]; 4];
    for i in 0..4 {
        matrix[i][i] = 1.0;
    }
    matrix
}
