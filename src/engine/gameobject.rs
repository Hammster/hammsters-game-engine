use sdl2::render::Renderer;
use sdl2::event::Event;
use std::collections::HashMap;
use std::fmt;

pub trait GameObject: fmt::Debug {
    // Return type of Trait implementer
    fn new(x: i32, y: i32, w: u32, h: u32) -> Self where Self: Sized;

    // Required implement functions
    fn draw(&self, renderer: &mut Renderer);
    fn update(&mut self, event: &Vec<Event>, deltatime: f64);
}

#[derive(Debug)]
pub struct GameObjectManager {
    pub objects: HashMap<&'static str, Box<GameObject>>,
}

impl GameObjectManager {
    pub fn new() -> Self {
        // TODO, suffixing duplicats
        let mut objects: HashMap<&'static str, Box<GameObject>> = HashMap::new();
        GameObjectManager { objects: objects }
    }

    // Add wrapper
    pub fn insert(&mut self, id: &'static str, gameobject: Box<GameObject>) {
        // TODO, suffixing duplicats
        self.objects.insert(id, gameobject);
    }

    // Remove wrapper
    pub fn remove(&mut self, id: &'static str) {
        self.objects.remove(id);
    }

    // object()
    pub fn get(&mut self, id: &'static str) -> Option<&Box<GameObject>> {
        self.objects.get(id)
    }

    pub fn objects(&mut self) -> &mut HashMap<&'static str, Box<GameObject>> {
        let ret : &mut HashMap<&'static str, Box<GameObject>> = &mut self.objects;
        ret
    }
}

// scenemanager.scenes.scene.gameobjectmanager.objects.gameojects