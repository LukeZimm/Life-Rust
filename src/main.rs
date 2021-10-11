use std::io::{stdin, stdout, Write};

fn main() {
    let mut bytes: [u8; 8] = [0b0000_0010, 0b0000_0100, 0b0000_0111, 0, 0, 0, 0, 0];
    display_chunk(bytes);
    loop {
        println!();
        pause();
        println!();
        bytes = iterate(bytes);
        println!();
        display_chunk(bytes);
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
