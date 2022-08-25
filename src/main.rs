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

mod list;

mod path;
use path::{find_longest_path, find_shortest_path};

mod look_and_say;

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

    let a = circuit.get_wire("a".to_string());
    println!("a: {}", a);

    circuit.reset();
    circuit.set_wire("b".to_string(), a);
    circuit.assemble(input.trim().lines().collect());

    let a = circuit.get_wire("a".to_string());
    println!("a2: {}", a);

    // Presents list stuff
    let input = fs::read_to_string("data/day8.txt").unwrap();
    let mut acc: usize = 0;

    for i in input.trim().lines() {
        acc += list::diff(true, i.as_bytes());
    }
    println!("decode: {}", acc);

    let mut acc: usize = 0;

    for i in input.trim().lines() {
        acc += list::diff(false, i.as_bytes());
    }
    println!("encode: {}", acc);

    // Shortest/Longest path stuff
    let input = fs::read_to_string("data/day9.txt").unwrap();
    println!("shortest distance: {}", find_shortest_path(&input));
    println!("longest distance: {}", find_longest_path(&input));

    // Look and say stuff
    let input = String::from("1113122113");
    println!("Final length: {}", look_and_say::process(input));
}
