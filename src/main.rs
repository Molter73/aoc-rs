use std::collections::HashSet;
use std::fs;
use std::str::FromStr;

mod building;
use building::{get_basement, get_final_floor};

mod gifts;
use gifts::Gift;

mod grid;
use grid::Position;

mod adventcoins;
use adventcoins::AdventCoins;

mod naughtynice;

mod lights;
use lights::Grid;

mod circuit;
use circuit::Circuit;

fn main() {
    // Building stuff
    let input = fs::read_to_string("data/day1.txt").unwrap();

    println!("final floor: {}", get_final_floor(input.trim()).unwrap());
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
    let miner = AdventCoins::new("yzbqklnj", "00000");
    println!("AdventCoin: {}", miner.mine());

    // NaughtyNice stuff
    let list = fs::read_to_string("data/day5.txt").unwrap();
    let mut count = 0;

    for line in list.lines() {
        if naughtynice::validate(line) == naughtynice::Value::Nice {
            count += 1;
        }
    }
    println!("Nice strings: {}", count);

    // Light grid stuff
    let input = fs::read_to_string("data/day6.txt").unwrap();
    let mut grid = Grid::new(1000, 1000);
    grid.process(&input);
    println!("lights on: {}", grid.count());

    // Circuit stuff
    let input = fs::read_to_string("data/day7.txt").unwrap();
    let mut circuit = Circuit::new();
    circuit.assemble(input.trim().lines().collect());

    println!("a: {}", circuit.get_wire("a".to_string()));
}
