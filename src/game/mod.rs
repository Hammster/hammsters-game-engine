//! ### Game initializing
mod square;

// sdl2
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::event::Event;

//engine
use engine::gameobject::GameObject;
use engine::context::Context;

use game::square::Square;

#[derive(Debug)]
pub struct Game {
    pub context: Context,
    pub running: bool,
    // TODO, make this type flexible, Box could be used
    pub gameobjects: Vec<Square>,
}


impl Game {
    pub fn new() -> Game {
        let context = Context::new("Rust Engine", 800, 600);

        let mut objects = vec![];
        let object: Square = GameObject::new(10, 10, 30, 180);
        objects.push(object);
        let object: Square = GameObject::new(760, 10, 30, 180);
        objects.push(object);

        Game {
            context: context,
            running: true,
            gameobjects: objects,
        }
    }

    pub fn start(&mut self) {

        // borrow timer context
        let mut timer = self.context.sdl.timer().unwrap();

        let interval = 1_000 / 60; // capped physics frames

        let mut before = timer.ticks();
        let mut last_second = timer.ticks();
        let mut fps = 0u16;


        while self.running {
            let now = timer.ticks();
            let dt = now - before;
            // usable dt
            let deltatime = dt as f64 / 1_000.0;

            if dt < interval {
                timer.delay(interval - dt);
                continue;
            }

            before = now;
            fps += 1;

            if now - last_second > 1_000 {
                println!("FPS: {}", fps);
                last_second = now;
                fps = 0;
            }

            let event = &mut self.handle_events();
            self.update(event, deltatime);
            self.render();
        }
    }

    fn update(&mut self, event: &mut Vec<Event>, deltatime: f64) {
        for x in self.gameobjects.iter_mut() {
            x.update(event, deltatime);
        }
    }

    fn render(&mut self) {

        let mut renderer = &mut self.context.renderer;

        // background color
        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();

        // Set object default color
        renderer.set_draw_color(Color::RGB(255, 0, 0));

        // draw the objects
        for x in self.gameobjects.iter() {
            x.draw(&mut renderer);
        }

        renderer.present();
    }

    fn handle_events(&mut self) -> Vec<Event> {

        let mut event_pump = self.context.sdl.event_pump().unwrap();
        let mut active_events = vec![];

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => self.running = false,
                Event::MouseButtonDown { mouse_btn: MouseButton::Left, .. } => {
                    active_events.push(event)
                }
                _ => self.running = true,
            };
        }

        active_events
    }
}
