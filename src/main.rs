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
        0b0000_0010,
        0b1110_0100,
        0b0000_0111,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
    ]);
    game.insert_chunk(
        [1, 0],
        Chunk::from(
            [1, 0],
            [
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0111,
            ],
        ),
    );
    println!("{}", game.map.get(&[0, 0]).unwrap().left());

    game.draw(&mut window);
    while window.render() {
        game.draw(&mut window);
        game.iterate();
        sleep(Duration::from_millis(100));
    }
}
