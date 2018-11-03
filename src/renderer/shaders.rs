use glium::{program, Display};

const VERTEX_SOURCE: &str = include_str!("particle.vert.glsl");
const FRAGMENT_SOURCE: &str = include_str!("particle.frag.glsl");

pub fn create_program(display: &Display) -> program::Program {
    let source = program::ProgramCreationInput::SourceCode {
        vertex_shader: VERTEX_SOURCE,
        tessellation_control_shader: None,
        tessellation_evaluation_shader: None,
        geometry_shader: None,
        fragment_shader: FRAGMENT_SOURCE,
        transform_feedback_varyings: None,
        outputs_srgb: false,
        uses_point_size: true
    };
    program::Program::new(display, source).unwrap()
}
