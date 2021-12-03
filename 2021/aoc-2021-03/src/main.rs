use bitvec::prelude::*;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn parse_input(input: &str) -> Vec<u32> {
    input.lines().map(|line| u32::from_str_radix(line, 2).unwrap()).collect()
}

fn part1(input: &str) {
    let numbers = parse_input(input);


    let n_columns = input.lines().next().unwrap().trim().len();
    let n_rows = numbers.len();
    let bitvecs: Vec<BitVec<Msb0, u32>> =
        numbers.iter().map(|number| number.view_bits()[32-n_columns..32].to_bitvec()).collect();

    let mut column_bitvecs: Vec<BitVec> = Vec::with_capacity(n_columns);
    for _ in 0..n_columns {
        column_bitvecs.push(BitVec::with_capacity(n_rows));
    }
    for column in 0..n_columns {
        for row in 0..n_rows {
            column_bitvecs[column].push(*bitvecs[row].get(column).as_deref().unwrap());
        }
    }

    let mut gamma_bitvec: BitVec<Msb0, u32> = BitVec::new();
    let mut epsilon_bitvec: BitVec<Msb0, u32> = BitVec::new();
    for column in column_bitvecs {
        let n_ones = column.count_ones();
        let n_zeros = column.count_zeros();
        if n_ones > n_zeros {
            gamma_bitvec.push(true);
            epsilon_bitvec.push(false);
        } else {
            gamma_bitvec.push(false);
            epsilon_bitvec.push(true);
        }
    }
    let gamma = gamma_bitvec.load::<u32>();
    let epsilon = epsilon_bitvec.load::<u32>();
    println!("{}", gamma * epsilon);
}

fn part2(input: &str) {
    let n_columns = input.lines().next().unwrap().trim().len();
    let mut oxygen_numbers = parse_input(input);
    let mut co2_numbers = oxygen_numbers.clone();

    for i in 0..n_columns {
        if oxygen_numbers.len() == 1 {
            break;
        }
        let mut n_ones = 0;
        let mut n_zeros = 0;
        for number in oxygen_numbers.clone().into_iter() {
            if *number.view_bits::<Msb0>()[32-n_columns..32].get(i).as_deref().unwrap() {
                n_ones += 1;
            } else {
                n_zeros += 1;
            }
        }
        if n_ones >= n_zeros
        {
            oxygen_numbers = oxygen_numbers.iter().filter(|number| {
                *number.view_bits::<Msb0>()[32-n_columns..32].get(i).as_deref().unwrap()
            }).map(|number_ref| *number_ref).collect();

        } else {
            oxygen_numbers = oxygen_numbers.iter().filter(|number| {
                !*number.view_bits::<Msb0>()[32-n_columns..32].get(i).as_deref().unwrap()
            }).map(|number_ref| *number_ref).collect();
        }
    }

    for i in 0..n_columns {
        if co2_numbers.len() == 1 {
            break;
        }
        let mut n_ones = 0;
        let mut n_zeros = 0;
        for number in co2_numbers.clone().into_iter() {
            if *number.view_bits::<Msb0>()[32-n_columns..32].get(i).as_deref().unwrap() {
                n_ones += 1;
            } else {
                n_zeros += 1;
            }
        }
        if n_ones >= n_zeros
        {
            co2_numbers = co2_numbers.iter().filter(|number| {
                !*number.view_bits::<Msb0>()[32-n_columns..32].get(i).as_deref().unwrap()
            }).map(|number_ref| *number_ref).collect();

        } else {
            co2_numbers = co2_numbers.iter().filter(|number| {
                *number.view_bits::<Msb0>()[32-n_columns..32].get(i).as_deref().unwrap()
            }).map(|number_ref| *number_ref).collect();
        }
    }
    println!("{}", oxygen_numbers[0] * co2_numbers[0]);
}
