use glium::{program, Display};

const VERTEX_SOURCE: &str = include_str!("particle.vert.glsl");
const FRAGMENT_SOURCE: &str = include_str!("particle.frag.glsl");

pub fn create_program(display: &Display) -> program::Program {
    program::Program::from_source(display, VERTEX_SOURCE, FRAGMENT_SOURCE, None).unwrap()
}
