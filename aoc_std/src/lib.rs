/*
load file
read file into vec of lines (strings)
print answer
...
*/

use std::fs::read_to_string;

// safely reading file into string and returning it (trimmed)
pub fn load_file(file_name: &str) -> String {
    match read_to_string(file_name) {
        Ok(ok) => ok.trim().to_string(),
        Err(_) => String::from("error reading file"),
    }
}

// reading file into vector of lines (strings)
pub fn load_file_lines(file_name: &str) -> Vec<String> {
    let file: String = load_file(file_name);
    let mut file_lines: Vec<String> = file.split('\n').map(|x| x.to_string()).collect();
    if file_lines.last().unwrap().is_empty() {
        file_lines.pop();
    }
    file_lines
}

// print answer (usize)
pub fn print_answer(answer: usize) {
    println!("Answer: {}", answer);
}

// print answer (string)
pub fn print_answer_str(answer: &str) {
    println!("Answer: {}", answer);
}
