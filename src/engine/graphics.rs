use sdl2;
use sdl2::render::Renderer;

pub struct Graphics<'g> {
    pub sdl: sdl2::Sdl,
    pub screen: Renderer<'g>,
}

impl<'g> Graphics<'g> {
    // currently only needed for parsing the lifetime
}

pub fn init_renderer(title: &'static str, width: u32, height: u32) -> Graphics {

    // create raw SDL context
    let sdl_context = sdl2::init().unwrap();
    // TODO, ttf initializing

    // create window and renderer
    let window = sdl_context.video()
        .unwrap()
        .window(title, width, height)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let renderer = window.renderer().build().unwrap();

    // return struct
    Graphics {
        screen: renderer,
        sdl: sdl_context,
    }
}
