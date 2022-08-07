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

fn encode(input: &[u8]) -> Vec<u8> {
    let mut output: Vec<u8> = vec![b'\"'];

    for c in input.iter() {
        match c {
            b'\\' => {
                output.push(b'\\');
                output.push(b'\\');
            }
            b'\"' => {
                output.push(b'\\');
                output.push(b'\"');
            }
            ch => output.push(*ch),
        }
    }
    output.push(b'\"');

    output
}

fn hex_to_ascii(hex: u8) -> u8 {
    if (b'0'..=b'9').contains(&hex) {
        hex - b'0'
    } else {
        hex - b'a'
    }
}

pub fn diff(dec: bool, input: &[u8]) -> usize {
    let d = if dec {
        decode(input)
    } else {
        encode(input)
    };

    input.len().abs_diff(d.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode() {
        let input: &[u8] = br#""""#;
        let len = diff(true, input);

        assert_eq!(2, len);

        let input: &[u8] = br#""abc""#;
        let len = diff(true, input);

        assert_eq!(2, len);

        let input: &[u8] = br#""aaa\"aaa""#;
        let len = diff(true, input);

        assert_eq!(3, len);

        let input: &[u8] = br#""\x27""#;
        let len = diff(true, input);

        assert_eq!(5, len);
    }

    #[test]
    fn test_encode() {
        let input: &[u8] = br#""""#;
        let len = diff(false, input);

        assert_eq!(4, len);

        let input: &[u8] = br#""abc""#;
        let len = diff(false, input);

        assert_eq!(4, len);

        let input: &[u8] = br#""aaa\"aaa""#;
        let len = diff(false, input);

        assert_eq!(6, len);

        let input: &[u8] = br#""\x27""#;
        let len = diff(false, input);

        assert_eq!(5, len);
    }
}
