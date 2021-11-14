extern crate rayon;

#[path = "../../utils.rs"]
mod utils;

use crate::utils::*;

use rayon::iter::ParallelIterator;
use rayon::iter::IntoParallelIterator;

fn main() {
    let input: String = get_input(11).expect("Failed to retrieve input");
    part1(&input);
    part2(&input);
}

fn part1(input: &String) {
    let serial = input.trim().parse::<u16>().expect("parse fail");
    let mut grid = [[0i8;300];300];
    for y in 0..300 {
        for x in 0..300 {
           grid[x][y] = power_level(serial, x as u16+1, y as u16+1); 
        }
    }
    let mut max_power = 0;
    let mut max_power_pos = (0u16, 0u16);
    for y in 0..297 {
        for x in 0..297 {
            let mut square_power = 0;
            for i in 0..3 {
                for j in 0..3 {
                    square_power += grid[x+i][y+j];
                }
            }
            if square_power > max_power {
                max_power = square_power;
                max_power_pos = (x as u16+1, y as u16+1);
            }
        }
    }
    let answer = format!("{},{}", max_power_pos.0, max_power_pos.1);
    println!("{}", answer);
    println!("{}", submit_answer(11, 1, format!("{}", answer))
    .expect("Failed to submit answer"));
}

fn part2(input: &String) { 
    let serial = input.trim().parse::<u16>().expect("parse fail");
    let mut grid = [[0i8;300];300];
    for y in 0..300 {
        for x in 0..300 {
           grid[x][y] = power_level(serial, x as u16+1, y as u16+1); 
        }
    }
    let (max_power, max_power_pos) = (0..300).into_par_iter().map(|size:i32| {
        let mut max_power_for_size = 0; 
        let mut max_power_pos_for_size = (0,0,0);
        for y in 0..300-size {
            for x in 0..300-size {
                let mut square_power: i64 = 0;
                for i in 0..size {
                    for j in 0..size { 
                        square_power += grid[(x+i) as usize][(y+j) as usize] as i64;
                    }
                } 
                if square_power > max_power_for_size {
                    max_power_for_size = square_power;
                    max_power_pos_for_size = (x+1, y+1, size);
                }
            }
        }
        (max_power_for_size, max_power_pos_for_size)
    }).max_by_key(|(power, _)| *power).unwrap();
    let answer = format!("{},{},{}", max_power_pos.0, max_power_pos.1, max_power_pos.2);
    println!("{}", answer);
    println!("{}", submit_answer(11, 2, format!("{}", answer))
    .expect("Failed to submit answer"));
}

fn power_level(serial: u16, x: u16, y: u16) -> i8 {
    let mut power_level: u32 = 0;
    let rack_id = x + 10;
    power_level += (rack_id as u32 * y as u32);
    power_level += serial as u32;
    power_level *= rack_id as u32;
    power_level = (power_level / 100) % 10;
    power_level as i8 - 5
}
