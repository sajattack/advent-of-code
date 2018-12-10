#[path = "../../utils.rs"]
mod utils;

use crate::utils::*;

fn main() {
    let input: String = get_input(1).expect("Failed to retrieve input");
    part1(&input);
    part2(&input);
}

fn part1(input: &String) {
    let lines = input.split("\n");
    let mut sum = 0;
    for line in lines {
        let addend = &line.parse::<i32>();
        match addend {
            Ok(addend) => {sum += addend;}
            Err(_) => {},
        }
    }
    println!("Answer: {}", sum);
    println!("{}", submit_answer(1, 1, format!("{}", sum))
        .expect("Failed to submit answer"));
}

fn part2(input: &String) { 
    use std::collections::HashSet;
    let mut frequencies: HashSet<i32> = HashSet::new();
    frequencies.insert(0);
    let addends: Vec<i32> = input.split("\n").filter_map(|i| i.parse::<i32>().ok()).collect();
    let mut sum = 0;
    let mut answer = 0;
    let mut twice: bool = false; 
    while !twice {
        for addend in &addends {
            sum += addend;
            twice = frequencies.contains(&sum);
            frequencies.insert(sum);
            if twice {
                answer = sum; 
                break;
            }
        }
    }
    println!("Answer: {}", answer);
    println!("{}", submit_answer(1, 2, format!("{}", answer))
        .expect("Failed to submit answer"));
}
