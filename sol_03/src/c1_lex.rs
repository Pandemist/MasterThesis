#![allow(dead_code)]

use logos::Logos;

use std::fmt;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum C1Token {
    #[token("bool")]
    KwBoolean,

    #[token("do")]
    KwDo,

    #[token("else")]
    KwElse,

    #[token("float")]
    KwFloat,

    #[token("for")]
    KwFor,

    #[token("if")]
    KwIf,

    #[token("int")]
    KwInt,

    #[token("printf")]
    KwPrintf,

    #[token("return")]
    KwReturn,

    #[token("void")]
    KwVoid,

    #[token("String")]
    KwString,

    #[token("while")]
    KwWhile,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Asterisk,

    #[token("/")]
    Slash,

    #[token("=")]
    Assign,

    #[token("==")]
    Eq,

    #[token("!=")]
    Neq,

    #[token("<")]
    LSS,

    #[token(">")]
    GRT,

    #[token("<=")]
    LEQ,

    #[token(">=")]
    GEQ,

    #[token("&&")]
    And,

    #[token("||")]
    Or,

    #[token(",")]
    Comma,

    #[token(";")]
    Semicolon,

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("{")]
    LBrace,

    #[token("}")]
    RBrace,

    #[regex("[0-9]+", |lex| lex.slice().parse())]
    ConstInt(i64),

    #[regex(r"(\d+\.\d+)|(\.\d+([eE]([-+])?\d+)?)|(\d+[eE]([-+])?\d+)", |lex| lex.slice().parse())]
    ConstFloat(f64),

    #[regex("true|false", |lex| lex.slice().parse())]
    ConstBoolean(bool),

    #[regex("\"[^\n\"]*\"", |lex| lex.slice().parse())]
    ConstString(String),

    #[regex("[a-zA-Z]+[0-9a-zA-Z]*", |lex| lex.slice().parse())]
    Id(String),

    #[regex(r"/\*[^\*/]*\*/", logos::skip)]
    CComment,

    #[regex("//.*\n", logos::skip)]
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