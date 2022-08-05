enum Instruction {
    Turn(bool, Range),
    Toggle(Range),
}

struct Range {
    x: (usize, usize),
    y: (usize, usize),
}

impl Range {
    fn new(lower: &str, thr: &str, upper: &str) -> Result<Range, GridError> {
        if thr != "through" {
            return Err(GridError::InvalidInstruction);
        }

        let lower = Range::parse_bound(lower)?;
        let upper = Range::parse_bound(upper)?;

        Ok(Range {
            x: (lower.0, upper.0),
            y: (lower.1, upper.1),
        })
    }

    fn parse_bound(input: &str) -> Result<(usize, usize), GridError> {
        let input: Vec<&str> = input.split(',').collect();
        match input[..] {
            [x, y] => Ok((x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())),
            _ => Err(GridError::InvalidInstruction),
        }
    }
}

#[derive(Debug)]
enum GridError {
    InvalidInstruction,
    SizeLengthError,
}

pub struct Grid {
    lights: Vec<Vec<bool>>,
}

impl Grid {
    pub fn new(x: usize, y: usize) -> Grid {
        let lights = vec![vec![false; x]; y];

        Grid { lights }
    }

    fn generate_turn(input: Vec<&str>) -> Result<Instruction, GridError> {
        if input.len() != 5 {
            return Err(GridError::SizeLengthError);
        }

        match input[1] {
            "on" => Ok(Instruction::Turn(true, Range::new(input[2], input[3], input[4])?)),
            "off" => Ok(Instruction::Turn(false, Range::new(input[2], input[3], input[4])?)),
            _ => Err(GridError::InvalidInstruction),
        }
    }

    fn generate_toggle(input: Vec<&str>) -> Result<Instruction, GridError> {
        if input.len() != 4 {
            return Err(GridError::SizeLengthError);
        }

        Ok(Instruction::Toggle(Range::new(input[1], input[2], input[3])?))
    }

    fn parse(input: &str) -> Result<Instruction, GridError> {
        let input: Vec<&str> = input.split(' ').collect();

        match input[0] {
            "turn" => Grid::generate_turn(input),
            "toggle" => Grid::generate_toggle(input),
            _ => Err(GridError::InvalidInstruction),
        }
    }

    fn turn(&mut self, v: bool, r: Range) {
        for row in self.lights[r.y.0..=r.y.1].iter_mut() {
            for light in row[r.x.0..=r.x.1].iter_mut() {
                *light = v;
            }
        }
    }

    fn toggle(&mut self, r: Range) {
        for row in self.lights[r.y.0..=r.y.1].iter_mut() {
            for light in row[r.x.0..=r.x.1].iter_mut() {
                *light = !*light;
            }
        }
    }

    pub fn process(&mut self, input: &str) {
        for line in input.trim().lines() {
            let ins = Grid::parse(line).unwrap();

            match ins {
                Instruction::Turn(v, r) => self.turn(v, r),
                Instruction::Toggle(r) => self.toggle(r),
            }
        }
    }

    pub fn count(&self) -> usize {
        let mut c = 0;
        for row in &self.lights {
            for light in row {
                if *light {
                    c += 1;
                }
            }
        }
        c
    }
}
