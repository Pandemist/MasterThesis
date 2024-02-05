#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum SynTree {
	// Liste von Knoten, als Aufgabenfolge
	TreeNodeAdd(Box<SynTree>, Box<SynTree>),
	TreeNodeSub(Box<SynTree>, Box<SynTree>),
	TreeNodeMul(Box<SynTree>, Box<SynTree>),
	TreeNodeDiv(Box<SynTree>, Box<SynTree>),
	TreeNodeInt(i64),
	TreeNodeVar(char),
	TreeNodeVarSet(char, i64),
}

pub trait Visitor<T> {
	fn visit(&mut self, t: & SynTree) -> T;
}

pub fn create_tree_by_string(str: &str) -> SynTree {

	let mut node_vec = vec![];
	
	for c in str.chars() {
		match c {
			'0'..='9' => {
				node_vec.push(SynTree::TreeNodeInt(c.to_digit(10).unwrap().into()));
			},
			'+' => {
				if node_vec.len() >= 2 {
					let lhs = node_vec.pop().unwrap();
					let rhs = node_vec.pop().unwrap();
					node_vec.push(SynTree::TreeNodeAdd(Box::new(lhs), Box::new(rhs)));
				}else{
					panic!("Wrong format of String");
				}
			},
			'-' => {
				if node_vec.len() >= 2 {
					let lhs = node_vec.pop().unwrap();
					let rhs = node_vec.pop().unwrap();
					node_vec.push(SynTree::TreeNodeSub(Box::new(lhs), Box::new(rhs)));
				}else{
					panic!("Wrong format of String");
				}
			},
			'*' => {
				if node_vec.len() >= 2 {
					let lhs = node_vec.pop().unwrap();
					let rhs = node_vec.pop().unwrap();
					node_vec.push(SynTree::TreeNodeMul(Box::new(lhs), Box::new(rhs)));
				}else{
					panic!("Wrong format of String");
				}
			},
			'/' => {
				if node_vec.len() >= 2 {
					let lhs = node_vec.pop().unwrap();
					let rhs = node_vec.pop().unwrap();
					node_vec.push(SynTree::TreeNodeDiv(Box::new(lhs), Box::new(rhs)));
				}else{
					panic!("Wrong format of String");
				}
			},
			'a'..='z' => {
				node_vec.push(SynTree::TreeNodeVar(c));
			}
			_ => {
				panic!("Invalid char {:?}", c);
			},
		}
	}
	if node_vec.len() == 1 {
		node_vec.pop().unwrap()
	}else{
		panic!("Wrong format of String");
	}
}

pub fn create_assignment_node(c: char, value: i64) -> SynTree {
	SynTree::TreeNodeVarSet(c, value)
}
