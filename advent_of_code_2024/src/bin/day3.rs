use std::fs;

const MUL: &'static str = "mul";
const DO: &'static str = "do()";
const DONT: &'static str = "don't()";
const OPENING_PAREN: &'static str = "(";
const CLOSING_PAREN: &'static str = ")";
const COMMA: &'static str = ",";

#[derive(Debug)]
struct Parser<'a> {
    input: &'a String,
    position: usize,
}

impl<'a> Parser<'a> {
    fn new(input: &String) -> Parser {
        Parser {
            input: input,
            position: 0,
        }
    }

    fn match_slice(&mut self, slice: &str) -> bool {
        let start: usize = self.position;
        let end: usize = start + slice.len();
        if end > self.input.len() {
            return false;
        }
        // The following line is likely not preferred due to https://doc.rust-lang.org/book/ch08-02-strings.html#indexing-into-strings
        if &self.input[start..end] == slice {
            self.position = end;
            true
        } else {
            false
        }
    }

    fn parse_number(&mut self) -> Option<isize> {
        let start = self.position;
        while self.position < self.input.len() && self.input.chars().nth(self.position).unwrap().is_digit(10) {
            self.position += 1;
        }
        self.input[start..self.position].parse::<isize>().ok()
    }

    fn parse(&mut self) -> isize {
        let mut result: isize = 0;
        let mut mul_enabled = true;
        while self.position < self.input.len() {
            if self.match_slice(DO) {
                mul_enabled = true;
                continue;
            }
            if self.match_slice(DONT) {
                mul_enabled = false;
                continue;
            }
            if !self.match_slice(MUL) {
                self.position += 1;
                continue;
            }
            if !self.match_slice(OPENING_PAREN) {
                self.position += 1;
                continue;
            }
            let x = match self.parse_number() {
                Some(num) => num,
                None => {
                    self.position += 1;
                    continue;
                }
            };
            if !self.match_slice(COMMA) {
                self.position += 1;
                continue;
            }
            let y = match self.parse_number() {
                Some(num) => num,
                None => {
                    self.position += 1;
                    continue;
                }
            };
            if !self.match_slice(CLOSING_PAREN) {
                self.position += 1;
                continue;
            }
            if mul_enabled {
                result += x * y;
            }
        }
        result
    }
}

fn main() {
    let input = fs::read_to_string("input_data/day3.txt").unwrap();
    let mut parser = Parser::new(&input);
    let result = parser.parse();
    println!("Result: {}", result);
}
