use crate::structs::symbol_type::*;

#[derive(Debug, PartialEq)]
pub struct SyntaxTree {
	pub program: Vec<Program>,
}

#[derive(Debug, PartialEq)]
pub enum Program {
	DeclassAssign(DeclassAssignment),
	FunctionDefinition(FunctionDefinition),
}

#[derive(Debug, PartialEq)]
pub struct FunctionDefinition {
	pub func_type: SymbolType,
	pub func_name: String,
	pub func_param: Vec<Parameter>,
	pub func_body: FunctionBody,
}

#[derive(Debug, PartialEq)]
pub struct FunctionBody {
	pub body: StatementList,
}

#[derive(Debug, PartialEq)]
pub struct Parameter {
	pub typ: SymbolType,
	pub name: String,
}

#[derive(Debug, PartialEq)]
pub struct FunctionCall {
	pub func_name: String,
	pub func_assigns: Vec<Assignment>,
}

#[derive(Debug, PartialEq)]
pub struct StatementList {
	pub stmt_list: Vec<Block>,
}

#[derive(Debug, PartialEq)]
pub enum Block {
	StatementList(StatementList),
	Statement(Box<Statement>),
}

#[derive(Debug, PartialEq)]
pub enum Statement {
	IfStatement{
		assign: Assignment,
		if_block: Block,
		else_block: Option<Block>,
	},
	ForStatement{
		assign: DeclassStatAssignment,
		expression: Expr,
		statassign: StatAssignment,
		block: Block,
	},
	WhileStatement{
		assign: Assignment,
		block: Block,
	},
	ReturnStatement(Option<Assignment>),
	DoWhileStatement{
		assign: Assignment,
		block: Block,
	},
	Printf(Printf),
	DeclassAssignment(DeclassAssignment),
	StatAssignment(StatAssignment),
	FunctionCall(FunctionCall),
    EmptyStatement,
}

#[derive(Debug, PartialEq)]
pub enum Printf {
	PrintString(String),
	PrintAssign(Assignment),
}

#[derive(Debug, PartialEq)]
pub enum DeclassStatAssignment {
	DeclassAssignment(DeclassAssignment),
	StatAssignment(StatAssignment),
}

#[derive(Debug, PartialEq)]
pub struct DeclassAssignment {
	pub typ: SymbolType,
	pub name: String,
	pub assign: Option<Assignment>,
}

#[derive(Debug, PartialEq)]
pub struct StatAssignment {
	pub name: String,
	pub assign: Assignment,
}

#[derive(Debug, PartialEq)]
pub enum Assignment {
	Assign{
		id: String,
		assign: Box<Assignment>,
	},
	Expr(Box<Expr>),
}

#[derive(Debug, PartialEq)]
pub enum Expr {
	Add{lhs: Box<Expr>,
		rhs: Box<Expr>,
	},
	Sub{lhs: Box<Expr>,
		rhs: Box<Expr>,
	},
	Mul{lhs: Box<Expr>,
		rhs: Factor,
	},
	Div{lhs: Box<Expr>,
		rhs: Factor,
	},
	LogOr{lhs: Box<Expr>,
		rhs: Box<Expr>,
	},
	LogAnd{lhs: Box<Expr>,
		rhs: Factor,
	},
	Uminus(Box<Expr>),
	Eq{lhs: Box<Expr>,
		rhs: Box<Expr>,
	},
	Neq{lhs: Box<Expr>,
		rhs: Box<Expr>,
	},
	Grt{lhs: Box<Expr>,
		rhs: Box<Expr>,
	},
	Lst{lhs: Box<Expr>,
		rhs: Box<Expr>,
	},
	Geq{lhs: Box<Expr>,
		rhs: Box<Expr>,
	},
	Leq{lhs: Box<Expr>,
		rhs: Box<Expr>,
	},
	Factor(Box<Factor>),
}

#[derive(Debug, PartialEq)]
pub enum Factor {
	ConstInt(i64),
	ConstFloat(f64),
	ConstBool(bool),
	FunctionCall(FunctionCall),
	Id(String),
	Assignment(Assignment),
}