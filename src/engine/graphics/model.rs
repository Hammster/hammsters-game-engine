use engine;
use gl;
use gl::types::*;

#[derive(Debug)]
pub struct Model {
    pub vaoID: GLuint,
    pub vertexCount: GLsizei,
}

impl Model {
    pub fn new(vaoID: GLuint, vertexCount: GLsizei) -> Self {
        Model {
            vaoID: vaoID,
            vertexCount: vertexCount,
        }
    }
}

