use sdl2;
use sdl2::render::Renderer;

pub struct Graphics {
    pub sdl: sdl2::Sdl,
    pub screen: Renderer<'static>
}

impl Graphics {
    // currently only needed for parsing the lifetime
    fn new(renderer : Renderer<'static>, sdl_context : sdl2::Sdl) -> Self {
        Graphics {
            screen: renderer,
            sdl: sdl_context,
        }
    }
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
    return Graphics::new(renderer, sdl_context)
}
