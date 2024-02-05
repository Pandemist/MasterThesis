use crate::syntree::Visitor;

use crate::syntree::Tree;
use crate::syntree::Stmt;
use crate::syntree::Expr;

use std::cmp;

#[derive(Default)]
pub struct Calculator {
	var_dictionary: Vec<i64>,
	stmt_results: Vec<i64>,
}

impl Calculator {
	pub fn new() -> Self {
        Calculator {
            var_dictionary: vec![0; 26],
            ..Calculator::default()
        }
    }

	pub fn calculate(&mut self, t: & Tree) -> Vec<i64> {
		for stmt in &t.stmt_list {
			self.visit_stmt(&stmt);
		}
		self.stmt_results.clone()
	}
}

impl Visitor<i64> for Calculator {
	fn visit_stmt(&mut self, s: &Stmt) -> i64 {
		match s {
			Stmt::Expr(e) => {
				let e_res = self.visit_expr(e);
				self.stmt_results.push(e_res);
			},
			Stmt::Set(n, e) => {
				let expr:i64 = self.visit_expr(e);
				let index:usize = (((*n) as i32) - 97).try_into().unwrap();
				self.var_dictionary[index] = expr;
			}
		}
		0
	}

	fn visit_expr(&mut self, e: &Expr) -> i64 {
		match e {
			Expr::Int(val) => {
				*val
			},
			Expr::Var(pos) => {
				let index:usize = (((*pos) as i32) - 97).try_into().unwrap();
				self.var_dictionary[index]
			},
			Expr::Add(lhs, rhs) => {
				cmp::min(i64::MAX, self.visit_expr(lhs) + self.visit_expr(rhs))
			},
			Expr::Sub(lhs, rhs) => {
				cmp::max(i64::MIN, self.visit_expr(lhs) - self.visit_expr(rhs))
			},
			Expr::Mul(lhs, rhs) => {
				cmp::min(i64::MAX, self.visit_expr(lhs) * self.visit_expr(rhs))
			},
			Expr::Div(lhs, rhs) => {
				let rhs_result = self.visit_expr(rhs);
				
				if rhs_result != 0 {
					cmp::max(i64::MIN, self.visit_expr(lhs) / self.visit_expr(rhs))
				}else{
					panic!("Eine 0 division ist nicht zulässig.");
				}
				
			},
		}
	}
}