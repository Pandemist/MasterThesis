use crate::printer::Printer;
use crate::calculator::Calculator;

#[allow(unused)]
use crate::syntree::create_tree_by_string;
#[allow(unused)]
use crate::syntree::create_assignment_node;

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
    
    let tree = create_tree_by_string(text_input_arg);

    let mut printer = Printer::default();
    println!("{}", printer.pretty_print(&tree));

    let mut calc = Calculator::new();
    println!("{}", calc.calculate(&tree));
}

#[test]
fn simple_test() {
    let tree = create_tree_by_string("15-22*+");

    let mut printer = Printer::default();
    assert_eq!(printer.pretty_print(&tree), "((2*2)+(5-1))");

    let mut calc = Calculator::new();
    assert_eq!(calc.calculate(&tree), 8);
}

#[test]
// Soll panicen
fn zero_division_test() {
    let tree = create_tree_by_string("02/");

    let mut printer = Printer::default();
    assert_eq!(printer.pretty_print(&tree), "(2/0)");

    let mut calc = Calculator::new();
    assert_eq!(calc.calculate(&tree), 0);
}

#[test]
fn operator_add_test() {
    // Test Addition
    let tree = create_tree_by_string("11+");

    let mut printer = Printer::default();
    assert_eq!(printer.pretty_print(&tree), "(1+1)");

    let mut calc = Calculator::new();
    assert_eq!(calc.calculate(&tree), 2);
}

#[test]
fn operator_sub_test() {
    // Test Division
    let tree = create_tree_by_string("11-");

    let mut printer = Printer::default();
    assert_eq!(printer.pretty_print(&tree), "(1-1)");

    let mut calc = Calculator::new();
    assert_eq!(calc.calculate(&tree), 0);
}

#[test]
fn operator_mul_test() {
    // Test Multiplikation
    let tree = create_tree_by_string("11*");

    let mut printer = Printer::default();
    assert_eq!(printer.pretty_print(&tree), "(1*1)");

    let mut calc = Calculator::new();
    assert_eq!(calc.calculate(&tree), 1);
}

#[test]
fn operator_div_test() {
    // Test Division
    let tree = create_tree_by_string("11/");

    let mut printer = Printer::default();
    assert_eq!(printer.pretty_print(&tree), "(1/1)");

    let mut calc = Calculator::new();
    assert_eq!(calc.calculate(&tree), 1);
}

#[test]
fn work_with_vars() {
    let pre_init_1 = create_assignment_node('a', 3);
    let pre_init_2 = create_assignment_node('b', 6);
    let pre_init_3 = create_assignment_node('c', 7);
    let pre_init_4 = create_assignment_node('d', 1);
    let tree = create_tree_by_string("ab+dc-*");

    let mut printer = Printer::default();
    assert_eq!(printer.pretty_print(&tree), "((c-d)*(b+a))");

    let mut calc = Calculator::new();
    calc.calculate(&pre_init_1);
    calc.calculate(&pre_init_2);
    calc.calculate(&pre_init_3);
    calc.calculate(&pre_init_4);
    assert_eq!(calc.calculate(&tree), 54);
}
