use std::fmt;

use glutin;
use gl;
use engine;

pub struct Context {
    pub window: glutin::Window
}

impl Context {
    pub fn new() -> Self {
        let config = engine::config::Config::load();
        let mut builder = glutin::WindowBuilder::new();

        println!("{:?}", &config);

        if config.fullscreen {
            // currently only needed on fullscreen
            let monitor = glutin::get_available_monitors()
                .nth(0)
                .expect("Please enter a valid ID");
            // enable fullscreen
            builder = builder.with_fullscreen(monitor);
        }

        let window = builder
            .with_title(config.title)
            .build()
            .unwrap();

        unsafe { window.make_current() };

        unsafe {
            gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        }

        // for streams ;)
        // window.set_position(2000,200);

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
