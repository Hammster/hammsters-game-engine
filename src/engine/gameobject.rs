use std::collections::HashMap;
use std::fmt;

pub trait GameObject: fmt::Debug {
    // Return type of Trait implementer
    // TODO, i32 to f64 for glium
    fn new(x: f64, y: f64, w: u32, h: u32) -> Self where Self: Sized;

    // Required implement functions
    fn draw(&self);
    fn update(&mut self);
}

#[derive(Debug)]
pub struct GameObjectManager {
    pub objects: HashMap<&'static str, Box<GameObject>>,
}

impl GameObjectManager {
    pub fn new() -> Self {
        let objects: HashMap<&'static str, Box<GameObject>> = HashMap::new();
        GameObjectManager { objects: objects }
    }

    // Add wrapper
    pub fn insert(&mut self, id: &'static str, gameobject: Box<GameObject>) {
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
