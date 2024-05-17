fn get_floor_diff(c: char) -> Result<i32, char> {
    match c {
        '(' => Ok(1),
        ')' => Ok(-1),
        c => Err(c),
    }
}

pub fn get_final_floor(input: &str) -> Result<i32, char> {
    let mut floor = 0;

    for c in input.chars() {
        match get_floor_diff(c) {
            Ok(diff) => floor += diff,
            Err(e) => return Err(e),
        }
    }
    Ok(floor)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_floor_0() {
        let expected: i32 = 0;

        let input = "(())";
        assert_eq!(expected, get_final_floor(input).unwrap());

        let input = "()()";
        assert_eq!(expected, get_final_floor(input).unwrap());
    }

    #[test]
    fn test_floor_3() {
        let expected: i32 = 3;

        let input = "(((";
        assert_eq!(expected, get_final_floor(input).unwrap());

        let input = "(()(()(";
        assert_eq!(expected, get_final_floor(input).unwrap());

        let input = "))(((((";
        assert_eq!(expected, get_final_floor(input).unwrap());
    }

    #[test]
    fn test_floor_b1() {
        let expected: i32 = -1;

        let input = "())";
        assert_eq!(expected, get_final_floor(input).unwrap());

        let input = "))(";
        assert_eq!(expected, get_final_floor(input).unwrap());
    }

    #[test]
    fn test_floor_b3() {
        let expected: i32 = -3;

        let input = ")))";
        assert_eq!(expected, get_final_floor(input).unwrap());

        let input = ")())())";
        assert_eq!(expected, get_final_floor(input).unwrap());
    }

    #[test]
    fn test_basement() {
        let input = ")";
        assert_eq!(1, get_basement(input).unwrap());

        let input = "()())";
        assert_eq!(5, get_basement(input).unwrap());
    }
}
