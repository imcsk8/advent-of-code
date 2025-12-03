//https://adventofcode.com/2025/day/1
use std::fs;

static INPUT_DATA: &str = "../data/input";

fn main() {
    let mut password = 0;
    let mut current_position = 50;
    for line in fs::read_to_string(INPUT_DATA).unwrap().lines() {
        current_position = get_position(line, current_position);
        println!("Rotated: {}\t-> {}", line, current_position);
        if current_position == 0 {
            password += 1;
        }
    }

    println!("\npassword: {}", password);
}


/// Returns the new dialed position
fn get_position(rotation: &str, curr_position: i32) -> i32 {
    let delta: i32;
    let mut my_chars = rotation.chars();
    let direction = my_chars.next().unwrap();
    let rotation: i32 = my_chars.as_str().parse().unwrap();
    if rotation >= 100 {
        delta = rotation % 100;
    } else {
        delta = rotation;
    }

    // Check rotation direction
    if direction == 'L' {
        if delta > curr_position {
            let r: i32 = delta - curr_position;
            return 100 - r.abs();
        } else {
            curr_position - delta
        }
    } else {
        let r: i32 = delta + curr_position;
        if r >= 100 {
            let r: i32 = 100 - r;
            return r.abs();
        }
        r
    }
}

