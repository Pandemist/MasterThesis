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
	// EAX Register leeren
		self.eax = VariableTypes::Void;

	// Platz für die Globalen Variablen auf den Stack legen
		for global_var in &s.global_var_list {
			self.visit_refactored_declass_assignment(&global_var);
		}

	// Base Pointer des letzten Frames auf den Stack legen
		self.stack.push(VariableTypes::Size(self.ebp));
	// Rücksprungadresse in den EBP speichern
		self.ebp = self.stack.len();

		self.visit_refactored_func_body(&s.main_func_ref);

		if self.stack.len() > s.global_var_list.len() {
			panic!("Invalid State: More Entrys on Stack than allowed");
		}

		VariableTypes::Void
	}
	fn visit_refactored_func_body(&mut self, f: &FunctionBodyRefactored) -> VariableTypes {
	// Statements der Funktion ausführen
		self.visit_refactored_statement_list(&f.body);

	// Stack abräumen
		self.stack = self.stack[..self.ebp].to_vec();

	// EBP wieder auf dem Wert auf dem Stack setzen.
		self.ebp = match self.stack.pop() {
			Some(n) => match n {
				VariableTypes::Size(pnter) => pnter,
				_ => panic!("Stack Error. Top element wasn't a function."),
			},
			None => panic!("Stack Error. No element on the stack."),
		};

	// Return Flag wieder zurück setzen, da die Funktion jetzt zuende ist
		self.return_flag = false;

		VariableTypes::Void
	}
	fn visit_refactored_func_call(&mut self, f: &FunctionCallRefactored) -> VariableTypes {
	// EAX Register leeren
		self.eax = VariableTypes::Void;

	// Parameter auswerten und auflisten
		let mut parameter = vec![];
		for params in &f.func_assigns {
			parameter.push(self.visit_refactored_assignment(&params));
		}

	// Base Pointer des letzten Frames auf den Stack legen
		self.push_stack(VariableTypes::Size(self.ebp));
	// Rücksprungadresse in den EBP speichern
		self.ebp = self.stack.len();
	// Parameter auf den Stack legen
		for param in parameter {
			self.stack.push(param);
		}

	// Funtkionsbody ausführen
		self.visit_refactored_func_body(&f.func_ref);

	// Return Flag wieder zurück setzen, da die Funktion jetzt zuende ist
		self.return_flag = false;

	// EAX zurückgeben
		self.eax.clone()
	}
	fn visit_refactored_statement_list(&mut self, s: &StatementListRefactored) -> VariableTypes {
		for stmt in &s.stmt_list {
			self.visit_refactored_block(&stmt);
			if self.return_flag {
				break;
			}
		}
		VariableTypes::Void
	}
	fn visit_refactored_block(&mut self, b: &BlockRefactored) -> VariableTypes {
		match b {
			BlockRefactored::StatementList(stmt_list) => {
				self.visit_refactored_statement_list(stmt_list)
			},
			BlockRefactored::Statement(stmt) => {
				self.visit_refactored_statement(stmt)
			},
		}
	}
	fn visit_refactored_statement(&mut self, s: &StatementRefactored) -> VariableTypes {
		match s {
			StatementRefactored::IfStatement{assign, if_block, else_block} => {
				let expr_value = match self.visit_refactored_assignment(assign) {
					VariableTypes::Boolean(val) => val,
					_ => panic!("Only Boolean allowed."),
				};

				if expr_value {
					self.visit_refactored_block(if_block);
				}else{
					match else_block {
						Some(block) => {
							self.visit_refactored_block(block);
						},
						None => {},
					}
				}

				VariableTypes::Void
			},
			StatementRefactored::ForStatement{assign, expression, statassign, block} => {
				self.visit_refactored_declass_stat_assignment(assign);
				while match self.visit_refactored_expression(expression) {
					VariableTypes::Boolean(a) => a,
					_ => panic!("Only Boolean allowed."),
				} {
					self.visit_refactored_block(block);
					self.visit_refactored_stat_asignment(statassign);
				}

				VariableTypes::Void
			},
			StatementRefactored::WhileStatement{assign, block} => {
				while match self.visit_refactored_assignment(assign) {
					VariableTypes::Boolean(a) => a,
					_ => panic!("Only Boolean allowed."),
				} {
					self.visit_refactored_block(block);
				}

				VariableTypes::Void
			},
			StatementRefactored::ReturnStatement(opt_return) => {
				match opt_return {
					Some(assign) => {
						let assign_val = self.visit_refactored_assignment(assign);

					// Rückgabe der Funktion ins EAX Register schreiben
						self.eax = assign_val.clone();
					},
					None => {},
				}

				self.return_flag = true;

				VariableTypes::Void
			},
			StatementRefactored::DoWhileStatement{assign, block} => {
				loop {
					self.visit_refactored_block(block);

					if match self.visit_refactored_assignment(assign) {
						VariableTypes::Boolean(a) => a,
						_ => panic!("Only Boolean allowed."),
					} {
						break;
					}
				}

				VariableTypes::Void
			},
			StatementRefactored::Printf(prinf) => {
				self.visit_refactored_printf(prinf)
			},
			StatementRefactored::DeclassAssignment(decl_assign) => {
				self.visit_refactored_declass_assignment(decl_assign)
			},
			StatementRefactored::StatAssignment(stat_assign) => {
				self.visit_refactored_stat_asignment(stat_assign)
			},
			StatementRefactored::FunctionCall(call) => {
				self.visit_refactored_func_call(call)
			},
			StatementRefactored::EmptyStatement => {
				VariableTypes::Void
			},
		}
	}
	fn visit_refactored_printf(&mut self, p: &PrintfRefactored) -> VariableTypes {
		match p {
			PrintfRefactored::PrintString(s) => {
				self.ausgabe.push(s.to_string());
			},
			PrintfRefactored::PrintAssign(assign) => {
				let assign_result = self.visit_refactored_assignment(assign);
			//	println!("val:{:?} of: {:?}", assign_result, assign);
				match assign_result {
					VariableTypes::Boolean(a) => self.ausgabe.push(a.to_string()),
					VariableTypes::Integer(a) => self.ausgabe.push(a.to_string()),
					VariableTypes::Float(a) => self.ausgabe.push(a.to_string()),
				    _ => panic!("Print not supported for this type. Found: {:?}", assign_result),
				}
			},
		}
		VariableTypes::Void	
	}
	fn visit_refactored_declass_stat_assignment(&mut self, d: &DeclassStatAssignmentRefactored) -> VariableTypes {
		match d {
			DeclassStatAssignmentRefactored::DeclassAssignment(decl_assign) => {
				self.visit_refactored_declass_assignment(decl_assign)
			},
			DeclassStatAssignmentRefactored::StatAssignment(stat_assign) => {
				self.visit_refactored_stat_asignment(stat_assign)
			},
		}
	}
	fn visit_refactored_declass_assignment(&mut self, d: &DeclassAssignmentRefactored) -> VariableTypes {
		match d {
			DeclassAssignmentRefactored::Global(opt_assign) => {
				match opt_assign {
					Some(assign) => {
						let assign_result = self.visit_refactored_assignment(assign);
						self.push_stack(assign_result.clone());

						assign_result
					},
					None => {
						self.push_stack(VariableTypes::Void);
						VariableTypes::Void
					},
				}
			},
			DeclassAssignmentRefactored::Local(opt_assign) => {
				match opt_assign {
					Some(assign) => {
						let assign_result = self.visit_refactored_assignment(assign);
						self.push_stack(assign_result.clone());

						assign_result
					},
					None => {
						self.push_stack(VariableTypes::Void);
						VariableTypes::Void
					},
				}
			},
		}		
	}
	fn visit_refactored_stat_asignment(&mut self, s: &StatAssignmentRefactored) -> VariableTypes {
		match s {
			StatAssignmentRefactored::Global{stack_pos, assign} => {
				let assign_result = self.visit_refactored_assignment(assign);
				self.update_in_stack(*stack_pos, assign_result.clone());
				assign_result
			},
			StatAssignmentRefactored::Local{stack_pos, assign} => {
				let assign_result = self.visit_refactored_assignment(assign);
				self.update_in_stack(stack_pos + self.ebp, assign_result.clone());
				assign_result
			},
		}
	}
	fn visit_refactored_assignment(&mut self, a: &AssignmentRefactored) -> VariableTypes {
		match a {
			AssignmentRefactored::Assign(assign) => {
				self.visit_refactored_stat_asignment(assign)
			},
			AssignmentRefactored::IntToFloat(inner) => {
				match self.visit_refactored_assignment(inner) {
					VariableTypes::Integer(i) => {
						VariableTypes::Float(i as f64)
					},
					_ => {
						println!("type: {:?} node: {:?}", self.visit_refactored_assignment(inner), inner);
						panic!("Only Integer allowed.");
					},
				}
			},
			AssignmentRefactored::Expr(expr) => {
				self.visit_refactored_expression(expr)
			},
		}
	}
	fn visit_refactored_expression(&mut self, e: &ExprRefactored) -> VariableTypes {
		match e {
			ExprRefactored::Add{lhs, rhs} => {
				let exc_rhs = self.visit_refactored_expression(rhs);
				let exc_lhs = self.visit_refactored_expression(lhs);

				match (exc_lhs, exc_rhs) {
					(VariableTypes::Integer(val1), VariableTypes::Integer(val2)) => VariableTypes::Integer(val1 + val2),
					(VariableTypes::Float(val1), VariableTypes::Float(val2)) => VariableTypes::Float(val1 + val2),
					_ => panic!("Adition ist für diesen Typen nicht definiert."),
				}
			},
			ExprRefactored::Sub{lhs, rhs} => {
				let exc_rhs = self.visit_refactored_expression(rhs);
				let exc_lhs = self.visit_refactored_expression(lhs);

				match (exc_lhs, exc_rhs) {
					(VariableTypes::Integer(val1), VariableTypes::Integer(val2)) => VariableTypes::Integer(val1 - val2),
					(VariableTypes::Float(val1), VariableTypes::Float(val2)) => VariableTypes::Float(val1 - val2),
					_ => panic!("Subtraktion ist für diesen Typen nicht definiert."),
				}
			},
			ExprRefactored::Mul{lhs, rhs} => {
				let exc_rhs = self.visit_refactored_factor(rhs);
				let exc_lhs = self.visit_refactored_expression(lhs);

				match (exc_lhs, exc_rhs) {
					(VariableTypes::Integer(val1), VariableTypes::Integer(val2)) => VariableTypes::Integer(val1 * val2),
					(VariableTypes::Float(val1), VariableTypes::Float(val2)) => VariableTypes::Float(val1 * val2),
					_ => panic!("Multiplikation ist für diesen Typen nicht definiert."),
				}
			},
			ExprRefactored::Div{lhs, rhs} => {
				let exc_rhs = self.visit_refactored_factor(rhs);
				let exc_lhs = self.visit_refactored_expression(lhs);

				match (exc_lhs, exc_rhs) {
					(VariableTypes::Integer(val1), VariableTypes::Integer(val2)) => VariableTypes::Integer(val1 / val2),
					(VariableTypes::Float(val1), VariableTypes::Float(val2)) => VariableTypes::Float(val1 / val2),
					_ => panic!("Division ist für diesen Typen nicht definiert."),
				}
			},
			ExprRefactored::LogOr{lhs, rhs} => {
				let exc_rhs = self.visit_refactored_expression(rhs);
				let exc_lhs = self.visit_refactored_expression(lhs);

				match (exc_lhs, exc_rhs) {
					(VariableTypes::Boolean(val1), VariableTypes::Boolean(val2)) => VariableTypes::Boolean(val1 || val2),
					_ => panic!("Logisches oder ist für diesen Typen nicht definiert."),
				}
			},
			ExprRefactored::LogAnd{lhs, rhs} => {
				let exc_rhs = self.visit_refactored_factor(rhs);
				let exc_lhs = self.visit_refactored_expression(lhs);

				match (exc_lhs, exc_rhs) {
					(VariableTypes::Boolean(val1), VariableTypes::Boolean(val2)) => VariableTypes::Boolean(val1 && val2),
					_ => panic!("Logisches und ist für diesen Typen nicht definiert."),
				}
			},
			ExprRefactored::Uminus(inner) => {
				match self.visit_refactored_expression(inner) {
					VariableTypes::Float(f) => {
						VariableTypes::Float(f * -1.0)
					},
					VariableTypes::Integer(i) => {
						VariableTypes::Integer(i * -1)
					},
					_ => {panic!("Only Float, Integer allowed");},
				}
			},
			ExprRefactored::Eq{lhs, rhs} => {
				let exc_rhs = self.visit_refactored_expression(rhs);
				let exc_lhs = self.visit_refactored_expression(lhs);

				match (exc_lhs, exc_rhs) {
					(VariableTypes::Integer(val1), VariableTypes::Integer(val2)) => VariableTypes::Boolean(val1 == val2),
					(VariableTypes::Float(val1), VariableTypes::Float(val2)) => VariableTypes::Boolean(val1 == val2),
					(VariableTypes::Boolean(val1), VariableTypes::Boolean(val2)) => VariableTypes::Boolean(val1 == val2),
					_ => panic!("Operator '==' ist für diesen Typen nicht definiert."),
				}},
			ExprRefactored::Neq{lhs, rhs} => {
				let exc_rhs = self.visit_refactored_expression(rhs);
				let exc_lhs = self.visit_refactored_expression(lhs);

				match (exc_lhs, exc_rhs) {
					(VariableTypes::Integer(val1), VariableTypes::Integer(val2)) => VariableTypes::Boolean(val1 != val2),
					(VariableTypes::Float(val1), VariableTypes::Float(val2)) => VariableTypes::Boolean(val1 != val2),
					(VariableTypes::Boolean(val1), VariableTypes::Boolean(val2)) => VariableTypes::Boolean(val1 != val2),
					_ => panic!("Operator '!=' ist für diesen Typen nicht definiert."),
				}},
			ExprRefactored::Grt{lhs, rhs} => {
				let exc_rhs = self.visit_refactored_expression(rhs);
				let exc_lhs = self.visit_refactored_expression(lhs);

				match (exc_lhs, exc_rhs) {
					(VariableTypes::Integer(val1), VariableTypes::Integer(val2)) => VariableTypes::Boolean(val1 > val2),
					(VariableTypes::Float(val1), VariableTypes::Float(val2)) => VariableTypes::Boolean(val1 > val2),
					(VariableTypes::Boolean(val1), VariableTypes::Boolean(val2)) => VariableTypes::Boolean(val1 > val2),
					_ => panic!("Operator '>' ist für diesen Typen nicht definiert."),
				}
			},
			ExprRefactored::Lst{lhs, rhs} => {
				let exc_rhs = self.visit_refactored_expression(rhs);
				let exc_lhs = self.visit_refactored_expression(lhs);

				match (exc_lhs, exc_rhs) {
					(VariableTypes::Integer(val1), VariableTypes::Integer(val2)) => VariableTypes::Boolean(val1 < val2),
					(VariableTypes::Float(val1), VariableTypes::Float(val2)) => VariableTypes::Boolean(val1 < val2),
					(VariableTypes::Boolean(val1), VariableTypes::Boolean(val2)) => VariableTypes::Boolean(val1 < val2),
					_ => panic!("Operator '<' ist für diesen Typen nicht definiert."),
				}
			},
			ExprRefactored::Geq{lhs, rhs} => {
				let exc_rhs = self.visit_refactored_expression(rhs);
				let exc_lhs = self.visit_refactored_expression(lhs);

				match (exc_lhs, exc_rhs) {
					(VariableTypes::Integer(val1), VariableTypes::Integer(val2)) => VariableTypes::Boolean(val1 >= val2),
					(VariableTypes::Float(val1), VariableTypes::Float(val2)) => VariableTypes::Boolean(val1 >= val2),
					(VariableTypes::Boolean(val1), VariableTypes::Boolean(val2)) => VariableTypes::Boolean(val1 >= val2),
					_ => panic!("Operator '>=' ist für diesen Typen nicht definiert."),
				}
			},
			ExprRefactored::Leq{lhs, rhs} => {
				let exc_rhs = self.visit_refactored_expression(rhs);
				let exc_lhs = self.visit_refactored_expression(lhs);

				match (exc_lhs, exc_rhs) {
					(VariableTypes::Integer(val1), VariableTypes::Integer(val2)) => VariableTypes::Boolean(val1 <= val2),
					(VariableTypes::Float(val1), VariableTypes::Float(val2)) => VariableTypes::Boolean(val1 <= val2),
					(VariableTypes::Boolean(val1), VariableTypes::Boolean(val2)) => VariableTypes::Boolean(val1 <= val2),
					_ => panic!("Operator '<=' ist für diesen Typen nicht definiert."),
				}
			},
			ExprRefactored::Factor(factor) => {
				self.visit_refactored_factor(factor)
			},
			ExprRefactored::IntToFloat(inner) => {
				match self.visit_refactored_expression(inner) {
					VariableTypes::Integer(i) => {
						VariableTypes::Float(i as f64)
					},
					_ => {
						panic!("Only Integer allowed.");
					},
				}
			},
		}
	}
	fn visit_refactored_factor(&mut self, f: &FactorRefactored) -> VariableTypes {
		match f {
			FactorRefactored::ConstInt(val) => {
				VariableTypes::Integer(*val)
			},
			FactorRefactored::ConstFloat(val) => {
				VariableTypes::Float(*val)
			},
			FactorRefactored::ConstBool(val) => {
				VariableTypes::Boolean(*val)
			},
			FactorRefactored::FunctionCall(call) => {
				self.visit_refactored_func_call(call)
			},
			FactorRefactored::IdLocalVar(pos) => {
				let a = self.lookup_in_stack(*pos + self.ebp);
				a
			},
			FactorRefactored::IdGlobalVar(pos) => {
				let a = self.lookup_in_stack(*pos);
				a
			},
			FactorRefactored::Assignment(assign) => {
				self.visit_refactored_assignment(assign)
			},
			FactorRefactored::IntToFloat(inner) => {
				match self.visit_refactored_factor(inner) {
					VariableTypes::Integer(i) => {
						VariableTypes::Float(i as f64)
					},
					_ => {
						panic!("Only Integer allowed.");
					},
				}
			}
		}
	}
}