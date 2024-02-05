#[macro_use]
extern crate lalrpop_util;

pub mod structs;

pub mod c1_lex;
pub mod lexer;
pub mod semantic_checker;
pub mod interpreter;

lalrpop_mod!(pub c1_pars);

pub use structs::*;

pub use c1_lex::C1Token;
pub use lexer::C1Lexer;
pub use semantic_checker::SemanticChecker;
pub use interpreter::Interpreter;
