use std::collections::HashMap;

fn main() {
    let mut args = std::env::args();
    let path = args.nth(1).unwrap();
    let input = std::fs::read_to_string(path).unwrap();
    let map = TopographyMap::from(input.as_str());
    let trails = map.find_trails();
    println!("Number of trails: {}", trails.len());
    println!("Total rating of trails: {}", trails.values().sum::<u64>());
}

static DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Right,
    Direction::Down,
    Direction::Left,
];

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Position(i64, i64);

impl Position {
    fn add(&self, Position(x, y): &Position) -> Position {
        Position(self.0 + x, self.1 + y)
    }

    fn step(&self, dir: &Direction) -> Position {
        self.add(dir.into())
    }
}

impl From<&Direction> for &Position {
    fn from(value: &Direction) -> Self {
        match value {
            Direction::Up => &Position(-1, 0),
            Direction::Right => &Position(0, 1),
            Direction::Down => &Position(1, 0),
            Direction::Left => &Position(0, -1),
        }
    }
}

impl From<(usize, usize)> for Position {
    fn from((x, y): (usize, usize)) -> Self {
        Position(x as i64, y as i64)
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
struct TopographyMap {
    tiles: Vec<Vec<u8>>,
    starts: Vec<Position>,
}

impl TopographyMap {
    fn find_trails_inner(&self, pos: &Position, next: u8) -> Vec<Position> {
        if next > 9 {
            return vec![pos.clone()];
        }

        DIRECTIONS
            .iter()
            .filter(|dir| {
                let dir = *dir;
                let pos = pos.step(dir);
                self.in_map(&pos) && self.tiles[pos.0 as usize][pos.1 as usize] == next
            })
            .flat_map(|dir| {
                let pos = pos.step(dir);
                self.find_trails_inner(&pos, next + 1)
            })
            .collect()
    }

    fn find_trails(&self) -> HashMap<(Position, Position), u64> {
        let mut map = HashMap::new();
        for start in &self.starts {
            let ends = self.find_trails_inner(start, 1);
            for end in ends {
                let k = (start.clone(), end);
                *map.entry(k).or_default() += 1;
            }
        }
        map
    }

    fn in_map(&self, Position(x, y): &Position) -> bool {
        *x >= 0
            && *y >= 0
            && *x < self.tiles.len() as i64
            && *y < self.tiles.first().unwrap().len() as i64
    }
}

impl From<&str> for TopographyMap {
    fn from(value: &str) -> Self {
        let tiles = value
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '0'..='9' => c.to_digit(10).unwrap() as u8,
                        _ => u8::MAX,
                    })
                    .collect()
            })
            .collect();

        let starts = value
            .lines()
            .enumerate()
            .flat_map(|(i, line)| {
                line.chars().enumerate().filter_map(move |(j, c)| {
                    if c == '0' {
                        Some((i, j).into())
                    } else {
                        None
                    }
                })
            })
            .collect();

        TopographyMap { tiles, starts }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_trails() {
        let tests = [
            (
                r#"0123
1234
8765
9876"#,
                1,
            ),
            (
                r#"...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9"#,
                2,
            ),
            (
                r#"..90..9
...1.98
...2..7
6543456
765.987
876....
987...."#,
                4,
            ),
            (
                r#"10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01"#,
                3,
            ),
            (
                r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#,
                36,
            ),
        ];

        for (input, expected) in tests {
            let map = TopographyMap::from(input);
            println!("{map:?}");
            assert_eq!(map.find_trails().len(), expected);
        }
    }

    #[test]
    fn test_trails_by_rating() {
        let tests = [
            (
                r#".....0.
..4321.
..5..2.
..6543.
..7..4.
..8765.
..9...."#,
                3,
            ),
            (
                r#"..90..9
...1.98
...2..7
6543456
765.987
876....
987...."#,
                13,
            ),
            (
                r#"012345
123456
234567
345678
4.6789
56789."#,
                227,
            ),
        ];
        for (input, expected) in tests {
            let map = TopographyMap::from(input);
            let rating: u64 = map
                .find_trails()
                .values()
                .inspect(|v| println!("{v:?}"))
                .sum();
            assert_eq!(rating, expected);
        }
    }
}
