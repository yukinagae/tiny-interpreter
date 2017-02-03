#![allow(dead_code)]
#![allow(unused_imports)]

use std::io::stdin;
use std::io::BufRead;

use TokenType::INTEGER;
use TokenType::PLUS;
use TokenType::EOF;

#[derive(Debug, Clone)]
enum TokenType {
    INTEGER,
    PLUS,
    EOF,
}

#[derive(Debug, Clone)]
struct Token {
    typ: TokenType,
    value: Option<char>
}

#[derive(Debug)]
struct Interpreter<'a> {
    text: &'a str,
    pos: usize
}

impl<'a> Interpreter<'a> {

    fn new(line: &'a str) -> Interpreter<'a> {
        Interpreter { text: &line, pos: 0 }
    }

    fn get_next_token(&mut self) -> Token {

        if self.pos > (self.text.len() - 1) {
            return Token { typ: EOF, value: None };
        } else {
            let chars: Vec<char> = self.text.chars().collect();
            let current_char = chars[self.pos];

            if current_char.is_digit(10) {
                self.pos += 1;
                return Token { typ: INTEGER, value: Some(current_char) };
            }

            if current_char == '+' {
                self.pos += 1;
                return Token { typ: PLUS, value: Some(current_char) };
            }
        }

        panic!("Error parsing input!");
    }

    fn expr(&mut self) -> u32 {
        let left = self.get_next_token().value.unwrap().to_digit(10).unwrap();
        let op = self.get_next_token();
        let right = self.get_next_token().value.unwrap().to_digit(10).unwrap();
        return left + right;
    }

}

fn tokenize(input: &str) -> &str {

    let chars: Vec<char> = input.chars().collect();

    let mut index = 0;

    while chars.len() > index {

        let mut current = chars[index];

        if current.is_whitespace() {
            index += 1;
            continue;
        } else if current.is_alphabetic() {
            print!("ID: ");
            while current.is_alphabetic() {
                print!("{}", current);
                index += 1;
                current = chars[index];
            }
            println!("");
            continue;
        } else if current.is_digit(10) {
            print!("NM: ");
            while current.is_digit(10) {
                print!("{}", current);
                index += 1;
                current = chars[index];
            }
            println!("");
            continue;
        } else {
            println!("SY: {}", current);
            index += 1;
            continue;
        }

    }

    return input;
}

fn main() {
    // // let input = "while (abc > 10) { abc = abc - 1; }";
    // // let tokenize = tokenize(&input);
    // // println!("{}", tokenize);
    // // let mut index = 0;
    // // println!("{}", expr(&vec!["6", "-", "2"], &mut index));
    // let input = "( 10 + 20 ) * 3";
    // let ret = lexer(input);
    // println!("{:?}", ret);
    // let stdin = stdin();
    // for line in stdin.lock().lines() {
        // let line = line.unwrap();
        let line = "3+2";
        let mut i = Interpreter::new(&line);
        println!("{:?}", i.expr());
        // println!("{:?}", i.get_next_token());
        // println!("{:?}", i.get_next_token());
        // println!("{:?}", i.get_next_token());

    // }
}