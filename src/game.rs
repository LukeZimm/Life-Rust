use std::collections::HashMap;

use crate::chunk::Chunk;

pub struct Game {
    pub map: HashMap<[i32; 2], Option<Chunk>>,
}
impl Game {
    pub fn new() -> Game {
        let mut map: HashMap<[i32; 2], Option<Chunk>> = HashMap::new();
        map.insert([0, 0], Some(Chunk::new()));
        Game { map }
    }
    pub fn from(chunk: [u8; 8]) -> Game {
        let mut map: HashMap<[i32; 2], Option<Chunk>> = HashMap::new();
        map.insert([0, 0], Some(Chunk::from(chunk)));
        Game { map }
    }
    pub fn draw(&mut self, window: &mut kiss3d::window::Window) {
        for mut i in self.chunks() {
            match i {
                Some(j) => j.draw(window),
                None => {}
            }
        }
    }
    pub fn chunks(&mut self) -> Vec<&mut Option<Chunk>> {
        let mut vec: Vec<&mut Option<Chunk>> = Vec::new();
        for i in self.map.values_mut().collect::<Vec<_>>() {
            vec.push(i);
        }
        vec
    }
    pub fn iterate(&mut self) {
        for mut i in self.chunks() {
            match i {
                Some(j) => {
                    j.iterate();
                }
                None => {}
            }
        }
    }
    pub fn set_chunk(&mut self, pos: [i32; 2], chunk: [u8; 8]) {
        match self.map.get_mut(&pos).unwrap() {
            Some(i) => i.set(chunk),
            None => {
                self.map.insert(pos, Some(Chunk::from(chunk)));
            }
        }
    }
}
