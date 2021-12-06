fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &str) -> u32 {
}

fn part2(input: &str) -> u32 {
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};
    #[test]
    fn part1_example() {
        let input = std::fs::read_to_string("example.txt").unwrap();
        assert_eq!(part1(&input), 5);
    }
}
