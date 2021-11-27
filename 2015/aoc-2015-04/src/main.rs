use md5::{Md5, Digest};

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap().trim().to_owned();
    part1(&input);
    part2(&input);
}

fn part1(input: &String) {
    let mut number = 1;
    let mut hash_string = String::new();
    while !hash_string.starts_with("00000") {
        let mut hasher = Md5::new();
        hasher.update((input.to_owned() + &number.to_string()).as_bytes());
        let hash = hasher.finalize();
        hash_string = format!("{:x}", hash);
        number += 1;
    }
    println!("{}", number-1)
}

fn part2(input: &String) {
    let mut number = 1;
    let mut hash_string = String::new();
    while !hash_string.starts_with("000000") {
        let mut hasher = Md5::new();
        hasher.update((input.to_owned() + &number.to_string()).as_bytes());
        let hash = hasher.finalize();
        hash_string = format!("{:x}", hash);
        number += 1;
    }
    println!("{}", number-1)

}
