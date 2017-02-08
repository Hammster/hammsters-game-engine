extern crate sdl2;

mod engine;
mod game;

fn main() {
    //let renderer = engine::graphics::init_renderer("Rust Engine Test", 1040, 580);
    let mut game = game::Game::new();
    println!("{:?}", game);
    game.start();
}
