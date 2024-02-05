#[macro_use]
extern crate lalrpop_util;

use std::env;
use std::path::Path;

use std::fs;

use crate::structs::*;

use crate::lexer::C1Lexer;

use crate::semantic_checker::SemanticChecker;

mod structs;

mod c1_lex;
mod lexer;
mod semantic_checker;

lalrpop_mod!(pub c1_pars);

fn main() {
    // Argumente auslesen und den ersten Parameter
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Zu wenige Argumente.");
        return;
    }
    let path_arg = Path::new(&args[1]);

    let test_input = fs::read_to_string(path_arg).unwrap();

    let lexer = C1Lexer::new(test_input.as_str());
    println!("Lexer erstellt");
    let parser = c1_pars::fileParser::new();
    println!("Parser erstellt");
    let ast = parser.parse(lexer);

    match ast {
        Ok(v) => {
            let mut symtabcreator = SemanticChecker::new();

            let refactored_ast = symtabcreator.run(&v);

            println!("{:#?}", refactored_ast);
        },
        Err(v) => {
            println!("Semantik Analyse nicht erfolgreich");
            println!("Syntax Tree:");
            println!("{:#?}", v);
        },
    };
}