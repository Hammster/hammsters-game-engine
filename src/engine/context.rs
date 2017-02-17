use std::fmt;

use glutin;
use gl;

pub struct Context {
    pub window: glutin::Window
}

impl Context {
    pub fn new(window_title: &str) -> Self {
        let window = glutin::Window::new().unwrap();
        window.set_title(window_title);

        unsafe { window.make_current() };

        unsafe {
            gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        }

        Context {
            window: window
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
