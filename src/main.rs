extern crate sdl2;

mod engine;
mod game;

fn main() {
    let mut game = game::Game::new();
    // just some debug information
    println!("{:?}", game);
    game.start();
}
