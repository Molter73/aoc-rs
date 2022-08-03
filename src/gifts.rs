use std::cmp;
use std::fmt::Display;
use std::num::ParseIntError;
use std::str::FromStr;

pub struct Gift {
    l: usize,
    w: usize,
    h: usize,
}

impl Gift {
    pub fn new(l: usize, w: usize, h: usize) -> Gift {
        Gift { l, w, h }
    }

    pub fn get_area(&self) -> usize {
        let base = self.l * self.w;
        let front = self.l * self.h;
        let side = self.w * self.h;

        2 * (base + front + side)
    }

    pub fn get_volume(&self) -> usize {
        self.l * self.w * self.h
    }

    pub fn get_smallest_side(&self) -> usize {
        let base = self.l * self.w;
        let front = self.l * self.h;
        let side = self.w * self.h;

        cmp::min(base, cmp::min(front, side))
    }

    pub fn get_wrap_around(&self) -> usize {
        let around: Vec<usize> = vec![
            (2 * self.l) + (2 * self.w),
            (2 * self.l) + (2 * self.h),
            (2 * self.h) + (2 * self.w),
        ];

        let min_dist = around.iter().min();

        match min_dist {
            Some(d) => *d,
            None => 0,
        }
    }
}

impl FromStr for Gift {
    type Err = GiftError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pieces: Vec<&str> = s.split('x').collect();

        if pieces.len() != 3 {
            return Err(GiftError::SizeLengthError);
        }

        let l = pieces[0].parse::<usize>()?;
        let w = pieces[1].parse::<usize>()?;
        let h = pieces[2].parse::<usize>()?;

        Ok(Gift::new(l, w, h))
    }
}

#[derive(Debug)]
pub enum GiftError {
    ParseError(ParseIntError),
    SizeLengthError,
}

impl Display for GiftError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GiftError::SizeLengthError => write!(f, "invalid size"),
            GiftError::ParseError(parse_int_error) => write!(f, "{}", parse_int_error),
        }
    }
}

impl From<ParseIntError> for GiftError {
    fn from(e: ParseIntError) -> Self {
        GiftError::ParseError(e)
    }
}
