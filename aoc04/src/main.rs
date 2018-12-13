#[path = "../../utils.rs"]
mod utils;

use crate::utils::*;

use std::collections::HashMap;

fn main() {
    let input: String = get_input(4).expect("Failed to retrieve input");
    //part1(&input);
    part2(&input);
}

#[derive(Debug)]
struct Guard {
    number: u16,
    minutes_asleep: Vec<u8>,
}

fn part1(input: &String) {
    let mut lines = input.split("\n").collect::<Vec<&str>>();
    lines.remove(lines.len()-1);
    lines.sort();
    let mut guards: HashMap<u16, Guard> = HashMap::new();
    let mut asleep_start = 0u8; // put this here so it lasts from one iteration to the next
    let mut guard_number = 0u16; // same thing
    for line in lines {
        if line.bytes().nth(19).unwrap() == 'G' as u8 {
            guard_number = line[26..line.len()-13].parse::<u16>().expect("parse fail");
            if !guards.contains_key(&guard_number) {
                guards.insert(guard_number, Guard {
                    number: guard_number,
                    minutes_asleep: Vec::new(),
                });
            }
        } if line.bytes().nth(19).unwrap() == 'f' as u8 {
           asleep_start = line[15..17].parse::<u8>().expect("parse fail");
        } else if line.bytes().nth(19).unwrap() == 'w' as u8 {
            let asleep_end = line[15..17].parse::<u8>().expect("parse fail");
            for minute in asleep_start..asleep_end {
                guards.get_mut(&guard_number).unwrap().minutes_asleep.push(minute);
            }
        }
    }
    let most_asleep_guard = 
        guards.values().max_by_key(|guard| guard.minutes_asleep.len()).unwrap();
    let mut map = HashMap::new();
    for key in &most_asleep_guard.minutes_asleep {
        *map.entry(key).or_insert(0) += 1;
    }
    let most_asleep_minute =
        **map.iter().max_by_key(|&(_, count)| count).unwrap().0;
    let answer: u32 = most_asleep_guard.number as u32 * most_asleep_minute as u32;
    println!("{}", answer);
    println!("{}", submit_answer(4, 1, format!("{}", answer))
        .expect("Failed to submit answer"));
}

fn part2(input: &String) { 
    let mut lines = input.split("\n").collect::<Vec<&str>>();
    lines.remove(lines.len()-1);
    lines.sort();
    let mut guards: HashMap<u16, Guard> = HashMap::new();
    let mut asleep_start = 0u8; // put this here so it lasts from one iteration to the next
    let mut guard_number = 0u16; // same thing
    for line in lines {
        if line.bytes().nth(19).unwrap() == 'G' as u8 {
            guard_number = line[26..line.len()-13].parse::<u16>().expect("parse fail");
            if !guards.contains_key(&guard_number) {
                guards.insert(guard_number, Guard {
                    number: guard_number,
                    minutes_asleep: Vec::new(),
                });
            }
        } if line.bytes().nth(19).unwrap() == 'f' as u8 {
           asleep_start = line[15..17].parse::<u8>().expect("parse fail");
        } else if line.bytes().nth(19).unwrap() == 'w' as u8 {
            let asleep_end = line[15..17].parse::<u8>().expect("parse fail");
            for minute in asleep_start..asleep_end {
                guards.get_mut(&guard_number).unwrap().minutes_asleep.push(minute);
            }
        }
    }
    let mut max_count = 0;
    let mut minute = 0;
    let mut guard_number = 0;
    for guard in guards.values() {
        let mut map = HashMap::new();
        for key in &guard.minutes_asleep {
            *map.entry(key).or_insert(0) += 1;
        }
        let (most_asleep_minute, count) =
            map.iter().max_by_key(|&(_, count)| count).unwrap_or((&&0,&0));
        if count > &max_count {
            max_count = *count;
            minute = **most_asleep_minute;
            guard_number = guard.number;
        }
    }
    let answer = guard_number * minute as u16;
    println!("{}", answer);
    println!("{}", submit_answer(4, 2, format!("{}", answer))
        .expect("Failed to submit answer"));
}
