pub #[derive(Debug)]
enum Event {
    Keyboard,
    Mouse,
}

#[derive(Debug)]
pub struct EventManager {
    events: Vec<Event>,
}

impl EventManager {
    pub fn new() -> Self {
        EventManager { events: vec![] }
    }

    pub fn push(&mut self, et: Event) {
        self.events.push(et);
    }

    pub fn pop(&mut self) -> Option<Event> {
        self.events.pop()
    }
}
