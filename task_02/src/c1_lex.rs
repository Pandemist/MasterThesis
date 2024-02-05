#![allow(dead_code)]

use logos::Logos;

use std::fmt;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum C1Token {
    // TODO
    KwBoolean,

    // TODO
    KwDo,

    // TODO
    KwElse,

    // TODO
    KwFloat,

    // TODO
    KwFor,

    // TODO
    KwIf,

    // TODO
    KwInt,

    // TODO
    KwPrintf,

    // TODO
    KwReturn,

    // TODO
    KwVoid,

    // TODO
    KwString,

    // TODO
    KwWhile,

    // TODO
    Plus,

    // TODO
    Minus,

    // TODO
    Asterisk,

    // TODO
    Slash,

    // TODO
    Assign,

    // TODO
    Eq,

    // TODO
    Neq,

    // TODO
    LSS,

    // TODO
    GRT,

    // TODO
    LEQ,

    // TODO
    GEQ,

    // TODO
    And,

    // TODO
    Or,

    // TODO
    Comma,

    // TODO
    Semicolon,

    // TODO
    LParen,

    // TODO
    RParen,

    // TODO
    LBrace,

    // TODO
    RBrace,

    // TODO
    ConstInt(i64),

    // TODO
    ConstFloat(f64),

    // TODO
    ConstBoolean(bool),

    // TODO
    ConstString(String),

    // TODO
    Id(String),

    // TODO
    CComment,

    // TODO
    CPPComment,

    // We can also use this variant to define whitespace,
    // or any other matches we wish to skip.
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Whitespace,

    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[error]
    Error,
}

impl fmt::Display for C1Token {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}