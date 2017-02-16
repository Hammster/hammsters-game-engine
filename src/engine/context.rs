use std::fmt;
use glium;
use glium::backend::Facade;


pub struct Context {
    pub display: Facade,
}

impl Context {
    pub fn new(window_title: &str, screen_width: u32, screen_height: u32) -> Self {
        use glium::DisplayBuild;
        let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

        Context {
            display: display,
        }
    }
}



impl fmt::Debug for Context {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            _ => write!(f, "OK"),
        }
    }
}
