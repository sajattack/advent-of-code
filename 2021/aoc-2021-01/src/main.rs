fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let mut increases = 0;
    let mut last_val: u32 = 0;
    
    for line in input.lines() {
        let val = line.parse::<u32>().unwrap();
        if last_val < val {
            increases += 1;
        }
        last_val = val;
    }
    println!("{}", increases - 1);
}

fn part2(input: &str) {
    let mut increases = 0;
    let mut last_sum: u32 = 0;
    
    let lines: Vec<&str> = input.lines().collect();
    for win in lines.windows(3)
    {
        let val1 = win[0].parse::<u32>().unwrap();
        let val2 = win[1].parse::<u32>().unwrap();
        let val3 = win[2].parse::<u32>().unwrap();
        let sum = val1 + val2 + val3;
        if last_sum < sum {
            increases += 1;
        }
        last_sum = sum;
    }
    println!("{}", increases - 1);
}
