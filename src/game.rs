use std::collections::HashMap;

use crate::chunk::Activate;
use crate::chunk::Chunk;
use crate::chunk::Edges;

pub struct Game {
    pub map: HashMap<[i32; 2], Chunk>,
}
impl Game {
    pub fn new() -> Game {
        let mut map: HashMap<[i32; 2], Chunk> = HashMap::new();
        map.insert([0, 0], Chunk::new([0, 0]));
        Game { map }
    }
    pub fn from(chunk: [u8; 8]) -> Game {
        let mut map: HashMap<[i32; 2], Chunk> = HashMap::new();
        map.insert([0, 0], Chunk::from([0, 0], chunk));
        Game { map }
    }
    pub fn insert_chunk(&mut self, pos: [i32; 2], chunk: Chunk) {
        self.map.insert(pos, chunk);
    }
    pub fn draw(&mut self, window: &mut kiss3d::window::Window) {
        for i in self.chunks() {
            i.draw(window);
        }
    }
    pub fn chunks(&mut self) -> Vec<&mut Chunk> {
        let mut vec: Vec<&mut Chunk> = Vec::new();
        for i in self.map.values_mut().collect::<Vec<_>>() {
            vec.push(i);
        }
        vec
    }
    pub fn iterate(&mut self) {
        let mut activations: Vec<[i32; 2]> = Vec::new();
        let map_clone = self.map.clone();
        for i in self.chunks() {
            let edges = Edges {
                left: match map_clone.get(&[i.pos[0] - 1, i.pos[1]]) {
                    Some(j) => j.right(),
                    None => 0,
                },
                right: match map_clone.get(&[i.pos[0] + 1, i.pos[1]]) {
                    Some(j) => j.left(),
                    None => 0,
                },
                top: match map_clone.get(&[i.pos[0], i.pos[1] + 1]) {
                    Some(j) => j.bottom(),
                    None => 0,
                },
                bottom: match map_clone.get(&[i.pos[0], i.pos[1] - 1]) {
                    Some(j) => j.top(),
                    None => 0,
                },
            };
            let activate = i.iterate(edges);
            /* if activate.left {
                activations.push([i.pos[0] - 1, i.pos[1]]);
            }
            if activate.right {
                activations.push([i.pos[0] + 1, i.pos[1]]);
            }
            if activate.top {
                activations.push([i.pos[0], i.pos[1] + 1]);
            }
            if activate.bottom {
                activations.push([i.pos[0], i.pos[1] - 1]);
            } */
        }
        for i in activations {
            match self.map.get_mut(&i) {
                Some(j) => {
                    j.set_active(true);
                }
                None => {
                    self.map.insert(i, Chunk::new([0, 0]));
                }
            }
        }
    }
    pub fn set_chunk(&mut self, pos: [i32; 2], chunk: [u8; 8]) {
        self.map.get_mut(&pos).unwrap().set(chunk);
    }
}
