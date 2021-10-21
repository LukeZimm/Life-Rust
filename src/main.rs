extern crate kiss3d;
extern crate nalgebra as na;

use kiss3d::light::Light;
use kiss3d::nalgebra::{Point2, Point3, Translation2};
use kiss3d::window::Window;

mod chunk;
mod game;

use game::Game;

use std::io::{stdin, stdout, Write};
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

    // let mut nodes: Vec<kiss3d::scene::PlanarSceneNode> = Vec::new();
    // draw_chunk(bytes, &mut window, &mut nodes);
    game.draw(&mut window);
    while window.render() {
        game.draw(&mut window);
        game.iterate();
        /* draw_chunk(bytes, &mut window, &mut nodes); */

        // bytes = iterate(bytes);
        // pause();
        sleep(Duration::from_millis(100));
    }
    /* loop {
        println!();
        pause();
        println!();
        bytes = iterate(bytes);
        println!();
        display_chunk(bytes);
    } */
}

fn remove_nodes(
    window: &mut kiss3d::window::Window,
    nodes: &mut Vec<kiss3d::scene::PlanarSceneNode>,
) {
    for i in 0..nodes.len() {
        window.remove_planar_node(&mut nodes[i]);
    }
    nodes.clear();
}

fn draw_chunk(
    chunk: [u8; 8],
    window: &mut kiss3d::window::Window,
    nodes: &mut Vec<kiss3d::scene::PlanarSceneNode>,
) {
    for i in 0..8 {
        draw_byte(chunk[i], i, window, nodes);
    }
    window.draw_planar_line(
        &Point2::new(-35.5, -35.5),
        &Point2::new(-35.5, 45.5),
        &Point3::new(0.0, 1.0, 0.0),
    );
    window.draw_planar_line(
        &Point2::new(-35.5, -35.5),
        &Point2::new(45.5, -35.5),
        &Point3::new(0.0, 1.0, 0.0),
    );
    window.draw_planar_line(
        &Point2::new(-35.5, 45.5),
        &Point2::new(45.5, 45.5),
        &Point3::new(0.0, 1.0, 0.0),
    );
    window.draw_planar_line(
        &Point2::new(45.5, 45.5),
        &Point2::new(45.5, -35.5),
        &Point3::new(0.0, 1.0, 0.0),
    );
}

fn draw_byte(
    byte: u8,
    i: usize,
    window: &mut kiss3d::window::Window,
    nodes: &mut Vec<kiss3d::scene::PlanarSceneNode>,
) {
    for j in 0..8 {
        let mut c = window.add_rectangle(10.0, 10.0);
        if !get_bit_at(byte, j) {
            c.set_color(0.0, 0.0, 0.0);
        };
        c.append_translation(&Translation2::new(
            (j as f32 - 3.0) * 10.0,
            (i as f32 - 3.0) * 10.0,
        ));
        nodes.push(c);
    }
}

fn iterate(chunk: [u8; 8]) -> [u8; 8] {
    let mut new_chunk: [u8; 8] = [0; 8];
    for i in 0..8 {
        let mut byte: u8 = 0b0000_0000;
        for j in 0..8 {
            if survive(chunk, (i, j)) {
                byte = byte | (2 as u8).pow(j.into())
            };
        }
        new_chunk[i as usize] = byte;
    }
    new_chunk
}

fn survive(chunk: [u8; 8], point: (u8, u8)) -> bool {
    let mut count = 0;
    for i in 0..3 {
        if (point.0 == 0 && i == 0) || (point.0 == 7 && i == 2) {
            continue;
        }
        for j in 0..3 {
            if (point.1 == 0 && j == 0) || (point.1 == 7 && j == 2) {
                continue;
            }
            let new_point = (point.0 + i - 1, point.1 + j - 1);
            if get_bit_at_point(chunk, new_point) {
                count += 1
            }
        }
    }
    if get_bit_at_point(chunk, point) {
        count == 3 || count == 4
    } else {
        count == 3
    }
}

fn get_bit_at(input: u8, n: u8) -> bool {
    if n < 8 {
        input & (1 << n) != 0
    } else {
        panic!();
    }
}
fn get_bit_at_point(chunk: [u8; 8], point: (u8, u8)) -> bool {
    get_bit_at(chunk[point.0 as usize], point.1)
}
fn get_int_bit_at(input: u8, n: u8) -> u8 {
    if get_bit_at(input, n) {
        1
    } else {
        0
    }
}

fn display_chunk(chunk: [u8; 8]) {
    for i in 0..8 {
        display_byte(chunk[i]);
    }
}
fn display_byte(byte: u8) {
    for i in 0..8 {
        print!("{} ", get_int_bit_at(byte, i));
    }
    println!();
}

fn pause() {
    let mut stdout = stdout();
    stdin().read_line(&mut String::new()).unwrap();
    stdout.flush().unwrap();
}
