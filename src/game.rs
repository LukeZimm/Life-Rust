use std::collections::HashMap;

use crate::chunk::Chunk;
use crate::chunk::Edges;
use crate::chunk::HoverChunk;

use kiss3d::event::{Modifiers, MouseButton};
use kiss3d::nalgebra::Point2;

use std::fs::File;
use std::io::prelude::*;

pub struct Game {
    pub map: HashMap<[i32; 2], Chunk>,
    bit_size: f32,
    relative_pos: (f32, f32),
    debug: bool,
    hover_chunk: HoverChunk,
}
impl Game {
    pub fn new(debug: bool) -> Game {
        let mut map: HashMap<[i32; 2], Chunk> = HashMap::new();
        map.insert([0, 0], Chunk::new([0, 0], 10.0, (0.0, 0.0)));
        Game {
            map,
            bit_size: 10.0,
            relative_pos: (0.0, 0.0),
            debug,
            hover_chunk: HoverChunk::new([0, 0], 10.0, (0.0, 0.0)),
        }
    }
    pub fn from(chunk: [u8; 8], debug: bool) -> Game {
        let mut map: HashMap<[i32; 2], Chunk> = HashMap::new();
        map.insert([0, 0], Chunk::from([0, 0], chunk, 10.0, (0.0, 0.0)));
        Game {
            map,
            bit_size: 10.0,
            relative_pos: (0.0, 0.0),
            debug,
            hover_chunk: HoverChunk::new([0, 0], 10.0, (0.0, 0.0)),
        }
    }
    pub fn from_chunk(chunk: Chunk, debug: bool) -> Game {
        let mut map: HashMap<[i32; 2], Chunk> = HashMap::new();
        map.insert([0, 0], chunk);
        Game {
            map,
            bit_size: 10.0,
            relative_pos: (0.0, 0.0),
            debug,
            hover_chunk: HoverChunk::new([0, 0], 10.0, (0.0, 0.0)),
        }
    }
    pub fn click(&mut self, sel_pos: Point2<f32>, button: MouseButton, modif: Modifiers) {
        let pos = (
            sel_pos.coords[0] - self.relative_pos.0 * self.bit_size,
            sel_pos.coords[1] - self.relative_pos.1 * self.bit_size,
        );
        let chunk: [i32; 2] = [
            (pos.0 / (self.bit_size * 8.0) + 0.5).floor() as i32,
            (pos.1 / (self.bit_size * 8.0) + 0.5).floor() as i32,
        ];
        let bit: [u8; 2] = [
            (pos.0 / self.bit_size - chunk[0] as f32 * 8.0 + 4.0).floor() as u8,
            (-pos.1 / self.bit_size + chunk[1] as f32 * 8.0 + 4.0).floor() as u8,
        ];
        println!("{:?} {:?}", pos, chunk);
        match self.map.get_mut(&chunk) {
            Some(i) => {
                i.toggle_bit((bit[0], bit[1]));
            }
            None => {
                let mut newchunk: Chunk = Chunk::new(chunk, self.bit_size, self.relative_pos);
                newchunk.toggle_bit((bit[0], bit[1]));
                self.map.insert(
                    chunk,
                    newchunk,
                );
            }
        }
    }
    pub fn hover(&mut self, hover_pos: Point2<f32>) {
        let pos = (
            hover_pos.coords[0] - self.relative_pos.0 * self.bit_size,
            hover_pos.coords[1] - self.relative_pos.1 * self.bit_size,
        );
        let chunk: [i32; 2] = [
            (pos.0 / (self.bit_size * 8.0) + 0.5).floor() as i32,
            (pos.1 / (self.bit_size * 8.0) + 0.5).floor() as i32,
        ];
        // println!("{:?} {:?}", pos, chunk);
        match self.map.get(&chunk) {
            Some(_) => self.hover_chunk.set_inactive(),
            None => self.hover_chunk.update_pos(chunk),
        }
    }
    pub fn pos(&mut self, pos: (f32, f32)) {
        let relative_pos = (
            self.relative_pos.0 + pos.0 * -10.0 / self.bit_size,
            self.relative_pos.1 + pos.1 * -10.0 / self.bit_size,
        );
        self.relative_pos = relative_pos;
        for chunk in self.chunks().into_iter() {
            chunk.update_pos(relative_pos);
        }
        self.hover_chunk.update_relative_pos(relative_pos);
    }
    pub fn zoom(&mut self, zoom_in: bool) {
        self.bit_size *= if zoom_in { 1.2 } else { 0.8 };
        self.update_zoom();
    }
    pub fn update_zoom(&mut self) {
        let size = self.bit_size;
        for chunk in self.chunks().into_iter() {
            chunk.update_zoom(size);
        }
        self.hover_chunk.update_zoom(size);
    }
    pub fn insert_chunk(&mut self, pos: [i32; 2], chunk: Chunk) {
        self.map.insert(pos, chunk);
    }
    pub fn draw(&mut self, window: &mut kiss3d::window::Window) {
        let debug = self.debug;
        for i in self.chunks() {
            i.draw(window, debug);
        }
        self.hover_chunk.draw(window);
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
    pub fn iterate(&mut self, window: &mut kiss3d::window::Window) {
        let mut activations: Vec<[i32; 2]> = Vec::new();
        let mut map_clone = self.map.clone();
        let mut edge_map: HashMap<[i32; 2], Edges> = HashMap::new();
        for i in Game::chunks_from_map(&mut map_clone) {
            let corners = self.corners(i.pos);
            let edges = self.edges(i.pos, corners);
            edge_map.insert(i.pos, edges);
        }
        let mut stale_chunks: Vec<[i32; 2]> = Vec::new();
        for i in self.chunks() {
            let v = i.iterate(&edge_map.get(&i.pos).unwrap()); // interpret activations
            Game::interpret_activations(v, &mut activations, i.pos);
            if !i.active {
                i.remove_nodes(window);
                stale_chunks.push(i.pos);
            }
        }
        for i in stale_chunks {
            self.map.remove(&i);
        }
        for i in activations {
            match self.map.get(&i) {
                Some(_) => {}
                None => {
                    self.map
                        .insert(i, Chunk::new(i, self.bit_size, self.relative_pos));
                }
            }
        }
    }
    fn interpret_activations(v: u8, activations: &mut Vec<[i32; 2]>, pos: [i32; 2]) {
        if Chunk::get_bit_at(v, 0) {
            activations.push([pos[0] - 1, pos[1] + 1])
        }
        if Chunk::get_bit_at(v, 1) {
            activations.push([pos[0], pos[1] + 1])
        }
        if Chunk::get_bit_at(v, 2) {
            activations.push([pos[0] + 1, pos[1] + 1])
        }
        if Chunk::get_bit_at(v, 3) {
            activations.push([pos[0] - 1, pos[1]])
        }
        if Chunk::get_bit_at(v, 4) {
            activations.push([pos[0] + 1, pos[1]])
        }
        if Chunk::get_bit_at(v, 5) {
            activations.push([pos[0] - 1, pos[1] - 1])
        }
        if Chunk::get_bit_at(v, 6) {
            activations.push([pos[0], pos[1] - 1])
        }
        if Chunk::get_bit_at(v, 7) {
            activations.push([pos[0] + 1, pos[1] - 1])
        }
    }
    pub fn set_chunk(&mut self, pos: [i32; 2], chunk: [u8; 8]) {
        self.map.get_mut(&pos).unwrap().set(chunk);
    }
    pub fn save(&mut self) -> std::io::Result<()> {
        let mut file = File::create("save.cgl")?;
        let mut data: Vec<u8> = vec![];
        for chunk in self.chunks() {
            let pos = chunk.pos;
            let mut res1: Vec<u8> = [pos[0].to_ne_bytes(), pos[1].to_ne_bytes()].concat();
            let mut res2: Vec<u8> = [res1, chunk.chunk.to_vec()].concat();
            file.write_all(&res2)?;
        }
        println!("{:?}", file.metadata());
        Ok(())
    }
    pub fn open(&mut self, window: &mut kiss3d::window::Window) -> std::io::Result<()> {
        let mut file = File::open("save.cgl")?;
        let mut bytes: Vec<u8> = vec![];
        for chunk in self.chunks() {
            chunk.remove_nodes(window);
        }
        self.map = HashMap::new();
        match file.read_to_end(&mut bytes) {
            Ok(_usize) => {}
            Err(e) => {
                println!("{}", e);
                return Err(e);
            }
        }
        for i in 0..(bytes.len() / 16) {
            let pos = (
                i32::from_ne_bytes(clone_into_array(&bytes[i * 16..i * 16 + 4])),
                i32::from_ne_bytes(clone_into_array(&bytes[i * 16 + 4..i * 16 + 8])),
            );
            let chunk: [u8; 8] = clone_into_array(&bytes[i * 16 + 8..i * 16 + 16]);
            // println!("pos: {:?}, data: {:?}", pos, chunk);
            self.map.insert(
                [pos.0, pos.1],
                Chunk::from([pos.0, pos.1], chunk, 10.0, self.relative_pos),
            );
        }
        self.relative_pos = (0.0, 0.0);
        self.bit_size = 10.0;
        Ok(())
    }
}

use std::convert::AsMut;

fn clone_into_array<A, T>(slice: &[T]) -> A
where
    A: Default + AsMut<[T]>,
    T: Clone,
{
    let mut a = A::default();
    <A as AsMut<[T]>>::as_mut(&mut a).clone_from_slice(slice);
    a
}
