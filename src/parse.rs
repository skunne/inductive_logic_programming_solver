use std::str::FromStr;

use crate::examples::Example;
use crate::program::{Clause, Literal, Predicate, Symbol};

pub trait Parse {
    fn parse(text: &str) -> Result<Self, anyhow::Error>
    where
        Self: Sized;
}

impl Parse for Clause {
    fn parse(line: &str) -> Result<Self, anyhow::Error> {
        let x = line.trim_end_matches(".");
        let Some((head, body)) = x.split_once(":-") else {
            anyhow::bail!("Clause should be head:-body");
        };
        Ok(Clause {
            head: Predicate::parse(head)?,
            body: body
                .split(",")
                .map(Predicate::parse)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl Parse for Predicate {
    fn parse(p: &str) -> Result<Self, anyhow::Error> {
        let Some((name, args)) = p.trim_end_matches(")").split_once("(") else {
            anyhow::bail!("Predicate should be name(arg1, ...)")
        };
        let args = args.split(",").collect::<Vec<_>>();
        Ok(Predicate {
            name: name.to_owned(),
            arity: args.len(),
            members: args
                .into_iter()
                .map(Literal::parse)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl Parse for Literal {
    fn parse(l: &str) -> Result<Self, anyhow::Error> {
        Ok(Literal::Grounded(Symbol::from_str(l)?))
    }
}

impl Parse for Example {
    fn parse(p: &str) -> Result<Self, anyhow::Error> {
        if (p.starts_with("pos(") || p.starts_with("neg(")) && p.ends_with(").") {
            let kind = &p[..3];
            let pred_str = &p[4..p.len() - 2];
            let pred = Predicate::parse(pred_str)?;
            match kind {
                "pos" => Ok(Example::Pos(pred)),
                "neg" => Ok(Example::Neg(pred)),
                _ => unreachable!("Unreachable match branch because already inside if branch"),
            }
        } else {
            anyhow::bail!("Example should be pos(...) or neg(...)");
        }
    }
}
