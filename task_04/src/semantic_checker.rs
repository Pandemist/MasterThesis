use std::rc::Rc;

use std::collections::HashMap;

use crate::structs::symbol_type::*;
use crate::structs::syntax_tree::*;
use crate::structs::syntax_tree_visitor::*;
use crate::structs::refactored_syntax_tree::*;

use crate::symbol_table::SymbolTable;
use crate::symbol_table::SymbolClass;

#[derive(Default)]
pub struct SemanticChecker {
	pub sym_tab: SymbolTable,

    function_name: String,
    function_map: HashMap<String, Rc<FunctionBodyRefactored>>,
}

impl SemanticChecker {
	pub fn new() -> Self {
		Self{
			sym_tab: SymbolTable::new(),

            function_name: "".to_string(),
            function_map: HashMap::new(),
		}
	}

	pub fn run(&mut self, t: & SyntaxTree) -> SyntaxTreeRefactored {
		self.visit_syntax_tree(t)
	}

	fn insert_function_ref(&mut self, name: String, body_ref: FunctionBodyRefactored) {
        if self.function_map.contains_key(&name) {
            panic!("Error: Function allready defined.");
        }else{
            self.function_map.insert(name, body_ref.into());
        }
    }

    fn get_function_ref(&mut self, name: String) -> Option<&Rc<FunctionBodyRefactored>> {
        self.function_map.get(&name)
    }

    fn get_function_type(&mut self) -> SymbolType {
        match self.sym_tab.get(&mut self.function_name) {
			None => {
				panic!("Invalid State: No Function named {:?} declared.", self.function_name);
			}
			Some(symbol) => {
				symbol.symbol_type
			}
		}
    }
}

impl TreeVisitor for SemanticChecker {
	fn visit_syntax_tree(&mut self, s: &SyntaxTree) -> SyntaxTreeRefactored {
		// TODO
		todo!()
	}
	fn visit_program(&mut self, p: &Program) -> Option<DeclassAssignmentRefactored> {
		// TODO
		todo!()
	}
	fn visit_func_def(&mut self, f: &FunctionDefinition) -> () {
		// TODO
		todo!()
	}
	fn visit_func_body(&mut self, f: &FunctionBody) -> FunctionBodyRefactored {
		// TODO
		todo!()
	}
	fn visit_parameter(&mut self, p: &Parameter) -> SymbolType {
		// TODO
		todo!()
	}
	fn visit_func_call(&mut self, f: &FunctionCall) -> (FunctionCallRefactored, SymbolType) {
		// TODO
		todo!()
	}
	fn visit_statement_list(&mut self, s: &StatementList) -> StatementListRefactored {
		// TODO
		todo!()
	}
	fn visit_block(&mut self, b: &Block) -> BlockRefactored {
		// TODO
		todo!()
	}
	fn visit_statement(&mut self, s: &Statement) -> StatementRefactored {
		// TODO
		todo!()
	}
	fn visit_printf(&mut self, p: &Printf) -> PrintfRefactored {
		// TODO
		todo!()
	}
	fn visit_declass_stat_assignment(&mut self, d: &DeclassStatAssignment) -> DeclassStatAssignmentRefactored {
		// TODO
		todo!()
	}
	fn visit_declass_assignment(&mut self, d: &DeclassAssignment) -> DeclassAssignmentRefactored {
		// TODO
		todo!()
	}
	fn visit_stat_assignment(&mut self, s: &StatAssignment) -> (StatAssignmentRefactored, SymbolType) {
		// TODO
		todo!()
	}
	fn visit_assignment(&mut self, a: &Assignment) -> (AssignmentRefactored, SymbolType) {
		// TODO
		todo!()
	}
	fn visit_expression(&mut self, e: &Expr) -> (ExprRefactored, SymbolType) {
		// TODO
		todo!()
	}
	fn visit_factor(&mut self, f: &Factor) -> (FactorRefactored, SymbolType) {
		// TODO
		todo!()
	}
}