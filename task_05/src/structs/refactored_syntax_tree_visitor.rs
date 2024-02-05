use crate::refactored_syntax_tree::*;

pub trait RefectoredTreeVisitor<T> {
	fn visit_refactored_syntax_tree(&mut self, s: &SyntaxTreeRefactored) -> T;
	fn visit_refactored_func_body(&mut self, f: &FunctionBodyRefactored) -> T;
	fn visit_refactored_func_call(&mut self, f: &FunctionCallRefactored) -> T;
	fn visit_refactored_statement_list(&mut self, s: &StatementListRefactored) -> T;
	fn visit_refactored_block(&mut self, b: &BlockRefactored) -> T;
	fn visit_refactored_statement(&mut self, s: &StatementRefactored) -> T;
	fn visit_refactored_printf(&mut self, p: &PrintfRefactored) -> T;
	fn visit_refactored_declass_stat_assignment(&mut self, d: &DeclassStatAssignmentRefactored) -> T;
	fn visit_refactored_declass_assignment(&mut self, d: &DeclassAssignmentRefactored) -> T;
	fn visit_refactored_stat_asignment(&mut self, s: &StatAssignmentRefactored) -> T;
	fn visit_refactored_assignment(&mut self, a: &AssignmentRefactored) -> T;
	fn visit_refactored_expression(&mut self, e: &ExprRefactored) -> T;
	fn visit_refactored_factor(&mut self, f: &FactorRefactored) -> T;
}