fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &String) {
    let mut floor = 0;
    for c in input.chars() {
       if c == '(' {
           floor +=1;
       }
       else if c == ')' {
           floor -= 1;
       }
    }
    println!("{}", floor);
}

fn part2(input: &String) {
    let mut floor = 0;
    for (i, c) in input.chars().enumerate() {
        if c == '(' {
            floor +=1;
        }
        else if c == ')' {
            floor -= 1;
        }
        if floor == -1 {
            println!("{}", i+1); 
            break;
        }
    }
}
