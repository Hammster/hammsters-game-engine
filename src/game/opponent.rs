
use sdl2::event::Event;
use sdl2::render::Renderer;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
//use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

use rand;
use engine::gameobject::{GameObject};

#[derive(Debug)]
pub struct Opponent {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
    pub pong: bool,
    pub clicked: bool,
    pub color: Color,
}

impl Opponent {
    fn is_clicked(&self, x: i32, y: i32) -> bool {
        (x >= self.x && x <= self.x + self.w as i32) && (y >= self.y && y <= self.y + self.h as i32)
    }

    fn reset(&mut self) {
        self.clicked = false;
    }
}

impl GameObject for Opponent {
    fn new(x: i32, y: i32, w: u32, h: u32) -> Self {
        Opponent {
            x: x,
            y: y,
            w: w,
            h: h,
            pong: true,
            clicked: false,
            color: Color::RGB(255, 0, 0),
        }
    }

    // filling the backbuffer of the render function
    fn draw(&self, renderer: &mut Renderer) {
        renderer.set_draw_color(self.color);
        renderer.fill_rect(Rect::new(self.x, self.y, self.w, self.h)).unwrap();
    }

    // called on each update from the main loop
    fn update(&mut self, event: &Vec<Event>, deltatime: f64) {

        self.reset();

        for e in event.iter() {
            match *e {
                Event::MouseButtonDown { mouse_btn: MouseButton::Left, x: x1, y: y1, .. } => {
                    self.clicked = self.is_clicked(x1, y1)
                }
                _ => self.clicked = false,
            }
        }

        if self.clicked {
            println!("{:?}", rand::random::<Color>());
            self.color = rand::random::<Color>();
        }

        if self.pong {
            self.y += 3;
        } else {
            self.y -= 3;
        }

        if !self.pong && self.y <= 10 {
            self.pong = true;
        } else if self.pong && self.y >= 410 {
            self.pong = false;
        }
    }
}
