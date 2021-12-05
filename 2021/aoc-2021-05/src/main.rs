fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &str) -> u32 {
    let mut grid = [[0u8;1000];1000];
    let mut answer = 0;
    for string in input.lines() {
       let line: Line = string.into(); 
       let covered_coords = line.get_covered_coords_p1();
       for point in covered_coords {
           grid[point.1 as usize][point.0 as usize] += 1;
           if grid[point.1 as usize][point.0 as usize] == 2 {
               answer += 1
           }
       }
    }
    answer
}

fn part2(input: &str) -> u32 {
    let mut grid = [[0u8;1000];1000];
    let mut answer = 0;
    for string in input.lines() {
       let line: Line = string.into(); 
       let covered_coords = line.get_covered_coords_p2();
       for point in covered_coords {
           grid[point.1 as usize][point.0 as usize] += 1;
           if grid[point.1 as usize][point.0 as usize] == 2 {
               answer += 1
           }
       }
    }
    answer
}

#[derive(Debug, Clone, Copy)]
struct Line {
    x1: i16,
    y1: i16,
    x2: i16,
    y2: i16,
}

impl From<&str> for Line {
    fn from(string: &str) -> Self {
        let coords: Vec<i16> = string.split(" -> ").map(|s| s.split(',')).flatten().map(|s| s.parse::<i16>().unwrap()).collect();
        Self {
            x1: coords[0],
            y1: coords[1],
            x2: coords[2],
            y2: coords[3],
        }
    }
}

impl Line {
    fn get_covered_coords_p1(&self) -> Vec<(i16, i16)> { 
        let mut covered_coords: Vec<(i16, i16)> = Vec::new();
        if self.x1 == self.x2 {
            if self.y1 < self.y2 {
                covered_coords = (self.y1..=self.y2).map(|y| (self.x1, y)).collect();
            } else {
                covered_coords = (self.y2..=self.y1).map(|y| (self.x1, y)).collect();
            }
        }
        else if self.y1 == self.y2 {
            if self.x1 < self.x2 {
                covered_coords = (self.x1..=self.x2).map(|x| (x, self.y1)).collect();
            } else {
                covered_coords = (self.x2..=self.x1).map(|x| (x, self.y1)).collect();
            }
        }
        covered_coords
    }
    fn get_covered_coords_p2(&self) -> Vec<(i16, i16)> {
        let mut covered_coords: Vec<(i16, i16)> = Vec::new();
        if self.x1 == self.x2 {
            if self.y1 < self.y2 {
                covered_coords = (self.y1..=self.y2).map(|y| (self.x1, y)).collect();
            } else {
                covered_coords = (self.y2..=self.y1).map(|y| (self.x1, y)).collect();
            }
        }
        else if self.y1 == self.y2 {
            if self.x1 < self.x2 {
                covered_coords = (self.x1..=self.x2).map(|x| (x, self.y1)).collect();
            } else {
                covered_coords = (self.x2..=self.x1).map(|x| (x, self.y1)).collect();
            }
        }
        else if (self.x2 - self.x1).abs() == (self.y2 - self.y1).abs() {
            let dist = (self.x2 - self.x1).abs();
            if (self.x2 - self.x1).is_positive() && (self.y2 - self.y1).is_positive() {
                covered_coords = (0..=dist).map(|i| (self.x1+i, self.y1+i)).collect();
            } else if (self.x2-self.x1).is_negative() && (self.y2 - self.y1).is_positive() {
                covered_coords = (0..=dist).map(|i| (self.x1-i, self.y1+i)).collect();
            } else if (self.x2-self.x1).is_positive() && (self.y2 - self.y1).is_negative() {
                covered_coords = (0..=dist).map(|i| (self.x1+i, self.y1-i)).collect();
            } else if (self.x2-self.x1).is_negative() && (self.y2 - self.y1).is_negative() {
                covered_coords = (0..=dist).map(|i| (self.x1-i, self.y1-i)).collect();
            }
        }
        covered_coords
    }
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2, Line};
    #[test]
    fn part1_example() {
        let input = std::fs::read_to_string("example.txt").unwrap();
        assert_eq!(part1(&input), 5);
    }
    #[test]
    fn part2_example() {
        let input = std::fs::read_to_string("example.txt").unwrap();
        assert_eq!(part2(&input), 12);
    }
    #[test]
    fn diagonal1() {
        let line: Line = "1,1 -> 3,3".into();
        assert_eq!(line.get_covered_coords_p2(), vec![(1,1), (2,2), (3,3)]);
    }
    #[test]
    fn diagonal2() {
        let line: Line = "9,7 -> 7,9".into();
        assert_eq!(line.get_covered_coords_p2(), vec![(9,7), (8,8), (7,9)]);
    }
}
