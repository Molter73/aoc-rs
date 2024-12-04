use std::{error::Error, fmt::Display};

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args();
    let path = args.nth(1).unwrap();
    let input = std::fs::read_to_string(path).unwrap();
    let lists = Lists::try_from(input.as_str())?;
    println!("Distance is: {}", lists.get_distance());
    println!("Similarity is: {}", lists.get_similarity());
    Ok(())
}

// Holds the two lists of place IDs
struct Lists {
    left: Vec<u64>,
    right: Vec<u64>,
}

impl Lists {
    fn get_distance(&self) -> u64 {
        self.left
            .iter()
            .zip(self.right.iter())
            .map(|(left, right)| left.abs_diff(*right))
            .sum()
    }

    fn get_similarity(&self) -> u64 {
        self.left
            .iter()
            .map(|l| {
                let mult = self.right.iter().filter(|r| *r == l).count();
                l * mult as u64
            })
            .sum()
    }
}

#[derive(Debug)]
enum ListsError {
    ParseError(String),
}

impl Display for ListsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ListsError::ParseError(e) => write!(f, "{e}"),
        }
    }
}

impl From<std::num::ParseIntError> for ListsError {
    fn from(value: std::num::ParseIntError) -> Self {
        ListsError::ParseError(value.to_string())
    }
}

impl Error for ListsError {}

impl TryFrom<&str> for Lists {
    type Error = ListsError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut left = Vec::new();
        let mut right = Vec::new();
        for line in value.lines() {
            for elem in line.split_whitespace().enumerate() {
                match elem {
                    (0, v) => {
                        let v = v.parse::<u64>()?;
                        left.push(v);
                    }
                    (1, v) => {
                        let v = v.parse::<u64>()?;
                        right.push(v);
                    }
                    (i, v) => {
                        return Err(ListsError::ParseError(format!("Invalid input: ({i}) {v}")))
                    }
                }
            }
        }
        left.sort();
        right.sort();
        Ok(Lists { left, right })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_list() {
        let input = r#"3   4
4   3
2   5
1   3
3   9
3   3
"#;
        let left = [1, 2, 3, 3, 3, 4];
        let right = [3, 3, 3, 4, 5, 9];
        let lists = Lists::try_from(input).unwrap();
        for (index, (l, r)) in left.iter().zip(right.iter()).enumerate() {
            assert_eq!(lists.left[index], *l);
            assert_eq!(lists.right[index], *r);
        }
    }

    #[test]
    fn test_get_difference() {
        let input = r#"3   4
4   3
2   5
1   3
3   9
3   3
"#;
        let lists = Lists::try_from(input).unwrap();
        assert_eq!(lists.get_distance(), 11);
    }
    #[test]
    fn test_get_similarity() {
        let input = r#"3   4
4   3
2   5
1   3
3   9
3   3
"#;
        let lists = Lists::try_from(input).unwrap();
        assert_eq!(lists.get_similarity(), 31);
    }
}
