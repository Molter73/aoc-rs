fn main() {
    let mut args = std::env::args();
    let path = args.nth(1).unwrap();
    let input = std::fs::read_to_string(path).unwrap();
    let mut lab = Lab::from(input.as_str());
    println!("Walked on: {}", lab.simulate().unwrap());

    // Reload the lab before running the loops test
    let lab = Lab::from(input.as_str());
    println!("Loops: {}", lab.simulate_loops());
}

#[derive(Debug, Clone)]
enum Tile {
    Empty,
    Obstacle,
    WalkedOn(Direction),
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        if value == '#' {
            Tile::Obstacle
        } else {
            Tile::Empty
        }
    }
}

#[derive(Debug, Clone)]
struct Position(i64, i64);

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone)]
struct Guard {
    position: Position,
    direction: Direction,
}

impl Guard {
    fn new(x: i64, y: i64, direction: char) -> Self {
        let direction = match direction {
            '^' => Direction::Up,
            '>' => Direction::Right,
            'v' => Direction::Down,
            '<' => Direction::Left,
            _ => unreachable!(),
        };
        let position = Position(x, y);

        Guard {
            position,
            direction,
        }
    }

    fn step(&mut self, floor: &[Vec<Tile>]) {
        let Position(x, y) = self.position;
        let (x, y) = match self.direction {
            Direction::Up => (x - 1, y),
            Direction::Right => (x, y + 1),
            Direction::Down => (x + 1, y),
            Direction::Left => (x, y - 1),
        };

        if Lab::guard_left(x, y, floor) {
            self.position = Position(x, y);
            return;
        }

        match floor[x as usize][y as usize] {
            Tile::Obstacle => self.rotate(),
            _ => self.position = Position(x, y),
        }
    }

    fn rotate(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

impl TryFrom<&str> for Guard {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        for (i, line) in value.lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                if matches!(c, '^' | '<' | '>' | 'v') {
                    return Ok(Guard::new(i as i64, j as i64, c));
                }
            }
        }
        Err(String::from("No guard in lab!"))
    }
}

#[derive(Debug, Clone)]
struct Lab {
    guard: Guard,
    floor: Vec<Vec<Tile>>,
}

impl Lab {
    fn simulate(&mut self) -> Option<usize> {
        loop {
            let Position(x, y) = self.guard.position.clone();
            if Lab::guard_left(x, y, &self.floor) {
                break;
            }

            if matches!(self.floor[x as usize][y as usize], Tile::Empty) {
                self.floor[x as usize][y as usize] = Tile::WalkedOn(self.guard.direction.clone());
            } else if let Tile::WalkedOn(direction) = &self.floor[x as usize][y as usize] {
                if *direction == self.guard.direction {
                    // Guard just entered a loop
                    return None;
                }
            }

            self.guard.step(&self.floor);
        }
        let count = self
            .floor
            .iter()
            .flat_map(|line| line.iter().filter(|tile| matches!(tile, Tile::WalkedOn(_))))
            .count();
        Some(count)
    }

    fn simulate_loops(&self) -> usize {
        let Position(x, y) = self.guard.position.clone();
        self.floor
            .iter()
            .enumerate()
            .flat_map(|(i, line)| {
                line.iter().enumerate().filter(move |(j, tile)| {
                    let mut lab = self.clone();
                    if matches!(tile, Tile::Obstacle) || (i == x as usize && *j == y as usize) {
                        return false;
                    }
                    lab.floor[i][*j] = Tile::Obstacle;
                    lab.simulate().is_none()
                })
            })
            .count()
    }

    fn guard_left(x: i64, y: i64, floor: &[Vec<Tile>]) -> bool {
        x < 0 || y < 0 || x >= floor.len() as i64 || y >= floor[0].len() as i64
    }
}

impl From<&str> for Lab {
    fn from(value: &str) -> Self {
        let floor: Vec<Vec<_>> = value
            .lines()
            .map(|line| line.chars().map(Tile::from).collect())
            .collect();
        let guard = Guard::try_from(value).unwrap();

        Lab { guard, floor }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulate() {
        let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"#;
        let mut lab = Lab::from(input);
        assert_eq!(lab.simulate(), Some(41));
    }

    #[test]
    fn test_simulate_loops() {
        let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"#;
        let lab = Lab::from(input);
        assert_eq!(lab.simulate_loops(), 6);
    }
}
