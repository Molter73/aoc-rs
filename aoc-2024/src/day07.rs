fn main() {
    let mut args = std::env::args();
    let path = args.nth(1).unwrap();
    let input = std::fs::read_to_string(path).unwrap();
    let calibrations: Vec<_> = input.lines().map(Calibration::from).collect();
    println!("Result: {}", process(&calibrations));
}

fn process(calibrations: &[Calibration]) -> i64 {
    calibrations
        .iter()
        .filter(|c| c.is_valid())
        .fold(0, |acc, c| acc + c.res)
}

#[derive(Debug, Clone)]
enum Operator {
    Add,
    Multiply,
    Concat,
}

impl Operator {
    fn calc(&self, lhs: i64, rhs: i64) -> i64 {
        match self {
            Operator::Add => lhs + rhs,
            Operator::Multiply => lhs * rhs,
            Operator::Concat => {
                let mut rhs_copy = rhs;
                let mut lhs = lhs;
                while rhs_copy != 0 {
                    rhs_copy /= 10;
                    lhs *= 10;
                }
                lhs + rhs
            }
        }
    }
}

struct Calibration {
    res: i64,
    operands: Vec<i64>,
}

impl Calibration {
    fn is_valid(&self) -> bool {
        let len = self.operands.len() - 2;
        let operators_base = [Operator::Add, Operator::Multiply, Operator::Concat];
        let mut operators: Vec<_> = operators_base.iter().map(|op| vec![op.clone()]).collect();

        // Get a permutation of operators
        for _ in 0..len {
            operators = operators_base
                .iter()
                .flat_map(|base| {
                    operators
                        .iter()
                        .map(|op| {
                            let mut op = op.clone();
                            op.push(base.clone());
                            op
                        })
                        .collect::<Vec<_>>()
                })
                .collect();
        }

        operators.iter().any(|ops| {
            let res = self
                .operands
                .iter()
                .skip(1)
                .zip(ops)
                .fold(self.operands[0], |lhs, (rhs, op)| op.calc(lhs, *rhs));
            self.res == res
        })
    }
}

impl From<&str> for Calibration {
    fn from(value: &str) -> Self {
        let i = value.find(':').unwrap();
        let (res, operands) = value.split_at(i);
        let res = res.parse::<i64>().unwrap();
        let operands = operands
            .split_whitespace()
            .skip(1)
            .map(|i| i.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        Calibration { res, operands }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_calibration() {
        let input = "3267: 81 40 27";
        let res = 3267;
        let operands = [81, 40, 27];

        let calibration = Calibration::from(input);
        calibration.is_valid();
        assert_eq!(calibration.res, res);
        assert_eq!(calibration.operands, operands);
    }

    #[test]
    fn test_is_valid() {
        let input = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"#;
        let expected = [true, true, false, true, true, false, true, false, true];
        for (i, calibration) in input.lines().map(Calibration::from).enumerate() {
            assert_eq!(calibration.is_valid(), expected[i], "Failed on {i}");
        }
    }

    #[test]
    fn test_process() {
        let input = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"#;
        let expected = 11387;
        let calibrations: Vec<_> = input.lines().map(Calibration::from).collect();
        assert_eq!(process(&calibrations), expected);
    }
}
