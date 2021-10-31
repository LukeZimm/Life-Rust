extern crate kiss3d;
extern crate nalgebra as na;

use kiss3d::light::Light;
use kiss3d::window::Window;

mod chunk;
mod game;

use chunk::Chunk;
use game::Game;

use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut window = Window::new("Conway's Game of Life");
    window.set_light(Light::StickToCamera);
    let mut game = Game::from([
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0000_0100,
        0b0000_1000,
        0b0000_1110,
        0b0000_0000,
        0b0000_0000,
    ]);
    for x in -3..4 {
        for y in -3..4 {
            if !(x == 0 && y == 0) {
                game.insert_chunk([x, y], Chunk::new([x, y]));
            }
        }
    }
    game.insert_chunk([-1, -1], Chunk::new([-1, -1]).set_bit((7, 0), true));
    println!("{}", game.corners([0, 0]));
    game.draw(&mut window);
    while window.render() {
        game.draw(&mut window);
        game.iterate();
        sleep(Duration::from_millis(500));
    }
}
