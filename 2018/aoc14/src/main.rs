#[path = "../../utils.rs"]
mod utils;

use crate::utils::*;

fn main() {
    let input: String = get_input(14).expect("Failed to retrieve input");
    part1(&input);
    part2(&input);
}

fn part1(input: &String) {
    let target_num_recipes = input.trim().parse::<usize>().expect("parse fail");
    let mut recipes: Vec<u8> = vec![3, 7];
    let mut elf1_index = 0;
    let mut elf2_index = 1;
    while recipes.len() < target_num_recipes + 10 {
        let sum = recipes[elf1_index] + recipes[elf2_index];
        if sum < 10 {
            recipes.push(sum);
        } else {
            recipes.push(1);
            recipes.push(sum%10);
        }
       elf1_index = (elf1_index + recipes[elf1_index] as usize +1) % recipes.len(); 
       elf2_index = (elf2_index + recipes[elf2_index] as usize +1) % recipes.len(); 
    }
    let last10 = &recipes[recipes.len()-10..];
    let mut answer = "".to_string();
    for x in last10 {
        answer.push(x.to_string().chars().nth(0).unwrap());
    }
    println!("{}", answer);
    println!("{}", submit_answer(14, 1, format!("{}", answer))
    .expect("Failed to submit answer"));
}

fn part2(input: &String) {
    let target_recipes = input.trim();
    let mut recipes: Vec<u8> = vec![3, 7];
    let mut recipes_string = "37".to_string();
    let mut elf1_index = 0;
    let mut elf2_index = 1;
    while !recipes_string.ends_with(&target_recipes) {
        let sum = recipes[elf1_index] + recipes[elf2_index];
        if sum < 10 {
            recipes.push(sum);
            recipes_string.push(sum.to_string().chars().nth(0).unwrap())
        } else {
            recipes.push(1);
            recipes.push(sum%10);
            recipes_string.push('1');
            if (recipes_string.ends_with(&target_recipes)) { break; }
            recipes_string.push((sum%10).to_string().chars().nth(0).unwrap())
        }
       elf1_index = (elf1_index + recipes[elf1_index] as usize +1) % recipes.len(); 
       elf2_index = (elf2_index + recipes[elf2_index] as usize +1) % recipes.len(); 
    }
    let answer = recipes_string.len() - target_recipes.len();
    println!("{}", answer);
    println!("{}", submit_answer(14, 2, format!("{}", answer))
    .expect("Failed to submit answer"));
}
