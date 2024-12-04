use std::{iter::Peekable, str::Chars};

fn main() {
    let mut args = std::env::args();
    let path = args.nth(1).unwrap();
    let input = std::fs::read_to_string(path).unwrap();
    let parser = Parser::new(&input);
    println!("Result: {}", parser.sum::<i64>());
}

#[derive(Debug, PartialEq)]
enum Token {
    Int(i64),

    Mul,
    Do,
    Dont,

    Comma,
    LParen,
    RParen,

    Invalid,
}

struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    fn new(input: Chars<'a>) -> Self {
        Lexer {
            input: input.peekable(),
        }
    }
}

impl<'a> From<&'a str> for Lexer<'a> {
    fn from(value: &'a str) -> Self {
        Lexer::new(value.chars())
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.input.next();
        match c {
            Some(c) => match c {
                '(' => Some(Token::LParen),
                ')' => Some(Token::RParen),
                ',' => Some(Token::Comma),
                d if d.is_ascii_digit() => {
                    let mut num = String::from(d);
                    while let Some(c) = self.input.next_if(|c| c.is_ascii_digit()) {
                        num.push(c);
                    }
                    let num = num.parse::<i64>().unwrap();
                    Some(Token::Int(num))
                }
                'd' => {
                    if self.input.next_if(|c| *c == 'o').is_none() {
                        return Some(Token::Invalid);
                    }

                    if self.input.next_if(|c| *c == 'n').is_none() {
                        return Some(Token::Do);
                    }

                    if self.input.next_if(|c| *c == '\'').is_none() {
                        return Some(Token::Invalid);
                    }

                    if self.input.next_if(|c| *c == 't').is_none() {
                        return Some(Token::Invalid);
                    }

                    Some(Token::Dont)
                }
                'm' => {
                    if self.input.next_if(|c| *c == 'u').is_none() {
                        return Some(Token::Invalid);
                    }

                    if self.input.next_if(|c| *c == 'l').is_none() {
                        return Some(Token::Invalid);
                    }

                    Some(Token::Mul)
                }
                _ => Some(Token::Invalid),
            },
            None => None,
        }
    }
}

type Params = Vec<Token>;

struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
    enabled: bool,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        let lexer = Lexer::from(input).peekable();
        Parser {
            lexer,
            enabled: true,
        }
    }

    fn parse_call(&mut self) -> Option<Params> {
        self.lexer.next_if(|t| t == &Token::LParen)?;

        let mut params = Vec::new();
        while let Some(token) = self.lexer.peek() {
            match token {
                Token::Int(i) => params.push(Token::Int(*i)),
                Token::RParen => break,
                _ => return None,
            }
            self.lexer.next();

            match self.lexer.peek() {
                Some(Token::RParen) => break,
                Some(Token::Comma) => {}
                _ => return None,
            }
            self.lexer.next();
        }

        self.lexer.next();
        Some(params)
    }

    fn parse_mul(&mut self) -> Option<i64> {
        let params = self.parse_call();
        let Some([Token::Int(lhs), Token::Int(rhs)]) = params.as_deref() else {
            return None;
        };
        Some(lhs * rhs)
    }

    fn set_enabled(&mut self, value: bool) {
        if let Some(v) = self.parse_call() {
            if v.is_empty() {
                self.enabled = value;
            }
        }
    }

    fn parse_do(&mut self) {
        self.set_enabled(true);
    }

    fn parse_dont(&mut self) {
        self.set_enabled(false);
    }
}

impl Iterator for Parser<'_> {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(token) = self.lexer.next() {
            match token {
                Token::Mul => {
                    if !self.enabled {
                        continue;
                    }

                    if let Some(i) = self.parse_mul() {
                        return Some(i);
                    }
                }
                Token::Do => self.parse_do(),
                Token::Dont => self.parse_dont(),
                _ => {}
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let input = "qmul(1234,4321)docsdon't";
        let mut lexer = Lexer::from(input);
        assert_eq!(lexer.next(), Some(Token::Invalid));
        assert_eq!(lexer.next(), Some(Token::Mul));
        assert_eq!(lexer.next(), Some(Token::LParen));
        assert_eq!(lexer.next(), Some(Token::Int(1234)));
        assert_eq!(lexer.next(), Some(Token::Comma));
        assert_eq!(lexer.next(), Some(Token::Int(4321)));
        assert_eq!(lexer.next(), Some(Token::RParen));
        assert_eq!(lexer.next(), Some(Token::Do));
        assert_eq!(lexer.next(), Some(Token::Invalid));
        assert_eq!(lexer.next(), Some(Token::Invalid));
        assert_eq!(lexer.next(), Some(Token::Dont));
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_parser() {
        let tests = [
            (
                "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
                vec![8, 25, 88, 40],
            ),
            (
                "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
                vec![8, 40],
            ),
        ];

        for (input, expected) in tests {
            let parser = Parser::new(input);
            for (i, value) in parser.enumerate() {
                assert_eq!(value, expected[i]);
            }
        }
    }
}
