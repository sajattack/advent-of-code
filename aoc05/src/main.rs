#[path = "../../utils.rs"]
mod utils;

use crate::utils::*;

use std::str;

fn main() {
    let input: String = get_input(5).expect("Failed to retrieve input");
    part1(input.clone());
    part2(input.clone());
}

fn part1(mut input: String) {
    input = input.trim().to_string();
    loop {
        let prev_input_len = input.len();
        for combo in (b'a'..=b'z').zip(b'A'..=b'Z') {
            input = input.replace(str::from_utf8(&[combo.0, combo.1]).unwrap(), "");
            input = input.replace(str::from_utf8(&[combo.1, combo.0]).unwrap(), "");
        }
        if input.len() == prev_input_len { break; }
    }
    let answer = input.len();
    println!("{}", answer);
    println!("{}", submit_answer(5, 1, format!("{}", answer))
        .expect("Failed to submit answer"));
}

fn part2(mut input: String) { 
    input = input.trim().to_string();
    let mut lengths: Vec<usize> = Vec::new();
    for combo in (b'a'..=b'z').zip(b'A'..=b'Z') {
        let mut working_copy = input.clone();
        working_copy = working_copy.replace(str::from_utf8(&[combo.0]).unwrap(), "");
        working_copy = working_copy.replace(str::from_utf8(&[combo.1]).unwrap(), "");
        loop {
            let prev_input_len = working_copy.len();
            for combo in (b'a'..=b'z').zip(b'A'..=b'Z') {
                working_copy = working_copy.replace(
                    str::from_utf8(&[combo.0, combo.1]).unwrap(), "");
                working_copy = working_copy.replace(
                    str::from_utf8(&[combo.1, combo.0]).unwrap(), "");
            }
            if working_copy.len() == prev_input_len { break; }
        }
        lengths.push(working_copy.len());
    }
    let answer = lengths.iter().min().unwrap();
    println!("{}", answer);
    println!("{}", submit_answer(5, 2, format!("{}", answer))
        .expect("Failed to submit answer"));
}
