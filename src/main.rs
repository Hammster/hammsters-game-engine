extern crate glium;
extern crate glium_text;
extern crate rand;

mod engine;
mod game2;

fn main() {
    let mut game = game2::Game::new();
    game.start();
}
