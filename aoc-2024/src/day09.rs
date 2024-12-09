use std::cell::RefCell;

fn main() {
    let mut args = std::env::args();
    let path = args.nth(1).unwrap();
    let input = std::fs::read_to_string(path).unwrap();
    let disk = Disk::from(input.as_str());
    disk.defrag();
    println!("Checksum: {}", disk.checksum());

    let mut disk = Disk::from(input.as_str());
    disk.defrag_files();
    println!("Checksum files: {}", disk.checksum());
}

#[derive(Debug)]
struct Disk {
    layout: RefCell<Vec<Option<usize>>>,
    free: Vec<(usize, usize)>,
}

impl Disk {
    fn chunk(x: &Option<usize>, y: &Option<usize>) -> bool {
        match (x, y) {
            (Some(x), Some(y)) if x == y => true,
            (None, None) => true,
            _ => false,
        }
    }

    fn defrag(&self) {
        let mut idx = 0;
        let pairs = self
            .layout
            .borrow()
            .iter()
            .enumerate()
            .filter(|(_, x)| x.is_none())
            .filter_map(|(i, _)| {
                let (j, _) = self
                    .layout
                    .borrow()
                    .iter()
                    .enumerate()
                    .skip(i + 1)
                    .rev()
                    .filter(|(_, y)| y.is_some())
                    .nth(idx)?;
                idx += 1;
                Some((i, j))
            })
            .collect::<Vec<_>>();

        for (i, j) in pairs {
            self.layout.borrow_mut().swap(i, j);
        }
    }

    fn defrag_files(&mut self) {
        let mut i = 0;
        let files = self
            .layout
            .borrow()
            .chunk_by(Disk::chunk)
            .filter_map(|v| {
                let start = i;
                i += v.len();
                if v.first().unwrap().is_some() {
                    Some((start, v.len()))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        for (start, len) in files.iter().rev() {
            for free in self.free.iter_mut() {
                if free.1 >= *len && free.0 < *start {
                    for i in 0..*len {
                        self.layout.borrow_mut().swap(free.0 + i, start + i);
                    }
                    free.0 += len;
                    free.1 -= len;
                    break;
                }
            }
        }
    }

    fn checksum(&self) -> usize {
        self.layout
            .borrow()
            .iter()
            .enumerate()
            .filter(|(_, x)| x.is_some())
            .fold(0, |acc, (i, x)| acc + i * x.unwrap())
    }
}

impl From<&str> for Disk {
    fn from(value: &str) -> Self {
        let mut free = Vec::new();

        let layout = value
            .trim()
            .chars()
            .enumerate()
            .flat_map(|(i, x)| {
                if i % 2 == 0 {
                    // File
                    std::iter::repeat(Some(i / 2))
                } else {
                    // Empty space
                    std::iter::repeat(None)
                }
                .take(x.to_digit(10).unwrap() as usize)
            })
            .collect::<Vec<_>>();

        let mut i = 0;
        for s in layout.chunk_by(Disk::chunk) {
            let len = s.len();
            let start = i;
            i += len;
            if s[0].is_none() {
                free.push((start, len));
            }
        }

        Disk {
            layout: RefCell::new(layout),
            free,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checksum() {
        let input = "2333133121414131402";
        let expected = 1928;
        let disk = Disk::from(input);
        disk.defrag();
        assert_eq!(disk.checksum(), expected);
    }

    #[test]
    fn test_checksum_defrag_files() {
        let input = "2333133121414131402";
        let expected = 2858;
        let mut disk = Disk::from(input);
        disk.defrag_files();
        assert_eq!(disk.checksum(), expected);
    }
}
