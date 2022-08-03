use std::collections::HashSet;
use std::fs;
use std::str::FromStr;

mod building;
use building::get_basement;

mod gifts;
use gifts::Gift;

mod grid;
use grid::Position;

mod adventcoins;
use adventcoins::AdventCoins;

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
    let mut santa = Position::new(0, 0);
    let mut robo_santa = Position::new(0, 0);
    let mut houses = HashSet::new();

    houses.insert(santa);

    for (i, c) in travel.trim().chars().enumerate() {
        let santa: &mut Position = if i % 2 == 0 {
            &mut santa
        } else {
            &mut robo_santa
        };

        if let Err(c) = santa.update(c) {
            println!("Failed to parse {}", c);
            continue;
        }
        houses.insert(*santa);
    }
    println!("houses: {}", houses.len());

    // AdventCoins stuff
    let miner = AdventCoins::new("yzbqklnj");
    println!("AdventCoin: {}", miner.mine());
}
