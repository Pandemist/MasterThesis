use logos::{Logos, SpannedIter};

use crate::c1_lex::C1Token;

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

#[derive(Debug)]
pub enum LexicalError {
    InvalidToken,
}

pub struct C1Lexer<'input> {
    // instead of an iterator over characters, we have a token iterator
    token_stream: SpannedIter<'input, C1Token>,
}

impl<'input> C1Lexer<'input> {
  pub fn new(input: &'input str) -> Self {
    // the Token::lexer() method is provided by the Logos trait
    Self {
      token_stream: C1Token::lexer(input).spanned()
    }
  }
}

impl<'input> Iterator for C1Lexer<'input> {
  type Item = Spanned<C1Token, usize, LexicalError>;

  fn next(&mut self) -> Option<Self::Item> {
    let to = self.token_stream.next();

    to.map(|(token, span)| {
      match token {
        // an invalid token was met
        C1Token::Error => {
          panic!("Error: Unknown token at from position {:?} to {:?}", span.start, span.end);
        },
        _ => {Ok((span.start, token, span.end))},
      }
    })
  }
}