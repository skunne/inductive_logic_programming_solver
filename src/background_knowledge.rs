use crate::{parse::Parse, program::Predicate};

#[derive(Debug)]
pub struct BackgroungKnowledge {
    facts: Vec<Predicate>,
}

impl Parse for BackgroungKnowledge {
    fn parse(text: &str) -> Result<Self, anyhow::Error> {
        Ok(Self {
            facts: text
                .lines()
                .map(Predicate::parse)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}
