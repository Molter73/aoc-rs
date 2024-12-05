use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

fn main() {
    let mut args = std::env::args();
    let path = args.nth(1).unwrap();
    let input = std::fs::read_to_string(path).unwrap();
    let manual = Manual::try_from(input.as_str()).unwrap();
    println!("Total: {}", manual.process());
    println!("Total fixed: {}", manual.process_fixed());
}

#[derive(Debug)]
struct Manual {
    rules: HashMap<u64, HashSet<u64>>,
    pages: Vec<Vec<u64>>,
}

impl Manual {
    fn is_valid(&self, page: &[u64]) -> bool {
        let (first, tail) = match page.split_first() {
            Some(t) => t,
            None => return true,
        };

        if !self.is_valid_forward(*first, tail) {
            return false;
        }

        let rev = page.iter().cloned().rev().collect::<Vec<_>>();
        let (first, tail) = match rev.split_first() {
            Some(t) => t,
            None => return true,
        };
        self.is_valid_backwards(*first, tail)
    }

    fn is_valid_forward(&self, first: u64, tail: &[u64]) -> bool {
        if tail.iter().any(|i| match self.rules.get(i) {
            Some(s) => s.contains(&first),
            None => false,
        }) {
            false
        } else {
            let (first, tail) = match tail.split_first() {
                Some(t) => t,
                None => return true,
            };
            self.is_valid_forward(*first, tail)
        }
    }

    fn is_valid_backwards(&self, first: u64, tail: &[u64]) -> bool {
        if tail.iter().any(|i| match self.rules.get(&first) {
            Some(s) => s.contains(i),
            None => false,
        }) {
            false
        } else {
            let (first, tail) = match tail.split_first() {
                Some(t) => t,
                None => return true,
            };
            self.is_valid_backwards(*first, tail)
        }
    }

    fn fix_line(&self, line: &[u64]) -> Vec<u64> {
        let mut res = line.to_vec();
        res.sort_by(|a, b| match self.rules.get(b) {
            Some(set) => {
                if set.contains(a) {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }
            None => match self.rules.get(a) {
                Some(set) => {
                    if set.contains(b) {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                }
                None => Ordering::Equal,
            },
        });
        res
    }

    fn process(&self) -> u64 {
        self.pages
            .iter()
            .filter(|line| self.is_valid(line))
            .map(|line| {
                let mid = line.len() / 2;
                line.get(mid).unwrap()
            })
            .sum()
    }

    fn process_fixed(&self) -> u64 {
        self.pages
            .iter()
            .filter(|line| !self.is_valid(line))
            .map(|line| {
                let line = self.fix_line(line);
                let mid = line.len() / 2;
                *line.get(mid).unwrap()
            })
            .sum()
    }
}

impl TryFrom<&str> for Manual {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut found = false;
        let i = value.lines().fold(0, |acc, line| {
            if line.is_empty() {
                found = true;
            }

            if !found {
                acc + 1
            } else {
                acc
            }
        });

        let rules_input = value.lines().take(i).collect::<Vec<_>>();
        let pages_input = value.lines().skip(i + 1).collect::<Vec<_>>();

        let rules_input = rules_input
            .iter()
            .map(|line| {
                let components = line
                    .split("|")
                    .map(|c| c.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>();
                match components.as_slice() {
                    [_, _] => Ok(components),
                    _ => Err(format!(
                        "Invalid number of elements in rule: {}",
                        components.len()
                    )),
                }
            })
            .collect::<Result<Vec<Vec<u64>>, _>>()?;

        let mut rules: HashMap<u64, HashSet<u64>> = HashMap::new();
        for rule in rules_input {
            match rule.as_slice() {
                [lhs, rhs] => match rules.get_mut(lhs) {
                    Some(s) => {
                        s.insert(*rhs);
                    }
                    None => {
                        let mut set = HashSet::new();
                        set.insert(*rhs);
                        rules.insert(*lhs, set);
                    }
                },
                _ => unreachable!(),
            }
        }

        let pages = pages_input
            .iter()
            .map(|line| {
                line.split(',')
                    .map(|c| c.parse::<u64>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<Vec<_>>>();

        Ok(Manual { rules, pages })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

    #[test]
    fn test_is_valid() {
        let expected = [true, true, true, false, false, false];
        let manual = Manual::try_from(INPUT).unwrap();
        for (i, page) in manual.pages.iter().enumerate() {
            assert_eq!(manual.is_valid(page), expected[i]);
        }
    }

    #[test]
    fn test_process() {
        let expected = 143;
        let manual = Manual::try_from(INPUT).unwrap();
        assert_eq!(manual.process(), expected);
    }

    #[test]
    fn test_process_fixed() {
        let expected = 123;
        let manual = Manual::try_from(INPUT).unwrap();
        assert_eq!(manual.process_fixed(), expected);
    }
}
