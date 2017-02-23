use std::ptr;
use std::mem;

use gl;
use gl::types::*;

use engine::graphics::model::Model;

#[derive(Debug)]
pub struct Loader {
    vbos: Vec<GLuint>,
    vaos: Vec<GLuint>
}

impl Loader {
    pub fn new() -> Self {
        Loader {
            vbos: vec![],
            vaos: vec![]
        }
    }

    pub fn create_vao(&mut self) -> GLuint {
        // generate VAO
        let mut vao_id : GLuint = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao_id);
        }

        self.vaos.push(vao_id.clone());

        // bind VAO
        unsafe { 
            gl::BindVertexArray(vao_id); 
        }

        vao_id
    }

    pub fn unbind_vao(&mut self) {
        // set current VAO bind to 0
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    pub fn store_vbo(&mut self, attributeNumber: GLuint, data: &Vec<GLfloat>) {
        // Generate VBO's
        let mut vboID : GLuint = 0;

        unsafe {
            gl::GenBuffers(1, &mut vboID);
        }
        
        self.vbos.push(vboID);

        // BindVBO's
        unsafe { 
            gl::BindBuffer(gl::ARRAY_BUFFER, vboID); 
            gl::BufferData(
                gl::ARRAY_BUFFER, 
                (data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                mem::transmute(&data), 
                gl::STATIC_DRAW
            );
            gl::EnableVertexAttribArray(attributeNumber);
            gl::VertexAttribPointer(attributeNumber, 3, gl::FLOAT, gl::FALSE, 0, ptr::null());
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }


    pub fn load_to_vao(&mut self, positions: &Vec<GLfloat>) -> Model {
        // let obj = engine::util::obj::Obj::new();
        let vaoID : u32 = self.create_vao();
        
        self.store_vbo(0,&positions);
        // unbind the VAO
        self.unbind_vao();
        // length divided by 3 because we are using xyz positions
        Model::new(vaoID,(&positions.len()/3) as i32)
    }

    pub fn clear(&mut self) {
        println!("{:?}", &self);

        for vao in &self.vaos{
            unsafe {
                gl::DeleteVertexArrays(1, *&vao);
            }
        }
        for vbo in &self.vbos{
            unsafe {
                gl::DeleteBuffers(1, *&vbo);
            }
        }
    }
}

