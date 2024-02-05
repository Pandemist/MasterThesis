
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct SyntaxTreeRefactored {
	pub main_func_ref: Rc<FunctionBodyRefactored>,
	pub global_var_list: Vec<DeclassAssignmentRefactored>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionBodyRefactored {
	pub body: StatementListRefactored,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionCallRefactored {
	pub func_assigns: Vec<AssignmentRefactored>,
	pub func_ref: Rc<FunctionBodyRefactored>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StatementListRefactored {
	pub stmt_list: Vec<BlockRefactored>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum BlockRefactored {
	StatementList(StatementListRefactored),
	Statement(Box<StatementRefactored>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum StatementRefactored {
	IfStatement{
		assign: AssignmentRefactored,
		if_block: BlockRefactored,
		else_block: Option<BlockRefactored>,
	},
	ForStatement{
		assign: DeclassStatAssignmentRefactored,
		expression: ExprRefactored,
		statassign: StatAssignmentRefactored,
		block: BlockRefactored,
	},
	WhileStatement{
		assign: AssignmentRefactored,
		block: BlockRefactored,
	},
	ReturnStatement(Option<AssignmentRefactored>),
	DoWhileStatement{
		assign: AssignmentRefactored,
		block: BlockRefactored,
	},
	Printf(PrintfRefactored),
	DeclassAssignment(DeclassAssignmentRefactored),
	StatAssignment(StatAssignmentRefactored),
	FunctionCall(FunctionCallRefactored),
    EmptyStatement,
}

#[derive(Clone, Debug, PartialEq)]
pub enum PrintfRefactored {
	PrintString(String),
	PrintAssign(AssignmentRefactored),
}

#[derive(Clone, Debug, PartialEq)]
pub enum DeclassStatAssignmentRefactored {
	DeclassAssignment(DeclassAssignmentRefactored),
	StatAssignment(StatAssignmentRefactored),
}

#[derive(Clone, Debug, PartialEq)]
pub enum DeclassAssignmentRefactored {
	Global(Option<AssignmentRefactored>),
	Local(Option<AssignmentRefactored>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum StatAssignmentRefactored {
	Global{
		stack_pos: usize,
		assign: Box<AssignmentRefactored>,
	},
	Local{
		stack_pos: usize,
		assign: Box<AssignmentRefactored>,
	},
}

#[derive(Clone, Debug, PartialEq)]
pub enum AssignmentRefactored {
	Assign(StatAssignmentRefactored),
	IntToFloat(Box<AssignmentRefactored>),
	Expr(Box<ExprRefactored>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExprRefactored {
	Add{lhs: Box<ExprRefactored>,
		rhs: Box<ExprRefactored>,
	},
	Sub{lhs: Box<ExprRefactored>,
		rhs: Box<ExprRefactored>,
	},
	Mul{lhs: Box<ExprRefactored>,
		rhs: FactorRefactored,
	},
	Div{lhs: Box<ExprRefactored>,
		rhs: FactorRefactored,
	},
	LogOr{lhs: Box<ExprRefactored>,
		rhs: Box<ExprRefactored>,
	},
	LogAnd{lhs: Box<ExprRefactored>,
		rhs: FactorRefactored,
	},
	Uminus(Box<ExprRefactored>),
	Eq{lhs: Box<ExprRefactored>,
		rhs: Box<ExprRefactored>,
	},
	Neq{lhs: Box<ExprRefactored>,
		rhs: Box<ExprRefactored>,
	},
	Grt{lhs: Box<ExprRefactored>,
		rhs: Box<ExprRefactored>,
	},
	Lst{lhs: Box<ExprRefactored>,
		rhs: Box<ExprRefactored>,
	},
	Geq{lhs: Box<ExprRefactored>,
		rhs: Box<ExprRefactored>,
	},
	Leq{lhs: Box<ExprRefactored>,
		rhs: Box<ExprRefactored>,
	},
	Factor(Box<FactorRefactored>),
    IntToFloat(Box<ExprRefactored>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum FactorRefactored {
	ConstInt(i64),
	ConstFloat(f64),
	ConstBool(bool),
	FunctionCall(FunctionCallRefactored),
	IdLocalVar(usize),
	IdGlobalVar(usize),
	Assignment(AssignmentRefactored),
    IntToFloat(Box<FactorRefactored>),
}