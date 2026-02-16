use std::path::Path;

use crate::background_knowledge::BackgroungKnowledge;
use crate::examples::Examples;
use crate::parse::Parse;
use crate::program::Program;

#[derive(Debug)]
pub struct Problem {
    exs: Examples,
    bk: BackgroungKnowledge,
}

impl Problem {
    pub fn new(foldername: &str) -> Result<Self, anyhow::Error> {
        let folder_path = Path::new(foldername);

        let exs = Examples::parse(&std::fs::read_to_string(folder_path.join("exs.pl"))?)?;
        let bk = BackgroungKnowledge::parse(&std::fs::read_to_string(folder_path.join("bk.pl"))?)?;
        Ok(Self { exs, bk })
    }

    pub fn learn_solution(&self) -> Option<Program> {
        Some(Program {
            name: "Learned program".to_string(),
            clauses: vec![],
        })
    }
}
