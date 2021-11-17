extern crate kiss3d;

use kiss3d::event::{Action, WindowEvent};
use kiss3d::light::Light;
use kiss3d::nalgebra::{Point2, Vector2};
use kiss3d::planar_camera::*;
use kiss3d::window::Window;

mod chunk;
mod game;

use chunk::Chunk;
use game::Game;

use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut window = Window::new("Conway's Game of Life");
    let mut camera = kiss3d::planar_camera::FixedView::new();
    window.set_light(Light::StickToCamera);
    let mut game = Game::from(
        [
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
        ],
        true,
    );
    for x in -3..4 {
        for y in -3..4 {
            if !(x == 0 && y == 0) {
                game.insert_chunk([x, y], Chunk::new([x, y], 10.0, (0.0, 0.0)));
            }
        }
    }
    game.insert_chunk(
        [-3, 3],
        Chunk::from(
            [-3, 3],
            [
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0110_0000,
                0b0110_0000,
                0b0000_0000,
                0b0000_0000,
            ],
            10.0,
            (0.0, 0.0),
        ),
    );
    game.insert_chunk(
        [-2, 3],
        Chunk::from(
            [-2, 3],
            [
                0b0000_0000,
                0b0000_0000,
                0b0000_0110,
                0b0000_1000,
                0b0001_0000,
                0b0001_0001,
                0b0001_0000,
                0b0000_1000,
            ],
            10.0,
            (0.0, 0.0),
        ),
    );
    game.insert_chunk(
        [-1, 3],
        Chunk::from(
            [-1, 3],
            [
                0b0000_0000,
                0b0000_0001,
                0b0000_0110,
                0b1000_0110,
                0b0100_0110,
                0b0110_0001,
                0b0100_0000,
                0b1000_0000,
            ],
            10.0,
            (0.0, 0.0),
        ),
    );
    game.insert_chunk(
        [-2, 2],
        Chunk::from(
            [-2, 2],
            [
                0b0000_0110,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
            ],
            10.0,
            (0.0, 0.0),
        ),
    );
    game.insert_chunk(
        [0, 3],
        Chunk::from(
            [0, 3],
            [
                0b0100_0000,
                0b0100_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0100_0000,
                0b0100_0000,
                0b0000_0000,
            ],
            10.0,
            (0.0, 0.0),
        ),
    );
    game.insert_chunk(
        [1, 3],
        Chunk::from(
            [1, 3],
            [
                0b0000_0000,
                0b0000_0000,
                0b0001_1000,
                0b0001_1000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
            ],
            10.0,
            (0.0, 0.0),
        ),
    );
    let mut last_pos = Point2::new(0.0f32, 0.0f32);
    let mut sel_pos = Point2::new(0.0f32, 0.0f32);
    let mut run: bool = false;
    game.draw(&mut window);
    while window.render_with(None, Some(&mut camera), None) {
        for event in window.events().iter() {
            match event.value {
                WindowEvent::CursorPos(x, y, _modif) => {
                    let window_size =
                        Vector2::new(window.size()[0] as f32, window.size()[1] as f32);
                    last_pos = Point2::new(x as f32, y as f32);
                    sel_pos = camera.unproject(&last_pos, &window_size);
                    game.hover(sel_pos)
                }
                WindowEvent::MouseButton(button, Action::Press, modif) => {
                    // println!("mouse press event on {:?} with {:?}", button, modif);
                    let window_size =
                        Vector2::new(window.size()[0] as f32, window.size()[1] as f32);
                    sel_pos = camera.unproject(&last_pos, &window_size);
                    // println!("{:?}", sel_pos);
                    game.click(sel_pos, button, modif);
                    game.draw(&mut window);
                }
                WindowEvent::Key(key, action, modif) => {
                    println!("key event {:?} on {:?} with {:?}", key, action, modif);
                    // Play, Pause, Iterate and Clear
                    if key == kiss3d::event::Key::Space
                        && action == kiss3d::event::Action::Release
                        && modif == kiss3d::event::Modifiers::Control
                    {
                        run = !run;
                    } else if key == kiss3d::event::Key::Space
                        && action == kiss3d::event::Action::Release
                    {
                        run = false;
                        game.iterate(&mut window);
                    }
                    if key == kiss3d::event::Key::Back && action == kiss3d::event::Action::Release {
                        // Backspace
                        for chunk in game.chunks().into_iter() {
                            chunk.set([0; 8]);
                        }
                    }
                    // Zooming
                    if key == kiss3d::event::Key::Equals && action == kiss3d::event::Action::Press {
                        // +
                        game.zoom(true);
                    }
                    if key == kiss3d::event::Key::Minus && action == kiss3d::event::Action::Press {
                        // -
                        game.zoom(false);
                    }
                    // Arrow Keys
                    if key == kiss3d::event::Key::Up && action == kiss3d::event::Action::Press {
                        // Up
                        let val = if modif == kiss3d::event::Modifiers::Shift {
                            5.0
                        } else {
                            1.0
                        };
                        game.pos((0.0, val));
                    }
                    if key == kiss3d::event::Key::Down && action == kiss3d::event::Action::Press {
                        // Down
                        let val = if modif == kiss3d::event::Modifiers::Shift {
                            5.0
                        } else {
                            1.0
                        };
                        game.pos((0.0, -val));
                    }
                    if key == kiss3d::event::Key::Left && action == kiss3d::event::Action::Press {
                        // Left
                        let val = if modif == kiss3d::event::Modifiers::Shift {
                            5.0
                        } else {
                            1.0
                        };
                        game.pos((-val, 0.0));
                    }
                    if key == kiss3d::event::Key::Right && action == kiss3d::event::Action::Press {
                        // Right
                        let val = if modif == kiss3d::event::Modifiers::Shift {
                            5.0
                        } else {
                            1.0
                        };
                        game.pos((val, 0.0));
                    }
                    // Ctrl Commands
                    if key == kiss3d::event::Key::S
                        && action == kiss3d::event::Action::Release
                        && modif == kiss3d::event::Modifiers::Control
                    {
                        // Save
                        game.save();
                    }
                    if key == kiss3d::event::Key::O
                        && action == kiss3d::event::Action::Release
                        && modif == kiss3d::event::Modifiers::Control
                    {
                        // Open
                        match game.open(&mut window) {
                            Ok(t) => {}
                            Err(e) => {
                                println!("{:?}", e)
                            }
                        }
                    }
                }
                WindowEvent::CursorPos(x, y, _modif) => {
                    last_pos = Point2::new(x as f32, y as f32);
                }
                WindowEvent::Close => {
                    //save
                    println!("closing");
                }
                WindowEvent::Touch(var1, x, y, touchAction, modif) => {
                    println!("{} {} {} {:?} {:?}", var1, x, y, touchAction, modif);
                }
                _ => {}
            }
        }
        game.draw(&mut window);
        if run {
            game.iterate(&mut window);
            // sleep(Duration::from_millis(100)); // write a better alternative
        }
    }
}
