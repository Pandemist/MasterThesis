use crate::syntax_tree::*;

use crate::structs::symbol_type::*;

use crate::refactored_syntax_tree::*;

pub trait TreeVisitor {
	fn visit_syntax_tree(&mut self, s: &SyntaxTree) -> SyntaxTreeRefactored;
	fn visit_program(&mut self, p: &Program) -> Option<DeclassAssignmentRefactored>;
	fn visit_func_def(&mut self, f: &FunctionDefinition) -> ();
	fn visit_func_body(&mut self, f: &FunctionBody) -> FunctionBodyRefactored;
	fn visit_parameter(&mut self, p: &Parameter) -> SymbolType;
	fn visit_func_call(&mut self, f: &FunctionCall) -> (FunctionCallRefactored, SymbolType);
	fn visit_statement_list(&mut self, s: &StatementList) -> StatementListRefactored;
	fn visit_block(&mut self, b: &Block) -> BlockRefactored;
	fn visit_statement(&mut self, s: &Statement) -> StatementRefactored;
	fn visit_printf(&mut self, p: &Printf) -> PrintfRefactored;
	fn visit_declass_stat_assignment(&mut self, d: &DeclassStatAssignment) -> DeclassStatAssignmentRefactored;
	fn visit_declass_assignment(&mut self, d: &DeclassAssignment) -> DeclassAssignmentRefactored;
	fn visit_stat_assignment(&mut self, s: &StatAssignment) -> (StatAssignmentRefactored, SymbolType);
	fn visit_assignment(&mut self, a: &Assignment) -> (AssignmentRefactored, SymbolType);
	fn visit_expression(&mut self, e: &Expr) -> (ExprRefactored, SymbolType);
	fn visit_factor(&mut self, f: &Factor) -> (FactorRefactored, SymbolType);
}