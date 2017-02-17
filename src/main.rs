extern crate gl;
extern crate libc;
extern crate glutin;
extern crate rand;
extern crate yaml_rust;

mod engine;
mod game;

fn main() {

    let mut config = engine::config::Config::load();

    let mut game = game::Game::new();
    game.start();
}
