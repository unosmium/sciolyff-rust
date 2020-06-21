use sciolyff::interpreter::Interpreter;
use std::fs;

fn main() {
    let file = "data/2019-11-23_palatine_invitational_c.yaml";
    let contents = fs::read_to_string(file).unwrap();
    let i = Interpreter::from_yaml(&contents);
    fs::write("output.html", i.to_html(true, "darkred")).unwrap();
}
