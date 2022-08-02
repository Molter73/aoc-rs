use std::fs;

fn get_floor_diff(input: &str) -> Result<i32, char> {
    let mut up_floor = 0;
    let mut down_floor = 0;

    for c in input.chars() {
        match c {
            '(' => up_floor += 1,
            ')' => down_floor += 1,
            c => return Err(c),
        }
    }
    Ok(up_floor - down_floor)
}

fn main() {
    let input = fs::read_to_string("./day1.txt").unwrap();

    println!("{}", get_floor_diff(input.trim()).unwrap());
}
