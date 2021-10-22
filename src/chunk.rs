use kiss3d::nalgebra::{Point2, Point3, Translation2};

pub struct Chunk {
    chunk: [u8; 8],
    nodes: Vec<kiss3d::scene::PlanarSceneNode>,
    active: bool,
    pixel_size: f32,
    center: (f32, f32),
}
impl Chunk {
    const COLORS: (Point3<f32>, Point3<f32>) =
        (Point3::new(1.0, 0.0, 0.0), Point3::new(0.0, 1.0, 0.0));
    pub fn new() -> Chunk {
        Chunk {
            chunk: [0b0000_0000; 8],
            nodes: Vec::new(),
            active: false,
            pixel_size: 10.0,
            center: (0.0, 0.0),
        }
    }
    pub fn from(chunk: [u8; 8]) -> Chunk {
        Chunk {
            chunk,
            nodes: Vec::new(),
            active: true,
            pixel_size: 10.0,
            center: (0.0, 0.0),
        }
    }
    pub fn set(&mut self, chunk: [u8; 8]) {
        self.chunk = chunk;
    }
    pub fn draw(&mut self, window: &mut kiss3d::window::Window) {
        self.remove_nodes(window);
        for i in 0..8 {
            self.draw_byte(self.chunk[i], i, window);
        }
        for i in 0..2 {
            for j in 0..2 {
                let a = (self.pixel_size * 4.0 + 0.5) * 2.0 * (i as f32 - 0.5);
                let b = (self.pixel_size * 4.0 + 0.5) * 2.0 * (j as f32 - 0.5);
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
    pub fn iterate(&mut self) {
        let mut new_chunk: [u8; 8] = [0; 8];
        for i in 0..8 {
            let mut byte: u8 = 0b0000_0000;
            for j in 0..8 {
                if self.survive((i, j)) {
                    byte = byte | (2 as u8).pow(j.into())
                };
            }
            new_chunk[i as usize] = byte;
        }
        self.chunk = new_chunk;
    }
    fn survive(&self, point: (u8, u8)) -> bool {
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
            let mut c = window.add_rectangle(self.pixel_size, self.pixel_size);
            if !self.get_bit_at(byte, j) {
                c.set_color(0.0, 0.0, 0.0);
            };
            c.append_translation(&Translation2::new(
                (j as f32 - 3.5) * self.pixel_size + self.center.0,
                (i as f32 - 3.5) * self.pixel_size + self.center.1,
            ));
            self.nodes.push(c);
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
