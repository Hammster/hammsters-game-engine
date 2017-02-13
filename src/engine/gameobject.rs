use sdl2::render::Renderer;
use sdl2::event::Event;
use std::collections::HashMap;
use std::fmt;

pub trait GameObject : fmt::Debug {
    // Return type of Trait implementer
    fn new(x: i32, y: i32, w: u32, h: u32) -> Self where Self:Sized;

    // static
    fn draw(&self, renderer: &mut Renderer);
    fn update(&mut self, event: &Vec<Event>, deltatime: f64);
}

#[derive(Debug)]
pub struct GameObjectManager {
     pub objects: HashMap<&'static str, Box<GameObject>>,
}

impl GameObjectManager {

    pub fn new() -> Self {
        //TODO, search for indetical names
        let objects: HashMap<&'static str, Box<GameObject>> = HashMap::new();
        GameObjectManager{
            objects: objects,
        }
    }

    // Add wrapper
    pub fn add(&mut self, id: &'static str, gameobject: Box<GameObject>) {
        self.objects.insert(id, gameobject);
    }

    // Remove wrapper
    pub fn remove(&mut self, id: &'static str){
        self.objects.remove(id);
    }

    pub fn get(&mut self, id: &'static str) -> Option<&Box<GameObject>>{
        //TODO, add result handling.
        self.objects.get(id)
    }

}
