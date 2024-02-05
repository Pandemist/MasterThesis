use std::fs;

use compiler::c1_pars::fileParser;

use compiler::lexer::C1Lexer;

#[test]
fn parser_no_crashing_test() {
    let test_input = fs::read_to_string("tests/test_inputs/beispiel_01.c1").unwrap();

    let lexer = C1Lexer::new(test_input.as_str());
    let parser = fileParser::new();
    let ast = parser.parse(lexer);

    assert!(ast.is_ok());
}

#[test]
fn parser_programm_01_test() {
    let test_input = fs::read_to_string("tests/test_inputs/beispiel_01.c1").unwrap();
    let test_output = fs::read_to_string("tests/test_inputs/beispiel_01_parser_output.txt").unwrap();

    let lexer = C1Lexer::new(test_input.as_str());
    let parser = fileParser::new();
    let ast = parser.parse(lexer);

    assert_eq!(format!("{:?}\n", ast), test_output);
}

#[test]
fn parser_programm_02_test() {
    let test_input = fs::read_to_string("tests/test_inputs/beispiel_02.c1").unwrap();
    let test_output = fs::read_to_string("tests/test_inputs/beispiel_02_parser_output.txt").unwrap();

    let lexer = C1Lexer::new(test_input.as_str());
    let parser = fileParser::new();
    let ast = parser.parse(lexer);

    assert_eq!(format!("{:?}\n", ast), test_output);
}

#[test]
fn parser_programm_03_test() {
    let test_input = fs::read_to_string("tests/test_inputs/beispiel_03.c1").unwrap();
    let test_output = fs::read_to_string("tests/test_inputs/beispiel_03_parser_output.txt").unwrap();

    let lexer = C1Lexer::new(test_input.as_str());
    let parser = fileParser::new();
    let ast = parser.parse(lexer);

    assert_eq!(format!("{:?}\n", ast), test_output);
}

#[test]
fn parser_programm_04_test() {
    let test_input = fs::read_to_string("tests/test_inputs/beispiel_04.c1").unwrap();
    let test_output = fs::read_to_string("tests/test_inputs/beispiel_04_parser_output.txt").unwrap();

    let lexer = C1Lexer::new(test_input.as_str());
    let parser = fileParser::new();
    let ast = parser.parse(lexer);

    assert_eq!(format!("{:?}\n", ast), test_output);
}

#[test]
fn parser_programm_05_test() {
    let test_input = fs::read_to_string("tests/test_inputs/beispiel_05.c1").unwrap();
    let test_output = fs::read_to_string("tests/test_inputs/beispiel_05_parser_output.txt").unwrap();

    let lexer = C1Lexer::new(test_input.as_str());
    let parser = fileParser::new();
    let ast = parser.parse(lexer);

    assert_eq!(format!("{:?}\n", ast), test_output);
}

#[test]
fn parser_programm_06_test() {
    let test_input = fs::read_to_string("tests/test_inputs/beispiel_06.c1").unwrap();
    let test_output = fs::read_to_string("tests/test_inputs/beispiel_06_parser_output.txt").unwrap();

    let lexer = C1Lexer::new(test_input.as_str());
    let parser = fileParser::new();
    let ast = parser.parse(lexer);

    assert_eq!(format!("{:?}\n", ast), test_output);
}

#[test]
fn parser_programm_07_test() {
    let test_input = fs::read_to_string("tests/test_inputs/beispiel_07.c1").unwrap();
    let test_output = fs::read_to_string("tests/test_inputs/beispiel_07_parser_output.txt").unwrap();

    let lexer = C1Lexer::new(test_input.as_str());
    let parser = fileParser::new();
    let ast = parser.parse(lexer);

    assert_eq!(format!("{:?}\n", ast), test_output);
}

#[test]
fn parser_programm_08_test() {
    let test_input = fs::read_to_string("tests/test_inputs/beispiel_08.c1").unwrap();
    let test_output = fs::read_to_string("tests/test_inputs/beispiel_08_parser_output.txt").unwrap();

    let lexer = C1Lexer::new(test_input.as_str());
    let parser = fileParser::new();
    let ast = parser.parse(lexer);

    assert_eq!(format!("{:?}\n", ast), test_output);
}

#[test]
fn parser_programm_09_test() {
    let test_input = fs::read_to_string("tests/test_inputs/beispiel_09.c1").unwrap();
    let test_output = fs::read_to_string("tests/test_inputs/beispiel_09_parser_output.txt").unwrap();

    let lexer = C1Lexer::new(test_input.as_str());
    let parser = fileParser::new();
    let ast = parser.parse(lexer);

    assert_eq!(format!("{:?}\n", ast), test_output);
}



#[test]
fn parser_test_if() {
    let test_input = fs::read_to_string("tests/test_inputs/parser_if_else_01.c1").unwrap();
    let test_output = fs::read_to_string("tests/test_inputs/parser_if_else_01_output.txt").unwrap();

    let lexer = C1Lexer::new(test_input.as_str());
    let parser = fileParser::new();
    let ast = parser.parse(lexer);

    assert_eq!(format!("{:?}\n", ast), test_output);
}

#[test]
fn parser_test_if_else() {
    let test_input = fs::read_to_string("tests/test_inputs/parser_if_else_02.c1").unwrap();
    let test_output = fs::read_to_string("tests/test_inputs/parser_if_else_02_output.txt").unwrap();

    let lexer = C1Lexer::new(test_input.as_str());
    let parser = fileParser::new();
    let ast = parser.parse(lexer);

    assert_eq!(format!("{:?}\n", ast), test_output);
}

#[test]
fn parser_test_if_im_if() {
    let test_input = fs::read_to_string("tests/test_inputs/parser_if_else_03.c1").unwrap();
    let test_output = fs::read_to_string("tests/test_inputs/parser_if_else_03_output.txt").unwrap();

    let lexer = C1Lexer::new(test_input.as_str());
    let parser = fileParser::new();
    let ast = parser.parse(lexer);

    assert_eq!(format!("{:?}\n", ast), test_output);
}

#[test]
fn parser_test_if_im_if_mit_else() {
    let test_input = fs::read_to_string("tests/test_inputs/parser_if_else_04.c1").unwrap();
    let test_output = fs::read_to_string("tests/test_inputs/parser_if_else_04_output.txt").unwrap();

    let lexer = C1Lexer::new(test_input.as_str());
    let parser = fileParser::new();
    let ast = parser.parse(lexer);

    assert_eq!(format!("{:?}\n", ast), test_output);
}

#[test]
fn parser_test_oneline_if_im_oneline_if() {
    let test_input = fs::read_to_string("tests/test_inputs/parser_if_else_05.c1").unwrap();
    let test_output = fs::read_to_string("tests/test_inputs/parser_if_else_05_output.txt").unwrap();

    let lexer = C1Lexer::new(test_input.as_str());
    let parser = fileParser::new();
    let ast = parser.parse(lexer);

    assert_eq!(format!("{:?}\n", ast), test_output);
}

#[test]
fn parser_test_oneline_if_im_oneline_if_mit_else() {
    let test_input = fs::read_to_string("tests/test_inputs/parser_if_else_06.c1").unwrap();
    let test_output = fs::read_to_string("tests/test_inputs/parser_if_else_06_output.txt").unwrap();

    let lexer = C1Lexer::new(test_input.as_str());
    let parser = fileParser::new();
    let ast = parser.parse(lexer);

    assert_eq!(format!("{:?}\n", ast), test_output);
}

#[test]
fn parser_test_oneline_if_im_oneline_while_im_oneline_if_mit_else() {
    let test_input = fs::read_to_string("tests/test_inputs/parser_if_else_07.c1").unwrap();
    let test_output = fs::read_to_string("tests/test_inputs/parser_if_else_07_output.txt").unwrap();

    let lexer = C1Lexer::new(test_input.as_str());
    let parser = fileParser::new();
    let ast = parser.parse(lexer);

    assert_eq!(format!("{:?}\n", ast), test_output);
}