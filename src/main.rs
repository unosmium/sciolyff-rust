use sciolyff::interpreter::{html::HTMLOptions, Interpreter};
use std::env;
use std::fs;

fn main() {
    let file = &env::args().collect::<Vec<_>>()[1];
    let contents = fs::read_to_string(file).unwrap();
    let i = Interpreter::from_yaml(&contents);
    print!("{}", i.to_html(&HTMLOptions::default()));
}
