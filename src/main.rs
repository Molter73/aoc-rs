use std::fs;
use std::str::FromStr;

mod gifts;
use gifts::Gift;

fn get_floor_diff(c: char) -> Result<i32, char> {
    match c {
        '(' => Ok(1),
        ')' => Ok(-1),
        c => return Err(c),
    }
}

fn get_basement(input: &str) -> Result<usize, char> {
    let mut floor = 0;

    for (i, c) in input.chars().enumerate() {
        match get_floor_diff(c) {
            Ok(diff) => floor += diff,
            Err(e) => return Err(e),
        }

        if floor < 0 {
            return Ok(i + 1);
        }
    }
    Err('_')
}

fn main() {
    // Building stuff
    let input = fs::read_to_string("./day1.txt").unwrap();

    println!("{}", get_basement(input.trim()).unwrap());

    // Gifts stuff
    let gifts = fs::read_to_string("./day2.txt").unwrap();
    let mut wrapper: usize = 0;
    let mut ribbon: usize = 0;

    for line in gifts.lines() {
        let gift = Gift::from_str(line).unwrap();

        // Calculate required wrapper
        let area = gift.get_area();
        let smallest = gift.get_smallest_side();

        wrapper += area;
        wrapper += smallest;

        // Calculate required ribbon
        let volume = gift.get_volume();
        let wrap_around = gift.get_wrap_around();

        ribbon += volume;
        ribbon += wrap_around
    }
    println!("wrapper: {}", wrapper);
    println!("ribbon: {}", ribbon);
}
