#[derive(PartialEq, Eq, Debug)]
pub enum Value {
    Nice,
    Naughty,
}

const VOWELS: &str = "aeiou";
const FORBIDDEN_COMBINATIONS: &'static [&'static str] = &["ab", "cd", "pq", "xy"];

pub fn validate(input: &str) -> Value {
    let mut vowels = 0;
    let mut repeats_letter = false;

    for fc in FORBIDDEN_COMBINATIONS {
        if input.contains(fc) {
            return Value::Naughty;
        }
    }

    for (i, c) in input.chars().enumerate() {
        if VOWELS.contains(c) {
            vowels += if vowels < 3 { 1 } else { 0 };
        }

        if i < input.len() - 1 && c == input.as_bytes()[i + 1] as char {
            repeats_letter = true;
        }

        if vowels >= 3 && repeats_letter {
            return Value::Nice;
        }
    }
    Value::Naughty
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation() {
        let input = "ugknbfddgicrmopn";
        assert_eq!(Value::Nice, validate(input));

        let input = "aaa";
        assert_eq!(Value::Nice, validate(input));

        let input = "jchzalrnumimnmhp";
        assert_eq!(Value::Naughty, validate(input));

        let input = "haegwjzuvuyypxyu";
        assert_eq!(Value::Naughty, validate(input));

        let input = "dvszwmarrgswjxmb";
        assert_eq!(Value::Naughty, validate(input));
    }
}
