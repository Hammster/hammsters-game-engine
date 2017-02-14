use std::fmt;

use sdl2::{self, Sdl};
use sdl2::render::Renderer;
use sdl2::video::{Window, WindowBuildError};

pub struct Context {
    pub sdl: Sdl,
    pub renderer: Renderer<'static>,
}

impl Context {
    pub fn new(window_title: &str, screen_width: u32, screen_height: u32) -> Self {
        let sdl = sdl2::init().unwrap();
        let video = sdl.video().unwrap();
        let window = init_window(video, window_title, screen_width, screen_height).unwrap();
        let renderer = window.renderer().accelerated().build().unwrap();

        Context {
            sdl: sdl,
            renderer: renderer,
        }
    }
}

fn init_window(video: sdl2::VideoSubsystem,
               window_title: &str,
               screen_width: u32,
               screen_height: u32)
               -> Result<Window, WindowBuildError> {
    video.window(window_title, screen_width, screen_height)
        .position_centered()
        .opengl()
        .build()
}

impl fmt::Debug for Context {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            _ => write!(f, "OK"),
        }
    }
}
