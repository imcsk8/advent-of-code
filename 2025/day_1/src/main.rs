//https://adventofcode.com/2025/day/1
use std::fs;

static INPUT_DATA: &str = "../data/input";
//static INPUT_DATA: &str = "../data/test_input";

fn main() {
    let mut password = 0;
    let mut zero_clicks = 0;
    let mut current_position = 50;
    for line in fs::read_to_string(INPUT_DATA).unwrap().lines() {
        let curr_zero_clicks;
        (current_position, curr_zero_clicks) = get_position(line, current_position);
        zero_clicks += curr_zero_clicks;
        println!("Rotated: {}\t-> Current: {}\t-> Zero Clicks: {}",
            line, current_position, zero_clicks);
        if current_position == 0 {
            password += 1;
        }
    }

    println!("\nPassword: {}", password);
    println!("\nZero Clicks: {}", zero_clicks);
    println!("\nNew Password: {}", password + zero_clicks);
}


/// Returns the new dialed position
fn get_position(rotation: &str, curr_position: i32) -> (i32, i32) {
    let delta: i32;
    let mut zero_clicks = 0;
    let mut my_chars = rotation.chars();
    let direction = my_chars.next().unwrap();
    let rotation: i32 = my_chars.as_str().parse().unwrap();
    if rotation >= 100 {
        delta = rotation % 100;
        zero_clicks = rotation / 100;
    } else {
        delta = rotation;
    }

    // Check rotation direction
    if direction == 'L' {
        if delta > curr_position {
            let mut r: i32 = delta - curr_position;
            r = 100 - r.abs();
            /* Every time we rotate left to a delta bigger than the current
              position we click 0
            */
            // If position is 0 it does not count as zero click
            if r != 0 && curr_position !=0 {
                zero_clicks += 1;
            }
            return (r, zero_clicks);
        } else {
            ((curr_position - delta), zero_clicks)
        }
    } else {
        let r: i32 = delta + curr_position;
        if r >= 100 {
            let r: i32 = 100 - r;
            /*
            * When we rotate to the right and the result is bigger
            * than 100 it means a full rotation that clicks 0
            * if position is 0 it does not count as a click
            */
            if r != 0 {
                zero_clicks += 1;
            }
            return (r.abs(), zero_clicks);
        }
        (r, zero_clicks)
    }
}

