//! ### Game initializing
mod movabledev;

// sdl2
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::event::Event;

//engine
use engine::gameobject::{GameObject, Entity};
use engine::context::Context;

use game::movabledev::Movabledev;

// static frames
// TODO, This could be further improved
static DELAY_TIME: u32 = 1000 / 60 as u32; // 1_000/60 ~ 60fps

#[derive(Debug)]
pub struct Game {
    // add refactored context
    pub context: Context,
    pub running: bool,
    // TODO, make this type flexible
    pub gameobjects: Vec<Movabledev>,
}

impl Game {

    pub fn new() -> Game {
        // TODO, make it better remove the testing things
        let context = Context::new("Rust Engine",800,600);

        let mut objects = vec![];
        let object: Movabledev = GameObject::new(10, 10, 30, 30);
        objects.push(object);

        Game {
            context: context,
            running: true,
            gameobjects: objects,
        }
    }

    pub fn start(&mut self) {
        let mut timer = self.context.sdl.timer().unwrap();
        let mut frame_start: u32;
        let mut deltatime: u32;

        while self.running {
            frame_start = timer.ticks();

            let event = &mut self.handle_events();
            self.update(event);
            self.render();

            deltatime = timer.ticks() - frame_start;
            if deltatime < DELAY_TIME {
                timer.delay((DELAY_TIME - deltatime) as u32);
            }
        }
    }

    fn update(&mut self, event: &mut Vec<Event>) {
        for x in self.gameobjects.iter_mut() {
            x.update(event);
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

        let mut event_pump      = self.context.sdl.event_pump().unwrap();
        let mut active_events   = vec![];

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => self.running = false,
                Event::MouseButtonDown { mouse_btn: MouseButton::Left, .. } => active_events.push(event),
                _ => self.running = true,
            };
        }

        if !active_events.is_empty() {
            println!("{:?}", active_events );
        }

        active_events
    }

    fn loop_gameobjects(&mut self){
        for object in self.gameobjects.iter() {

        }
    }
}
