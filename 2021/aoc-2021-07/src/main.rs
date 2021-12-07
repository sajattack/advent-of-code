fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let initial_state: Vec<i32> = input.trim().split(",").map(|s | s.parse::<i32>().unwrap()).collect();
    find_least_fuel(&initial_state)
}

fn part2(input: &str) -> i32 {
    let initial_state: Vec<i32> = input.trim().split(",").map(|s | s.parse::<i32>().unwrap()).collect();
    find_least_fuel_p2(&initial_state)
}

#[cfg(test)]
mod tests {
    use crate::find_least_fuel;
    use crate::find_least_fuel_p2;
    #[test]
    fn part1_example() {
        let input = std::fs::read_to_string("example.txt").unwrap();
        let initial_state: Vec<i32> = input.trim().split(",").map(|s| s.parse::<i32>().unwrap()).collect();
        assert_eq!(find_least_fuel(&initial_state), 37);
    }
    #[test]
    fn part2_example() {
        let input = std::fs::read_to_string("example.txt").unwrap();
        let initial_state: Vec<i32> = input.trim().split(",").map(|s| s.parse::<i32>().unwrap()).collect();
        assert_eq!(find_least_fuel_p2(&initial_state), 168);
    }
}

fn find_least_fuel(initial_state: &[i32]) -> i32 {
    let mut min_fuel = i32::MAX;
    for i in 0..*initial_state.into_iter().max().unwrap() {
        let mut fuel = 0;
        for pos in initial_state {
           fuel += ((pos - i) as i32).abs(); 
        }
        if fuel < min_fuel { min_fuel = fuel };
    }
    min_fuel
}

fn find_least_fuel_p2(initial_state: &[i32]) -> i32 {
    let mut min_fuel = i32::MAX;
    for i in 0..*initial_state.into_iter().max().unwrap() {
        let mut fuel = 0;
        for pos in initial_state {
           fuel += triangle_number(((pos - i) as i32).abs()); 
        }
        if fuel < min_fuel { min_fuel = fuel };
    }
    min_fuel
}

fn triangle_number(num: i32) -> i32 {
    (num.pow(2) + num)/2
}
