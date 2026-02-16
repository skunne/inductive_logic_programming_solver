use std::str::FromStr;

#[derive(Debug)]
pub struct Program {
    pub name: String,
    pub clauses: Vec<Clause>,
}

#[derive(Debug)]
pub struct Clause {
    pub head: Predicate,
    pub body: Vec<Predicate>,
}

#[derive(Debug)]
pub struct Predicate {
    pub name: String,
    pub arity: usize,
    pub members: Vec<Literal>,
}

#[derive(Debug)]
pub enum Literal {
    Grounded(Symbol),
    Ungrounded(Symbol),
}

#[derive(Debug)]
pub struct Symbol {
    name: String,
}

impl FromStr for Symbol {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self { name: s.into() })
    }
}
