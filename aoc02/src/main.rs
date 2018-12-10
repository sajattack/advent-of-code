#[path = "../../utils.rs"]
mod utils;

use crate::utils::*;

fn main() {
    let input: String = get_input(2).expect("Failed to retrieve input");
    part1(&input);
    part2(&input);
}

fn part1(input: &String) {
    let lines = input.split("\n");
    let mut twos = 0;
    let mut threes = 0;
    for line in lines {
        let mut two_found = false;
        let mut three_found = false;
        for ch in "abcdefghijklmnopqrstuvwxyz".chars() {
            if line.matches(ch).collect::<Vec<&str>>().len() == 2 && !two_found {
                twos += 1;
                two_found = true;
            } 
            if line.matches(ch).collect::<Vec<&str>>().len() == 3 && !three_found {
                threes += 1;
                three_found = true;
            }
        }
    }
    let answer = twos * threes;
    println!("{}", answer);
    println!("{}", submit_answer(2, 1, format!("{}", answer))
        .expect("Failed to submit answer"));
}

fn part2(input: &String) { 
    use difference::Changeset;
    let lines = input.split("\n").collect::<Vec<&str>>();
    'outer: for line_a in &lines {
        for line_b in &lines {
            let changeset = Changeset::new(line_a, line_b, "");
            if changeset.distance == 2 {
            let mut answer = format!("{}", format!("{:?}", changeset.diffs[0])
                .replace("Same", "")
                .trim_matches(|c| c == '(' || c == ')' || c == '"'));
            answer += format!("{:?}", changeset.diffs[3])
                .replace("Same", "")
                .trim_matches(|c| c == '(' || c == ')' || c == '"');
            println!("{}", answer);
            println!("{}", submit_answer(2, 2, format!("{}", answer))
                .expect("Failed to submit answer"));
            break 'outer;
            } 
        }
    }
}
