use game::Game;
use engine::gameobject::{GameObject, GameObjectManager};

use gl::types::*;
use std::mem;
use std::ptr;
use std::str;
use std::ffi::CString;

// Vertex data
static VERTEX_DATA: [GLfloat; 6] = [
     0.0,  0.5,
     0.5, -0.5,
    -0.5, -0.5
];

// Shader sources
static VS_SRC: &'static str =
   "#version 150\n\
    in vec2 position;\n\
    void main() {\n\
       gl_Position = vec4(position, 0.0, 1.0);\n\
    }";

static FS_SRC: &'static str =
   "#version 150\n\
    out vec4 out_color;\n\
    void main() {\n\
       out_color = vec4(1.0, 1.0, 1.0, 1.0);\n\
    }";


#[derive(Debug)]
pub struct Triangle {
    pub x: f64,
    pub y: f64,
    pub w: u32,
    pub h: u32,
}

impl Triangle {

}

impl GameObject for Triangle {
    fn new(x: f64, y: f64, w: u32, h: u32) -> Self {
        Triangle {
            x: x,
            y: y,
            w: w,
            h: h,
        }
    }

    // filling the backbuffer of the render function
    fn draw(&self, &window) {
        // let window = glutin::Window::new().unwrap();
        // unsafe { window.make_current() }.unwrap();
        // gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        let vs = compile_shader(VS_SRC, gl::VERTEX_SHADER);
        let fs = compile_shader(FS_SRC, gl::FRAGMENT_SHADER);
        let program = link_program(vs, fs);

        let mut vao = 0;
        let mut vbo = 0;

        unsafe {
            // Create Vertex Array Object
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            // Create a Vertex Buffer Object and copy the vertex data to it
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER,
                        (VERTEX_DATA.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                        mem::transmute(&VERTEX_DATA[0]),
                        gl::STATIC_DRAW);

            // Use shader program
            gl::UseProgram(program);
            gl::BindFragDataLocation(program, 0,
                                    CString::new("out_color").unwrap().as_ptr());

            // Specify the layout of the vertex data
            let pos_attr = gl::GetAttribLocation(program,
                                                CString::new("position").unwrap().as_ptr());
            gl::EnableVertexAttribArray(pos_attr as GLuint);
            gl::VertexAttribPointer(pos_attr as GLuint, 2, gl::FLOAT,
                                    gl::FALSE as GLboolean, 0, ptr::null());
        }
    }

    // called on each update from the main loop
    fn update(&mut self) {
        self.reset();
    }

    fn compile_shader(src: &str, ty: GLenum) -> GLuint {
        let shader;
        unsafe {
            shader = gl::CreateShader(ty);
            // Attempt to compile the shader
            let c_str = CString::new(src.as_bytes()).unwrap();
            gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
            gl::CompileShader(shader);

            // Get the compile status
            let mut status = gl::FALSE as GLint;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

            // Fail on error
            if status != (gl::TRUE as GLint) {
                let mut len = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf = Vec::with_capacity(len as usize);
                buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
                gl::GetShaderInfoLog(shader, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
                panic!("{}", str::from_utf8(&buf).ok().expect("ShaderInfoLog not valid utf8"));
            }
        }
        shader
    }

    fn link_program(vs: GLuint, fs: GLuint) -> GLuint { unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::LinkProgram(program);
        // Get the link status
        let mut status = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len: GLint = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetProgramInfoLog(program, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
            panic!("{}", str::from_utf8(&buf).ok().expect("ProgramInfoLog not valid utf8"));
        }
        program
    } }
}