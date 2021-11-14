use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap().trim().to_owned();
    part1(&input);
    part2(&input);
}

fn part1(input: &String) {
    let mut unique_positions: HashSet<Position> = HashSet::new();
    let mut cur_pos = Position { x: 0, y: 0}; 
    for c in input.chars() {
        unique_positions.insert(cur_pos);
        let next_pos = cur_pos.make_move(c.into());
        cur_pos = next_pos;
    }
    println!("{}", unique_positions.len());
}

fn part2(input: &String) {
    // kinda bad duplicated code all over the place
    let mut unique_positions: HashSet<Position> = HashSet::new();
    let mut santa_cur_pos = Position { x: 0, y: 0}; 
    let mut robo_santa_cur_pos = Position { x: 0, y: 0}; 
    let mut char_iter = input.chars();
    let mut santa_move = char_iter.next();
    while santa_move.is_some() {
        unique_positions.insert(santa_cur_pos);
        let santa_next_pos = santa_cur_pos.make_move(santa_move.unwrap().into());
        santa_cur_pos = santa_next_pos;

        unique_positions.insert(robo_santa_cur_pos);
        let robo_santa_move = char_iter.next().unwrap().into();
        let robo_santa_next_pos = robo_santa_cur_pos.make_move(robo_santa_move);
        robo_santa_cur_pos = robo_santa_next_pos;
        santa_move = char_iter.next();
    }
    println!("{}", unique_positions.len());

}

enum Move {
    North,
    South,
    East,
    West,
}

impl From<char> for Move {
    fn from(c: char) -> Self {
        match c {
            '^' => Move::North,
            'v' => Move::South,
            '>' => Move::East,
            '<' => Move::West,
            _ => panic!()
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32
}

impl Position {
    fn make_move(&self, direction: Move) -> Position {
        match direction {
            Move::North => Position {x: self.x, y: self.y+1},
            Move::South => Position {x: self.x, y: self.y-1},
            Move::East => Position {x: self.x+1, y: self.y},
            Move::West => Position {x: self.x-1, y: self.y}
        }
    }
}
