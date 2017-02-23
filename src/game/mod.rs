use engine;
use engine::context;
use engine::context::Context;
use engine::gameobject::{GameObject, GameObjectManager};
use engine::scene::{Scene, SceneManager, State};

use glutin;
use gl;
use gl::types::*;

use std::ptr;
use std::mem;

use rand;

#[derive(Debug)]
pub struct Game {
    pub context: Context,
    pub scenemanager: SceneManager,
    pub currentscene: &'static str,
}

impl Game {
    pub fn new() -> Game {
       
        let mut context = Context::new();
        let mut scenemanager = SceneManager::new();
        
        scenemanager.create("main");
        scenemanager.create("menu");
        scenemanager.get("main").unwrap().state = State::Running;

        Game {
            context: context,
            scenemanager: scenemanager, 
            currentscene: "main"
        }
    }

    pub fn start(&mut self) {
        
        let mut vertices : Vec<GLfloat> = vec![
        // First triangle
        -0.9, -0.5, 0.0,  // Left 
        -0.0, -0.5, 0.0,  // Right
        -0.45, 0.5, 0.0,  // Top 
        // Second triangle
         0.0, -0.5, 0.0,  // Left
         0.9, -0.5, 0.0,  // Right
         0.45, 0.5, 0.0   // Top 
        ];

        let mut vbo = 0;
        let mut vao = 0;

        unsafe {
            // gen and bind VAO
            gl::GenVertexArrays(1, &mut vao);  
            gl::GenBuffers(1, &mut vbo);
            

            // gen and bind VBO
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);  
            gl::BufferData(
                gl::ARRAY_BUFFER, 
                (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                mem::transmute(&vertices), 
                gl::STATIC_DRAW
            );
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, (vertices.len() * mem::size_of::<GLfloat>()) as i32, ptr::null());
            gl::EnableVertexAttribArray(0);

            // unbind
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
        //let mut loader = engine::graphics::loader::Loader::new();
        //let mut model = loader.load_to_vao(&vertices);

        println!("{:?}", &vao);

        for event in self.context.window.wait_events() {

            unsafe { 
                gl::ClearColor(0.0, 0.0, 0.0, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
                gl::BindVertexArray(vao);
                gl::DrawArrays(gl::TRIANGLES, 0, (vertices.len() / 3) as i32);
                gl::BindVertexArray(0);
            }

            self.context.window.swap_buffers();

            match event {
                // match window close event
                glutin::Event::Closed => break,
                // match keyboard events
                glutin::Event::KeyboardInput(glutin::ElementState::Pressed, code, key) => {
                    // match keystrockes nased on `glutin::VirtualKeyCode` enum
                    match key.unwrap(){
                        glutin::VirtualKeyCode::Escape => break,
                        _ => ()
                    }
                },
                _ => ()
            }
            
        }
        
        unsafe {
            gl::DeleteBuffers(1, &vbo);
            gl::DeleteVertexArrays(1, &vao);
        }

        // stop the mainscene, which stops the game.
        self.scenemanager.get("main").unwrap().stop();
    }
}
