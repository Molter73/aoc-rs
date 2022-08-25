fn dump_char(c: char, count: usize) -> String {
    if count == 0 {
        String::from("")
    } else {
        count.to_string() + &c.to_string()
    }
}

fn process_once(input: String) -> String {
    let mut curr: char = '\0';
    let mut count: usize = 0;
    let mut output = String::new();

    for c in input.chars() {
        if c != curr {
            output += &dump_char(curr, count);
            curr = c;
            count = 1;
        } else {
            count += 1;
        }
    }

    output + &dump_char(curr, count)
}

pub fn process(input: String) -> usize {
    let mut s = input;

    for _ in 0..50 {
        s = process_once(s);
    }
    s.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_look_say() {
        let input = String::from("1");
        assert_eq!("11", process_once(input));

        let input = String::from("11");
        assert_eq!("21", process_once(input));

        let input = String::from("21");
        assert_eq!("1211", process_once(input));

        let input = String::from("1211");
        assert_eq!("111221", process_once(input));

        let input = String::from("111221");
        assert_eq!("312211", process_once(input));
    }
}
