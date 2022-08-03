use std::collections::HashSet;
use std::fs;
use std::str::FromStr;

mod building;
use building::get_basement;

mod gifts;
use gifts::Gift;

mod grid;
use grid::Position;

fn main() {
    // Building stuff
    let input = fs::read_to_string("data/day1.txt").unwrap();

    println!("basement reached: {}", get_basement(input.trim()).unwrap());

    // Gifts stuff
    let gifts = fs::read_to_string("data/day2.txt").unwrap();
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

    // Grid stuff
    let travel = fs::read_to_string("data/day3.txt").unwrap();
    let mut position = Position::new(0, 0);
    let mut houses = HashSet::new();

    houses.insert(position);

    for c in travel.trim().chars() {
        if let Err(c) = position.update(c) {
            println!("Failed to parse {}", c);
            continue;
        }
        houses.insert(position);
    }
    println!("houses: {}", houses.len());
}
