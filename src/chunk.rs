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
            active: true,
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
    pub fn set_bit(mut self, pos: (u8, u8), val: bool) -> Self {
        if val {
            self.chunk[pos.1 as usize] |= (2 as u8).pow((7 - pos.0) as u32)
        } else {
            self.chunk[pos.1 as usize] &= (2 as u8).pow((7 - pos.0) as u32)
        }
        self
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
            if Chunk::get_bit_at(self.chunk[i], 0) {
                out = out | (2 as u8).pow((7 - i) as u32)
            }
        }
        out
    }
    pub fn right(&self) -> u8 {
        let mut out: u8 = 0;
        for i in 0..8 {
            if Chunk::get_bit_at(self.chunk[i], 7) {
                out |= (2 as u8).pow((7 - i) as u32)
            }
        }
        out
    }
    pub fn draw(&mut self, window: &mut kiss3d::window::Window) {
        self.remove_nodes(window);
        for y in 0..8 {
            self.draw_byte(self.chunk[y], y, window);
        }
        for x in 0..2 {
            for y in 0..2 {
                let a = (self.pixel_size * 4.0 - 0.5) * 2.0 * (x as f32 - 0.5);
                let b = (self.pixel_size * 4.0 - 0.5) * 2.0 * (y as f32 - 0.5);
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
    pub fn iterate(&mut self, edges: &Edges) {
        if !((edges.left == 0)
            && (edges.right == 0)
            && (edges.top == 0)
            && (edges.bottom == 0)
            && (edges.corners == 0))
        {
            self.active = true;
        }
        if self.active {
            let mut new_chunk: [u8; 8] = [0; 8];
            let mut empty = true;
            for y in 0..8 {
                let mut byte: u8 = 0b0000_0000;
                for x in 0..8 {
                    if self.survive((x, y), &edges) {
                        byte |= (2 as u8).pow((7 - x) as u32);
                        empty = false;
                    };
                }
                new_chunk[y as usize] = byte;
            }
            self.chunk = new_chunk;
            if empty {
                if (edges.left == 0)
                    && (edges.right == 0)
                    && (edges.top == 0)
                    && (edges.bottom == 0)
                    && (edges.corners == 0)
                {
                    self.active = false;
                }
            }
        }
    }
    pub fn survive(&self, point: (i8, i8), edges: &Edges) -> bool {
        let mut count = 0;
        for x in -1..2 {
            for y in -1..2 {
                if point.0 == 0 && x == -1 {
                    // left edge
                    if point.1 + y >= 0 && point.1 + y < 8 {
                        if Chunk::get_bit_at(edges.left, (point.1 + y) as u8) {
                            count += 1;
                        }
                    } else {
                        // corner case
                        if point.1 + y == -1 {
                            // top corner
                            if Chunk::get_bit_at(edges.corners, 2) {
                                count += 1;
                            }
                        }
                        if point.1 + y == 8 {
                            // bottom corner
                            if Chunk::get_bit_at(edges.corners, 0) {
                                count += 1;
                            }
                        }
                    }
                    continue;
                }
                if point.0 == 7 && x == 1 {
                    // right edge
                    if point.1 + y >= 0 && point.1 + y < 8 {
                        if Chunk::get_bit_at(edges.right, (point.1 + y) as u8) {
                            count += 1;
                        }
                    } else {
                        // corner case
                        if point.1 + y == -1 {
                            // top corner
                            if Chunk::get_bit_at(edges.corners, 3) {
                                count += 1;
                            }
                        }
                        if point.1 + y == 8 {
                            // bottom corner
                            if Chunk::get_bit_at(edges.corners, 1) {
                                count += 1;
                            }
                        }
                    }
                    continue;
                }
                if point.1 == 0 && y == -1 {
                    // top edge
                    if point.0 + x >= 0 && point.0 + x < 8 {
                        if Chunk::get_bit_at(edges.top, (point.0 + x) as u8) {
                            count += 1;
                        }
                    } else {
                    }
                    continue;
                }
                if point.1 == 7 && y == 1 {
                    // bottom edge
                    if point.0 + x >= 0 && point.0 + x < 8 {
                        if Chunk::get_bit_at(edges.bottom, (point.0 + x) as u8) {
                            count += 1;
                        }
                    } else {
                    }
                    continue;
                }
                if point.0 == 0 && point.1 == 1 {
                    let new_point = (point.0 + x, point.1 + y);
                    let b = self.get_bit_at_point(new_point);
                    // println!("({},{}): {}", x, y, b);
                }
                let new_point = (point.0 + x, point.1 + y);
                if self.get_bit_at_point(new_point) {
                    count += 1;
                }
            }
        }
        if self.get_bit_at_point(point) {
            count == 3 || count == 4
        } else {
            count == 3
        }
    }
    pub fn draw_byte(&mut self, byte: u8, y: usize, window: &mut kiss3d::window::Window) {
        for x in 0..8 {
            if Chunk::get_bit_at(byte, x) {
                let mut c = window.add_rectangle(self.pixel_size, self.pixel_size);
                // c.set_color(0.0, 0.0, 0.0);
                c.append_translation(&Translation2::new(
                    (x as f32 - 3.5) * self.pixel_size + self.center.0,
                    ((7 - y) as f32 - 3.5) * self.pixel_size + self.center.1,
                ));
                self.nodes.push(c);
            };
        }
    }
    pub fn get_bit_at(input: u8, n: u8) -> bool {
        let n = 7 - n;
        if n < 8 {
            input & (1 << n) != 0
        } else {
            panic!("{} is out of bounds", n);
        }
    }
    pub fn get_bit_at_point(&self, point: (i8, i8)) -> bool {
        Chunk::get_bit_at(self.chunk[point.1 as usize], point.0 as u8)
    }
    pub fn get_byte_at(&self, n: usize) -> u8 {
        self.chunk[n]
    }
    pub fn remove_nodes(&mut self, window: &mut kiss3d::window::Window) {
        for i in 0..self.nodes.len() {
            window.remove_planar_node(&mut self.nodes[i]);
        }
        self.nodes.clear();
    }
    pub fn print(&self) {
        for i in 0..8 {
            for j in 0..8 {
                print!(
                    "{}",
                    if Chunk::get_bit_at(self.chunk[i], j) {
                        1
                    } else {
                        0
                    }
                );
            }
            println!();
        }
    }
}

pub struct Edges {
    pub left: u8,
    pub right: u8,
    pub top: u8,
    pub bottom: u8,
    pub corners: u8,
}
