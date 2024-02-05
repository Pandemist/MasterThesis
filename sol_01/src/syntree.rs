#[allow(dead_code)]
#[derive(Debug, Default, PartialEq)]
pub struct Tree {
	pub stmt_list: Vec<Stmt>,
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
	Expr(Expr),
	Set(char, Expr)
}

#[derive(Debug, PartialEq)]
pub enum Expr {
	Int(i64),
	Var(char),
	Add(Box<Expr>, Box<Expr>),
	Sub(Box<Expr>, Box<Expr>),
	Mul(Box<Expr>, Box<Expr>),
	Div(Box<Expr>, Box<Expr>),
}

pub trait Visitor<T> {
	fn visit_stmt(&mut self, s: &Stmt) -> T;
	fn visit_expr(&mut self, e: &Expr) -> T;
}

impl Tree {
	pub fn push_stmt(&mut self, s: Stmt) {
		self.stmt_list.push(s);
	}
}

pub fn create_tree_by_string(str: &str) -> Stmt {

	let mut node_vec = vec![];
	
	for c in str.chars() {
		match c {
			'0'..='9' => {
				node_vec.push(Expr::Int(c.to_digit(10).unwrap().into()));
			},
			'+' => {
				if node_vec.len() >= 2 {
					let lhs = node_vec.pop().unwrap();
					let rhs = node_vec.pop().unwrap();
					node_vec.push(Expr::Add(Box::new(lhs), Box::new(rhs)));
				}else{
					panic!("Wrong formated of String");
				}
			},
			'-' => {
				if node_vec.len() >= 2 {
					let lhs = node_vec.pop().unwrap();
					let rhs = node_vec.pop().unwrap();
					node_vec.push(Expr::Sub(Box::new(lhs), Box::new(rhs)));
				}else{
					panic!("Wrong formated of String");
				}
			},
			'*' => {
				if node_vec.len() >= 2 {
					let lhs = node_vec.pop().unwrap();
					let rhs = node_vec.pop().unwrap();
					node_vec.push(Expr::Mul(Box::new(lhs), Box::new(rhs)));
				}else{
					panic!("Wrong formated of String");
				}
			},
			'/' => {
				if node_vec.len() >= 2 {
					let lhs = node_vec.pop().unwrap();
					let rhs = node_vec.pop().unwrap();
					node_vec.push(Expr::Div(Box::new(lhs), Box::new(rhs)));
				}else{
					panic!("Wrong formated of String");
				}
			},
			'a'..='z' => {
				node_vec.push(Expr::Var(c));
			}
			_ => {
				panic!("Invalid char {:?}", c);
			},
		}
	}
	if node_vec.len() == 1 {
		Stmt::Expr(node_vec.pop().unwrap())
	}else{
		panic!("Wrong formated of String");
	}
}

pub fn create_assignment_node(c: char, value: i64) -> Stmt {
	Stmt::Set(c, Expr::Int(value))
}
