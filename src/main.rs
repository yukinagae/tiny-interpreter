use std::io::stdin;
use std::io::BufRead;

// Expr ::= Mul  ( ( '+' | '-' ) Mul  )*
// Mul  ::= Fact ( ( '*' | '/' ) Fact )*
// Fact ::= 数値

// Fact ::= 数値
fn fact(tokens: &Vec<&str>, index: &mut usize) -> isize {
    tokens[*index].parse::<isize>().unwrap()
}

// Mul  ::= Fact ( ( '*' | '/' ) Fact )*
fn mul(tokens: &Vec<&str>, index: &mut usize) -> isize {
    let mut v1 = fact(tokens, index);

    while tokens.len() > (*index+1) && (tokens[*index+1] == "*" || tokens[*index+1] == "/") {
        *index += 1;
        let op = tokens[*index];
        *index += 1;
        let v2 = fact(tokens, index);

        if op == "*" {
            v1 = v1 * v2;
        } else {
            v1 = v1 / v2;
        }
    }
    return v1;
}

// Expr ::= Mul  ( ( '+' | '-' ) Mul  )*
fn expr(tokens: &Vec<&str>, index: &mut usize) -> isize {
    let mut v1 = mul(tokens, index);
    while tokens.len() > (*index+1) && (tokens[*index+1] == "+" || tokens[*index+1] == "-") {
        *index += 1;
        let op = tokens[*index];
        *index += 1;
        let v2 = mul(tokens, index);

        if op == "+" {
            v1 = v1 + v2;
        } else {
            v1 = v1 - v2;
        }
    }
    return v1;
}

#[derive(Debug)]
struct Tokens<'a> {
    value: Vec<&'a str>
}

fn lexer(input: &str) -> Vec<&str> {
    let words: Vec<&str> = input.split_whitespace().collect();

    // for word in words {
    //     if word == "("
    // }


    return words;
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
    let stdin = stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        // println!("{}", line);
        let ret = lexer(&line);
        println!("{:?}", ret);
    }
}