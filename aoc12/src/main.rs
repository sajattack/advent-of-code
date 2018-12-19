#[path = "../../utils.rs"]
mod utils;

use crate::utils::*;

fn main() {
    let input: String = get_input(12).expect("Failed to retrieve input");
    part1(&input);
    part2(&input);
}

fn part1(input: &String) {
    let answer = run(input.clone(), 20);
    println!("{}", answer);
    println!("{}", submit_answer(12, 1, format!("{}", answer))
    .expect("Failed to submit answer"));
}

fn part2(input: &String) { 
    println!("{}", run(input.clone(), 500));
    println!("{}", run(input.clone(), 5000));
    println!("{}", run(input.clone(), 50000));
}

fn run(input: String, iterations: usize) -> isize {
    let mut lines: Vec<&str> = input.trim().split("\n").collect();
    let mut state = lines[0][15..].to_string();
    lines.remove(0);
    lines.remove(0);
    let initial_len = state.len();

    for _ in 0..iterations {
        for _ in 0..4 {
            state.insert(0, '.');
            state.push('.');
        }
        let mut next_state = state.clone();
        for note in lines.iter() {
            for i in (2..state.len()-2) {
                if state.as_bytes()[i-2..=i+2] == note.as_bytes()[0..5] {
                    unsafe {
                        next_state.as_bytes_mut()[i] = note.as_bytes()[9] 
                    }
                }
            }
        }
        state = next_state; 
    }
    let offset = (initial_len as isize - state.len() as isize) / 2;
    let mut answer = 0;
    for (i, ch) in state.as_bytes().iter().enumerate() {
        if *ch == b'#' {
            answer += offset + i as isize
        }
    }
    answer 
}
