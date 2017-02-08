//! ### Game initializing

// sdl2
use sdl2;
use sdl2::render::Renderer;
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;

//engine
use engine::gameobject::{GameObject, Entity};
use engine::context::Context;

// static frames
// TODO, This could be further improved
static DELAY_TIME: u32 = 1000 / 3 as u32; // 1_000/60 ~ 60fps

#[derive(Debug)]
pub struct Game {
    // add refactored context
    pub context: Context,
    pub running: bool,
    pub gameobjects: Vec<Entity>,
}

impl Game {

    pub fn new() -> Game {
        // TODO, make it better remove the testing things
        let context = Context::new("Rust Engine",800,600);

        let mut objects = vec![];
        let object: Entity = GameObject::new(10, 10, 30, 30);
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

            self.handle_events();
            self.update();
            self.render();

            deltatime = timer.ticks() - frame_start;
            if deltatime < DELAY_TIME {
                timer.delay((DELAY_TIME - deltatime) as u32);
            }
        }
    }

    fn update(&mut self) {
        for x in self.gameobjects.iter_mut() {
            x.update();
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

    fn handle_events(&mut self) {

        let mut event_pump = self.context.sdl.event_pump().unwrap();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => self.running = false,
                _ => self.running = true,
            };
        }

    }
}
