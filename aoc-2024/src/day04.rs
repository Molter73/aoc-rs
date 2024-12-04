fn main() {
    let mut args = std::env::args();
    let path = args.nth(1).unwrap();
    let input = std::fs::read_to_string(path).unwrap();
    let soup = LetterSoup::new(input.as_str(), "XMAS");
    println!("XMAS occurrences: {}", soup.count_word());
    let soup = LetterSoup::new(input.as_str(), "MAS");
    println!("X-MAS occurrences: {}", soup.count_x());
}

#[derive(Debug, Clone, PartialEq)]
struct Direction(i64, i64);

impl From<Position> for Direction {
    fn from(value: Position) -> Self {
        Direction(value.0, value.1)
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Position(i64, i64);

impl Position {
    fn distance(&self, other: &Position) -> usize {
        let x = self.0.abs_diff(other.0) as usize;
        let y = self.1.abs_diff(other.1) as usize;
        x * x + y * y
    }

    fn diff(&self, other: &Position) -> Direction {
        let x = self.0 - other.0;
        let y = self.1 - other.1;
        Direction(x, y)
    }

    fn mov(&self, dir: &Direction) -> Position {
        let x = self.0 + dir.0;
        let y = self.1 + dir.1;
        Position(x, y)
    }
}

impl From<(i64, i64)> for Position {
    fn from(value: (i64, i64)) -> Self {
        let (x, y) = value;
        Position(x, y)
    }
}

struct LetterSoup {
    letters: Vec<Vec<u8>>,
    positions: Vec<Vec<Position>>,
}

impl LetterSoup {
    fn new(letters: &str, word: &str) -> Self {
        let letters: Vec<Vec<_>> = letters.lines().map(|line| line.bytes().collect()).collect();
        let mut positions = Vec::new();
        for needle in word.bytes() {
            let mut letter_position: Vec<Position> = Vec::new();
            for (i, line) in letters.iter().enumerate() {
                for (j, c) in line.iter().enumerate() {
                    if *c == needle {
                        letter_position.push((i as i64, j as i64).into());
                    }
                }
            }
            positions.push(letter_position);
        }

        LetterSoup { letters, positions }
    }

    fn count_word_inner(
        curr: &Position,
        positions: &[Vec<Position>],
        direction: Option<&Direction>,
    ) -> usize {
        match positions.split_first() {
            Some((first, tail)) => first
                .iter()
                .filter(|p| match &direction {
                    Some(dir) => **p == curr.mov(dir),
                    None => p.distance(curr) <= 2,
                })
                .map(|p| {
                    let dir = match &direction {
                        Some(dir) => dir,
                        None => &p.diff(curr),
                    };
                    LetterSoup::count_word_inner(p, tail, Some(dir))
                })
                .sum(),
            None => 1,
        }
    }

    fn count_word(&self) -> usize {
        match self.positions.split_first() {
            Some((first, tail)) => first
                .iter()
                .map(|p| LetterSoup::count_word_inner(p, tail, None))
                .sum(),
            None => 0,
        }
    }

    fn count_x(&self) -> usize {
        let len = (self.positions.len() - 1) as i64;
        let directions = [
            Direction(1, 1),
            Direction(-1, 1),
            Direction(1, -1),
            Direction(-1, -1),
        ];

        match self.positions.split_first() {
            Some((first, tail)) => {
                first
                    .iter()
                    .map(|curr| {
                        let mut count = 0;
                        for dir in &directions {
                            if LetterSoup::count_word_inner(curr, tail, Some(dir)) != 0 {
                                // Try moving on X
                                let xdir = Direction(-dir.0, dir.1);
                                let xpos = curr.0 + if xdir.0 < 0 { len } else { -len };
                                let xcurr = Position(xpos, curr.1);
                                let new_curr_found = self
                                    .positions
                                    .first()
                                    .unwrap()
                                    .iter()
                                    .find(|e| **e == xcurr)
                                    .is_some();
                                if new_curr_found
                                    && LetterSoup::count_word_inner(&xcurr, tail, Some(&xdir)) != 0
                                {
                                    count += 1;
                                }

                                // Try moving on Y
                                let ydir = Direction(dir.0, -dir.1);
                                let ypos = curr.1 + if ydir.1 < 0 { len } else { -len };
                                let ycurr = Position(curr.0, ypos);
                                let new_curr_found = self
                                    .positions
                                    .first()
                                    .unwrap()
                                    .iter()
                                    .find(|e| **e == ycurr)
                                    .is_some();
                                if new_curr_found
                                    && LetterSoup::count_word_inner(&ycurr, tail, Some(&ydir)) != 0
                                {
                                    count += 1;
                                }
                            }
                        }
                        count
                    })
                    .sum::<usize>()
                    / 2
            }
            None => 0,
        }
    }

    fn print_word(&self, first: &Position, dir: &Direction) -> Option<String> {
        let mut word = Vec::new();
        let mut pos = first.clone();
        for i in 0..self.positions.len() {
            let Some(row) = self.letters.get(pos.0 as usize) else {
                println!("Invalid entry 1: ({i}) {first:?} - {dir:?}");
                return None;
            };
            if pos.1 as usize >= row.len() {
                println!("Invalid entry 2: ({i}) {first:?} - {dir:?}");
                return None;
            }
            let letter = row[pos.1 as usize];
            word.push(letter);
            pos = pos.mov(dir);
        }
        let word = String::from_utf8(word).unwrap();
        println!("word: {word} - {first:?} - {dir:?}");
        Some(word)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_count() {
        let input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;
        let soup = LetterSoup::new(input, "XMAS");
        assert_eq!(soup.count_word(), 18);
    }

    #[test]
    fn test_count_x() {
        let input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;
        let soup = LetterSoup::new(input, "MAS");
        assert_eq!(soup.count_x(), 9);
    }

    #[test]
    fn test_count_x_bug() {
        let input = r#"M.S.......
.A........
S.S.......
..........
..........
..........
..........
..........
..........
.........."#;
        let soup = LetterSoup::new(input, "MAS");
        assert_eq!(soup.count_x(), 0);
    }
}
