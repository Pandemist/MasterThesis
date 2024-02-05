#[allow(unused)]
use logos::Logos;

#[allow(unused)]
use crate::c1_lex::C1Token;

mod c1_lex;

use std::env;
use std::path::Path;

use std::fs;

fn main() {
    // Argumente auslesen und den ersten Parameter
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Zu wenige Argumente.");
        return;
    }
    let path_arg = Path::new(&args[1]);

    let test_input = fs::read_to_string(path_arg).unwrap();

    let mut lexer = C1Token::lexer(test_input.as_str());

    while let Some(val) = lexer.next() {
        let parse_result = format!("{:?}: {:?}", val, lexer.slice());
        println!("{}", parse_result);
    }
}