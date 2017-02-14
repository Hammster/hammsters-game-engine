use sdl2::event::Event;
use sdl2::render::Renderer;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
//use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

use rand;
use game::Game;
use engine::gameobject::GameObject;

#[derive(Debug)]
pub struct Ball {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
    pub pong: bool,
    pub color: Color,
    pub angle: f64,
}

impl Ball {
    fn is_colliding(&self, x: i32, y: i32) -> bool {
        (x >= self.x && x <= self.x + self.w as i32) && (y >= self.y && y <= self.y + self.h as i32)
    }

    fn reset(&mut self) {
        self.angle = 90.0;
        self.color = Color::RGB(255, 255, 255);
    }
}

impl GameObject for Ball {
    fn new(x: i32, y: i32, w: u32, h: u32) -> Self {
        Ball {
            x: x,
            y: y,
            w: w,
            h: h,
            pong: true,
            angle: 90.0,
            color: Color::RGB(255, 255, 255),
        }
    }

    // filling the backbuffer of the render function
    fn draw(&self, renderer: &mut Renderer) {
        renderer.set_draw_color(self.color);
        renderer.fill_rect(Rect::new(self.x, self.y, self.w, self.h)).unwrap();
    }

    // called on each update from the main loop
    fn update(&mut self, event: &Vec<Event>, deltatime: f64, game: &Game) {

        self.reset();

        if self.pong {
            self.x += 3;
        } else {
            self.x -= 3;
        }

        if !self.pong && self.x <= 10 {
            self.pong = true;
        } else if self.pong && self.x >= 410 {
            self.pong = false;
        }
    }
}
