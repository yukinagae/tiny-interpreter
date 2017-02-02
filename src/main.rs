use std::io::stdin;
use std::io::BufRead;

enum TokenType {
    INTEGER,
    PLUS,
    EOF,
}

struct Token {
    type: TokenType,
    value: char
}

#[derive(Debug)]
struct Interpreter<'a> {
    text: &'a str
}

impl<'a> Interpreter<'a> {

    fn get_next_token(&self) -> char {
        self.text.chars().next().unwrap()
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
        let line = "1+2";
        let i = Interpreter { text: &line };
        println!("{:?}", i.get_next_token());
        println!("{:?}", i.get_next_token());
    // }
}