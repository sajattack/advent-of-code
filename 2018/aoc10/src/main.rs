#[path = "../../utils.rs"]
mod utils;

use crate::utils::get_input;

fn main() {
    let input: String = get_input(10).expect("Failed to retrieve input");
    part1(&input);
    part2(&input);
}

fn part1(input: &String) {
    let lines = input.trim().split("\n");
    let mut pos_vel = lines.map(|line| {
       let pos_x: i32 = line[10..16].trim().parse::<i32>().expect("parse fail"); 
       let pos_y: i32 = line[18..24].trim().parse::<i32>().expect("parse fail");
       let vel_x: i8 = line[36..38].trim().parse::<i8>().expect("parse fail");
       let vel_y: i8 = line[40..42].trim().parse::<i8>().expect("parse fail");
       ((pos_x, pos_y), (vel_x, vel_y))
    }).collect::<Vec<((i32, i32), (i8, i8))>>();
    
    loop {
        let dist_x =  (pos_vel.iter().map(|pos_vel| (pos_vel.0).0).max().unwrap() -
                        pos_vel.iter().map(|pos_vel| (pos_vel.0).0).min().unwrap()).abs();
        let dist_y =  (pos_vel.iter().map(|pos_vel| (pos_vel.0).1).max().unwrap() -
                        pos_vel.iter().map(|pos_vel| (pos_vel.0).1).min().unwrap()).abs();
        if dist_x < 80 && dist_y < 25 {
            break;
        }
        let new_pos_vel = pos_vel.iter().map(|((pos_x, pos_y), (vel_x, vel_y))| {
                    ((pos_x + *vel_x as i32, 
                pos_y + *vel_y as i32),
                (*vel_x, *vel_y))
        }).collect::<Vec<((i32, i32), (i8, i8))>>();
        pos_vel = new_pos_vel;
    }
    loop { 
        let dist_x =  (pos_vel.iter().map(|pos_vel| (pos_vel.0).0).max().unwrap() -
                        pos_vel.iter().map(|pos_vel| (pos_vel.0).0).min().unwrap()).abs();
        let dist_y =  (pos_vel.iter().map(|pos_vel| (pos_vel.0).1).max().unwrap() -
                        pos_vel.iter().map(|pos_vel| (pos_vel.0).1).min().unwrap()).abs();
        if dist_x >= 80 || dist_y >= 25 {
            break;
        }
        plot(&pos_vel);
        let new_pos_vel = pos_vel.iter().map(|((pos_x, pos_y), (vel_x, vel_y))| {
                ((pos_x + *vel_x as i32, 
                pos_y + *vel_y as i32),
                (*vel_x, *vel_y))
        }).collect::<Vec<((i32, i32), (i8, i8))>>();
        pos_vel = new_pos_vel;
    }
    // Won't be able to auto submit today
}

fn plot(pos_vel: &Vec<((i32, i32), (i8, i8))>) {
    let mut plot = [['.';25];80];
    let min_x = pos_vel.iter().map(|pos_vel| (pos_vel.0).0).min().unwrap();
    let min_y = pos_vel.iter().map(|pos_vel| (pos_vel.0).1).min().unwrap();
    let mut offset_x: i32 = 0;
    let mut offset_y: i32 = 0;
    if min_x < 0 {
        offset_x = min_x.abs()
    } else {
        offset_x = -min_x;
    }
    if min_y < 0 {
        offset_y = min_y.abs();
    } else {
        offset_y = -min_y;
    }
    for (x,y) in pos_vel.iter().map(|pos_vel| pos_vel.0) {
        plot[(x+offset_x) as usize][(y+offset_y) as usize] = '#'; 
    }
    for y in 0..25 {
        for x in 0..80 {
            print!("{}", plot[(x) as usize][(y) as usize]);
        }
        println!();
    }
    println!();
}

fn part2(input: &String) { 
    let lines = input.trim().split("\n");
    let mut pos_vel = lines.map(|line| {
       let pos_x: i32 = line[10..16].trim().parse::<i32>().expect("parse fail"); 
       let pos_y: i32 = line[18..24].trim().parse::<i32>().expect("parse fail");
       let vel_x: i8 = line[36..38].trim().parse::<i8>().expect("parse fail");
       let vel_y: i8 = line[40..42].trim().parse::<i8>().expect("parse fail");
       ((pos_x, pos_y), (vel_x, vel_y))
    }).collect::<Vec<((i32, i32), (i8, i8))>>();
    
    let mut iter_count = 0;

    loop {
        let dist_x =  (pos_vel.iter().map(|pos_vel| (pos_vel.0).0).max().unwrap() -
                        pos_vel.iter().map(|pos_vel| (pos_vel.0).0).min().unwrap()).abs();
        let dist_y =  (pos_vel.iter().map(|pos_vel| (pos_vel.0).1).max().unwrap() -
                        pos_vel.iter().map(|pos_vel| (pos_vel.0).1).min().unwrap()).abs();
        if dist_x < 80 && dist_y < 25 {
            break;
        }
        let new_pos_vel = pos_vel.iter().map(|((pos_x, pos_y), (vel_x, vel_y))| {
                    ((pos_x + *vel_x as i32, 
                pos_y + *vel_y as i32),
                (*vel_x, *vel_y))
        }).collect::<Vec<((i32, i32), (i8, i8))>>();
        pos_vel = new_pos_vel;
        iter_count += 1
    }
    loop { 
        let dist_x =  (pos_vel.iter().map(|pos_vel| (pos_vel.0).0).max().unwrap() -
                        pos_vel.iter().map(|pos_vel| (pos_vel.0).0).min().unwrap()).abs();
        let dist_y =  (pos_vel.iter().map(|pos_vel| (pos_vel.0).1).max().unwrap() -
                        pos_vel.iter().map(|pos_vel| (pos_vel.0).1).min().unwrap()).abs();
        if dist_x >= 80 || dist_y >= 25 {
            break;
        }
        println!("{}", iter_count);
        plot(&pos_vel);
        let new_pos_vel = pos_vel.iter().map(|((pos_x, pos_y), (vel_x, vel_y))| {
                ((pos_x + *vel_x as i32, 
                pos_y + *vel_y as i32),
                (*vel_x, *vel_y))
        }).collect::<Vec<((i32, i32), (i8, i8))>>();
        pos_vel = new_pos_vel;
        iter_count += 1;
    }
    // Won't be able to auto submit today
}
