use engine::context;
use engine::context::Context;
use engine::gameobject::{GameObject, GameObjectManager};
use engine::scene::{Scene, SceneManager, State};

use glutin;
use gl;

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
        
        for event in self.context.window.wait_events() {

            // refresh drawing buffer
            unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };
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
        
        // stop the mainscene, which stops the game.
        self.scenemanager.get("main").unwrap().stop();
    }
}
