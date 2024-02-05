use std::fs;

use logos::Logos;

use compiler::C1Token;

#[test]
fn simple_test_program() {
    let test_input = fs::read_to_string("tests/test_inputs/lexer_test_input.c1").unwrap();
    let test_expected = fs::read_to_string("tests/test_inputs/lexer_test_output.txt").unwrap();
    
    let mut lexer = C1Token::lexer(test_input.as_str());

    let mut tokens = Vec::new();
    while let Some(val) = lexer.next() {
        let parse_result = format!("{:?}: {:?}", val, lexer.slice());
        tokens.push(parse_result);
    }

    for (parsed, expected) in tokens.iter().zip(test_expected.lines()) {
        assert_eq!(parsed, expected);
    }
}

#[test]
fn test_all_tokens() {
    let test_input = fs::read_to_string("tests/test_inputs/lexer_all_token_test_input.c1").unwrap();
    let test_expected = fs::read_to_string("tests/test_inputs/lexer_all_token_test_output.txt").unwrap();
    
    let mut lexer = C1Token::lexer(test_input.as_str());

    let mut tokens = Vec::new();
    while let Some(val) = lexer.next() {
        let parse_result = format!("{:?}: {:?}", val, lexer.slice());
        tokens.push(parse_result);
    }

    for (parsed, expected) in tokens.iter().zip(test_expected.lines()) {
        assert_eq!(parsed, expected);
    }
}