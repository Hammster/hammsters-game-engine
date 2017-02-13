extern crate sdl2;
extern crate rand;

mod engine;
mod game;

fn main() {
    let mut game = game::Game::new();
    game.start();
}
