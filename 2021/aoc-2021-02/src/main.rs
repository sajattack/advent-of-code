fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let mut depth = 0;
    let mut horizontal = 0;
    for line in input.lines() {
        let line_bytes: Vec<u8> = line.bytes().collect();
        if line.contains("forward")
        {
            horizontal += (line_bytes[line.len()-1]-0x30) as u32
        }
        if line.contains("down")
        {
            depth += (line_bytes[line.len()-1]-0x30) as u32
        }
        if line.contains("up")
        {
            depth -= (line_bytes[line.len()-1]-0x30) as u32
        }
    }
    println!("{}", horizontal * depth);
}

fn part2(input: &str) {
    let mut depth = 0;
    let mut aim = 0;
    let mut horizontal = 0;
    for line in input.lines() {
        let line_bytes: Vec<u8> = line.bytes().collect();
        if line.contains("forward")
        {
            horizontal += (line_bytes[line.len()-1]-0x30) as u32;
            depth += aim * (line_bytes[line.len()-1]-0x30) as u32;
        }
        if line.contains("down")
        {
            aim += (line_bytes[line.len()-1]-0x30) as u32
        }
        if line.contains("up")
        {
            aim -= (line_bytes[line.len()-1]-0x30) as u32
        }
    }
    println!("{}", horizontal * depth);
}
