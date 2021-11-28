const ALPHABET: &'static str = "abcdefghijklmnopqrstuvwxyz";

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap().trim().to_owned();
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let mut nice_string_count = 0;
    for line in input.lines() {
        if part1_rule1(line) && part1_rule2(line) && part1_rule3(line) {
            nice_string_count += 1;
        }
    }
    println!("{}", nice_string_count);
}

fn part2(input: &str) {
    let mut nice_string_count = 0;
    for line in input.lines() {
        if part2_rule1(line) && part2_rule2(line) {
            nice_string_count += 1;
        }
    }
    println!("{}", nice_string_count);

}


fn part1_rule1(string: &str) -> bool {
    count_vowels(string) >= 3
}


fn part1_rule2(string: &str) -> bool {
    for letter in ALPHABET.chars() {
        let double_letter = format!("{}{}", letter, letter);
        if string.contains(&double_letter) {
            return true
        }
    }
    false
}

fn part1_rule3(string: &str) -> bool {
   !(string.contains("ab") || string.contains("cd") || string.contains("pq") || string.contains("xy")) 
}

fn part2_rule1(string: &str) -> bool {
    let mut ret = false;
    'outer: for char1 in string.chars() {
        for char2 in string.chars() {
            let letter_pair_string = format!("{}{}",char1, char2);
            let first_match = string.find(&letter_pair_string);
            if first_match.is_some() {
                let second_match = string[first_match.unwrap()+2..string.len()].find(&letter_pair_string);
                ret = first_match.is_some() && second_match.is_some();
                if ret == true { break 'outer; }
            }
        }
    }
    ret
}

fn part2_rule2(string: &str) -> bool {
    string.as_bytes().windows(3).map(|win| win[0] == win[2]).any(|b| b == true)
}

fn count_vowels(string: &str) -> i32 {
    const VOWELS: &'static str = "aeiou";
    let mut number_of_vowels = 0;
    let mut working_str = string;
    let mut vowel_iter = VOWELS.chars();
    let mut vowel_opt = vowel_iter.next();
    loop {
        if let Some(vowel) = vowel_opt {
            if let Some(found) = working_str.find(vowel) {
                number_of_vowels += 1;
                working_str = &working_str[found+1..working_str.len()];   
            } else {
                working_str = string;
                vowel_opt = vowel_iter.next();
            }
        }
        else {
            break;
        }
    }
    number_of_vowels
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_vowels() {
        assert_eq!(count_vowels("fasjdfiodasfjs"), 4)
    }
    #[test]
    fn test_xyx() {
        assert_eq!(part2_rule2("xyx"), true)
    }
    #[test]
    fn test_abcdefeghi() {
        assert_eq!(part2_rule2("acdefeghi"), true)
    }
    #[test]
    fn test_aaa() {
        assert_eq!(part2_rule2("aaa"), true)
    }
    #[test]
    fn test_qjhvhtzxzqqjkmpb() {
        assert_eq!(part2_rule1("qjhvtzxzqqjkmpb") && part2_rule2("qjhvhtzxzqqjkmpb"), true)
    }
    #[test]
    fn test_xxyxx() {
        assert_eq!(part2_rule1("xxyxx") && part2_rule2("xxyxx"), true)
    }
    #[test]
    fn test_uurcxstgmygtbstg() {
        assert_eq!(part2_rule1("uurcxstgmygtbstg") && part2_rule2("uurcxstgmygtbstg"), false)
    }
    #[test]
    fn test_ieodomkazucvgmuy() {
        assert_eq!(part2_rule1("ieodomkazucvgmuy") && part2_rule2("ieodomkazucvgmuy"), false)
    }
}

