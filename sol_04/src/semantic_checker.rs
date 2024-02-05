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
		let mut global_vars = vec![];

		for prog_part in &s.program {
			match self.visit_program(&prog_part) {
				Some(var) => {
					global_vars.push(var);
				},
				None => {},
			}
		}

		let main_ref = match self.get_function_ref("main".to_string()) {
			None => {
				panic!("Not main function defined.");
			},
			Some(n) => n.clone(),
		};
		SyntaxTreeRefactored{
			main_func_ref: main_ref,
			global_var_list: global_vars,
		}
	}
	fn visit_program(&mut self, p: &Program) -> Option<DeclassAssignmentRefactored> {
		match p {
			Program::DeclassAssign(decl_assign) => {
				Some(self.visit_declass_assignment(decl_assign))
			},
			Program::FunctionDefinition(func) => {
				self.visit_func_def(func);
				None
			},
		}
	}
	fn visit_func_def(&mut self, f: &FunctionDefinition) -> () {
		let typ = &f.func_type;
		let id = &f.func_name;
		let param_list = &f.func_param;
		let func_body = &f.func_body;

		match self.sym_tab.insert_function(id.to_string(), *typ) {
			Err(error) => {
				panic!("{:?}", error);
			}
			Ok(_) => {}
		}

		self.function_name = id.clone();

		self.sym_tab.enter_scope();

		let mut param_list_types = vec![];
		for param in param_list {
			param_list_types.push(self.visit_parameter(&param));
		}

		let block_tree = self.visit_func_body(func_body);

		self.sym_tab.leave_scope();

		self.sym_tab.append_parameter_typs_to_function_by_name(id.to_string(), &mut param_list_types);
		self.insert_function_ref(id.to_string(), block_tree.clone());
	}
	fn visit_func_body(&mut self, f: &FunctionBody) -> FunctionBodyRefactored {
		FunctionBodyRefactored{body: self.visit_statement_list(&f.body)}
	}
	fn visit_parameter(&mut self, p: &Parameter) -> SymbolType {
		let typ = &p.typ;
		let name = &p.name;

		match self.sym_tab.insert_parameter(name.to_string(), *typ) {
			Err(error) => {
				panic!("{:?}", error);
			}
			Ok(_) => {}
		}

		if *typ == SymbolType::Void {
			panic!("Void not allowed as parameter type for parameter: {:?}", name);
		}
		*typ
	}
	fn visit_func_call(&mut self, f: &FunctionCall) -> (FunctionCallRefactored, SymbolType) {
		let id = &f.func_name;
		let assign_list = &f.func_assigns;

		let mut tree_list = vec![];
		let mut typ_list = vec![];
		for assign in assign_list {
			let (assign_tree, assign_type) = self.visit_assignment(assign);
			tree_list.push(assign_tree);
			typ_list.push(assign_type);
		}

		let func_typ = match self.sym_tab.get(&id) {
			Some(symbol) => {
				match &symbol.symbol_class {
					SymbolClass::Function { parameters } => {

						if parameters.len() > assign_list.len() {
							panic!("Not enough arguments for call of: {:?}", id);
						}
						if parameters.len() < assign_list.len() {
							panic!("Too many arguments for call of: {:?}", id);
						}

						for (param, typ) in parameters.iter().zip(typ_list.iter()) {
							if param != typ {
								panic!("Typemissmatch in functioncall: {:?}. Expected {:?}, found: {:?}", id, param, typ);
							}
						}
						symbol.symbol_type
					},
					_ => {
						panic!("cannot call a variable, function with name {:?} expected", id);
					},
				}
			},
			None => {
				panic!("Function {:?} not found!", id);
			},
		};

		let node_ref = match self.get_function_ref(id.to_string()) {
			None => {
				panic!("Funciton {:?} not found.", id);
			},
			Some(n) => n.clone(),
		};

		let new_func_call = FunctionCallRefactored{
			func_assigns: tree_list,
			func_ref: node_ref,
		};

		(new_func_call, func_typ)
	}
	fn visit_statement_list(&mut self, s: &StatementList) -> StatementListRefactored {
		let mut tree_list = vec![];
		for block in &s.stmt_list {
			tree_list.push(self.visit_block(&block))
		}

		StatementListRefactored {
			stmt_list: tree_list,
		}
	}
	fn visit_block(&mut self, b: &Block) -> BlockRefactored {
		match b {
			Block::StatementList(stmt_list) => {
				BlockRefactored::StatementList(self.visit_statement_list(stmt_list))
			},
			Block::Statement(stmt) => {
				BlockRefactored::Statement(Box::new(self.visit_statement(stmt)))
			}
		}
	}
	fn visit_statement(&mut self, s: &Statement) -> StatementRefactored {
		match s {
			Statement::IfStatement{assign, if_block, else_block} => {
				let (refactored_assign, _) = self.visit_assignment(assign);

				let refactored_if_block = self.visit_block(if_block);

				let refactored_else_block = match else_block {
					Some(block) => {
						Some(self.visit_block(block))
					},
					None => {None},
				};
				StatementRefactored::IfStatement{
					assign: refactored_assign, 
					if_block: refactored_if_block, 
					else_block: refactored_else_block,
				}
			},
			Statement::ForStatement{assign, expression, statassign, block} => {
				self.sym_tab.enter_scope();

				let refeactored_assign = self.visit_declass_stat_assignment(assign);
				let (refeactored_expr, _) = self.visit_expression(expression);
				let (refeactored_stat_assign, _) = self.visit_stat_assignment(statassign);

				let refactored_block = self.visit_block(block);

				self.sym_tab.leave_scope();

				StatementRefactored::ForStatement{
					assign: refeactored_assign,
					expression: refeactored_expr,
					statassign: refeactored_stat_assign,
					block: refactored_block,
				}
			},
			Statement::WhileStatement{assign, block} => {
				let (refactored_assign, _) = self.visit_assignment(assign);

				self.sym_tab.enter_scope();

				let refactored_block = self.visit_block(block);

				self.sym_tab.leave_scope();

				StatementRefactored::WhileStatement{
					assign: refactored_assign,
					block: refactored_block,
				}
			},
			Statement::ReturnStatement(assign) => {
				let function_type = self.get_function_type();
				match assign {
					Some(a) => {
						let (mut refactored_assign, assign_type) = self.visit_assignment(a);

						// Wenn nötig von float zu Int konvertieren
						if (function_type == SymbolType::Float) && (assign_type == SymbolType::Integer) {
							refactored_assign = AssignmentRefactored::IntToFloat(Box::new(refactored_assign));
						}else{
							if function_type != assign_type {
								panic!("Typemissmatch: {:?} & {:?} does not match. For return.", function_type, assign_type);
							}
						}

						StatementRefactored::ReturnStatement(Some(refactored_assign))
					},
					None => {
						if function_type != SymbolType::Void {
							panic!("Typemissmatch: {:?} & Void does not match. For return.", function_type);
						}

						StatementRefactored::ReturnStatement(None)
					},
				}
			},
			Statement::DoWhileStatement{assign, block} => {
				let (refactored_assign, _) = self.visit_assignment(assign);

				self.sym_tab.enter_scope();

				let refactored_block = self.visit_block(block);

				self.sym_tab.leave_scope();

				StatementRefactored::DoWhileStatement{
					assign: refactored_assign,
					block: refactored_block,
				}
			},
			Statement::Printf(printf) => {
				StatementRefactored::Printf(self.visit_printf(printf))
			},
			Statement::DeclassAssignment(decl_assign) => {
				StatementRefactored::DeclassAssignment(self.visit_declass_assignment(decl_assign))
			},
			Statement::StatAssignment(stat_assign) => {
				let (assign_tree, _) = self.visit_stat_assignment(stat_assign);
				StatementRefactored::StatAssignment(assign_tree)
			},
			Statement::FunctionCall(fun_call) => {
				let (call_tree, _) = self.visit_func_call(fun_call);
				StatementRefactored::FunctionCall(call_tree)
			},
			Statement::EmptyStatement => {
				StatementRefactored::EmptyStatement
			},
		}
	}
	fn visit_printf(&mut self, p: &Printf) -> PrintfRefactored {
		match p {
			Printf::PrintAssign(assign) => {
				let (refactored_assign, assign_type) = self.visit_assignment(assign);

				match assign_type {
					SymbolType::Void => {
						panic!("Printf is not defined for Void")
					},
					_ => {},
				}

				PrintfRefactored::PrintAssign(refactored_assign)
			},
			Printf::PrintString(s) => {
				PrintfRefactored::PrintString(s.clone())
			},
		}
		
	}
	fn visit_declass_stat_assignment(&mut self, d: &DeclassStatAssignment) -> DeclassStatAssignmentRefactored {
		match d {
			DeclassStatAssignment::DeclassAssignment(inner) => {
				DeclassStatAssignmentRefactored::DeclassAssignment(self.visit_declass_assignment(inner))
			},
			DeclassStatAssignment::StatAssignment(inner) => {
				let (stat_tree, _) = self.visit_stat_assignment(inner);
				DeclassStatAssignmentRefactored::StatAssignment(stat_tree)
			},
		}
	}
	fn visit_declass_assignment(&mut self, d: &DeclassAssignment) -> DeclassAssignmentRefactored {
		let symbol = match self.sym_tab.insert_variable(d.name.to_string(), d.typ) {
			Err(error) => {
				panic!("{:?}", error);
			}
			Ok(n) => n,
		};

		let refactored_assign = match &d.assign {
			Some(a) => {
				let (new_assign, assign_type) = self.visit_assignment(&a);

				if assign_type != d.typ {
					panic!("Typemissmatch: {:?} & {:?} does not match. For Variable Declaration of {:?}", assign_type, d.typ, d.name);
				}

				Some(new_assign)
			},
			None => {
				None
			},
		};

		if symbol.is_global {
			DeclassAssignmentRefactored::Global(refactored_assign)
		}else{
			DeclassAssignmentRefactored::Local(refactored_assign)
		}
	}
	fn visit_stat_assignment(&mut self, s: &StatAssignment) -> (StatAssignmentRefactored, SymbolType) {
		let (mut refactored_assign, assign_type) = self.visit_assignment(&s.assign);

		let (glob_loc_node, var_pos, var_type) = match self.sym_tab.get(&s.name) {
			None => {
				panic!("Function {:?} not found!", s.name);
			},
			Some(symbol) => {
				match symbol.symbol_class {
					SymbolClass::Parameter |
					SymbolClass::Variable => {
						(symbol.is_global, symbol.pos, symbol.symbol_type)
					},
					_ => {
						panic!("cannot call a variable, function with name {:?} expected", s.name);
					},
				}
			}
		};

		match (assign_type, var_type) {
			(SymbolType::Integer, SymbolType::Integer) => {},
			(SymbolType::Integer, SymbolType::Float) => {
				refactored_assign = AssignmentRefactored::IntToFloat(Box::new(refactored_assign));
			},
			(SymbolType::Float, SymbolType::Float) => {},
			(SymbolType::Boolean, SymbolType::Boolean) => {},
			_ => panic!("Typemissmatch: {:?} & {:?} does not match. For Variable Declaration of {:?}", assign_type, var_type, s.name),
		}

		let refactored_stat_assign = if glob_loc_node {
			StatAssignmentRefactored::Global{
				stack_pos: var_pos,
				assign: Box::new(refactored_assign),
			}
		}else{
			StatAssignmentRefactored::Local{
				stack_pos: var_pos,
				assign: Box::new(refactored_assign),
			}
		};

		(refactored_stat_assign, var_type)
	}
	fn visit_assignment(&mut self, a: &Assignment) -> (AssignmentRefactored, SymbolType) {
		match a {
			Assignment::Assign{
				id,
				assign,
			} => {
				let (mut refactored_assign, mut assign_type) = self.visit_assignment(assign);

				let (glob_loc_node, var_pos, var_type) = match self.sym_tab.get(&id) {
					None => {
						panic!("Function {:?} not found!", id);
					},
					Some(symbol) => {
						match symbol.symbol_class {
							SymbolClass::Parameter |
							SymbolClass::Variable => {
								(symbol.is_global, symbol.pos, symbol.symbol_type)
							},
							_ => {
								panic!("cannot call a variable, function with name {:?} expected", id);
							},
						}
					}
				};

				match (assign_type, var_type) {
					(SymbolType::Integer, SymbolType::Integer) => {},
					(SymbolType::Integer, SymbolType::Float) => {
						refactored_assign = AssignmentRefactored::IntToFloat(Box::new(refactored_assign));
						assign_type = SymbolType::Float;
					},
					(SymbolType::Float, SymbolType::Float) => {},
					(SymbolType::Boolean, SymbolType::Boolean) => {},
					_ => panic!("Typemissmatch: {:?} & {:?} does not match. For Variable Declaration of {:?}", assign_type, var_type, id),
				}

				let refactored_stat_assign = if glob_loc_node {
					StatAssignmentRefactored::Global{
						stack_pos: var_pos,
						assign: Box::new(refactored_assign),
					}
				}else{
					StatAssignmentRefactored::Local{
						stack_pos: var_pos,
						assign: Box::new(refactored_assign),
					}
				};

				(AssignmentRefactored::Assign(refactored_stat_assign), assign_type)
			},
			Assignment::Expr(e) => {
				let (refactored_expr, expr_type) = self.visit_expression(e);
				(AssignmentRefactored::Expr(Box::new(refactored_expr)), expr_type)
			},
		}
	}
	fn visit_expression(&mut self, e: &Expr) -> (ExprRefactored, SymbolType) {
		match e {
			Expr::Add{lhs, rhs} => {
				let (lhs_tree, lhs_type) = self.visit_expression(lhs);
				let (rhs_tree, rhs_type) = self.visit_expression(rhs);

				let (new_lhs_tree, new_rhs_tree, typ) = match (lhs_type, rhs_type) {
					(SymbolType::Integer, SymbolType::Integer) => {(lhs_tree, rhs_tree, SymbolType::Integer)},
					(SymbolType::Integer, SymbolType::Float) => {
						(ExprRefactored::IntToFloat(Box::new(lhs_tree)), rhs_tree, SymbolType::Float)
					},
					(SymbolType::Float, SymbolType::Integer) => {
						(lhs_tree, ExprRefactored::IntToFloat(Box::new(rhs_tree)), SymbolType::Float)
					},
					(SymbolType::Float, SymbolType::Float) => {(lhs_tree, rhs_tree, SymbolType::Float)},
					_ => panic!("Typemissmatch: {:?} & {:?} does not match. For + operator", lhs_type, rhs_type)
				};

				(ExprRefactored::Add{
					lhs: Box::new(new_lhs_tree), 
					rhs: Box::new(new_rhs_tree),
				},typ)
			},
			Expr::Sub{lhs, rhs} => {
				let (lhs_tree, lhs_type) = self.visit_expression(lhs);
				let (rhs_tree, rhs_type) = self.visit_expression(rhs);

				let (new_lhs_tree, new_rhs_tree, typ) = match (lhs_type, rhs_type) {
					(SymbolType::Integer, SymbolType::Integer) => {(lhs_tree, rhs_tree, SymbolType::Integer)},
					(SymbolType::Integer, SymbolType::Float) => {
						(ExprRefactored::IntToFloat(Box::new(lhs_tree)), rhs_tree, SymbolType::Float)
					},
					(SymbolType::Float, SymbolType::Integer) => {
						(lhs_tree, ExprRefactored::IntToFloat(Box::new(rhs_tree)), SymbolType::Float)
					},
					(SymbolType::Float, SymbolType::Float) => {(lhs_tree, rhs_tree, SymbolType::Float)},
					_ => panic!("Typemissmatch: {:?} & {:?} does not match. For - operator", lhs_type, rhs_type)
				};

				(ExprRefactored::Sub{
					lhs: Box::new(new_lhs_tree), 
					rhs: Box::new(new_rhs_tree),
				},typ)
			},
			Expr::Mul{lhs, rhs} => {
				let (lhs_tree, lhs_type) = self.visit_expression(lhs);
				let (rhs_tree, rhs_type) = self.visit_factor(rhs);

				let (new_lhs_tree, new_rhs_tree, typ) = match (lhs_type, rhs_type) {
					(SymbolType::Integer, SymbolType::Integer) => {(lhs_tree, rhs_tree, SymbolType::Integer)},
					(SymbolType::Integer, SymbolType::Float) => {
						(ExprRefactored::IntToFloat(Box::new(lhs_tree)), rhs_tree, SymbolType::Float)
					},
					(SymbolType::Float, SymbolType::Integer) => {
						(lhs_tree, FactorRefactored::IntToFloat(Box::new(rhs_tree)), SymbolType::Float)
					},
					(SymbolType::Float, SymbolType::Float) => {(lhs_tree, rhs_tree, SymbolType::Float)},
					_ => panic!("Typemissmatch: {:?} & {:?} does not match. For * operator", lhs_type, rhs_type)
				};

				(ExprRefactored::Mul{
					lhs: Box::new(new_lhs_tree), 
					rhs: *Box::new(new_rhs_tree),
				},typ)
			},
			Expr::Div{lhs, rhs} => {
				let (lhs_tree, lhs_type) = self.visit_expression(lhs);
				let (rhs_tree, rhs_type) = self.visit_factor(rhs);

				let (new_lhs_tree, new_rhs_tree, typ) = match (lhs_type, rhs_type) {
					(SymbolType::Integer, SymbolType::Integer) => {(lhs_tree, rhs_tree, SymbolType::Integer)},
					(SymbolType::Integer, SymbolType::Float) => {
						(ExprRefactored::IntToFloat(Box::new(lhs_tree)), rhs_tree, SymbolType::Float)
					},
					(SymbolType::Float, SymbolType::Integer) => {
						(lhs_tree, FactorRefactored::IntToFloat(Box::new(rhs_tree)), SymbolType::Float)
					},
					(SymbolType::Float, SymbolType::Float) => {(lhs_tree, rhs_tree, SymbolType::Float)},
					_ => panic!("Typemissmatch: {:?} & {:?} does not match. For / operator", lhs_type, rhs_type)
				};

				(ExprRefactored::Div{
					lhs: Box::new(new_lhs_tree), 
					rhs: *Box::new(new_rhs_tree),
				},typ)
			},
			Expr::LogOr{lhs, rhs} => {
				let (lhs_tree, lhs_type) = self.visit_expression(lhs);
				let (rhs_tree, rhs_type) = self.visit_expression(rhs);

				if (rhs_type != SymbolType::Boolean) && (lhs_type != SymbolType::Boolean) {
					panic!("Typemissmatch: {:?} & {:?} does not match. For operator: ||", lhs_type, rhs_type);
				}

				(ExprRefactored::LogOr{
					lhs: Box::new(lhs_tree), 
					rhs: Box::new(rhs_tree),
				},lhs_type)
			},
			Expr::LogAnd{lhs, rhs} => {
				let (lhs_tree, lhs_type) = self.visit_expression(lhs);
				let (rhs_tree, rhs_type) = self.visit_factor(rhs);

				if (rhs_type != SymbolType::Boolean) && (lhs_type != SymbolType::Boolean) {
					panic!("Typemissmatch: {:?} & {:?} does not match. For operator: &&", lhs_type, rhs_type);
				}

				(ExprRefactored::LogAnd{
					lhs: Box::new(lhs_tree), 
					rhs: *Box::new(rhs_tree),
				},lhs_type)
			},
			Expr::Uminus(inner) => {
				let (refactored_expression, expression_typ) = self.visit_expression(inner);
				(ExprRefactored::Uminus(Box::new(refactored_expression)), expression_typ)
			},
			Expr::Eq{lhs, rhs} => {
				let (lhs_tree, lhs_type) = self.visit_expression(lhs);
				let (rhs_tree, rhs_type) = self.visit_expression(rhs);

				let (new_lhs_tree, new_rhs_tree) = match (lhs_type, rhs_type) {
					(SymbolType::Integer, SymbolType::Integer) => {(lhs_tree, rhs_tree)},
					(SymbolType::Integer, SymbolType::Float) => {
						(ExprRefactored::IntToFloat(Box::new(lhs_tree)), rhs_tree)
					},
					(SymbolType::Float, SymbolType::Integer) => {
						(lhs_tree, ExprRefactored::IntToFloat(Box::new(rhs_tree)))
					},
					(SymbolType::Float, SymbolType::Float) => {(lhs_tree, rhs_tree)},
					(SymbolType::Boolean, SymbolType::Boolean) => {(lhs_tree, rhs_tree)},
					_ => panic!("Typemissmatch: {:?} & {:?} does not match. For operator: <", lhs_type, rhs_type)
				};

				(ExprRefactored::Eq{
					lhs: Box::new(new_lhs_tree), 
					rhs: Box::new(new_rhs_tree),
				},SymbolType::Boolean)
			},
			Expr::Neq{lhs, rhs} => {
				let (lhs_tree, lhs_type) = self.visit_expression(lhs);
				let (rhs_tree, rhs_type) = self.visit_expression(rhs);

				let (new_lhs_tree, new_rhs_tree) = match (lhs_type, rhs_type) {
					(SymbolType::Integer, SymbolType::Integer) => {(lhs_tree, rhs_tree)},
					(SymbolType::Integer, SymbolType::Float) => {
						(ExprRefactored::IntToFloat(Box::new(lhs_tree)), rhs_tree)
					},
					(SymbolType::Float, SymbolType::Integer) => {
						(lhs_tree, ExprRefactored::IntToFloat(Box::new(rhs_tree)))
					},
					(SymbolType::Float, SymbolType::Float) => {(lhs_tree, rhs_tree)},
					(SymbolType::Boolean, SymbolType::Boolean) => {(lhs_tree, rhs_tree)},
					_ => panic!("Typemissmatch: {:?} & {:?} does not match. For operator: <", lhs_type, rhs_type)
				};

				(ExprRefactored::Neq{
					lhs: Box::new(new_lhs_tree), 
					rhs: Box::new(new_rhs_tree),
				},SymbolType::Boolean)
			},
			Expr::Grt{lhs, rhs} => {
				let (lhs_tree, lhs_type) = self.visit_expression(lhs);
				let (rhs_tree, rhs_type) = self.visit_expression(rhs);

				let (new_lhs_tree, new_rhs_tree) = match (lhs_type, rhs_type) {
					(SymbolType::Integer, SymbolType::Integer) => {(lhs_tree, rhs_tree)},
					(SymbolType::Integer, SymbolType::Float) => {
						(ExprRefactored::IntToFloat(Box::new(lhs_tree)), rhs_tree)
					},
					(SymbolType::Float, SymbolType::Integer) => {
						(lhs_tree, ExprRefactored::IntToFloat(Box::new(rhs_tree)))
					},
					(SymbolType::Float, SymbolType::Float) => {(lhs_tree, rhs_tree)},
					(SymbolType::Boolean, SymbolType::Boolean) => {(lhs_tree, rhs_tree)},
					_ => panic!("Typemissmatch: {:?} & {:?} does not match. For operator: <", lhs_type, rhs_type)
				};

				(ExprRefactored::Grt{
					lhs: Box::new(new_lhs_tree), 
					rhs: Box::new(new_rhs_tree),
				},SymbolType::Boolean)
			},
			Expr::Lst{lhs, rhs} => {
				let (lhs_tree, lhs_type) = self.visit_expression(lhs);
				let (rhs_tree, rhs_type) = self.visit_expression(rhs);

				let (new_lhs_tree, new_rhs_tree) = match (lhs_type, rhs_type) {
					(SymbolType::Integer, SymbolType::Integer) => {(lhs_tree, rhs_tree)},
					(SymbolType::Integer, SymbolType::Float) => {
						(ExprRefactored::IntToFloat(Box::new(lhs_tree)), rhs_tree)
					},
					(SymbolType::Float, SymbolType::Integer) => {
						(lhs_tree, ExprRefactored::IntToFloat(Box::new(rhs_tree)))
					},
					(SymbolType::Float, SymbolType::Float) => {(lhs_tree, rhs_tree)},
					(SymbolType::Boolean, SymbolType::Boolean) => {(lhs_tree, rhs_tree)},
					_ => panic!("Typemissmatch: {:?} & {:?} does not match. For operator: <", lhs_type, rhs_type)
				};

				(ExprRefactored::Lst{
					lhs: Box::new(new_lhs_tree), 
					rhs: Box::new(new_rhs_tree),
				},SymbolType::Boolean)
			},
			Expr::Geq{lhs, rhs} => {
				let (lhs_tree, lhs_type) = self.visit_expression(lhs);
				let (rhs_tree, rhs_type) = self.visit_expression(rhs);

				let (new_lhs_tree, new_rhs_tree) = match (lhs_type, rhs_type) {
					(SymbolType::Integer, SymbolType::Integer) => {(lhs_tree, rhs_tree)},
					(SymbolType::Integer, SymbolType::Float) => {
						(ExprRefactored::IntToFloat(Box::new(lhs_tree)), rhs_tree)
					},
					(SymbolType::Float, SymbolType::Integer) => {
						(lhs_tree, ExprRefactored::IntToFloat(Box::new(rhs_tree)))
					},
					(SymbolType::Float, SymbolType::Float) => {(lhs_tree, rhs_tree)},
					(SymbolType::Boolean, SymbolType::Boolean) => {(lhs_tree, rhs_tree)},
					_ => panic!("Typemissmatch: {:?} & {:?} does not match. For operator: <", lhs_type, rhs_type)
				};

				(ExprRefactored::Geq{
					lhs: Box::new(new_lhs_tree), 
					rhs: Box::new(new_rhs_tree),
				},SymbolType::Boolean)
			},
			Expr::Leq{lhs, rhs} => {
				let (lhs_tree, lhs_type) = self.visit_expression(lhs);
				let (rhs_tree, rhs_type) = self.visit_expression(rhs);

				let (new_lhs_tree, new_rhs_tree) = match (lhs_type, rhs_type) {
					(SymbolType::Integer, SymbolType::Integer) => {(lhs_tree, rhs_tree)},
					(SymbolType::Integer, SymbolType::Float) => {
						(ExprRefactored::IntToFloat(Box::new(lhs_tree)), rhs_tree)
					},
					(SymbolType::Float, SymbolType::Integer) => {
						(lhs_tree, ExprRefactored::IntToFloat(Box::new(rhs_tree)))
					},
					(SymbolType::Float, SymbolType::Float) => {(lhs_tree, rhs_tree)},
					(SymbolType::Boolean, SymbolType::Boolean) => {(lhs_tree, rhs_tree)},
					_ => panic!("Typemissmatch: {:?} & {:?} does not match. For operator: <", lhs_type, rhs_type)
				};

				(ExprRefactored::Leq{
					lhs: Box::new(new_lhs_tree), 
					rhs: Box::new(new_rhs_tree),
				},SymbolType::Boolean)
			},
			Expr::Factor(f) => {
				let (refactored_factor, factor_typ) = self.visit_factor(f);
				(ExprRefactored::Factor(Box::new(refactored_factor)), factor_typ)
			}
		}
	}
	fn visit_factor(&mut self, f: &Factor) -> (FactorRefactored, SymbolType) {
		match f {
			Factor::ConstInt(val) => {
				(FactorRefactored::ConstInt(*val), SymbolType::Integer)
			},
			Factor::ConstFloat(val) => {
				(FactorRefactored::ConstFloat(*val), SymbolType::Float)
			},
			Factor::ConstBool(val) => {
				(FactorRefactored::ConstBool(*val), SymbolType::Boolean)
			},
			Factor::FunctionCall(fun) => {
				let (refactored_func_call, func_call_typ) = self.visit_func_call(fun);
				(FactorRefactored::FunctionCall(refactored_func_call), func_call_typ)
				
			},
			Factor::Id(id) => {
				let (var_node, var_type) = match self.sym_tab.get(&id) {
					None => {
						panic!("Undeclared Symbol {}", id);
					}
					Some(symbol) => {
						match symbol.symbol_class {
							SymbolClass::Parameter |
						    SymbolClass::Variable => {
								let node = if symbol.is_global {
									FactorRefactored::IdGlobalVar(symbol.pos)
								}else{
									FactorRefactored::IdLocalVar(symbol.pos)
								};
								(node, symbol.symbol_type)
						    },
							_ => {
								panic!("Error: Called value is no variable: {}", id);
							},
						}
					}
				};
				(var_node, var_type)
			},
			Factor::Assignment(assign) => {
				let (refactored_assign, assign_typ) = self.visit_assignment(assign);
				(FactorRefactored::Assignment(refactored_assign), assign_typ)
			},
		}
	}
}