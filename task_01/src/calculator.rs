use crate::syntree::Visitor;

use crate::syntree::SynTree;

#[derive(Default)]
pub struct Calculator {
	// TODO
}

impl Calculator {
	pub fn new() -> Self {
        // TODO
    }

	pub fn calculate(&mut self, t: & SynTree) -> i64 {
		self.visit(t)
	}
}

impl Visitor<i64> for Calculator {
	fn visit(&mut self, t: & SynTree) -> i64 {
		// TODO
	}
}