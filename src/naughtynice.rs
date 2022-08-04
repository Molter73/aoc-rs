#[derive(PartialEq, Eq, Debug)]
pub enum Value {
    Nice,
    Naughty,
}

fn validate_pairs(input: &str) -> bool {
    for (i, _) in input[..input.len() - 2].chars().enumerate() {
        let letters = &input[i..i + 2];

        if input[i + 2..].contains(letters) {
            return true;
        }
    }
    false
}

fn validate_palindrome(input: &str) -> bool {
    for (i, c) in input[..input.len() - 2].chars().enumerate() {
        if c == input.as_bytes()[i + 2] as char {
            return true;
        }
    }
    false
}

pub fn validate(input: &str) -> Value {
    let pairs = validate_pairs(input);
    let palindrome = validate_palindrome(input);

    if pairs && palindrome {
        return Value::Nice;
    }
    Value::Naughty
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation() {
        let input = "qjhvhtzxzqqjkmpb";
        let result = validate(input);
        assert_eq!(Value::Nice, result);

        let input = "xxyxx";
        let result = validate(input);
        assert_eq!(Value::Nice, result);

        let input = "uurcxstgmygtbstg";
        let result = validate(input);
        assert_eq!(Value::Naughty, result);

        let input = "ieodomkazucvgmuy";
        let result = validate(input);
        assert_eq!(Value::Naughty, result);
    }
}
