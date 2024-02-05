use crate::structs::refactored_syntax_tree::*;
use crate::structs::refactored_syntax_tree_visitor::*;

// Deklaration der Typen mit Payload
#[derive(Debug, Clone, PartialEq)]
pub enum VariableTypes {
    Void,
    Boolean(bool),
    Integer(i64),
    Float(f64),
    Size(usize),
}

pub struct Interpreter {
	stack: Vec<VariableTypes>,	// Speicherstack
	eax: VariableTypes,			// Ausgaberegister
	ebp: usize,					// Base Pointer
	return_flag: bool,			// Flag, ob eine Return Statement ausgeführt wurde

	ausgabe: Vec<String>,
}

impl Interpreter {
	pub fn new() -> Self {
		Self{
			eax: VariableTypes::Void,
			stack: vec![],
			ebp: 0,
			return_flag: false,

			ausgabe: vec![],
		}
	}

	pub fn run(&mut self, t: & SyntaxTreeRefactored) -> String {
		self.visit_refactored_syntax_tree(t);

		self.ausgabe.join("\n")
	}

	fn push_stack(&mut self, value: VariableTypes) {
		self.stack.push(value.clone());
	}

	fn lookup_in_stack(&mut self, pos: usize) -> VariableTypes {
		if pos < self.stack.len() {
			self.stack[pos].clone()
		}else {
			panic!("Error: Try to access Stackentry outside of bounds. Accessed: {:?} Size: {:?}", pos, self.stack.len());
		}
	}

	fn update_in_stack(&mut self, pos: usize, value: VariableTypes) {
		if pos < self.stack.len() {
			self.stack[pos] = value;
		}else {
			panic!("Error: Try to update Stackentry outside of bounds. Accessed: {:?} Size: {:?}", pos, self.stack.len());
		}
	}
}

impl RefectoredTreeVisitor<VariableTypes> for Interpreter {
	fn visit_refactored_syntax_tree(&mut self, s: &SyntaxTreeRefactored) -> VariableTypes {
		// TODO
		todo!()
	}
	fn visit_refactored_func_body(&mut self, f: &FunctionBodyRefactored) -> VariableTypes {
		// TODO
		todo!()
	}
	fn visit_refactored_func_call(&mut self, f: &FunctionCallRefactored) -> VariableTypes {
		// TODO
		todo!()
	}
	fn visit_refactored_statement_list(&mut self, s: &StatementListRefactored) -> VariableTypes {
		// TODO
		todo!()
	}
	fn visit_refactored_block(&mut self, b: &BlockRefactored) -> VariableTypes {
		// TODO
		todo!()
	}
	fn visit_refactored_statement(&mut self, s: &StatementRefactored) -> VariableTypes {
		// TODO
		todo!()
	}
	fn visit_refactored_printf(&mut self, p: &PrintfRefactored) -> VariableTypes {
		// TODO
		todo!()
	}
	fn visit_refactored_declass_stat_assignment(&mut self, d: &DeclassStatAssignmentRefactored) -> VariableTypes {
		// TODO
		todo!()
	}
	fn visit_refactored_declass_assignment(&mut self, d: &DeclassAssignmentRefactored) -> VariableTypes {
		// TODO
		todo!()	
	}
	fn visit_refactored_stat_asignment(&mut self, s: &StatAssignmentRefactored) -> VariableTypes {
		// TODO
		todo!()
	}
	fn visit_refactored_assignment(&mut self, a: &AssignmentRefactored) -> VariableTypes {
		// TODO
		todo!()
	}
	fn visit_refactored_expression(&mut self, e: &ExprRefactored) -> VariableTypes {
		// TODO
		todo!()
	}
	fn visit_refactored_factor(&mut self, f: &FactorRefactored) -> VariableTypes {
		// TODO
		todo!()
	}
}