use sdl2::render::Renderer;
use sdl2::rect::Rect;

// Base Information
// TODO, Texture
#[derive(Debug)]
pub struct Entity {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
}

impl Entity {
    // nothing
}

pub trait GameObject {
    fn new(x: i32, y: i32, w: u32, h: u32) -> Self;
    fn draw(&self, renderer: &mut Renderer);
    fn update(&mut self);
}

impl GameObject for Entity {
    fn new(x: i32, y: i32, w: u32, h: u32) -> Self {
        Entity {
            x: x,
            y: y,
            w: w,
            h: h,
        }
    }

    // filling the backbuffer of the render function
    fn draw(&self, renderer: &mut Renderer) {
        renderer.draw_rect(Rect::new(self.x, self.y, self.w, self.h)).unwrap();
        // '?' operator for error handling instead of unwrap() ???
    }

    // called on each update from the main loop
    fn update(&mut self) {
        // for example
        self.y += self.h as i32 + 3;
    }
}
