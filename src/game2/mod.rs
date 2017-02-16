use engine::gameobject::{GameObject, GameObjectManager};
use engine::scene::{Scene, SceneManager, State};
use engine::context::Context;

use glium::Surface;

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

        Game {
            context: context,
            scenemanager: scenemanager, 
            currentscene: "main"
        }
    }

    pub fn start(&mut self) {

        //TODO, new loop

        /*
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
        */
    }

    fn update(&mut self) {
        for (id, object) in &mut self.scenemanager.get(self.currentscene).unwrap().gameobjectmanager.objects {
            object.update();
        }
    }

    fn render(&mut self) {
        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.finish().unwrap();

        // draw the objects
        for (id, object) in &mut self.scenemanager.get(self.currentscene).unwrap().gameobjectmanager.objects {
            //target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
            object.draw(&mut renderer);
        }
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
