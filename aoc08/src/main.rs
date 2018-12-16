#[path = "../../utils.rs"]
mod utils;

use crate::utils::*;

fn main() {
    let input: String = get_input(8).expect("Failed to retrieve input");
    //part1(&input);
    part2(&input);
}

fn part1(input: &String) {
    let mut numbers: Vec<u8> = input.trim_right().split(" ")
        .map(|n| n.parse::<u8>().unwrap()).collect();
    let tree = parse_node(&mut numbers).expect("outer parse failed"); 
    let answer = sum_metadata(&tree); 
    println!("{}", answer);
    println!("{}", submit_answer(8, 1, format!("{}", answer))
    .expect("Failed to submit answer"));
}

fn part2(input: &String) { 
    let mut numbers: Vec<u8> = input.trim_right().split(" ")
        .map(|n| n.parse::<u8>().unwrap()).collect();
    let tree = parse_node(&mut numbers).expect("outer parse failed"); 
    let answer = sum_values(&tree); 
    println!("{}", answer);
    println!("{}", submit_answer(8, 2, format!("{}", answer))
    .expect("Failed to submit answer"));
}

fn parse_node(numbers: &mut Vec<u8>) -> Option<Node> {
    if numbers.len() >= 2 {
        let child_count = numbers.remove(0);
        let metadata_count = numbers.remove(0); 
        let children:Vec<Node>;
        if child_count != 0 {
            children = (0..child_count)
                .map(|_| parse_node(numbers)).filter_map(|x| x).collect();
        } else {
            children = Vec::new(); 
        }
        let mut metadata: Vec<u32> = Vec::new();
        for _ in 0..metadata_count {
            metadata.push(numbers.remove(0).into())
        }
        Some(Node {
            child_count: child_count,
            metadata_count: metadata_count,
            children: children,
            metadata: metadata,
        })
    }
    else {
        None
    }
}

fn sum_metadata(tree: &Node) -> u32 {
    let my_sum: u32 = tree.metadata.iter().sum();
    let children_sum: u32 = tree.children.iter().map(sum_metadata).sum();
    my_sum + children_sum
}

fn sum_values(tree: &Node) -> u32 {
    if tree.children.is_empty() {
        tree.metadata.iter().sum()
    } else {
        tree.metadata.iter()
            .filter(|&&m| m > 0 && m - 1 < tree.children.len() as u32)
            .map(|&m| sum_values(&tree.children[m as usize - 1]))
            .sum()
    }
}

#[derive(Debug)]
struct Node {
    child_count: u8,
    metadata_count: u8,
    children: Vec<Node>,
    metadata: Vec<u32>,
}
