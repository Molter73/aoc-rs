const PASSWORD_LENGTH: usize = 8;
const FORBIDDEN_LETTERS: &[u8] = &[b'i', b'o', b'l'];

fn check_straight(input: &[u8]) -> bool {
    for (i, c) in input.iter().enumerate() {
        if i == PASSWORD_LENGTH - 2 {
            break;
        }

        if input[i + 1] == c + 1 && input[i + 2] == c + 2 {
            return true;
        }
    }
    false
}

fn check_forbidden_letters(input: &[u8]) -> bool {
    for c in input.iter() {
        if FORBIDDEN_LETTERS.contains(c) {
            return false;
        }
    }
    true
}

fn check_pairs(input: &[u8]) -> bool {
    let mut first: Option<&[u8]> = None;

    for (i, c) in input.iter().enumerate() {
        if i == PASSWORD_LENGTH - 1 {
            return false;
        }

        if input[i + 1] == *c {
            match first {
                Some(pair) => {
                    if !pair.contains(c) {
                        return true;
                    }
                }
                None => first = Some(&input[i..=i + 1]),
            }
        }
    }
    false
}

fn check_password(input: &[u8]) -> bool {
    check_forbidden_letters(input) && check_straight(input) && check_pairs(input)
}

fn increment(input: &mut [u8]) {
    for i in (0..input.len()).rev() {
        input[i] += 1;
        if input[i] > b'z' {
            input[i] = b'a';
        } else {
            break;
        }
    }
}

pub fn get_next_pass(input: &mut [u8]) {
    loop {
        increment(input);

        if check_password(input) {
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_straight() {
        let input: Vec<u8> = "hijklmmn".bytes().collect();
        assert!(check_straight(&input));

        let input: Vec<u8> = "abbceffg".bytes().collect();
        assert!(!check_straight(&input));
    }

    #[test]
    fn test_forbidden() {
        let input: Vec<u8> = "hijklmmn".bytes().collect();
        assert!(!check_forbidden_letters(&input));
    }

    #[test]
    fn test_pairs() {
        let input: Vec<u8> = "abbceffg".bytes().collect();
        assert!(check_pairs(&input));

        let input: Vec<u8> = "abbcegjk".bytes().collect();
        assert!(!check_pairs(&input));
    }

    #[test]
    fn test_valid_passwords() {
        let input: Vec<u8> = "abcdffaa".bytes().collect();
        assert!(check_password(&input));

        let input: Vec<u8> = "ghjaabcc".bytes().collect();
        assert!(check_password(&input));
    }

    #[test]
    fn test_next_pass() {
        let mut input: Vec<u8> = "abcdefgh".bytes().collect();
        get_next_pass(&mut input);
        assert_eq!("abcdffaa".bytes().collect::<Vec<u8>>(), input);

        let mut input: Vec<u8> = "ghijklmn".bytes().collect();
        get_next_pass(&mut input);
        assert_eq!("ghjaabcc".bytes().collect::<Vec<u8>>(), input);
    }

    #[test]
    fn test_increment() {
        let mut input: Vec<u8> = "xx".bytes().collect();

        increment(&mut input);
        assert_eq!("xy".bytes().collect::<Vec<u8>>(), input);

        increment(&mut input);
        assert_eq!("xz".bytes().collect::<Vec<u8>>(), input);

        increment(&mut input);
        assert_eq!("ya".bytes().collect::<Vec<u8>>(), input);

        increment(&mut input);
        assert_eq!("yb".bytes().collect::<Vec<u8>>(), input);
    }
}
