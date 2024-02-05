use crate::syntree::Visitor;

use crate::syntree::SynTree;

#[derive(Default)]
pub struct Printer {}

impl Printer {
	pub fn pretty_print(&mut self, t: & SynTree) -> String {
		self.visit(t)
	}
}

impl Visitor<String> for Printer {
	fn visit(&mut self, t: & SynTree) -> String {
		// TODO
	}
}
