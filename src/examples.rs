use crate::{parse::Parse, program::Predicate};

#[derive(Debug)]
pub struct Examples {
    pos: Vec<Predicate>,
    neg: Vec<Predicate>,
}
pub enum Example {
    Pos(Predicate),
    Neg(Predicate),
}

impl Parse for Examples {
    fn parse(text: &str) -> Result<Self, anyhow::Error> {
        let mut pos = vec![];
        let mut neg = vec![];
        let lines = text.lines();
        for line in lines {
            let pred = Example::parse(line)?;
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
