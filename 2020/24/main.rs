#!/home/dvdarch/.cargo/bin/cargo -Zscript

use std::fs::read_to_string;

/*
*  e, se, sw, w, nw, ne
*/

fn parse(tile: String) -> Vec<String> {
    let mut i = 0;
    let mut out: Vec<String> = Vec::new();
    let t_chars: Vec<char> = tile.chars().collect();

    while i < tile.len() {
        let mut tmp = t_chars[i];
        match tmp {
            'e' => out.push("e".to_string()),
            'w' => out.push("w".to_string()),
            's' => {
                out.push(format!("{}{}", t_chars[i], t_chars[i + 1]));
                i += 1;
            }
            _ => (),
        }
        i += 1;
    }

    out
}

fn main() {
    let data = read_to_string("example.txt").unwrap();

    for line in data.lines() {
        println!("{line}");
    }
}
