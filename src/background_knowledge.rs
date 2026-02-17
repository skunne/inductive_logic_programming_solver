use crate::{parse::Parse, program::Predicate};

#[derive(Debug)]
pub struct BackgroungKnowledge {
    facts: Vec<Predicate>,
}

impl Parse for BackgroungKnowledge {
    /// Parse background knowledge, which is a list of facts = predicates, one fact per line, with a '.' at the end of each fact
    fn parse(text: &str) -> Result<Self, anyhow::Error> {
        Ok(Self {
            facts: text
                .lines()
                .map(|p| Predicate::parse(p.trim().trim_end_matches('.')))
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}
