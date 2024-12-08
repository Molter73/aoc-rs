use std::collections::{HashMap, HashSet};

fn main() {
    let mut args = std::env::args();
    let path = args.nth(1).unwrap();
    let input = std::fs::read_to_string(path).unwrap();
    let antennas = AntennaMap::from(input.as_str());
    println!("Antinodes: {}", antennas.get_antinodes().len());
    println!(
        "Resonant Antinodes: {}",
        antennas.get_antinodes_resonant().len()
    );
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Position(i64, i64);

impl Position {
    fn diff(&self, other: &Position) -> Position {
        Position(other.0 - self.0, other.1 - self.1)
    }

    fn add(&self, other: &Position) -> Position {
        Position(self.0 + other.0, self.1 + other.1)
    }

    fn comp(&self) -> Position {
        Position(-self.0, -self.1)
    }
}

impl From<(usize, usize)> for Position {
    fn from(value: (usize, usize)) -> Self {
        let (x, y) = value;
        Position(x as i64, y as i64)
    }
}

#[derive(Debug)]
struct AntennaMap {
    antennas: HashMap<char, Vec<Position>>,
    x: usize,
    y: usize,
}

impl AntennaMap {
    fn get_antinodes(&self) -> HashSet<Position> {
        self.antennas
            .iter()
            .flat_map(|(_, pos)| {
                pos.iter().enumerate().flat_map(|(i, p1)| {
                    pos.iter()
                        .skip(i + 1)
                        .flat_map(|p2| {
                            let diff = p1.diff(p2);
                            let comp = diff.comp();
                            [p2.add(&diff), p1.add(&comp)]
                        })
                        .filter(|p| self.position_is_valid(p))
                })
            })
            .collect()
    }

    fn pos_vec_to_antinodes(&self, p: &mut Position, vec: &Position) -> HashSet<Position> {
        let mut set = HashSet::new();
        *p = p.add(vec);
        while self.position_is_valid(p) {
            set.insert(p.clone());
            *p = p.add(vec);
        }
        set
    }

    fn get_antinodes_resonant(&self) -> HashSet<Position> {
        self.antennas
            .iter()
            .flat_map(|(_, pos)| {
                pos.iter().enumerate().flat_map(|(i, p1)| {
                    pos.iter().skip(i + 1).flat_map(|p2| {
                        let diff = p1.diff(p2);
                        let comp = diff.comp();

                        let mut p = p1.clone();
                        let mut set = self.pos_vec_to_antinodes(&mut p, &diff);

                        p = p2.clone();
                        set.extend(self.pos_vec_to_antinodes(&mut p, &comp));

                        set
                    })
                })
            })
            .collect()
    }

    fn position_is_valid(&self, Position(x, y): &Position) -> bool {
        *x >= 0 && *y >= 0 && (*x as usize) < self.x && (*y as usize) < self.y
    }
}

impl From<&str> for AntennaMap {
    fn from(value: &str) -> Self {
        let mut antennas: HashMap<char, Vec<_>> = HashMap::new();
        for (c, pos) in value.lines().enumerate().flat_map(|(i, line)| {
            line.chars().enumerate().filter_map(move |(j, c)| match c {
                'A'..='Z' | 'a'..='z' | '0'..='9' => Some((c, Position::from((i, j)))),
                _ => None,
            })
        }) {
            antennas.entry(c).or_default().push(pos);
        }

        let x = value.lines().count();
        let y = value.lines().last().unwrap().chars().count();

        AntennaMap { antennas, x, y }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_antinodes() {
        let tests = [
            (
                r#"..........
..........
..........
....a.....
..........
.....a....
..........
..........
..........
.........."#,
                2,
            ),
            (
                r#"..........
..........
..........
....a.....
........a.
.....a....
..........
..........
..........
.........."#,
                4,
            ),
            (
                r#"..........
..........
..........
....a.....
........a.
.....a....
..........
......A...
..........
.........."#,
                4,
            ),
            (
                r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#,
                14,
            ),
        ];
        for (input, expected) in tests {
            let antennas = AntennaMap::from(input);
            let antinodes = antennas.get_antinodes();
            assert_eq!(antinodes.len(), expected);
        }
    }

    #[test]
    fn test_get_antinodes_resonant() {
        let tests = [
            (
                r#"T....#....
...T......
.T....#...
.........#
..#.......
..........
...#......
..........
....#.....
.........."#,
                9,
            ),
            (
                r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#,
                34,
            ),
        ];
        for (input, expected) in tests {
            let antennas = AntennaMap::from(input);
            let antinodes = antennas.get_antinodes_resonant();
            println!("{antinodes:?}");
            assert_eq!(antinodes.len(), expected);
        }
    }
}
