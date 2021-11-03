use std::collections::HashMap;

use crate::chunk::Chunk;
use crate::chunk::Edges;

use kiss3d::event::{Modifiers, MouseButton};
use kiss3d::nalgebra::Point2;

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
    pub fn from_chunk(chunk: Chunk) -> Game {
        let mut map: HashMap<[i32; 2], Chunk> = HashMap::new();
        map.insert([0, 0], chunk);
        Game { map }
    }
    pub fn click(&mut self, sel_pos: Point2<f32>, button: MouseButton, modif: Modifiers) {
        let chunk: [i32; 2] = [
            (sel_pos.coords[0] / 80.0 + 0.5).floor() as i32,
            (sel_pos.coords[1] / 80.0 + 0.5).floor() as i32,
        ]; // add support for bit size
           // println!("chunk: [{},{}]", chunk[0], chunk[1]);
        let bit: [u8; 2] = [
            (sel_pos.coords[0] / 10.0 - chunk[0] as f32 * 8.0 + 4.0).floor() as u8,
            (-sel_pos.coords[1] / 10.0 + chunk[1] as f32 * 8.0 + 4.0).floor() as u8,
        ];
        // println!("bit: [{},{}]", bit[0], bit[1]);
        match self.map.get_mut(&chunk) {
            Some(i) => {
                i.toggle_bit((bit[0], bit[1]));
            }
            None => {}
        }
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
    pub fn chunks_from_map(map: &mut HashMap<[i32; 2], Chunk>) -> Vec<&mut Chunk> {
        let mut vec: Vec<&mut Chunk> = Vec::new();
        for i in map.values_mut().collect::<Vec<_>>() {
            vec.push(i);
        }
        vec
    }
    pub fn edges(&self, pos: [i32; 2], corners: u8) -> Edges {
        let edges = Edges {
            left: match self.map.get(&[pos[0] - 1, pos[1]]) {
                Some(j) => j.right(),
                None => 0,
            },
            right: match self.map.get(&[pos[0] + 1, pos[1]]) {
                Some(j) => j.left(),
                None => 0,
            },
            top: match self.map.get(&[pos[0], pos[1] + 1]) {
                Some(j) => j.bottom(),
                None => 0,
            },
            bottom: match self.map.get(&[pos[0], pos[1] - 1]) {
                Some(j) => j.top(),
                None => 0,
            },
            corners: corners,
        };
        edges
    }
    pub fn edges_from_map(map: &mut HashMap<[i32; 2], Chunk>, pos: [i32; 2], corners: u8) -> Edges {
        let edges = Edges {
            left: match map.get(&[pos[0] - 1, pos[1]]) {
                Some(j) => j.right(),
                None => 0,
            },
            right: match map.get(&[pos[0] + 1, pos[1]]) {
                Some(j) => j.left(),
                None => 0,
            },
            top: match map.get(&[pos[0], pos[1] + 1]) {
                Some(j) => j.bottom(),
                None => 0,
            },
            bottom: match map.get(&[pos[0], pos[1] - 1]) {
                Some(j) => j.top(),
                None => 0,
            },
            corners: corners,
        };
        edges
    }
    pub fn corners(&self, pos: [i32; 2]) -> u8 {
        let map_clone = self.map.clone();
        let mut chunks = [
            self.map.get(&[pos[0] - 1, pos[1] - 1]),
            self.map.get(&[pos[0] + 1, pos[1] - 1]),
            self.map.get(&[pos[0] - 1, pos[1] + 1]),
            self.map.get(&[pos[0] + 1, pos[1] + 1]),
        ];
        let mut corners = [
            match chunks[0] {
                Some(i) => i.get_bit_at_point((7, 0)),
                None => false,
            },
            match chunks[1] {
                Some(i) => i.get_bit_at_point((0, 0)),
                None => false,
            },
            match chunks[2] {
                Some(i) => i.get_bit_at_point((7, 7)),
                None => false,
            },
            match chunks[3] {
                Some(i) => i.get_bit_at_point((0, 7)),
                None => false,
            },
        ];
        let mut int: u8 = 0;
        for i in 0..4 {
            if corners[i] {
                int |= (2 as u8).pow((7 - i) as u32);
            }
        }
        int
    }
    pub fn iterate(&mut self) {
        let mut activations: Vec<[i32; 2]> = Vec::new();
        let mut map_clone = self.map.clone();
        let mut edge_map: HashMap<[i32; 2], Edges> = HashMap::new();
        for i in Game::chunks_from_map(&mut map_clone) {
            let corners = self.corners(i.pos);
            let edges = self.edges(i.pos, corners);
            edge_map.insert(i.pos, edges);
        }
        for i in self.chunks() {
            i.iterate(&edge_map.get(&i.pos).unwrap());
        }
    }
    pub fn set_chunk(&mut self, pos: [i32; 2], chunk: [u8; 8]) {
        self.map.get_mut(&pos).unwrap().set(chunk);
    }
}
