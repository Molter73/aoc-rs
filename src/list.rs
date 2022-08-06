fn decode(input: &[u8]) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();
    let mut escaping = false;
    let mut hex: usize = 0;

    for (i, c) in input.iter().enumerate() {
        match c {
            b'\\' => {
                if escaping {
                    output.push(b'\\');
                }
                escaping = !escaping;
            }
            b'\"' => {
                if escaping {
                    output.push(b'\"');
                }
                escaping = false;
            }
            b'x' => {
                let raw_char = if escaping {
                    hex = 2;
                    let upper = input[i + 1];
                    let lower = input[i + 2];
                    hex_to_ascii(upper) * 16 + hex_to_ascii(lower)
                } else {
                    b'x'
                };
                escaping = false;
                output.push(raw_char);
            }
            ch => {
                escaping = false;
                if hex != 0 {
                    hex -= 1;
                } else {
                    output.push(*ch)
                }
            }
        }
    }
    output
}

fn hex_to_ascii(hex: u8) -> u8 {
    if hex >= b'0' && hex <= b'9' {
        hex - b'0'
    } else {
        hex - b'a'
    }
}

pub fn diff(input: &[u8]) -> usize {
    let d = decode(input);

    input.len() - d.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list() {
        let input: &[u8] = br#""""#;
        let len = diff(input);

        assert_eq!(2, len);

        let input: &[u8] = br#""abc""#;
        let len = diff(input);

        assert_eq!(2, len);

        let input: &[u8] = br#""aaa\"aaa""#;
        let len = diff(input);

        assert_eq!(3, len);

        let input: &[u8] = br#""\x27""#;
        let len = diff(input);

        assert_eq!(5, len);
    }
}
