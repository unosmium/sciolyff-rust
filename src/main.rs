use sciolyff::interpreter::Interpreter;
use std::fs;

fn main() {
    let file = "2020-02-22_golden_gate_invitational_c.yaml";

    let contents = fs::read_to_string(file).unwrap();
    let _i = Interpreter::new(&contents);
    //println!("{:#?}", i.penalties[0].borrow().team.as_ref().unwrap().upgrade());
}
