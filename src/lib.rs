mod chunk;
mod game;

use chunk::Chunk;
use chunk::Edges;
use game::Game;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_bit() {
        let byte = 0b1000_0010;
        assert_eq!(Chunk::get_bit_at(byte, 0), true);
        assert_eq!(Chunk::get_bit_at(byte, 1), false);
        assert_eq!(Chunk::get_bit_at(byte, 6), true);
        assert_eq!(Chunk::get_bit_at(byte, 7), false);
    }
    #[test]
    fn get_bit_point() {
        let chunk = Chunk::from(
            [0, 0],
            [
                0b0100_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0010_0000,
            ],
            10.0,
        );
        assert_eq!(chunk.get_bit_at_point((0, 0)), false);
        assert_eq!(chunk.get_bit_at_point((1, 0)), true);
        assert_eq!(chunk.get_bit_at_point((0, 1)), false);
        assert_eq!(chunk.get_bit_at_point((2, 7)), true);
    }
    #[test]
    fn get_byte() {
        let chunk = Chunk::from(
            [0, 0],
            [
                0b0000_0001,
                0b0000_0010,
                0b0000_0100,
                0b0001_0111,
                0b1110_0000,
                0b0101_0101,
                0b0111_1111,
                0b1111_1111,
            ],
            10.0,
        );
        assert_eq!(chunk.get_byte_at(0), 0b0000_0001);
        assert_eq!(chunk.get_byte_at(7), 0b1111_1111);
        assert_eq!(chunk.get_byte_at(3), 0b0001_0111);
    }
    #[test]
    fn set_bit() {
        let mut chunk = Chunk::new([0, 0], 10.0)
            .set_bit((0, 0), true)
            .set_bit((1, 0), true)
            .set_bit((0, 1), true)
            .set_bit((2, 2), true);
        assert_eq!(chunk.get_byte_at(0), 0b1100_0000);
        assert_eq!(chunk.get_byte_at(1), 0b1000_0000);
        assert_eq!(chunk.get_byte_at(2), 0b0010_0000);
        chunk = chunk
            .set_bit((0, 0), false)
            .set_bit((2, 0), false)
            .set_bit((2, 1), true);
        assert_eq!(chunk.get_byte_at(0), 0b0100_0000);
        assert_eq!(chunk.get_byte_at(1), 0b1010_0000);
        chunk = chunk.set_bit((0, 0), false).set_bit((1, 0), true);
        assert_eq!(chunk.get_byte_at(0), 0b0100_0000);
        chunk.toggle_bit((0, 0));
        chunk.toggle_bit((1, 0));
        assert_eq!(chunk.get_byte_at(0), 0b1000_0000);
    }
    #[test]
    fn survive_simple() {
        let chunk = Chunk::from(
            [0, 0],
            [
                0b0000_0000,
                0b0001_0111,
                0b0000_0000,
                0b0010_0000,
                0b0111_0000,
                0b0010_0000,
                0b0000_0000,
                0b0000_0000,
            ],
            10.0,
        );
        let edges = Edges {
            left: 0,
            right: 0,
            top: 0,
            bottom: 0,
            corners: 0,
        };
        assert_eq!(chunk.survive((1, 1), &edges), false);
        assert_eq!(chunk.survive((3, 1), &edges), false);
        assert_eq!(chunk.survive((4, 1), &edges), false);
        assert_eq!(chunk.survive((5, 1), &edges), false);
        assert_eq!(chunk.survive((6, 1), &edges), true);
        assert_eq!(chunk.survive((6, 2), &edges), true);
        assert_eq!(chunk.survive((2, 4), &edges), false);
        assert_eq!(chunk.survive((3, 4), &edges), true);
        assert_eq!(chunk.survive((2, 5), &edges), true);
        assert_eq!(chunk.survive((3, 5), &edges), true);
    }
    #[test]
    fn survive_neighbor() {}
    #[test]
    fn game() {
        let mut game = Game::from([0b0111_1111; 8]);
        assert_eq!(game.map.get(&[0, 0]).unwrap().get_byte_at(0), 0b0111_1111);
        game.insert_chunk([1, 0], Chunk::from([1, 0], [0b1011_1111; 8], 10.0));
        assert_eq!(game.map.get(&[1, 0]).unwrap().get_byte_at(0), 0b1011_1111);
        game.insert_chunk([0, 0], Chunk::new([0, 0], 10.0));
        assert_eq!(game.map.get(&[0, 0]).unwrap().get_byte_at(0), 0b0000_0000);
        assert_eq!(
            game.chunks()[1].get_byte_at(0) == game.map.get(&[0, 0]).unwrap().get_byte_at(0)
                || game.chunks()[0].get_byte_at(0) == game.map.get(&[0, 0]).unwrap().get_byte_at(0),
            true
        );
    }
    #[test]
    fn chunk_iteration() {
        let mut c = Chunk::from(
            [0, 0],
            [
                0b0000_0000,
                0b0010_0000,
                0b0111_0000,
                0b0010_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0110,
                0b0000_0110,
            ],
            10.0,
        );
        let c2 = Chunk::from(
            [0, 0],
            [
                0b0000_0000,
                0b0111_0000,
                0b0101_0000,
                0b0111_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0110,
                0b0000_0110,
            ],
            10.0,
        );
        let edges = Edges {
            left: 0,
            right: 0,
            top: 0,
            bottom: 0,
            corners: 0,
        };
        c.iterate(&edges);
        assert_eq!(chunks_eq(&c, &c2), true);
        c.iterate(&edges);
        assert_eq!(chunks_eq(&c, &c2), false);
    }
    #[test]
    fn edges() {
        let mut game = Game::from([
            0b0000_0000,
            0b0000_0000,
            0b0000_0001,
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
                    0b1100_0000,
                    0b0000_0000,
                    0b0000_0000,
                    0b0000_0000,
                    0b0000_0000,
                    0b0000_0000,
                ],
                10.0,
            ),
        );
        game.iterate();
        let c = game.map.get(&[1, 0]).unwrap();
        let c2 = game.map.get(&[0, 0]).unwrap();
        assert_eq!(c.get_bit_at_point((0, 1)), true);
        assert_eq!(c.get_bit_at_point((0, 2)), true);
        assert_eq!(c.get_bit_at_point((0, 3)), true);
        assert_eq!(c2.get_bit_at_point((7, 2)), false);
        game.iterate();
        let c = game.map.get(&[1, 0]).unwrap();
        let c2 = game.map.get(&[0, 0]).unwrap();
        assert_eq!(c.get_bit_at_point((0, 1)), false);
        assert_eq!(c.get_bit_at_point((0, 2)), true);
        assert_eq!(c.get_bit_at_point((0, 3)), false);
        assert_eq!(c.get_bit_at_point((1, 2)), true);
        assert_eq!(c2.get_bit_at_point((7, 2)), true);
    }
    #[test]
    fn corners() {
        let mut game = Game::from([
            0b1000_0001,
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
            0b1000_0001,
        ]);
        game.insert_chunk([-1, -1], Chunk::new([-1, -1], 10.0).set_bit((7, 0), true));
        game.insert_chunk([1, -1], Chunk::new([1, -1], 10.0).set_bit((0, 0), true));
        game.insert_chunk([-1, 1], Chunk::new([-1, 1], 10.0).set_bit((7, 7), true));
        game.insert_chunk([1, 1], Chunk::new([1, 1], 10.0).set_bit((0, 7), true));
        let corners = game.corners([0, 0]);
        assert_eq!(corners, 0b1111_0000);
        game.insert_chunk([-1, -1], Chunk::new([-1, -1], 10.0).set_bit((7, 0), false));
        game.insert_chunk([1, -1], Chunk::new([1, -1], 10.0).set_bit((0, 0), true));
        game.insert_chunk([-1, 1], Chunk::new([-1, 1], 10.0).set_bit((7, 7), true));
        game.insert_chunk([1, 1], Chunk::new([1, 1], 10.0).set_bit((0, 7), false));
        let corners = game.corners([0, 0]);
        assert_eq!(corners, 0b0110_0000);
        game.insert_chunk([-1, -1], Chunk::new([-1, -1], 10.0).set_bit((7, 0), true));
        game.insert_chunk([1, -1], Chunk::new([1, -1], 10.0).set_bit((0, 0), false));
        game.insert_chunk([-1, 1], Chunk::new([-1, 1], 10.0).set_bit((7, 7), true));
        game.insert_chunk([1, 1], Chunk::new([1, 1], 10.0).set_bit((0, 7), false));
        let corners = game.corners([0, 0]);
        assert_eq!(corners, 0b1010_0000);
        game.insert_chunk([-1, -1], Chunk::new([-1, -1], 10.0).set_bit((7, 0), false));
        game.insert_chunk([1, -1], Chunk::new([1, -1], 10.0).set_bit((0, 0), false));
        game.insert_chunk([-1, 1], Chunk::new([-1, 1], 10.0).set_bit((7, 7), false));
        game.insert_chunk([1, 1], Chunk::new([1, 1], 10.0).set_bit((0, 7), false));
        let corners = game.corners([0, 0]);
        assert_eq!(corners, 0b0000_0000);
    }
}

fn chunks_eq(c1: &Chunk, c2: &Chunk) -> bool {
    let mut val = true;
    for i in 0..8 {
        if !(c1.get_byte_at(i) == c2.get_byte_at(i)) {
            val = false;
        }
    }
    val
}
