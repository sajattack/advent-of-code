extern crate regex;

#[path = "../../utils.rs"]
mod utils;

use crate::utils::*;
use std::boxed::Box;

use regex::Regex;

fn main() {
    let input: String = get_input(3).expect("Failed to retrieve input");
    part1(&input);
    part2(&input);
}

#[derive(Debug)]
struct Claim {
    number: u16,
    pos_x: u16,
    pos_y: u16,
    size_x: u16,
    size_y: u16,
}

impl From<&str> for Claim {
    fn from(s: &str) -> Claim {
        let re = Regex::new(
        r"(?P<number>\d+) @ (?P<pos_x>\d+),(?P<pos_y>\d+): (?P<size_x>\d+)x(?P<size_y>\d+)"
        ).expect("regex fail");
        let caps = re.captures(s).expect("capture fail");
        Claim {
            number: caps["number"].parse::<u16>().expect("parse fail"),
            pos_x: caps["pos_x"].parse::<u16>().expect("parse fail"),
            pos_y: caps["pos_y"].parse::<u16>().expect("parse fail"),
            size_x: caps["size_x"].parse::<u16>().expect("parse fail"),
            size_y: caps["size_y"].parse::<u16>().expect("parse fail")
        }
    }
}

fn part1(input: &String) {
    let lines = input.split("\n");
    let claims = lines.filter(|s| !s.trim().is_empty())
        .map(|s| Claim::from(s)).collect::<Vec<Claim>>();
    let mut fabric = vec![['.'; 1000];1000].into_boxed_slice();
    let mut answer = 0;
    for claim in &claims {
        for x in claim.pos_x as usize..(claim.pos_x + claim.size_x) as usize {
            for y in claim.pos_y as usize ..(claim.pos_y + claim.size_y) as usize {
                if fabric[x][y] == '.' {
                    fabric[x][y] = '#'; 
                } else if fabric[x][y] == '#' {
                    fabric[x][y] = 'X';
                    answer += 1
                }
            }
        }
    }
    println!("{}", answer);
    println!("{}", submit_answer(3, 1, format!("{}", answer))
        .expect("Failed to submit answer"));
}

fn part2(input: &String) { 
    let lines = input.split("\n");
    let claims = lines.filter(|s| !s.trim().is_empty())
        .map(|s| Claim::from(s)).collect::<Vec<Claim>>();
    let mut fabric = vec![[-1i16; 1000];1000].into_boxed_slice();
    let mut answer = 0;
    for claim in &claims {
        for x in claim.pos_x as usize..(claim.pos_x + claim.size_x) as usize {
            for y in claim.pos_y as usize ..(claim.pos_y + claim.size_y) as usize {
                if fabric[x][y] == -1 {
                    fabric[x][y] = claim.number as i16;
                } else if fabric[x][y] > 0 {
                    fabric[x][y] = -2;
                }
            }
        }
    }
    for x in 0..1000 {
        'outer: for y in 0..1000 {
            if fabric[x][y] > 0 {
                let claim = &claims[fabric[x as usize][y as usize] as usize -1];
                for x_2 in claim.pos_x as usize..(claim.pos_x+claim.size_x) as usize {
                    for y_2 in claim.pos_y as usize ..(claim.pos_y+claim.size_y) as usize {
                        if fabric[x_2][y_2] != claim.number as i16 {
                            continue 'outer;
                        }
                    }
                }
                answer = fabric[x][y]
            }
        }
    }
    println!("{}", answer);
    println!("{}", submit_answer(3, 2, format!("{}", answer))
        .expect("Failed to submit answer"));
}
