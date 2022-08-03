fn get_floor_diff(c: char) -> Result<i32, char> {
    match c {
        '(' => Ok(1),
        ')' => Ok(-1),
        c => Err(c),
    }
}

pub fn get_basement(input: &str) -> Result<usize, char> {
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
