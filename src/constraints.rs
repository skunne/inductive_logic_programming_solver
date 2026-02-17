use crate::{parse::Parse, program::Predicate};

#[derive(Debug)]
pub struct Constraints {}

// impl Parse for Constraints {
//     fn parse(text: &str) -> Result<Self, anyhow::Error> {
//         Ok(Self {
//             facts: text
//                 .lines()
//                 .map(Predicate::parse)
//                 .collect::<Result<Vec<_>, _>>()?,
//         })
//     }
// }
