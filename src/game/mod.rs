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
       
        let mut context = Context::new("Rust Engine");
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
            unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };
            self.context.window.swap_buffers();

            match event {
                glutin::Event::Closed => break,
                _ => ()
            }
            
        }
      
        self.scenemanager.get("main").unwrap().stop();
    }
}
