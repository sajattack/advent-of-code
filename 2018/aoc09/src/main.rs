#[path = "../../utils.rs"]
mod utils;

use crate::utils::*;

fn main() {
    let input: String = get_input(9).expect("Failed to retrieve input");
    part1(&input);
    part2(&input);
}

fn part1(input: &String) {
    let mut words = input.split(" ");
    let player_count = words.nth(0).unwrap().parse::<usize>().expect("parse fail");
    let last_marble = words.nth(5).unwrap().parse::<usize>().expect("parse fail");
    let answer = find_high_score(player_count, last_marble); 
    println!("{}", answer);
    println!("{}", submit_answer(9, 1, format!("{}", answer))
    .expect("Failed to submit answer"));
}

// Come back in a little over an hour and a half
fn part2(input: &String) { 
    let mut words = input.split(" ");
    let player_count = words.nth(0).unwrap().parse::<usize>().expect("parse fail");
    let last_marble = words.nth(5).unwrap().parse::<usize>().expect("parse fail") * 100;
    let answer = find_high_score(player_count, last_marble); 
    println!("{}", answer);
    println!("{}", submit_answer(9, 2, format!("{}", answer))
    .expect("Failed to submit answer"));
}

fn find_high_score(player_count: usize, last_marble: usize) -> u32 {
    let mut circle = Vec::new();
    circle.push(0);
    let mut scores = vec![0u32; player_count];
    let mut prev_index = 0;
    for i in 0..last_marble {
        let player_index = i % player_count; 
        let index = (prev_index + 2) % circle.len();
        let marble_number = i as u32 + 1;
        if marble_number % 23 == 0 {
            scores[player_index] += marble_number;
            let special_marble_index = (prev_index + circle.len() - 7) % circle.len();
            scores[player_index] += circle.remove(special_marble_index);
            prev_index = special_marble_index;
        } else {
            circle.insert(index, marble_number);
            prev_index = index;
        }
    }
    *scores.iter().max().unwrap()
}
