use crate::syntree::Visitor;

use crate::syntree::Tree;
use crate::syntree::Stmt;
use crate::syntree::Expr;

#[derive(Default)]
pub struct Printer {
	stmt_results: Vec<String>,
}

impl Printer {
	pub fn pretty_print(&mut self, t: & Tree) -> String {
		for stmt in &t.stmt_list {
			self.visit_stmt(&stmt);
		}
		self.stmt_results.join("\n")
	}
}

impl Visitor<String> for Printer {
	fn visit_stmt(&mut self, s: &Stmt) -> String {
		match s {
			Stmt::Expr(e) => {
				let e_res = self.visit_expr(e);
				self.stmt_results.push(e_res);
			},
			Stmt::Set(..) => {},
		}
		"".to_string()
	}

	fn visit_expr(&mut self, e: &Expr) -> String {
		match e {
			Expr::Int(val) => {
				val.to_string()
			},
			Expr::Var(val) => {
				val.to_string()
			},
			Expr::Add(lhs, rhs) => {
				let lh_val = self.visit_expr(lhs).to_string();
				let rh_val = self.visit_expr(rhs).to_string();

				format!("({}+{})", lh_val, rh_val)
			},
			Expr::Sub(lhs, rhs) => {
				let lh_val = self.visit_expr(lhs).to_string();
				let rh_val = self.visit_expr(rhs).to_string();

				format!("({}-{})", lh_val, rh_val)
			},
			Expr::Mul(lhs, rhs) => {
				let lh_val = self.visit_expr(lhs).to_string();
				let rh_val = self.visit_expr(rhs).to_string();

				format!("({}*{})", lh_val, rh_val)
			},
			Expr::Div(lhs, rhs) => {
				let lh_val = self.visit_expr(lhs).to_string();
				let rh_val = self.visit_expr(rhs).to_string();

				format!("({}/{})", lh_val, rh_val)
			},
		}
	}
}
