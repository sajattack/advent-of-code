fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &str) -> u64 {
    let initial_state: Vec<u8> = input.trim().split(",").map(|s | s.parse::<u8>().unwrap()).collect();
    count_fish(&initial_state, 80)
}

fn part2(input: &str) -> u64 {
    let initial_state: Vec<u8> = input.trim().split(",").map(|s | s.parse::<u8>().unwrap()).collect();
    count_fish(&initial_state, 256)
}

#[cfg(test)]
mod tests {
    use crate::count_fish;
    #[test]
    fn part1_example() {
        let input = std::fs::read_to_string("example.txt").unwrap();
        let initial_state: Vec<u8> = input.trim().split(",").map(|s| s.parse::<u8>().unwrap()).collect();
        assert_eq!(count_fish(&initial_state, 18), 26);
    }
}

fn count_fish(initial_state: &[u8], days: u32) -> u64 {
    let mut number_on_day = [0u64;9];
    let mut number_on_next_day = [0u64;9];
    for number in initial_state {
        for i in 0..=8 {
            if *number == i {
                number_on_day[i as usize] += 1;
            }
        }
    }
    for _ in 0..days {
        for j in 1..=8 {
            number_on_next_day[j-1] = number_on_day[j];
        }
        number_on_next_day[6] = number_on_day[0] + number_on_day[7];
        number_on_next_day[8] = number_on_day[0];
        number_on_day = number_on_next_day;
    }
       
    number_on_day.into_iter().sum()
}
