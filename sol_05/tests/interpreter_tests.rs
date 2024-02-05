use std::fs;

use compiler::c1_pars::fileParser;

use compiler::lexer::C1Lexer;

use compiler::semantic_checker::SemanticChecker;

use compiler::interpreter::Interpreter;

#[test]
fn interpreter_program_01_test() {
    let test_input = fs::read_to_string("tests/test_inputs/beispiel_01.c1").unwrap();
    let test_output = fs::read_to_string("tests/test_inputs/beispiel_01_compiler_output.txt").unwrap();

    let lexer = C1Lexer::new(test_input.as_str());
    let parser = fileParser::new();
    let ast = parser.parse(lexer);

    let mut symtabcreator = SemanticChecker::new();
    let refactored_ast = symtabcreator.run(&ast.unwrap());
    let mut interpreter = Interpreter::new();

    assert_eq!(format!("{}\n", interpreter.run(&refactored_ast)), test_output);
}

#[test]
fn interpreter_program_02_test() {
    let test_input = fs::read_to_string("tests/test_inputs/beispiel_02.c1").unwrap();
    let test_output = fs::read_to_string("tests/test_inputs/beispiel_02_compiler_output.txt").unwrap();

    let lexer = C1Lexer::new(test_input.as_str());
    let parser = fileParser::new();
    let ast = parser.parse(lexer);

    let mut symtabcreator = SemanticChecker::new();
    let refactored_ast = symtabcreator.run(&ast.unwrap());
    let mut interpreter = Interpreter::new();

    assert_eq!(format!("{}\n", interpreter.run(&refactored_ast)), test_output);
}

#[test]
fn interpreter_program_03_test() {
    let test_input = fs::read_to_string("tests/test_inputs/beispiel_03.c1").unwrap();
    let test_output = fs::read_to_string("tests/test_inputs/beispiel_03_compiler_output.txt").unwrap();

    let lexer = C1Lexer::new(test_input.as_str());
    let parser = fileParser::new();
    let ast = parser.parse(lexer);

    let mut symtabcreator = SemanticChecker::new();
    let refactored_ast = symtabcreator.run(&ast.unwrap());
    let mut interpreter = Interpreter::new();

    assert_eq!(format!("{}\n", interpreter.run(&refactored_ast)), test_output);
}

#[test]
fn interpreter_program_04_test() {
    let test_input = fs::read_to_string("tests/test_inputs/beispiel_04.c1").unwrap();
    let test_output = fs::read_to_string("tests/test_inputs/beispiel_04_compiler_output.txt").unwrap();

    let lexer = C1Lexer::new(test_input.as_str());
    let parser = fileParser::new();
    let ast = parser.parse(lexer);

    let mut symtabcreator = SemanticChecker::new();
    let refactored_ast = symtabcreator.run(&ast.unwrap());
    let mut interpreter = Interpreter::new();

    assert_eq!(format!("{}\n", interpreter.run(&refactored_ast)), test_output);
}

#[test]
fn interpreter_program_05_test() {
    let test_input = fs::read_to_string("tests/test_inputs/beispiel_05.c1").unwrap();
    let test_output = fs::read_to_string("tests/test_inputs/beispiel_05_compiler_output.txt").unwrap();

    let lexer = C1Lexer::new(test_input.as_str());
    let parser = fileParser::new();
    let ast = parser.parse(lexer);

    let mut symtabcreator = SemanticChecker::new();
    let refactored_ast = symtabcreator.run(&ast.unwrap());
    let mut interpreter = Interpreter::new();

    assert_eq!(format!("{}\n", interpreter.run(&refactored_ast)), test_output);
}

#[test]
fn interpreter_program_06_test() {
    let test_input = fs::read_to_string("tests/test_inputs/beispiel_06.c1").unwrap();
    let test_output = fs::read_to_string("tests/test_inputs/beispiel_06_compiler_output.txt").unwrap();

    let lexer = C1Lexer::new(test_input.as_str());
    let parser = fileParser::new();
    let ast = parser.parse(lexer);

    let mut symtabcreator = SemanticChecker::new();
    let refactored_ast = symtabcreator.run(&ast.unwrap());
    let mut interpreter = Interpreter::new();

    assert_eq!(format!("{}\n", interpreter.run(&refactored_ast)), test_output);
}

#[test]
fn interpreter_program_07_test() {
    let test_input = fs::read_to_string("tests/test_inputs/beispiel_07.c1").unwrap();
    let test_output = fs::read_to_string("tests/test_inputs/beispiel_07_compiler_output.txt").unwrap();
    let lexer = C1Lexer::new(test_input.as_str());
    let parser = fileParser::new();
    let ast = parser.parse(lexer);

    let mut symtabcreator = SemanticChecker::new();
    let refactored_ast = symtabcreator.run(&ast.unwrap());
    let mut interpreter = Interpreter::new();

    assert_eq!(format!("{}\n", interpreter.run(&refactored_ast)), test_output);
}

#[test]
fn interpreter_program_08_test() {
    let test_input = fs::read_to_string("tests/test_inputs/beispiel_08.c1").unwrap();
    let test_output = fs::read_to_string("tests/test_inputs/beispiel_08_compiler_output.txt").unwrap();

    let lexer = C1Lexer::new(test_input.as_str());
    let parser = fileParser::new();
    let ast = parser.parse(lexer);

    let mut symtabcreator = SemanticChecker::new();
    let refactored_ast = symtabcreator.run(&ast.unwrap());
    let mut interpreter = Interpreter::new();

    assert_eq!(format!("{}\n", interpreter.run(&refactored_ast)), test_output);
}

#[test]
fn interpreter_program_09_test() {
    let test_input = fs::read_to_string("tests/test_inputs/beispiel_09.c1").unwrap();
    let test_output = fs::read_to_string("tests/test_inputs/beispiel_09_compiler_output.txt").unwrap();

    let lexer = C1Lexer::new(test_input.as_str());
    let parser = fileParser::new();
    let ast = parser.parse(lexer);

    let mut symtabcreator = SemanticChecker::new();
    let refactored_ast = symtabcreator.run(&ast.unwrap());
    let mut interpreter = Interpreter::new();

    assert_eq!(format!("{}\n", interpreter.run(&refactored_ast)), test_output);
}
