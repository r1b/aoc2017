use std::iter::{Iterator, Peekable};

struct Captcha<'a, I> where I: Iterator<Item=char> + 'a {
    cur: Option<char>,
    first: char,
    input: &'a mut Peekable<I>,
}

impl<'a, I> Captcha<'a, I> where I: Iterator<Item=char> + 'a {
    fn new(input: &'a mut Peekable<I>, first: char) -> Captcha<'a, I> {
        Captcha {
            cur: None,
            first: first,
            input: input,
        }
    }

    fn digit_value(&mut self, cur: char) -> u32 {
        let tabulate = |a: &char, b: &char| {
            if a == b { a.to_digit(10).unwrap() } else { 0 }
        };

        match self.input.peek() {
            None => tabulate(&cur, &self.first),
            Some(next) => tabulate(&cur, &next)
        }
    }
}

impl<'a, I> Iterator for Captcha<'a, I> where I: Iterator<Item=char> + 'a {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        self.cur = self.input.next();

        match self.cur {
            None        => None,
            Some(cur)   => Some(self.digit_value(cur))
        }
    }
}

pub fn solve_captcha(input: &'static str) -> u32 {
    // Make it right: Generic for any lookahead (works for both star 1 & 2)
    // Make it right: Better public interface
    // Make it fast: Is this possible with a single iterator?
    let mut iter = input.chars().peekable();
    let first = input.chars().nth(0).unwrap();
    if input.len() < 2 { 0 } else { Captcha::new(&mut iter, first).sum() }
}
