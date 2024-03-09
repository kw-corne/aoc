use std::path::Path;

use crate::util::get_lines;

fn get_packet_type_and_id(bits: &[u8]) -> (u8, u8) {
    let first3 = bits[0] >> 1 & 0b00001110;
    let packet_type = u8::from_le_bytes([first3]);

    let bit4 = bits[0] & 0b00000001;
    let bit56 = bits[1] & 0b00000011;
    println!("{:08b} {:01b} {:02b}", bit4, bit56, bits[1]);
    (0, 0)
}

fn hex_str_to_bits(s: &str) -> Vec<u8> {
    let mut bits = Vec::with_capacity(s.len() * 4);

    for ch in s.chars() {
        let b = u8::from_str_radix(ch.encode_utf8(&mut [0u8; 4]), 16).unwrap();
        bits.push(b);
    }

    bits
}

fn p2(lines: Vec<String>) {}

fn p1(lines: Vec<String>) {
    let bits = hex_str_to_bits(&lines[0]);
    let (packet_type, packet_id) = get_packet_type_and_id(&bits);
    dbg!(packet_type);
}

pub fn d16() {
    let input_file = Path::new("src/d16/dbg.txt");

    p1(get_lines(input_file));
    // p2(get_lines(input_file));
}
