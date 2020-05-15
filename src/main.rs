use sciolyff::interpreter::Interpreter;
use std::fs;

fn main() {
    let file = "2020-02-22_golden_gate_invitational_c.yaml";

    let contents = fs::read_to_string(file).unwrap();
    let i = Interpreter::new(&contents);
    for t in i.teams {
        println!("{:#?}", t.school_abbreviation());
    }
}