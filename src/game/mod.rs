//! ### Game initializing

// sdl2
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;

//engine
use engine::gameobject::{GameObject, Entity};
use engine::graphics::Graphics;

// static frames
// TODO, This could be further improved
static DELAY_TIME: u32 = 1000 / 3 as u32; // 1_000/60 ~ 60fps

pub struct Game<'a> {
    pub display: Graphics<'a>,
    pub running: bool,
    pub gameobjects: Vec<Entity>,
}

pub fn new(display: Graphics) -> Game {

    let mut objects = vec![];

    // testdata
    let object: Entity = GameObject::new(10, 10, 30, 30, Color::RGB(255, 0, 0));
    let objecta: Entity = GameObject::new(50, 10, 30, 30, Color::RGB(0, 255, 0));
    let objectb: Entity = GameObject::new(90, 10, 30, 30, Color::RGB(0, 0, 255));

    // pupulate objects
    objects.push(object);
    objects.push(objecta);
    objects.push(objectb);

    Game {
        display: display,
        running: true,
        gameobjects: objects,
    }
}

impl<'a> Game<'a> {
    pub fn start(&mut self) {

        let mut timer = self.display.sdl.timer().unwrap();

        let mut frame_start: u32;
        let mut deltatime: u32;

        while self.running {
            frame_start = timer.ticks();

            self.handle_events();
            self.update();
            self.render();

            // same delay for slow down frame rate
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
        let mut renderer = &mut self.display.screen;

        // Painting the background
        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();

        // Call a Gamenobject
        for x in self.gameobjects.iter() {
            x.draw(&mut renderer);
        }
        renderer.present();
    }

    fn handle_events(&mut self) {
        let mut event_pump = self.display.sdl.event_pump().unwrap();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => self.running = false,
                _ => self.running = true,
            };
        }

    }
}
