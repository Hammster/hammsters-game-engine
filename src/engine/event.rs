use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

#[derive(Debug)]
pub enum EventType {
    Keycode,
    MouseButton,
}

#[derive(Debug)]
pub struct EventManager {
    events: Vec<EventType>,
}

impl EventManager {
    pub fn new() -> Self {
        EventManager { events: vec![] }
    }

    pub fn push(&mut self, et: EventType) {
        self.events.push(et);
    }

    pub fn pop(&mut self) -> Option<EventType> {
        self.events.pop()
    }
}
