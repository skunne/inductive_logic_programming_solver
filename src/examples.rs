use crate::{parse::Parse, program::Predicate};

#[derive(Debug)]
pub struct Examples {
    pub pos: Vec<Predicate>,
    pub neg: Vec<Predicate>,
}
pub enum Example {
    Pos(Predicate),
    Neg(Predicate),
}

impl Examples {
    pub fn is_empty(&self) -> bool {
        self.pos.is_empty() && self.neg.is_empty()
    }
}

impl Parse for Examples {
    fn parse(text: &str) -> Result<Self, anyhow::Error> {
        let mut pos = vec![];
        let mut neg = vec![];
        let lines = text.lines();
        for line in lines {
            let pred = Example::parse(line.trim().trim_end_matches('.'))?;
            match pred {
                Example::Pos(pred) => {
                    pos.push(pred);
                }
                Example::Neg(pred) => {
                    neg.push(pred);
                }
            }
        }
        Ok(Self { pos, neg })
    }
}
