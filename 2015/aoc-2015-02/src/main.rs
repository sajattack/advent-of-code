// laziness
use itertools::min;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &String) {
    let mut sum = 0;
    for line in input.lines() {
        let dimensions: Vec<&str> = line.split('x').collect();
        let (l, w, h) = (dimensions[0].parse().unwrap(), dimensions[1].parse().unwrap(), dimensions[2].parse().unwrap());
        let side1 = l*w;
        let side2 = w*h;
        let side3 = h*l;
        sum += surface_area(l, w, h) + min([side1, side2, side3].into_iter()).unwrap();
    }
    println!("{}", sum);
}

fn part2(input: &String) {
    let mut sum = 0;
    for line in input.lines() {
        let dimensions: Vec<&str> = line.split('x').collect();
        let (l, w, h) = (dimensions[0].parse().unwrap(), dimensions[1].parse().unwrap(), dimensions[2].parse().unwrap());
        let side1_perim = perimeter(l, w);
        let side2_perim = perimeter(w, h);
        let side3_perim = perimeter(h, l);
        let volume = l*w*h; 
        sum += volume + min([side1_perim, side2_perim, side3_perim].into_iter()).unwrap();
    }
    println!("{}", sum);

}

fn surface_area(l: u32, w: u32, h: u32) -> u32 {
    2*l*w + 2*w*h + 2*h*l
}

fn perimeter(x: u32, y: u32) -> u32 {
    x*2 + y*2
}
