use sdl2::event::Event;
use sdl2::render::Renderer;
use sdl2::rect::Rect;

use engine::gameobject::GameObject;

#[derive(Debug)]
pub struct Movabledev {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
    pub pong: bool,
}

impl Movabledev {
    pub fn is_clicked (&self, event: &Vec<Event>) -> bool{
        //( x >= self.x && x <= self.x + self.w as i32 ) && ( y >= self.y && y <= self.y + self.h as i32 )
        false
    }
}

impl GameObject for Movabledev {
    fn new(x: i32, y: i32, w: u32, h: u32) -> Self {
        Movabledev {
            x: x,
            y: y,
            w: w,
            h: h,
            pong: true,
        }
    }

    // filling the backbuffer of the render function
    fn draw(&self, renderer: &mut Renderer) {
        renderer.fill_rect(Rect::new(self.x, self.y, self.w, self.h)).unwrap();
    }

    // called on each update from the main loop
    fn update(&mut self, event: &Vec<Event>) {
        if self.pong {
            self.y += 1;
        } else {
            self.y -= 1;
        }

        if !self.pong && self.y <= 10 {
            self.pong = true;
        } else if self.pong && self.y >= 100 {
            self.pong = false;
        }

        if self.is_clicked(event) {
            println!("object was clicked");
        }

    }

}
