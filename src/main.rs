use sciolyff::interpreter::Interpreter;
use std::fs;

fn main() {
    let file = "2020-02-22_golden_gate_invitational_c.yaml";

    let contents = fs::read_to_string(file).unwrap();
    let i = Interpreter::new(&contents);
    for t in &i.teams {
        println!("{} {} {}", t.number(), t.rank(), t.points());
    }
    //for e in &i.events {
    //    println!("{} {}", e.name(), e.trial());
    //}
    //println!("{:?}", true.cmp(&true))
}
