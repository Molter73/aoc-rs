fn main() {
    let mut args = std::env::args();
    let path = args.nth(1).unwrap();
    let input = std::fs::read_to_string(path).unwrap();
    let reports = input
        .lines()
        .map(Report::try_from)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let count = reports.iter().filter(|r| r.is_safe()).count();
    println!("Safe reports: {count}");

    let count = reports.iter().filter(|r| r.dampened_is_safe()).count();
    println!("Dampened reports: {count}");
}

#[derive(Clone)]
struct Report {
    levels: Vec<u64>,
}

impl Report {
    fn is_safe(&self) -> bool {
        self.failure().is_none()
    }

    fn dampened_is_safe(&self) -> bool {
        match self.failure() {
            Some(i) => {
                for offset in [-1, 0, 1] {
                    let i = i + offset;
                    if i < 0 || i >= self.levels.len() as isize {
                        continue;
                    }

                    let mut r = self.clone();
                    r.levels.remove(i as usize);
                    if r.is_safe() {
                        return true;
                    }
                }
                false
            }
            None => true,
        }
    }

    fn failure(&self) -> Option<isize> {
        let is_increasing = self.levels[0] < self.levels[1];
        for (idx, (i, j)) in self.levels.iter().zip(self.levels[1..].iter()).enumerate() {
            if i.abs_diff(*j) > 3 || i == j {
                return Some(idx as isize);
            }

            match is_increasing {
                true if i > j => return Some(idx as isize),
                false if i < j => return Some(idx as isize),
                _ => {}
            }
        }
        None
    }
}

impl TryFrom<&str> for Report {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let levels = value
            .split_whitespace()
            .map(|e| match e.parse::<u64>() {
                Ok(v) => Ok(v),
                Err(e) => Err(format!("{e}")),
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Report { levels })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        let tests = [
            ("7 6 4 2 1", vec![7, 6, 4, 2, 1]),
            ("1 2 7 8 9", vec![1, 2, 7, 8, 9]),
            ("9 7 6 2 1", vec![9, 7, 6, 2, 1]),
            ("1 3 2 4 5", vec![1, 3, 2, 4, 5]),
            ("8 6 4 4 1", vec![8, 6, 4, 4, 1]),
            ("1 3 6 7 9", vec![1, 3, 6, 7, 9]),
        ];

        for (input, expected) in tests {
            let report = Report::try_from(input).unwrap();
            assert_eq!(report.levels, expected);
        }
    }

    #[test]
    fn test_is_safe() {
        let tests = [
            ("7 6 4 2 1", true),
            ("1 2 7 8 9", false),
            ("9 7 6 2 1", false),
            ("1 3 2 4 5", false),
            ("8 6 4 4 1", false),
            ("1 3 6 7 9", true),
            ("9 7 6 4 2", true),
            ("20 17 14 13 12 11 8 5", true),
            ("25 28 29 31 32 34 37 39", true),
            ("25 25 25 25 25", false),
            ("71 72 74 76 78 80 82 82", false),
        ];

        for (input, expected) in tests {
            let report = Report::try_from(input).unwrap();
            println!("{input}, {expected}");
            assert_eq!(report.is_safe(), expected);
        }
    }

    #[test]
    fn test_dampened_is_safe() {
        let tests = [
            ("7 6 4 2 1", true),
            ("1 2 7 8 9", false),
            ("9 7 6 2 1", false),
            ("1 3 2 4 5", true),
            ("8 6 4 4 1", true),
            ("1 3 6 7 9", true),
            ("9 7 6 4 2", true),
            ("20 17 14 13 12 11 8 5", true),
            ("25 28 29 31 32 34 37 39", true),
            ("25 25 25 25 25", false),
            ("71 72 74 76 78 80 82 82", true),
            ("71 72 64 74 76 78 80 82", true),
            ("72 71 72 74 76 78 80 82", true),
        ];

        for (input, expected) in tests {
            let report = Report::try_from(input).unwrap();
            println!("{input}, {expected}");
            assert_eq!(report.dampened_is_safe(), expected);
        }
    }
}
