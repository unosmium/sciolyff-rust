use crate::rep::Rep;

pub struct Interpreter;

impl Interpreter {
    pub fn new(source: &str) -> Interpreter {
        let rep: Rep = serde_yaml::from_str(&source).unwrap();
        let rep2 = rep.tournament;
        Interpreter
    }
}
