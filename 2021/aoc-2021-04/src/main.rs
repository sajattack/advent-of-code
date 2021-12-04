fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let mut line_iter = input.lines();
    let bingo_numbers: Vec<u8> = line_iter.next().unwrap().split(',').map(|n| n.parse::<u8>().unwrap()).collect();

    let lines: Vec<&str> = line_iter.collect();
    let mut bingo_cards: Vec<BingoCard> =
    lines.chunks(6).map(|chunk| {
        let mut bingo_card_str = String::new();
        for i in 0..6 {
            let line = chunk[i];
            if !line.trim().is_empty() {
                bingo_card_str += format!("{}\n",line).as_str();
            }
        }
        let bingo_card: BingoCard = bingo_card_str.as_str().into();
        bingo_card
    }).collect();
    'outer: for number in bingo_numbers {
        for card in &mut bingo_cards {
            card.mark_number(number);
            if card.is_winner() {
                let sum: u32 = card.get_unmarked_numbers().iter().map(|byte| *byte as u32).sum();
                println!("{}", sum * (number as u32));
                break 'outer;
            }
        }
    }
}

fn part2(input: &str) {
    let mut line_iter = input.lines();
    let bingo_numbers: Vec<u8> = line_iter.next().unwrap().split(',').map(|n| n.parse::<u8>().unwrap()).collect();

    let lines: Vec<&str> = line_iter.collect();
    let mut bingo_cards: Vec<BingoCard> =
    lines.chunks(6).map(|chunk| {
        let mut bingo_card_str = String::new();
        for i in 0..6 {
            let line = chunk[i];
            if !line.trim().is_empty() {
                bingo_card_str += format!("{}\n",line).as_str();
            }
        }
        let bingo_card: BingoCard = bingo_card_str.as_str().into();
        bingo_card
    }).collect();
    let mut winners: Vec<usize> = Vec::new();
    let mut scores: Vec<u32> = Vec::new();
    for number in bingo_numbers {
        for (card_num, card) in bingo_cards.iter_mut().enumerate() {
            card.mark_number(number);
            if card.is_winner() && !winners.contains(&card_num) {
                let sum: u32 = card.get_unmarked_numbers().iter().map(|byte| *byte as u32).sum();
                let score = sum * (number as u32);
                winners.push(card_num);
                scores.push(score);
            }
        }
    }
    println!("{}", scores[scores.len()-1]);
}

#[derive(Debug, Clone, Copy)]
struct BingoCard {
   number_grid: [[u8; 5]; 5],
   marked: [[bool; 5]; 5],
}

impl BingoCard {
    fn mark_number(&mut self, number: u8) {
        for y in 0..5 {
            for x in 0..5 {
                if self.number_grid[y][x] == number {
                    self.marked[y][x] = true;
                }
            }
        }
    }

    fn is_winner(&self) -> bool {
        (0..5).map(|y|  self.marked[y] == [true, true, true, true, true]).chain(
        (0..5).map(|x| 
            self.marked[0][x] == true
            && self.marked[1][x] == true
            && self.marked[2][x] == true 
            && self.marked[3][x] == true  
            && self.marked[4][x] == true  
        )).any(|b| b == true)
    }

    fn get_unmarked_numbers(&self) -> Vec<u8> {
        let mut unmarked_numbers: Vec<u8> = Vec::new();
        for y in 0..5 {
            for x in 0..5 {
               if self.marked[y][x] == false 
               {
                   unmarked_numbers.push(self.number_grid[y][x]);
               }
            }
        }
        unmarked_numbers
    }
}

impl From<&str> for BingoCard {
    fn from(input: &str) -> Self {
        let mut number_grid = [[0u8;5];5];
        for (y, line) in input.lines().enumerate() {
            for (x, s) in line.split_whitespace().enumerate() {
               number_grid[y][x] = s.parse::<u8>().unwrap();
            }
        }
        let marked = [[false;5];5];
        Self {
            number_grid,
            marked,
        }
    }
}
