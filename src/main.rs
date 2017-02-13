extern crate sdl2;
extern crate rand;

mod engine;
mod game;

fn main() {

    engine::gameobject::test();

    /*
    let mut game = game::Game::new();
    // just some debug information
    println!("{:?}", game);
    game.start();*/
}
