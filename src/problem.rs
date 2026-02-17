use std::path::Path;

use crate::background_knowledge::BackgroungKnowledge;
use crate::constraints::Constraints;
use crate::examples::Examples;
use crate::parse::Parse;
use crate::program::Program;

#[derive(Debug)]
pub struct Problem {
    exs: Examples,
    bk: BackgroungKnowledge,
    i: usize, // index for method generate, to be removed later
}

impl Problem {
    /// Create a new program by opening and parsing files exs.pl and bk.pl in folder
    pub fn new(foldername: &str) -> Result<Self, anyhow::Error> {
        let folder_path = Path::new(foldername);

        let exs = Examples::parse(&std::fs::read_to_string(folder_path.join("exs.pl"))?)?;
        let bk = BackgroungKnowledge::parse(&std::fs::read_to_string(folder_path.join("bk.pl"))?)?;
        Ok(Self { exs, bk, i: 0 })
    }

    /// Solve problem using Generate -> Test -> Constrain -> Generate loop
    pub fn learn_solution(&mut self) -> Option<Program> {
        let mut solved = false;
        let mut program = None;
        while !solved {
            program = self.generate();
            let failed_examples = self.test(&program);
            solved = failed_examples.is_empty();
            let constraints = self.constrain(&program, &failed_examples);
            self.add_constraints(&constraints);
        }
        program
    }

    /// Generate a candidate program given the constraints and examples of the problem
    fn generate(&mut self) -> Option<Program> {
        const PROGRAM_TEXTS: &[&'static str] = &[
            r#"grandparent(a,b):-"#,
            r#"grandparent(a,b):-mother(a,c),mother(c,b)"#,
            r#"grandparent(a,b):-father(a,c),father(c,b)"#,
            r#"grandparent(a,b):-father(a,c),father(c,b)
            grandparent(a,b):-father(a,c),mother(c,b)"#,
            r#"grandparent(a,b):-father(a,c),mother(c,b)
            grandparent(a,b):-mother(a,c),mother(c,b)"#,
            r#"grandparent(a,b):-parent(a,c),parent(c,b)
            parent(a,b):-mother(a,b)
            parent(a,b):-father(a,c)"#,
        ];
        if self.i < PROGRAM_TEXTS.len() {
            self.i += 1;
            Some(Program::parse(PROGRAM_TEXTS[self.i]).unwrap())
        } else {
            None
        }
    }

    /// Test a candidate program on the examples
    /// Return the false positives and false negatives
    fn test(&self, program: &Option<Program>) -> Examples {
        Examples {
            pos: vec![],
            neg: vec![],
        }
    }

    /// Generate new constraints using the candidate program and false positives and false negatives
    fn constrain(&self, program: &Option<Program>, failed_examples: &Examples) -> Constraints {
        Constraints {}
    }

    /// Add the new constraints to the problem
    fn add_constraints(&mut self, constraints: &Constraints) {}
}
