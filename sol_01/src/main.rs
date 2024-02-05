use crate::printer::Printer;
use crate::calculator::Calculator;

use crate::syntree::Tree;

#[allow(unused)]
use crate::syntree::create_tree_by_string;
#[allow(unused)]
use crate::syntree::create_assignment_node;

#[allow(unused)]
use proptest::prelude::*;

#[allow(unused)]
use std::cmp;

mod syntree;
mod printer;
mod calculator;

use std::env;

fn main() {
    // Argumente auslesen und den ersten Parameter
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Zu wenige Argumente.");
        return;
    }
    let text_input_arg = &args[1];
    
    let mut tree = Tree::default();
    tree.push_stmt(create_tree_by_string(text_input_arg));

    let mut printer = Printer::default();
    println!("{}", printer.pretty_print(&tree));

    let mut calc = Calculator::new();
    println!("{:?}", calc.calculate(&tree));
}

#[test]
fn simple_test() {
    let mut tree = Tree::default();
    tree.push_stmt(create_tree_by_string("15-22*+"));

    let mut printer = Printer::default();
    assert_eq!(printer.pretty_print(&tree), "((2*2)+(5-1))");

    let mut calc = Calculator::new();
    assert_eq!(calc.calculate(&tree), vec![8]);
}

#[test]
#[should_panic]
fn zero_division_test() {
    let mut tree = Tree::default();
    tree.push_stmt(create_tree_by_string("02/"));

    let mut printer = Printer::default();
    assert_eq!(printer.pretty_print(&tree), "(2/0)");

    let mut calc = Calculator::new();
    assert_eq!(calc.calculate(&tree), vec![0]);
}

#[test]
fn operator_add_test() {
    // Test Addition
    let mut tree = Tree::default();
    tree.push_stmt(create_tree_by_string("11+"));

    let mut printer = Printer::default();
    assert_eq!(printer.pretty_print(&tree), "(1+1)");

    let mut calc = Calculator::new();
    assert_eq!(calc.calculate(&tree), vec![2]);
}

#[test]
fn operator_sub_test() {
    // Test Division
    let mut tree = Tree::default();
    tree.push_stmt(create_tree_by_string("11-"));

    let mut printer = Printer::default();
    assert_eq!(printer.pretty_print(&tree), "(1-1)");

    let mut calc = Calculator::new();
    assert_eq!(calc.calculate(&tree), vec![0]);
}

#[test]
fn operator_mul_test() {
    // Test Multiplikation
    let mut tree = Tree::default();
    tree.push_stmt(create_tree_by_string("11*"));

    let mut printer = Printer::default();
    assert_eq!(printer.pretty_print(&tree), "(1*1)");

    let mut calc = Calculator::new();
    assert_eq!(calc.calculate(&tree), vec![1]);
}

#[test]
fn operator_div_test() {
    // Test Division
    let mut tree = Tree::default();
    tree.push_stmt(create_tree_by_string("11/"));

    let mut printer = Printer::default();
    assert_eq!(printer.pretty_print(&tree), "(1/1)");

    let mut calc = Calculator::new();
    assert_eq!(calc.calculate(&tree), vec![1]);
}

#[test]
fn work_with_vars() {
    let mut tree = Tree::default();
    tree.push_stmt(create_assignment_node('a', 3));
    tree.push_stmt(create_assignment_node('b', 6));
    tree.push_stmt(create_assignment_node('c', 7));
    tree.push_stmt(create_assignment_node('d', 1));
    tree.push_stmt(create_tree_by_string("ab+dc-*"));

    let mut printer = Printer::default();
    assert_eq!(printer.pretty_print(&tree), "((c-d)*(b+a))");

    let mut calc = Calculator::new();
    assert_eq!(calc.calculate(&tree), vec![54]);
}

proptest! {
    #[test]
    fn test_with_random_inputs(
        in_zahlen in prop::array::uniform32(0i64..9),
        in_operatoren in prop::array::uniform31(0u32..3),
        ) {
        let mut new_operator = vec![];

        for op in in_operatoren {
            match op {
                0 => new_operator.push('+'),
                1 => new_operator.push('-'),
                2 => new_operator.push('*'),
                3 => new_operator.push('/'),
                _ => {},
            }
        }

        let first_ziffer = in_zahlen[0];
        let rest_ziffern = &in_zahlen[1..32];

        let mut node_vec = vec![];
        let mut construct_string = first_ziffer.clone().to_string();

        node_vec.push(first_ziffer);

        for (in_ziffer, op) in rest_ziffern.iter().zip(new_operator.iter()) {
            let mut stack_ziffer = node_vec.pop().unwrap();

            construct_string.push_str(&in_ziffer.clone().to_string());
            construct_string.push_str(&op.clone().to_string());

        // 0 Divisionen verhindern
            if (stack_ziffer == 0) && (op == &'/') {
                stack_ziffer = 1;
            }

            match op {
                '+' => {
                    let erg = cmp::min(i64::MAX, in_ziffer + stack_ziffer);
                    node_vec.push(erg);
                },
                '-' => {
                    let erg = cmp::max(i64::MIN, in_ziffer - stack_ziffer);
                    node_vec.push(erg);
                },
                '*' => {
                    let erg = cmp::min(i64::MAX, in_ziffer * stack_ziffer);
                    node_vec.push(erg);
                },
                '/' => {
                    let erg = if in_ziffer != &0i64 {
                        cmp::max(i64::MIN, in_ziffer / stack_ziffer)
                    }else{
                        0
                    };
                    
                    node_vec.push(erg);
                },
                _ => {},
            }
        }


        let ergebnis = node_vec.pop().unwrap();

        let mut tree = Tree::default();
        tree.push_stmt(create_tree_by_string(&construct_string));

        let mut calc = Calculator::new();
        assert_eq!(calc.calculate(&tree), vec![ergebnis]);
    }
}