//! ### Game initializing
mod square;
mod player;
mod ball;
mod opponent;

//engine
use engine::gameobject::{GameObject, GameObjectManager};
use engine::event::EventManager;
use engine::scene::{Scene, SceneManager, State};
use engine::context::Context;

use game::square::Square;
use game::ball::Ball;
use game::player::Player;
use game::opponent::Opponent;

#[derive(Debug)]
pub struct Game {
    pub context: Context,
    pub scenemanager: SceneManager,
    pub currentscene: &'static str,
}

impl Game {
    pub fn new() -> Game {
        let ww = 800; 
        let wh = 600;

        let context = Context::new("Rust Engine", ww, wh);

        let mut scenemanager = SceneManager::new();

        scenemanager.create("main");
        scenemanager.create("menu");
        
        scenemanager.get("main").unwrap().state = State::Running;

        let player: Player = GameObject::new(10, 10, 30, (wh/3)-20 as u32);
        let opponent: Square = GameObject::new((ww - 40) as i32, 10, 30, (wh/3)-20 as u32);
        let object: Opponent = GameObject::new(750, 10, 30, 170);

        let ball: Ball = GameObject::new((ww/2) as i32, (wh/2) as i32, 30, 30);

        scenemanager.get("main").unwrap().gameobjectmanager.insert("Player", Box::new(player));
        scenemanager.get("main").unwrap().gameobjectmanager.insert("Opponent", Box::new(opponent));
        scenemanager.get("main").unwrap().gameobjectmanager.insert("Ball", Box::new(ball));

        Game {
            context: context,
            scenemanager: scenemanager, 
            currentscene: "main"
        }
    }

    pub fn start(&mut self) {

        // borrow timer context
        let mut timer = self.context.sdl.timer().unwrap();

        let interval = 1_000 / 60; // capped physics frames

        let mut before = timer.ticks();
        let mut last_second = timer.ticks();
        let mut fps = 0u16;

        while self.scenemanager.is_running("main") {
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
        for (id, object) in &mut self.scenemanager.get(self.currentscene).unwrap().gameobjectmanager.objects {
            object.update(event, deltatime);
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
        for (id, object) in &mut self.scenemanager.get(self.currentscene).unwrap().gameobjectmanager.objects {
            object.draw(&mut renderer);
        }

        renderer.present();
    }

    fn handle_events(&mut self) -> Vec<Event> {

        let mut event_pump = self.context.sdl.event_pump().unwrap();
        let mut active_events = vec![];

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    self.scenemanager.get("main").unwrap().stop()
                }
                Event::MouseButtonDown { mouse_btn: MouseButton::Left, .. } => {
                    active_events.push(event)
                }
                _ => (),
            };
        }

        active_events
    }
}
