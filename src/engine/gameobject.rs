use sdl2::render::Renderer;
use sdl2::event::Event;
use std::collections::HashMap;


pub trait GameObject {
    // Return type of Trait implementer
    fn new(x: i32, y: i32, w: u32, h: u32) -> Self where Self:Sized;

    // static
    fn draw(&self, renderer: &mut Renderer);
    fn update(&mut self, event: &Vec<Event>, deltatime: f64);
}

pub struct GameObjectManager {
     objects: HashMap<&'static str, Box<GameObject>>,
}

impl GameObjectManager {

    fn new() -> Self {
        //TODO, search for indetical names
        let objects: HashMap<&'static str, Box<GameObject>> = HashMap::new();
        GameObjectManager{
            objects: objects,
        }
    }

    // Add wrapper
    fn add(&mut self, id: &'static str, gameobject: Box<GameObject>) {
        self.objects.insert(id, gameobject);
    }

    // Remove wrapper
    fn remove(&mut self, id: &'static str){
        self.objects.remove(id);
    }

    fn get(&mut self, id: &'static str) -> Option<&Box<GameObject>>{
        //TODO, add result handling.
        self.objects.get(id)
    }

}
