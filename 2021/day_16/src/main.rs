#![allow(dead_code, unused_variables, unused_mut)]

use aoc_std::*;

fn convert_to_binary_from_hex(hex: &str) -> String {
    hex.chars().map(to_binary).collect()
}

fn to_binary(c: char) -> String {
    let b = match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    };
    b.to_string()
}

fn chop_left(bits: &mut String, how_many: usize) -> String {
    let to_return = &bits.clone()[0..how_many];
    for _ in 0..how_many {
        bits.remove(0);
    }
    to_return.to_string()
}

fn all_zeros(bits: &str) -> bool {
    for chr in bits.chars() {
        if chr != '0' {
            return false;
        }
    }
    dbg!("here");
    true
}

static mut TOTAL_VERSIONS: usize = 0;

fn handle_packet(packet: String) -> (usize, String) {
    let mut packet = packet;
    let version: &str = &chop_left(&mut packet, 3);
    let version_usize: usize = usize::from_str_radix(&version, 2).unwrap();
    // dbg!(&version_usize);
    unsafe {
        println!("previous: {}, adding: {}", TOTAL_VERSIONS, version_usize);
        TOTAL_VERSIONS += version_usize;
        println!("after: {}", TOTAL_VERSIONS);
    }
    let type_id: &str = &chop_left(&mut packet, 3);
    let mut value = String::new();
    match type_id {
        "100" => {
            // type 4 -> literal value
            dbg!("TYPE 4 PACKET");
            while packet.starts_with('1') {
                value += &chop_left(&mut packet, 5)[1..];
            }
            value += &chop_left(&mut packet, 5)[1..];
        }
        _ => {
            // operator packet
            dbg!("OPERATOR PACKET");
            let length_type_id: &str = &chop_left(&mut packet, 1);
            let mut total_len_in_bits: Option<usize> = None;
            let mut num_of_next_packets: Option<usize> = None;
            match length_type_id {
                "0" => {
                    total_len_in_bits = Some(
                        match usize::from_str_radix(&chop_left(&mut packet, 15), 2) {
                            Ok(ok) => ok,
                            Err(err) => 0,
                        },
                    );
                }
                "1" => {
                    num_of_next_packets = Some(
                        match usize::from_str_radix(&chop_left(&mut packet, 11), 2) {
                            Ok(ok) => ok,
                            Err(err) => 0,
                        },
                    );
                }
                _ => {}
            }
            dbg!(&total_len_in_bits, &num_of_next_packets);
            if total_len_in_bits.is_some() {
                println!("NEST1");
                let mut sub_value = handle_packet(chop_left(&mut packet, total_len_in_bits.unwrap()));

                while !all_zeros(&sub_value.1) {
                    println!("NEST3");
                    sub_value = handle_packet(sub_value.1);
                }

                dbg!(&sub_value);
            }
            if num_of_next_packets.is_some() {
                for i in 0..num_of_next_packets.unwrap() {
                    println!("NEST2");
                    let mut sub_value = handle_packet(packet.clone());
                    while !all_zeros(&sub_value.1) {
                        println!("NEST3");
                        sub_value = handle_packet(sub_value.1);
                    }
                    dbg!(&sub_value);
                }
            }
            dbg!(&total_len_in_bits, &num_of_next_packets);
        }
    };
    dbg!(&version, &type_id, &value, &packet);
    (usize::from_str_radix(&value, 2).unwrap_or(0), packet)
}

fn part1(input_file: String) {
    let mut answer;
    let input_file = input_file.trim();
    let input_binary = convert_to_binary_from_hex(input_file);
    dbg!(&input_binary);
    answer = handle_packet(input_binary);
    unsafe {
        print_answer(TOTAL_VERSIONS);
    }
}

fn part2(input_file: String) {
    let mut answer: usize = 0;
    print_answer(answer);
}

fn main() {
    let part = 1;

    let input_file = load_file("input.txt");

    if part == 1 {
        part1(input_file);
    } else if part == 2 {
        part2(input_file);
    }
}
