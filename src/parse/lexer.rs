use regex::Regex;
use parse::token::Token;

pub struct Lexer {
    input: String,
    pos: usize,
    match_space: Regex,
    match_word: Regex
}

enum Char {
    Word,
    Var,
    Space,
    BlockOpen,
    BlockClose,
    String,
    BracketOpen,
    BracketClose,
    Unknown
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        Lexer {
            input: input,
            pos: 0usize,
            match_space: Regex::new(r"\s+").unwrap(),
            match_word: Regex::new("[^\\s\\}\\]\"]+").unwrap()
        }
    }

    fn peek(&self) -> Option<Char> {
        if self.pos >= self.input.len() {
            return None
        }
        let s = &self.input[self.pos..(self.pos+1)];
        match s {
            "$" => Some(Char::Var),
            "[" => Some(Char::BracketOpen),
            "]" => Some(Char::BracketClose),
            "{" => Some(Char::BlockOpen),
            "}" => Some(Char::BlockClose),
            "\"" => Some(Char::String),
            _ => {
                if self.match_space.is_match(s) {
                    return Some(Char::Space)
                } else if self.match_word.is_match(s) {
                    return Some(Char::Word)
                } else {
                    println!("Unknown token '{}'", s);
                    return Some(Char::Unknown)
                }
            }
        }
    }

    fn end_of_word(&self) -> usize {
        match self.match_word.find_at(&*self.input, self.pos) {
            Some((_, e)) => e,
            _ => self.pos
        }
    }

    fn end_of_space(&self) -> usize {
        match self.match_space.find_at(&*self.input, self.pos) {
            Some((_, e)) => e,
            _ => self.pos
        }
    }

    fn eat_until(&mut self, pos: usize) -> String {
        let p = if pos > self.input.len() { self.input.len() } else { pos };
        let res = &self.input[self.pos..p];
        self.pos = p;
        res.to_string()
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.peek() {
            Some(Char::Var) => {
                self.pos += 1;
                let p = self.end_of_word();
                Some(Token::Var{ name: self.eat_until(p) })
            },
            Some(Char::Word) => {
                let p = self.end_of_word();
                Some(Token::Word { contents: self.eat_until(p) })
            },
            Some(Char::Space) => {
                let p = self.end_of_space();
                Some(Token::Whitespace { contents: self.eat_until(p) })
            },
            Some(Char::String) => {
                self.pos += 1;
                Some(Token::String)
            },
            Some(Char::BracketOpen) => {
                self.pos += 1;
                Some(Token::BracketOpen)
            },
            Some(Char::BracketClose) => {
                self.pos += 1;
                Some(Token::BracketClose)
            },
            Some(Char::BlockOpen) => {
                self.pos += 1;
                Some(Token::BlockOpen)
            },
            Some(Char::BlockClose) => {
                self.pos += 1;
                Some(Token::BlockClose)
            },
            Some(Char::Unknown) => None,
            _ => None
        }
    }
}