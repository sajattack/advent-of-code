#[path = "../../utils.rs"]
mod utils;

use crate::utils::*;

use std::collections::HashMap;

fn main() {
    let input: String = get_input(6).expect("Failed to retrieve input");
    part1(&input);
    part2(&input);
}

#[derive(Debug)]
struct Point {
    x: i16,
    y: i16,
}

impl Point {
    fn manhattan_distance(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() as i32 + (self.y - other.y).abs() as i32
    }
}

impl From<&str> for Point {
    fn from(s: &str) -> Point {
        s.trim();
        Point {
            x: s[0..s.find(',').expect("find fail")].parse::<i16>().expect("parse fail"),
            y: s[s.find(',').expect("find fail")+2..s.len()]
                .parse::<i16>().expect("parse fail"),
        }
    }
}

impl From<(i16, i16)> for Point {
    fn from(tuple: (i16, i16)) -> Point {
        Point {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

fn part1(input: &String) {
    let lines = input.split("\n");
    let mut points = lines.filter(|s| !s.trim().is_empty()).enumerate()
        .map(|(i,s)|(i as u16+1, Point::from(s)))
             .collect::<HashMap<u16,Point>>();
    println!("{:?}", points);
    let max_x = points.values().max_by_key(|point| point.x).unwrap().x+1;
    let max_y = points.values().max_by_key(|point| point.y).unwrap().y+1;
    println!("{} {}", max_x, max_y);

    let mut grid = vec![vec![None;max_x as usize +1].into_boxed_slice(); max_y as usize+1]
        .into_boxed_slice();
    for y in 0..=max_y {
        for x in 0..=max_x {
            let mut buffer: Vec<(u16,i32)> = 
                points.iter().map(
                |point| (*point.0, point.1.manhattan_distance(&Point::from((x, y)))))
                .collect();
            buffer.sort_by_key(|&(_, dist)| dist);
            let closest_point: Option<u16>;
            if buffer[0].0 == buffer[1].0 {
                closest_point = None;        
            } else {
                closest_point = Some(buffer[0].0);
            }
            grid[y as usize][x as usize] = closest_point;
        }
    }
    
    let out_of_bounds = |x: i16, y: i16| x == 0 || x == max_x || y == 0 || y == max_y;

    for y in 0..=max_y {
        for x in 0..=max_x {
            if out_of_bounds(x,y) {
                points.remove(&grid[y as usize][x as usize].unwrap());
                grid[y as usize][x as usize] = None;
            }
        }
    }

    let mut map: HashMap<u16, u16> = HashMap::new();
    for y in 0..=max_y {
        for x in 0..=max_x {
            if grid[y as usize][x as usize].is_some() {
                *map.entry(grid[y as usize][x as usize].unwrap()).or_insert(0) += 1;
            }
        }
    }
    let tmp: Vec<(u16, u16)> = 
        points.keys().map(|k| (*k, map[k])).collect();
    println!("{:?}", tmp);
    let answer = tmp.iter().max_by_key(|&(_, count)| count).unwrap().1;
    println!("{}", answer);
    println!("{}", submit_answer(6, 1, format!("{}", answer))
        .expect("Failed to submit answer"));
}

fn part2(input: &String) { 
    let lines = input.split("\n");
    let points = lines.filter(|s| !s.trim().is_empty()).enumerate()
        .map(|(i,s)|(i as u16+1, Point::from(s)))
             .collect::<HashMap<u16,Point>>();
    println!("{:?}", points);
    let max_x = points.values().max_by_key(|point| point.x).unwrap().x+1;
    let max_y = points.values().max_by_key(|point| point.y).unwrap().y+1;
    println!("{} {}", max_x, max_y);

    let mut area = 0;
    for y in 0..=max_y {
        for x in 0..=max_x {
            let total_distance: i32 = 
                points.iter().map(
                |point|  point.1.manhattan_distance(&Point::from((x, y)))).sum();
            if total_distance < 10000 {
                area +=1;
            }
        }
    }
    println!("{}", submit_answer(6, 2, format!("{}", area))
    .expect("Failed to submit answer"));
}
