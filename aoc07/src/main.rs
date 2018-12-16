#[path = "../../utils.rs"]
mod utils;

use crate::utils::*;

use std::collections::{HashMap, HashSet};

fn main() {
    let input: String = get_input(7).expect("Failed to retrieve input");
    part1(&input);
    part2(&input, 5);
}

fn part1(input: &String) {
    let mut steps = get_steps(input);
    let mut answer = String::new();
    while !steps.is_empty() {
        let next_step = find_possible_steps(&steps)[0];
        answer.push(next_step);
        steps.remove(&next_step);
        for dependency_list in steps.values_mut() {
            dependency_list.remove(&next_step);
        }
    }
    println!("{:?}", answer);
    println!("{}", submit_answer(7, 1, format!("{}", answer))
    .expect("Failed to submit answer"));
}

fn part2(input: &String, worker_count: usize) { 
    let mut steps = get_steps(input); 
    let mut time = 0;
    let mut workers: Vec<Worker> = (0..worker_count).map(|_| Worker::new()).collect();
    loop {
        time += 1;
        let mut slackers = Vec::new();
        for worker in workers.iter_mut() {
            worker.tick();
            if worker.is_ready() {
                match worker.work {
                    Some(work) => {
                        for dependency_list in steps.values_mut() {
                            dependency_list.remove(&work);
                        }
                        worker.work = None;
                        slackers.push(worker);
                    },
                    None => {
                        slackers.push(worker);
                    }
                }
            }
        }
        if steps.is_empty() && slackers.len() == worker_count { break; }
        if slackers.is_empty() { continue; }
        let mut possibilities = find_possible_steps(&steps);
        for (slacker, step) in slackers.into_iter().zip(possibilities.drain(..)) {
            // Quit slacking you!
            steps.remove(&step);
            slacker.assign_work(step);
        }
    }
    let answer = time - 1;
    println!("{}", answer);
    println!("{}", submit_answer(7, 2, format!("{}", answer))
    .expect("Failed to submit answer"));
}

#[derive(Clone,Copy)]
struct Worker {
    work: Option<char>,
    time_left: u8, 
}

impl Worker {
    fn new() -> Worker {
        Worker {
            work: None,
            time_left: 0,
        }
    }

    fn is_ready(&self) -> bool {
        self.time_left == 0
    }

    fn assign_work(&mut self, work: char) {
        self.work = Some(work); 
        let mut buf = [0u8;1];
        work.encode_utf8(&mut buf); 
        self.time_left = 61 + buf[0] - 65; // 65 is ascii for A
    }

    fn tick(&mut self) {
        self.time_left = self.time_left.saturating_sub(1);
    }
}

fn get_steps(input: &String) -> HashMap<char, HashSet<char>> {
    let lines = input.split("\n").filter(|s| !s.trim().is_empty());
    let mut steps: HashMap<char, HashSet<char>> = HashMap::new();
    for line in lines {
        if !steps.contains_key(&line.chars().nth(36).unwrap()) {
            let mut dependency_list = HashSet::new();
            dependency_list.insert(line.chars().nth(5).unwrap());
            steps.insert(line.chars().nth(36).unwrap(), dependency_list); 
        } else {
            let dependency_list = steps.get_mut(&line.chars().nth(36).unwrap()).unwrap();
            dependency_list.insert(line.chars().nth(5).unwrap());
        }
    }
    let no_dependencies = steps.values().flatten()
        .filter(|dependency| !steps.contains_key(dependency))
        .cloned().collect::<Vec<char>>();
    for step in no_dependencies {
        steps.insert(step, HashSet::new());
    }
    steps
}

fn find_possible_steps(steps: &HashMap<char, HashSet<char>>) -> Vec<char> {
    let mut possibilities = steps.iter()
        .filter(|(_, dependencies)| dependencies.is_empty())
        .map(|(step, _)| step)
        .cloned().collect::<Vec<char>>();
    possibilities.sort();
    possibilities
}
