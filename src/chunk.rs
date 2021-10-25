use kiss3d::nalgebra::{Point2, Point3, Translation2};

#[derive(Clone)]
pub struct Chunk {
    chunk: [u8; 8],
    nodes: Vec<kiss3d::scene::PlanarSceneNode>,
    active: bool,
    pixel_size: f32,
    pub pos: [i32; 2],
    center: (f32, f32),
}
impl Chunk {
    const COLORS: (Point3<f32>, Point3<f32>) =
        (Point3::new(1.0, 0.0, 0.0), Point3::new(0.0, 1.0, 0.0));
    pub fn new(pos: [i32; 2]) -> Chunk {
        Chunk {
            chunk: [0b0000_0000; 8],
            nodes: Vec::new(),
            active: false,
            pixel_size: 10.0,
            pos,
            center: (pos[0] as f32 * 10.0 * 8.0, pos[1] as f32 * 10.0 * 8.0),
        }
    }
    pub fn from(pos: [i32; 2], chunk: [u8; 8]) -> Chunk {
        Chunk {
            chunk,
            nodes: Vec::new(),
            active: true,
            pixel_size: 10.0,
            pos,
            center: (pos[0] as f32 * 10.0 * 8.0, pos[1] as f32 * 10.0 * 8.0),
        }
    }
    pub fn set(&mut self, chunk: [u8; 8]) {
        self.chunk = chunk;
    }
    pub fn set_active(&mut self, val: bool) {
        self.active = val;
    }
    pub fn toggle_active(&mut self) {
        self.active = !self.active;
    }
    pub fn top(&self) -> u8 {
        self.chunk[0]
    }
    pub fn bottom(&self) -> u8 {
        self.chunk[7]
    }
    pub fn left(&self) -> u8 {
        let mut out: u8 = 0;
        for i in 0..8 {
            if self.get_bit_at(self.chunk[i], 7) {
                out = out | (2 as u8).pow(i as u32)
            }
        }
        out
    }
    pub fn right(&self) -> u8 {
        let mut out: u8 = 0;
        for i in 0..8 {
            if self.get_bit_at(self.chunk[i], 0) {
                out = out | (2 as u8).pow(i as u32)
            }
        }
        out
    }
    pub fn draw(&mut self, window: &mut kiss3d::window::Window) {
        self.remove_nodes(window);
        for i in 0..8 {
            self.draw_byte(self.chunk[i], i, window);
        }
        for i in 0..2 {
            for j in 0..2 {
                let a = (self.pixel_size * 4.0 - 0.5) * 2.0 * (i as f32 - 0.5);
                let b = (self.pixel_size * 4.0 - 0.5) * 2.0 * (j as f32 - 0.5);
                window.draw_planar_line(
                    &Point2::new(a + self.center.0, a + self.center.1),
                    &Point2::new(b + self.center.0, -b + self.center.1),
                    if self.active {
                        &Chunk::COLORS.1
                    } else {
                        &Chunk::COLORS.0
                    },
                );
            }
        }
    }
    pub fn iterate(&mut self, edges: Edges) -> Activate {
        let mut activate = Activate {
            left: false,
            right: false,
            top: false,
            bottom: false,
        };
        let mut new_chunk: [u8; 8] = [0; 8];
        for i in 0..8 {
            let mut byte: u8 = 0b0000_0000;
            for j in 0..8 {
                if self.survive((i, j), &edges) {
                    if i == 0 {
                        activate.top = true;
                    }
                    if i == 7 {
                        activate.bottom = true;
                    }
                    if j == 0 {
                        activate.left = true;
                    }
                    if j == 7 {
                        activate.right = true;
                    }
                    byte = byte | (2 as u8).pow(j.into())
                };
            }
            new_chunk[i as usize] = byte;
        }
        self.chunk = new_chunk;
        activate
    }
    fn survive(&self, point: (u8, u8), edges: &Edges) -> bool {
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
                if self.get_bit_at_point(self.chunk, new_point) {
                    count += 1
                }
            }
        }
        if self.get_bit_at_point(self.chunk, point) {
            count == 3 || count == 4
        } else {
            count == 3
        }
    }
    fn draw_byte(&mut self, byte: u8, i: usize, window: &mut kiss3d::window::Window) {
        for j in 0..8 {
            if self.get_bit_at(byte, j) {
                let mut c = window.add_rectangle(self.pixel_size, self.pixel_size);
                // c.set_color(0.0, 0.0, 0.0);
                c.append_translation(&Translation2::new(
                    ((7 - j) as f32 - 3.5) * self.pixel_size + self.center.0,
                    ((7 - i) as f32 - 3.5) * self.pixel_size + self.center.1,
                ));
                self.nodes.push(c);
            };
        }
    }
    fn get_bit_at(&self, input: u8, n: u8) -> bool {
        if n < 8 {
            input & (1 << n) != 0
        } else {
            panic!();
        }
    }
    fn get_bit_at_point(&self, chunk: [u8; 8], point: (u8, u8)) -> bool {
        self.get_bit_at(chunk[point.0 as usize], point.1)
    }
    fn remove_nodes(&mut self, window: &mut kiss3d::window::Window) {
        for i in 0..self.nodes.len() {
            window.remove_planar_node(&mut self.nodes[i]);
        }
        self.nodes.clear();
    }
}

pub struct Edges {
    pub left: u8,
    pub right: u8,
    pub top: u8,
    pub bottom: u8,
}

pub struct Activate {
    pub left: bool,
    pub right: bool,
    pub top: bool,
    pub bottom: bool,
}
