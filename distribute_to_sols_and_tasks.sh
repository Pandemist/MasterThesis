#!/bin/bash

# ln -sf ../../../compiler/tests/test_inputs/beispiel_05_parser_output.txt beispiel_05_parser_output.txt

#sol_02
# test files
cd sol_02/tests/
ln -sf ../../compiler/tests/lexer_tests.rs lexer_tests.rs
ln -sf ../../compiler/tests/lexer_tests_korrektur.rs lexer_tests_korrektur.rs

# test inputs
cd test_inputs/
ln -sf ../../../compiler/tests/test_inputs/lexer_all_token_test_input.c1 lexer_all_token_test_input.c1
ln -sf ../../../compiler/tests/test_inputs/lexer_all_token_test_output.txt lexer_all_token_test_output.txt
ln -sf ../../../compiler/tests/test_inputs/lexer_test_input.c1 lexer_test_input.c1
ln -sf ../../../compiler/tests/test_inputs/lexer_test_output.txt lexer_test_output.txt

# code
cd ../../src/
ln -sf ../../compiler/src/c1_lex.rs c1_lex.rs

# lib file löschen und dann neu anlegen
rm -f lib.rs
echo "pub mod c1_lex;" >> "lib.rs"
echo "" >> "lib.rs"
echo "pub use c1_lex::C1Token;" >> "lib.rs"
cd ../../


#sol_03
# test files
cd sol_03/tests/
ln -sf ../../compiler/tests/lexer_tests.rs lexer_tests.rs
ln -sf ../../compiler/tests/lexer_tests_korrektur.rs lexer_tests_korrektur.rs
ln -sf ../../compiler/tests/parser_tests.rs parser_tests.rs

# test inputs
cd test_inputs/
ln -sf ../../../compiler/tests/test_inputs/lexer_all_token_test_input.c1 lexer_all_token_test_input.c1
ln -sf ../../../compiler/tests/test_inputs/lexer_all_token_test_output.txt lexer_all_token_test_output.txt
ln -sf ../../../compiler/tests/test_inputs/lexer_test_input.c1 lexer_test_input.c1
ln -sf ../../../compiler/tests/test_inputs/lexer_test_output.txt lexer_test_output.txt

ln -sf ../../../compiler/tests/test_inputs/beispiel_01.c1 beispiel_01.c1
ln -sf ../../../compiler/tests/test_inputs/beispiel_02.c1 beispiel_02.c1
ln -sf ../../../compiler/tests/test_inputs/beispiel_03.c1 beispiel_03.c1
ln -sf ../../../compiler/tests/test_inputs/beispiel_04.c1 beispiel_04.c1
ln -sf ../../../compiler/tests/test_inputs/beispiel_05.c1 beispiel_05.c1
ln -sf ../../../compiler/tests/test_inputs/beispiel_06.c1 beispiel_06.c1
ln -sf ../../../compiler/tests/test_inputs/beispiel_07.c1 beispiel_07.c1
ln -sf ../../../compiler/tests/test_inputs/beispiel_08.c1 beispiel_08.c1
ln -sf ../../../compiler/tests/test_inputs/beispiel_09.c1 beispiel_09.c1

ln -sf ../../../compiler/tests/test_inputs/beispiel_01_parser_output.txt beispiel_01_parser_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_02_parser_output.txt beispiel_02_parser_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_03_parser_output.txt beispiel_03_parser_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_04_parser_output.txt beispiel_04_parser_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_05_parser_output.txt beispiel_05_parser_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_06_parser_output.txt beispiel_06_parser_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_07_parser_output.txt beispiel_07_parser_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_08_parser_output.txt beispiel_08_parser_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_09_parser_output.txt beispiel_09_parser_output.txt

ln -sf ../../../compiler/tests/test_inputs/parser_if_else_01.c1 parser_if_else_01.c1
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_02.c1 parser_if_else_02.c1
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_03.c1 parser_if_else_03.c1
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_04.c1 parser_if_else_04.c1
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_05.c1 parser_if_else_05.c1
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_06.c1 parser_if_else_06.c1
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_07.c1 parser_if_else_07.c1

ln -sf ../../../compiler/tests/test_inputs/parser_if_else_01_output.txt parser_if_else_01_output.txt
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_02_output.txt parser_if_else_02_output.txt
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_03_output.txt parser_if_else_03_output.txt
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_04_output.txt parser_if_else_04_output.txt
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_05_output.txt parser_if_else_05_output.txt
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_06_output.txt parser_if_else_06_output.txt
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_07_output.txt parser_if_else_07_output.txt

# code
cd ../../src/
ln -sf ../../compiler/src/c1_lex.rs c1_lex.rs
ln -sf ../../compiler/src/lexer.rs lexer.rs
ln -sf ../../compiler/src/c1_pars.lalrpop c1_pars.lalrpop

# lib file löschen und dann neu anlegen
rm -f lib.rs
echo "#[macro_use]" >> "lib.rs"
echo "extern crate lalrpop_util;" >> "lib.rs"
echo "" >> "lib.rs"
echo "pub mod structs;" >> "lib.rs"
echo "" >> "lib.rs"
echo "pub mod c1_lex;" >> "lib.rs"
echo "pub mod lexer;" >> "lib.rs"
echo "" >> "lib.rs"
echo "lalrpop_mod!(pub c1_pars);" >> "lib.rs"
echo "" >> "lib.rs"
echo "pub use structs::*;" >> "lib.rs"
echo "" >> "lib.rs"
echo "pub use c1_lex::C1Token;" >> "lib.rs"
echo "pub use lexer::C1Lexer;" >> "lib.rs"

# struct ordner anlegen
mkdir -p structs/
cd structs/

# mod file löschen und dann neu erstellen
rm -f mod.rs
echo "pub mod symbol_type;" >> "mod.rs"
echo "pub mod syntax_tree;" >> "mod.rs"

ln -sf ../../../compiler/src/structs/symbol_type.rs symbol_type.rs
ln -sf ../../../compiler/src/structs/syntax_tree.rs syntax_tree.rs
cd ../../../


#sol_04
# test files
cd sol_04/tests/
ln -sf ../../compiler/tests/lexer_tests.rs lexer_tests.rs
ln -sf ../../compiler/tests/lexer_tests_korrektur.rs lexer_tests_korrektur.rs
ln -sf ../../compiler/tests/parser_tests.rs parser_tests.rs
ln -sf ../../compiler/tests/semantic_checker_tests.rs semantic_checker_tests.rs

# test inputs
cd test_inputs/
ln -sf ../../../compiler/tests/test_inputs/lexer_all_token_test_input.c1 lexer_all_token_test_input.c1
ln -sf ../../../compiler/tests/test_inputs/lexer_all_token_test_output.txt lexer_all_token_test_output.txt
ln -sf ../../../compiler/tests/test_inputs/lexer_test_input.c1 lexer_test_input.c1
ln -sf ../../../compiler/tests/test_inputs/lexer_test_output.txt lexer_test_output.txt

ln -sf ../../../compiler/tests/test_inputs/beispiel_01.c1 beispiel_01.c1
ln -sf ../../../compiler/tests/test_inputs/beispiel_02.c1 beispiel_02.c1
ln -sf ../../../compiler/tests/test_inputs/beispiel_03.c1 beispiel_03.c1
ln -sf ../../../compiler/tests/test_inputs/beispiel_04.c1 beispiel_04.c1
ln -sf ../../../compiler/tests/test_inputs/beispiel_05.c1 beispiel_05.c1
ln -sf ../../../compiler/tests/test_inputs/beispiel_06.c1 beispiel_06.c1
ln -sf ../../../compiler/tests/test_inputs/beispiel_07.c1 beispiel_07.c1
ln -sf ../../../compiler/tests/test_inputs/beispiel_08.c1 beispiel_08.c1
ln -sf ../../../compiler/tests/test_inputs/beispiel_09.c1 beispiel_09.c1

ln -sf ../../../compiler/tests/test_inputs/beispiel_01_parser_output.txt beispiel_01_parser_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_02_parser_output.txt beispiel_02_parser_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_03_parser_output.txt beispiel_03_parser_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_04_parser_output.txt beispiel_04_parser_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_05_parser_output.txt beispiel_05_parser_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_06_parser_output.txt beispiel_06_parser_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_07_parser_output.txt beispiel_07_parser_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_08_parser_output.txt beispiel_08_parser_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_09_parser_output.txt beispiel_09_parser_output.txt

ln -sf ../../../compiler/tests/test_inputs/beispiel_01_semantic_output.txt beispiel_01_semantic_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_02_semantic_output.txt beispiel_02_semantic_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_03_semantic_output.txt beispiel_03_semantic_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_04_semantic_output.txt beispiel_04_semantic_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_05_semantic_output.txt beispiel_05_semantic_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_06_semantic_output.txt beispiel_06_semantic_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_07_semantic_output.txt beispiel_07_semantic_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_08_semantic_output.txt beispiel_08_semantic_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_09_semantic_output.txt beispiel_09_semantic_output.txt

ln -sf ../../../compiler/tests/test_inputs/parser_if_else_01.c1 parser_if_else_01.c1
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_02.c1 parser_if_else_02.c1
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_03.c1 parser_if_else_03.c1
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_04.c1 parser_if_else_04.c1
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_05.c1 parser_if_else_05.c1
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_06.c1 parser_if_else_06.c1
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_07.c1 parser_if_else_07.c1

ln -sf ../../../compiler/tests/test_inputs/parser_if_else_01_output.txt parser_if_else_01_output.txt
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_02_output.txt parser_if_else_02_output.txt
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_03_output.txt parser_if_else_03_output.txt
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_04_output.txt parser_if_else_04_output.txt
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_05_output.txt parser_if_else_05_output.txt
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_06_output.txt parser_if_else_06_output.txt
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_07_output.txt parser_if_else_07_output.txt

# code
cd ../../src/
ln -sf ../../compiler/src/c1_lex.rs c1_lex.rs
ln -sf ../../compiler/src/lexer.rs lexer.rs
ln -sf ../../compiler/src/c1_pars.lalrpop c1_pars.lalrpop
ln -sf ../../compiler/src/semantic_checker.rs semantic_checker.rs

# lib file löschen und dann neu anlegen
rm -f lib.rs
echo "#[macro_use]" >> "lib.rs"
echo "extern crate lalrpop_util;" >> "lib.rs"
echo "" >> "lib.rs"
echo "pub mod structs;" >> "lib.rs"
echo "" >> "lib.rs"
echo "pub mod c1_lex;" >> "lib.rs"
echo "pub mod lexer;" >> "lib.rs"
echo "pub mod semantic_checker;" >> "lib.rs"
echo "" >> "lib.rs"
echo "lalrpop_mod!(pub c1_pars);" >> "lib.rs"
echo "" >> "lib.rs"
echo "pub use structs::*;" >> "lib.rs"
echo "" >> "lib.rs"
echo "pub use c1_lex::C1Token;" >> "lib.rs"
echo "pub use lexer::C1Lexer;" >> "lib.rs"
echo "pub use semantic_checker::SemanticChecker;" >> "lib.rs"

# struct ordner anlegen
mkdir -p structs/
cd structs/

# mod file löschen und dann neu erstellen
rm -f mod.rs
echo "pub mod symbol_type;" >> "mod.rs"
echo "pub mod syntax_tree;" >> "mod.rs"
echo "pub mod syntax_tree_visitor;" >> "mod.rs"
echo "pub mod refactored_syntax_tree;" >> "mod.rs"
echo "pub mod symbol_table;" >> "mod.rs"

ln -sf ../../../compiler/src/structs/symbol_type.rs symbol_type.rs
ln -sf ../../../compiler/src/structs/syntax_tree.rs syntax_tree.rs
ln -sf ../../../compiler/src/structs/syntax_tree_visitor.rs syntax_tree_visitor.rs
ln -sf ../../../compiler/src/structs/refactored_syntax_tree.rs refactored_syntax_tree.rs
ln -sf ../../../compiler/src/structs/symbol_table.rs symbol_table.rs
cd ../../../


#sol_05
# test files
cd sol_05/tests/
ln -sf ../../compiler/tests/lexer_tests.rs lexer_tests.rs
ln -sf ../../compiler/tests/lexer_tests_korrektur.rs lexer_tests_korrektur.rs
ln -sf ../../compiler/tests/parser_tests.rs parser_tests.rs
ln -sf ../../compiler/tests/semantic_checker_tests.rs semantic_checker_tests.rs
ln -sf ../../compiler/tests/interpreter_tests.rs interpreter_tests.rs

# test inputs
cd test_inputs/
ln -sf ../../../compiler/tests/test_inputs/lexer_all_token_test_input.c1 lexer_all_token_test_input.c1
ln -sf ../../../compiler/tests/test_inputs/lexer_all_token_test_output.txt lexer_all_token_test_output.txt
ln -sf ../../../compiler/tests/test_inputs/lexer_test_input.c1 lexer_test_input.c1
ln -sf ../../../compiler/tests/test_inputs/lexer_test_output.txt lexer_test_output.txt

ln -sf ../../../compiler/tests/test_inputs/beispiel_01.c1 beispiel_01.c1
ln -sf ../../../compiler/tests/test_inputs/beispiel_02.c1 beispiel_02.c1
ln -sf ../../../compiler/tests/test_inputs/beispiel_03.c1 beispiel_03.c1
ln -sf ../../../compiler/tests/test_inputs/beispiel_04.c1 beispiel_04.c1
ln -sf ../../../compiler/tests/test_inputs/beispiel_05.c1 beispiel_05.c1
ln -sf ../../../compiler/tests/test_inputs/beispiel_06.c1 beispiel_06.c1
ln -sf ../../../compiler/tests/test_inputs/beispiel_07.c1 beispiel_07.c1
ln -sf ../../../compiler/tests/test_inputs/beispiel_08.c1 beispiel_08.c1
ln -sf ../../../compiler/tests/test_inputs/beispiel_09.c1 beispiel_09.c1

ln -sf ../../../compiler/tests/test_inputs/beispiel_01_parser_output.txt beispiel_01_parser_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_02_parser_output.txt beispiel_02_parser_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_03_parser_output.txt beispiel_03_parser_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_04_parser_output.txt beispiel_04_parser_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_05_parser_output.txt beispiel_05_parser_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_06_parser_output.txt beispiel_06_parser_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_07_parser_output.txt beispiel_07_parser_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_08_parser_output.txt beispiel_08_parser_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_09_parser_output.txt beispiel_09_parser_output.txt

ln -sf ../../../compiler/tests/test_inputs/beispiel_01_semantic_output.txt beispiel_01_semantic_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_02_semantic_output.txt beispiel_02_semantic_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_03_semantic_output.txt beispiel_03_semantic_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_04_semantic_output.txt beispiel_04_semantic_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_05_semantic_output.txt beispiel_05_semantic_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_06_semantic_output.txt beispiel_06_semantic_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_07_semantic_output.txt beispiel_07_semantic_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_08_semantic_output.txt beispiel_08_semantic_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_09_semantic_output.txt beispiel_09_semantic_output.txt

ln -sf ../../../compiler/tests/test_inputs/beispiel_01_compiler_output.txt beispiel_01_compiler_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_02_compiler_output.txt beispiel_02_compiler_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_03_compiler_output.txt beispiel_03_compiler_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_04_compiler_output.txt beispiel_04_compiler_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_05_compiler_output.txt beispiel_05_compiler_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_06_compiler_output.txt beispiel_06_compiler_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_07_compiler_output.txt beispiel_07_compiler_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_08_compiler_output.txt beispiel_08_compiler_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_09_compiler_output.txt beispiel_09_compiler_output.txt

ln -sf ../../../compiler/tests/test_inputs/parser_if_else_01.c1 parser_if_else_01.c1
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_02.c1 parser_if_else_02.c1
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_03.c1 parser_if_else_03.c1
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_04.c1 parser_if_else_04.c1
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_05.c1 parser_if_else_05.c1
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_06.c1 parser_if_else_06.c1
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_07.c1 parser_if_else_07.c1

ln -sf ../../../compiler/tests/test_inputs/parser_if_else_01_output.txt parser_if_else_01_output.txt
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_02_output.txt parser_if_else_02_output.txt
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_03_output.txt parser_if_else_03_output.txt
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_04_output.txt parser_if_else_04_output.txt
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_05_output.txt parser_if_else_05_output.txt
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_06_output.txt parser_if_else_06_output.txt
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_07_output.txt parser_if_else_07_output.txt

# code
cd ../../src/
ln -sf ../../compiler/src/c1_lex.rs c1_lex.rs
ln -sf ../../compiler/src/lexer.rs lexer.rs
ln -sf ../../compiler/src/c1_pars.lalrpop c1_pars.lalrpop
ln -sf ../../compiler/src/semantic_checker.rs semantic_checker.rs
ln -sf ../../compiler/src/interpreter.rs interpreter.rs

# lib file löschen und dann neu anlegen
rm -f lib.rs
echo "#[macro_use]" >> "lib.rs"
echo "extern crate lalrpop_util;" >> "lib.rs"
echo "" >> "lib.rs"
echo "pub mod structs;" >> "lib.rs"
echo "" >> "lib.rs"
echo "pub mod c1_lex;" >> "lib.rs"
echo "pub mod lexer;" >> "lib.rs"
echo "pub mod semantic_checker;" >> "lib.rs"
echo "pub mod interpreter;" >> "lib.rs"
echo "" >> "lib.rs"
echo "lalrpop_mod!(pub c1_pars);" >> "lib.rs"
echo "" >> "lib.rs"
echo "pub use structs::*;" >> "lib.rs"
echo "" >> "lib.rs"
echo "pub use c1_lex::C1Token;" >> "lib.rs"
echo "pub use lexer::C1Lexer;" >> "lib.rs"
echo "pub use semantic_checker::SemanticChecker;" >> "lib.rs"
echo "pub use interpreter::Interpreter;" >> "lib.rs"

# struct ordner anlegen
mkdir -p structs/
cd structs/

# mod file löschen und dann neu erstellen
rm -f mod.rs
echo "pub mod symbol_type;" >> "mod.rs"
echo "pub mod syntax_tree;" >> "mod.rs"
echo "pub mod syntax_tree_visitor;" >> "mod.rs"
echo "pub mod refactored_syntax_tree;" >> "mod.rs"
echo "pub mod refactored_syntax_tree_visitor;" >> "mod.rs"
echo "pub mod symbol_table;" >> "mod.rs"

ln -sf ../../../compiler/src/structs/symbol_type.rs symbol_type.rs
ln -sf ../../../compiler/src/structs/syntax_tree.rs syntax_tree.rs
ln -sf ../../../compiler/src/structs/syntax_tree_visitor.rs syntax_tree_visitor.rs
ln -sf ../../../compiler/src/structs/refactored_syntax_tree.rs refactored_syntax_tree.rs
ln -sf ../../../compiler/src/structs/refactored_syntax_tree_visitor.rs refactored_syntax_tree_visitor.rs
ln -sf ../../../compiler/src/structs/symbol_table.rs symbol_table.rs
cd ../../../


#task_02
# test files
cd task_02/tests/
ln -sf ../../compiler/tests/lexer_tests.rs lexer_tests.rs

# test inputs
cd test_inputs/
ln -sf ../../../compiler/tests/test_inputs/lexer_all_token_test_input.c1 lexer_all_token_test_input.c1
ln -sf ../../../compiler/tests/test_inputs/lexer_all_token_test_output.txt lexer_all_token_test_output.txt
ln -sf ../../../compiler/tests/test_inputs/lexer_test_input.c1 lexer_test_input.c1
ln -sf ../../../compiler/tests/test_inputs/lexer_test_output.txt lexer_test_output.txt

# code
cd ../../src/
ln -sf ../../sol_02/src/main.rs main.rs
ln -sf ../../sol_02/src/lib.rs lib.rs
cd ../../


#task_03
# test files
cd task_03/tests/
ln -sf ../../compiler/tests/lexer_tests.rs lexer_tests.rs
ln -sf ../../compiler/tests/parser_tests.rs parser_tests.rs

# test inputs
cd test_inputs/
ln -sf ../../../compiler/tests/test_inputs/lexer_all_token_test_input.c1 lexer_all_token_test_input.c1
ln -sf ../../../compiler/tests/test_inputs/lexer_all_token_test_output.txt lexer_all_token_test_output.txt
ln -sf ../../../compiler/tests/test_inputs/lexer_test_input.c1 lexer_test_input.c1
ln -sf ../../../compiler/tests/test_inputs/lexer_test_output.txt lexer_test_output.txt

ln -sf ../../../compiler/tests/test_inputs/beispiel_01.c1 beispiel_01.c1
ln -sf ../../../compiler/tests/test_inputs/beispiel_02.c1 beispiel_02.c1
ln -sf ../../../compiler/tests/test_inputs/beispiel_03.c1 beispiel_03.c1
ln -sf ../../../compiler/tests/test_inputs/beispiel_04.c1 beispiel_04.c1
ln -sf ../../../compiler/tests/test_inputs/beispiel_05.c1 beispiel_05.c1
ln -sf ../../../compiler/tests/test_inputs/beispiel_06.c1 beispiel_06.c1
ln -sf ../../../compiler/tests/test_inputs/beispiel_07.c1 beispiel_07.c1
ln -sf ../../../compiler/tests/test_inputs/beispiel_08.c1 beispiel_08.c1
ln -sf ../../../compiler/tests/test_inputs/beispiel_09.c1 beispiel_09.c1

ln -sf ../../../compiler/tests/test_inputs/beispiel_01_parser_output.txt beispiel_01_parser_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_02_parser_output.txt beispiel_02_parser_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_03_parser_output.txt beispiel_03_parser_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_04_parser_output.txt beispiel_04_parser_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_05_parser_output.txt beispiel_05_parser_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_06_parser_output.txt beispiel_06_parser_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_07_parser_output.txt beispiel_07_parser_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_08_parser_output.txt beispiel_08_parser_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_09_parser_output.txt beispiel_09_parser_output.txt

ln -sf ../../../compiler/tests/test_inputs/parser_if_else_01.c1 parser_if_else_01.c1
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_02.c1 parser_if_else_02.c1
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_03.c1 parser_if_else_03.c1
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_04.c1 parser_if_else_04.c1
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_05.c1 parser_if_else_05.c1
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_06.c1 parser_if_else_06.c1
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_07.c1 parser_if_else_07.c1

ln -sf ../../../compiler/tests/test_inputs/parser_if_else_01_output.txt parser_if_else_01_output.txt
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_02_output.txt parser_if_else_02_output.txt
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_03_output.txt parser_if_else_03_output.txt
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_04_output.txt parser_if_else_04_output.txt
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_05_output.txt parser_if_else_05_output.txt
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_06_output.txt parser_if_else_06_output.txt
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_07_output.txt parser_if_else_07_output.txt

# code
cd ../../src/
ln -sf ../../compiler/src/c1_lex.rs c1_lex.rs
ln -sf ../../compiler/src/lexer.rs lexer.rs
ln -sf ../../sol_03/src/main.rs main.rs
ln -sf ../../sol_03/src/lib.rs lib.rs

# struct ordner anlegen
mkdir -p structs/
cd structs/

# mod file löschen und dann neu erstellen
rm -f mod.rs
echo "pub mod symbol_type;" >> "mod.rs"
echo "pub mod syntax_tree;" >> "mod.rs"

ln -sf ../../../compiler/src/structs/symbol_type.rs symbol_type.rs
ln -sf ../../../compiler/src/structs/syntax_tree.rs syntax_tree.rs
cd ../../../

#task_04
# test files
cd task_04/tests/
ln -sf ../../compiler/tests/lexer_tests.rs lexer_tests.rs
ln -sf ../../compiler/tests/parser_tests.rs parser_tests.rs
ln -sf ../../compiler/tests/semantic_checker_tests.rs semantic_checker_tests.rs

# test inputs
cd test_inputs/
ln -sf ../../../compiler/tests/test_inputs/lexer_all_token_test_input.c1 lexer_all_token_test_input.c1
ln -sf ../../../compiler/tests/test_inputs/lexer_all_token_test_output.txt lexer_all_token_test_output.txt
ln -sf ../../../compiler/tests/test_inputs/lexer_test_input.c1 lexer_test_input.c1
ln -sf ../../../compiler/tests/test_inputs/lexer_test_output.txt lexer_test_output.txt

ln -sf ../../../compiler/tests/test_inputs/beispiel_01.c1 beispiel_01.c1
ln -sf ../../../compiler/tests/test_inputs/beispiel_02.c1 beispiel_02.c1
ln -sf ../../../compiler/tests/test_inputs/beispiel_03.c1 beispiel_03.c1
ln -sf ../../../compiler/tests/test_inputs/beispiel_04.c1 beispiel_04.c1
ln -sf ../../../compiler/tests/test_inputs/beispiel_05.c1 beispiel_05.c1
ln -sf ../../../compiler/tests/test_inputs/beispiel_06.c1 beispiel_06.c1
ln -sf ../../../compiler/tests/test_inputs/beispiel_07.c1 beispiel_07.c1
ln -sf ../../../compiler/tests/test_inputs/beispiel_08.c1 beispiel_08.c1
ln -sf ../../../compiler/tests/test_inputs/beispiel_09.c1 beispiel_09.c1

ln -sf ../../../compiler/tests/test_inputs/beispiel_01_parser_output.txt beispiel_01_parser_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_02_parser_output.txt beispiel_02_parser_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_03_parser_output.txt beispiel_03_parser_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_04_parser_output.txt beispiel_04_parser_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_05_parser_output.txt beispiel_05_parser_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_06_parser_output.txt beispiel_06_parser_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_07_parser_output.txt beispiel_07_parser_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_08_parser_output.txt beispiel_08_parser_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_09_parser_output.txt beispiel_09_parser_output.txt

ln -sf ../../../compiler/tests/test_inputs/beispiel_01_semantic_output.txt beispiel_01_semantic_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_02_semantic_output.txt beispiel_02_semantic_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_03_semantic_output.txt beispiel_03_semantic_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_04_semantic_output.txt beispiel_04_semantic_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_05_semantic_output.txt beispiel_05_semantic_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_06_semantic_output.txt beispiel_06_semantic_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_07_semantic_output.txt beispiel_07_semantic_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_08_semantic_output.txt beispiel_08_semantic_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_09_semantic_output.txt beispiel_09_semantic_output.txt

ln -sf ../../../compiler/tests/test_inputs/parser_if_else_01.c1 parser_if_else_01.c1
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_02.c1 parser_if_else_02.c1
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_03.c1 parser_if_else_03.c1
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_04.c1 parser_if_else_04.c1
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_05.c1 parser_if_else_05.c1
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_06.c1 parser_if_else_06.c1
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_07.c1 parser_if_else_07.c1

ln -sf ../../../compiler/tests/test_inputs/parser_if_else_01_output.txt parser_if_else_01_output.txt
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_02_output.txt parser_if_else_02_output.txt
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_03_output.txt parser_if_else_03_output.txt
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_04_output.txt parser_if_else_04_output.txt
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_05_output.txt parser_if_else_05_output.txt
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_06_output.txt parser_if_else_06_output.txt
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_07_output.txt parser_if_else_07_output.txt

# code
cd ../../src/
ln -sf ../../compiler/src/c1_lex.rs c1_lex.rs
ln -sf ../../compiler/src/lexer.rs lexer.rs
ln -sf ../../compiler/src/c1_pars.lalrpop c1_pars.lalrpop
ln -sf ../../sol_04/src/lib.rs lib.rs
ln -sf ../../sol_04/src/main.rs main.rs

# struct ordner anlegen
mkdir -p structs/
cd structs/

# mod file löschen und dann neu erstellen
rm -f mod.rs
echo "pub mod symbol_type;" >> "mod.rs"
echo "pub mod syntax_tree;" >> "mod.rs"
echo "pub mod syntax_tree_visitor;" >> "mod.rs"
echo "pub mod refactored_syntax_tree;" >> "mod.rs"
echo "pub mod symbol_table;" >> "mod.rs"

ln -sf ../../../compiler/src/structs/symbol_type.rs symbol_type.rs
ln -sf ../../../compiler/src/structs/syntax_tree.rs syntax_tree.rs
ln -sf ../../../compiler/src/structs/syntax_tree_visitor.rs syntax_tree_visitor.rs
ln -sf ../../../compiler/src/structs/refactored_syntax_tree.rs refactored_syntax_tree.rs
ln -sf ../../../compiler/src/structs/symbol_table.rs symbol_table.rs
cd ../../../


#task_05
# test files
cd task_05/tests/
ln -sf ../../compiler/tests/lexer_tests.rs lexer_tests.rs
ln -sf ../../compiler/tests/parser_tests.rs parser_tests.rs
ln -sf ../../compiler/tests/semantic_checker_tests.rs semantic_checker_tests.rs
ln -sf ../../compiler/tests/interpreter_tests.rs interpreter_tests.rs

# test inputs
cd test_inputs/
ln -sf ../../../compiler/tests/test_inputs/lexer_all_token_test_input.c1 lexer_all_token_test_input.c1
ln -sf ../../../compiler/tests/test_inputs/lexer_all_token_test_output.txt lexer_all_token_test_output.txt
ln -sf ../../../compiler/tests/test_inputs/lexer_test_input.c1 lexer_test_input.c1
ln -sf ../../../compiler/tests/test_inputs/lexer_test_output.txt lexer_test_output.txt

ln -sf ../../../compiler/tests/test_inputs/beispiel_01.c1 beispiel_01.c1
ln -sf ../../../compiler/tests/test_inputs/beispiel_02.c1 beispiel_02.c1
ln -sf ../../../compiler/tests/test_inputs/beispiel_03.c1 beispiel_03.c1
ln -sf ../../../compiler/tests/test_inputs/beispiel_04.c1 beispiel_04.c1
ln -sf ../../../compiler/tests/test_inputs/beispiel_05.c1 beispiel_05.c1
ln -sf ../../../compiler/tests/test_inputs/beispiel_06.c1 beispiel_06.c1
ln -sf ../../../compiler/tests/test_inputs/beispiel_07.c1 beispiel_07.c1
ln -sf ../../../compiler/tests/test_inputs/beispiel_08.c1 beispiel_08.c1
ln -sf ../../../compiler/tests/test_inputs/beispiel_09.c1 beispiel_09.c1

ln -sf ../../../compiler/tests/test_inputs/beispiel_01_parser_output.txt beispiel_01_parser_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_02_parser_output.txt beispiel_02_parser_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_03_parser_output.txt beispiel_03_parser_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_04_parser_output.txt beispiel_04_parser_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_05_parser_output.txt beispiel_05_parser_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_06_parser_output.txt beispiel_06_parser_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_07_parser_output.txt beispiel_07_parser_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_08_parser_output.txt beispiel_08_parser_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_09_parser_output.txt beispiel_09_parser_output.txt

ln -sf ../../../compiler/tests/test_inputs/beispiel_01_semantic_output.txt beispiel_01_semantic_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_02_semantic_output.txt beispiel_02_semantic_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_03_semantic_output.txt beispiel_03_semantic_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_04_semantic_output.txt beispiel_04_semantic_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_05_semantic_output.txt beispiel_05_semantic_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_06_semantic_output.txt beispiel_06_semantic_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_07_semantic_output.txt beispiel_07_semantic_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_08_semantic_output.txt beispiel_08_semantic_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_09_semantic_output.txt beispiel_09_semantic_output.txt

ln -sf ../../../compiler/tests/test_inputs/beispiel_01_compiler_output.txt beispiel_01_compiler_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_02_compiler_output.txt beispiel_02_compiler_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_03_compiler_output.txt beispiel_03_compiler_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_04_compiler_output.txt beispiel_04_compiler_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_05_compiler_output.txt beispiel_05_compiler_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_06_compiler_output.txt beispiel_06_compiler_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_07_compiler_output.txt beispiel_07_compiler_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_08_compiler_output.txt beispiel_08_compiler_output.txt
ln -sf ../../../compiler/tests/test_inputs/beispiel_09_compiler_output.txt beispiel_09_compiler_output.txt

ln -sf ../../../compiler/tests/test_inputs/parser_if_else_01.c1 parser_if_else_01.c1
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_02.c1 parser_if_else_02.c1
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_03.c1 parser_if_else_03.c1
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_04.c1 parser_if_else_04.c1
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_05.c1 parser_if_else_05.c1
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_06.c1 parser_if_else_06.c1
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_07.c1 parser_if_else_07.c1

ln -sf ../../../compiler/tests/test_inputs/parser_if_else_01_output.txt parser_if_else_01_output.txt
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_02_output.txt parser_if_else_02_output.txt
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_03_output.txt parser_if_else_03_output.txt
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_04_output.txt parser_if_else_04_output.txt
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_05_output.txt parser_if_else_05_output.txt
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_06_output.txt parser_if_else_06_output.txt
ln -sf ../../../compiler/tests/test_inputs/parser_if_else_07_output.txt parser_if_else_07_output.txt

# code
cd ../../src/
ln -sf ../../compiler/src/c1_lex.rs c1_lex.rs
ln -sf ../../compiler/src/lexer.rs lexer.rs
ln -sf ../../compiler/src/c1_pars.lalrpop c1_pars.lalrpop
ln -sf ../../compiler/src/semantic_checker.rs semantic_checker.rs
ln -sf ../../sol_05/src/main.rs main.rs
ln -sf ../../sol_05/src/lib.rs lib.rs

# struct ordner anlegen
mkdir -p structs/
cd structs/

# mod file löschen und dann neu erstellen
rm -f mod.rs
echo "pub mod symbol_type;" >> "mod.rs"
echo "pub mod syntax_tree;" >> "mod.rs"
echo "pub mod syntax_tree_visitor;" >> "mod.rs"
echo "pub mod refactored_syntax_tree;" >> "mod.rs"
echo "pub mod refactored_syntax_tree_visitor;" >> "mod.rs"
echo "pub mod symbol_table;" >> "mod.rs"

ln -sf ../../../compiler/src/structs/symbol_type.rs symbol_type.rs
ln -sf ../../../compiler/src/structs/syntax_tree.rs syntax_tree.rs
ln -sf ../../../compiler/src/structs/syntax_tree_visitor.rs syntax_tree_visitor.rs
ln -sf ../../../compiler/src/structs/refactored_syntax_tree.rs refactored_syntax_tree.rs
ln -sf ../../../compiler/src/structs/refactored_syntax_tree_visitor.rs refactored_syntax_tree_visitor.rs
ln -sf ../../../compiler/src/structs/symbol_table.rs symbol_table.rs
cd ../../../