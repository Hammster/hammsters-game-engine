extern crate gl;
extern crate glm;
extern crate glutin;
extern crate libc;
extern crate rand;
extern crate yaml_rust;

mod engine;
mod game;

fn main() {
    let mut game = game::Game::new();
    game.start();
}
