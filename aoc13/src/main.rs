#[path = "../../utils.rs"]
mod utils;

use crate::utils::*;

use std::fmt;

fn main() {
    let input: String = get_input(13).expect("Failed to retrieve input");
    part1(&input);
    part2(&input);
}

fn part1(input: &String) {
    let (track, mut carts) = parse(input);
    let mut answer: String = "".to_string();
    'outer: loop {
        carts.sort_by_key(|cart| (cart.y, cart.x));
        let mut next_carts = carts.clone();
        'inner: for (i, cart) in carts.iter().enumerate() {
            let track_under_cart = track[cart.x as usize][cart.y as usize];
            let mut next_direction = cart.direction;
            if track_under_cart == TrackPiece::Intersection {
                next_carts[i].intersection_count += 1;
                next_direction = match cart.intersection_count % 3 {
                    0 => cart.left(),
                    1 => cart.direction,
                    2 => cart.right(),
                    _ => cart.direction,
                };
            }
            let mut add_x = 0;
            let mut add_y = 0;
            if track_under_cart == TrackPiece::CurveLeft {
                next_direction = match cart.direction {
                    Direction::North | Direction::South => cart.left(),
                    Direction::East | Direction::West => cart.right(),
                    Direction::Unknown => Direction::Unknown,
                }
            } else if track_under_cart == TrackPiece::CurveRight {
                next_direction = match cart.direction {
                    Direction::North | Direction::South => cart.right(),
                    Direction::East | Direction::West => cart.left(),
                    Direction::Unknown => Direction::Unknown,
                }
            }
            match next_direction {
                Direction::North => {
                    add_y = -1;
                }
                Direction::East => {
                    add_x = 1;
                }
                Direction::South => {
                    add_y = 1;
                }
                Direction::West => {
                    add_x = -1;
                }
                Direction::Unknown => {}
            };
            next_carts[i].x += add_x;
            next_carts[i].y += add_y;
            next_carts[i].direction = next_direction;

            for (j, cart_j) in next_carts.iter().enumerate() {
                if i != j && next_carts[i].x == cart_j.x && next_carts[i].y == cart_j.y {
                    answer = format!("{},{}", next_carts[i].x, next_carts[i].y);
                    break 'outer;
                }
            }
        }
        carts = next_carts;
    }
    println!("{}", answer);
    println!("{}", submit_answer(13, 1, format!("{}", answer))
    .expect("Failed to submit answer"));
}

fn part2(input: &String) {
    let (track, mut carts) = parse(&input);
    println!("{}", carts.len());
    let mut answer: String = "".to_string();
    let mut ticks = 0;
    let mut crashed = Vec::new();
    'outer: while carts.len() != 1 {
        carts.sort_by_key(|cart| (cart.y, cart.x));
        let mut next_carts = carts.clone();
        'inner: for (i, cart) in carts.iter().enumerate() {
            if !crashed.contains(cart) {
                let track_under_cart = track[cart.x as usize][cart.y as usize];
                if track_under_cart == TrackPiece::Empty ||
                    track_under_cart == TrackPiece::Unknown {
                    println!(
                        "Going off the rails on a crazy train at {:?},{:?} heading {:?}",
                        cart.x, cart.y, cart.direction
                    );
                }
                let mut next_direction = cart.direction;
                let mut add_x = 0;
                let mut add_y = 0;
                if track_under_cart == TrackPiece::CurveLeft {
                    next_direction = match cart.direction {
                        Direction::North | Direction::South => cart.left(),
                        Direction::East | Direction::West => cart.right(),
                        Direction::Unknown => Direction::Unknown,
                    }
                } else if track_under_cart == TrackPiece::CurveRight {
                    next_direction = match cart.direction {
                        Direction::North | Direction::South => cart.right(),
                        Direction::East | Direction::West => cart.left(),
                        Direction::Unknown => Direction::Unknown,
                    }
                }

                if track_under_cart == TrackPiece::Intersection {
                    next_direction = match cart.intersection_count % 3 {
                        0 => cart.left(),
                        1 => cart.direction,
                        2 => cart.right(),
                        _ => cart.direction,
                    };
                    next_carts[i].intersection_count += 1;
                }

                match next_direction {
                    Direction::North => {
                        add_y = -1;
                    }
                    Direction::East => {
                        add_x = 1;
                    }
                    Direction::South => {
                        add_y = 1;
                    }
                    Direction::West => {
                        add_x = -1;
                    }
                    Direction::Unknown => {}
                };

                next_carts[i].x += add_x;
                next_carts[i].y += add_y;
                next_carts[i].direction = next_direction;
            }

            for (j, cart_j) in next_carts.iter().enumerate() {
                if i != j && next_carts[i].x == cart_j.x && next_carts[i].y == cart_j.y 
                && (!crashed.contains(cart_j) || !crashed.contains(&next_carts[i])) {
                    println!(
                        "crash between {:?} and {:?} at {},{} on tick {}",
                        next_carts[i], cart_j, cart_j.x, cart_j.y, ticks
                    );
                    crashed.push(*cart_j);
                    crashed.push(next_carts[i]);
                }
            }
        }
        next_carts.retain(|cart| !crashed.contains(&cart));
        crashed.clear();
        carts = next_carts;
        ticks += 1;
    }
    answer = format!("{},{}", carts[0].x, carts[0].y);
    println!("{}", submit_answer(13, 2, format!("{}", answer))
    .expect("Failed to submit answer"));
}

fn parse(input: &String) -> ([[TrackPiece; 150]; 150], Vec<Cart>) {
    let lines = input.lines();
    let mut track = [[TrackPiece::Empty; 150]; 150];
    let mut carts = Vec::new();
    for (y, line) in lines.enumerate() {
        for (x, c) in line.chars().enumerate() {
            let mut tp = TrackPiece::from(c);
            if tp == TrackPiece::Unknown {
                let cart = Cart::from((x, y, c));
                tp = match &cart.direction {
                    Direction::North => TrackPiece::Vertical,
                    Direction::South => TrackPiece::Vertical,
                    Direction::East => TrackPiece::Horizontal,
                    Direction::West => TrackPiece::Horizontal,
                    _ => TrackPiece::Unknown,
                };
                carts.push(cart);
            }
            track[x][y] = tp;
        }
    }
    (track, carts)
}

#[derive(Clone, Copy, PartialEq)]
enum TrackPiece {
    Empty,
    Vertical,
    Horizontal,
    CurveLeft,
    CurveRight,
    Intersection,
    Unknown,
}

impl From<char> for TrackPiece {
    fn from(c: char) -> TrackPiece {
        match c {
            ' ' => TrackPiece::Empty,
            '|' => TrackPiece::Vertical,
            '-' => TrackPiece::Horizontal,
            '\\' => TrackPiece::CurveLeft,
            '/' => TrackPiece::CurveRight,
            '+' => TrackPiece::Intersection,
            _ => TrackPiece::Unknown,
        }
    }
}

impl fmt::Debug for TrackPiece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ch = match self {
            TrackPiece::Empty => " ",
            TrackPiece::Vertical => "|",
            TrackPiece::Horizontal => "-",
            TrackPiece::CurveLeft => "\\",
            TrackPiece::CurveRight => "/",
            TrackPiece::Intersection => "+",
            TrackPiece::Unknown => "?",
        };
        write!(f, "{}", ch)
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
    Unknown,
}

impl fmt::Debug for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let dir = match self {
            Direction::North => "North",
            Direction::East => "East",
            Direction::South => "South",
            Direction::West => "West",
            Direction::Unknown => "Unknown",
        };
        write!(f, "{}", dir)
    }
}

impl From<char> for Direction {
    fn from(c: char) -> Direction {
        match c {
            '^' => Direction::North,
            '>' => Direction::East,
            'v' => Direction::South,
            '<' => Direction::West,
            _ => Direction::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Cart {
    x: isize,
    y: isize,
    direction: Direction,
    intersection_count: usize,
}

impl From<(usize, usize, char)> for Cart {
    fn from(tuple: (usize, usize, char)) -> Cart {
        Cart {
            x: tuple.0 as isize,
            y: tuple.1 as isize,
            direction: Direction::from(tuple.2),
            intersection_count: 0,
        }
    }
}

impl Cart {
    fn left(&self) -> Direction {
        match self.direction {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
            Direction::Unknown => Direction::Unknown,
        }
    }

    fn right(&self) -> Direction {
        match self.direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::Unknown => Direction::Unknown,
        }
    }
}
