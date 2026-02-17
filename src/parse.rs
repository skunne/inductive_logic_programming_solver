use std::str::FromStr;

use crate::examples::Example;
use crate::program::{Clause, Literal, Predicate, Program, Symbol};

pub trait Parse {
    fn parse(text: &str) -> Result<Self, anyhow::Error>
    where
        Self: Sized;
}

impl Parse for Program {
    /// Parse a program, which is a list of clause, one clause per line
    fn parse(text: &str) -> Result<Self, anyhow::Error> {
        eprintln!("Program::parse({})", text);
        Ok(Program {
            name: "Parsed program".to_owned(),
            clauses: text
                .lines()
                .map(Clause::parse)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl Parse for Clause {
    /// Parse a clause, such as `"last(a,b):-tail(a,c),reverse(c,d),head(d,b)"`
    fn parse(line: &str) -> Result<Self, anyhow::Error> {
        eprintln!("Clause::parse({})", line);
        let x = line.trim_end_matches(".");
        let Some((head, body)) = x.split_once(":-") else {
            anyhow::bail!("Clause should be head:-body");
        };
        Ok(Clause {
            head: Predicate::parse(head)?,
            body: split_commas(body)
                .iter()
                .map(|p| Predicate::parse(p))
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

/// Split on commas, but ignore commas inside parens
/// Example:
/// ```rust, ignore
/// assert_eq!(
///     split_commas("mother(a,c),mother(c,b)"),
///     vec!["mother(a,c)", "mother(c,b)"]
/// )
/// ```
fn split_commas(text: &str) -> Vec<&str> {
    let mut i = 0;
    let mut depth = 0;
    let text = text.as_bytes();
    let mut split_vec = Vec::new();
    for j in 0..text.len() {
        match text[j] {
            b'(' => depth += 1,
            b')' => depth -= 1,
            b',' if depth == 0 => {
                split_vec.push(str::from_utf8(&text[i..j]).unwrap());
                i = j + 1;
            }
            _ => {}
        }
    }
    split_vec.push(str::from_utf8(&text[i..]).unwrap());
    split_vec
}

impl Parse for Predicate {
    /// Parse a predicate, such as `"mother(a,b)"`
    fn parse(p: &str) -> Result<Self, anyhow::Error> {
        eprintln!("Predicate::parse({})", p);
        let Some((name, args)) = p.trim_end_matches(")").split_once("(") else {
            anyhow::bail!(
                "Predicate::parse({}): predicate should be name(arg1, ...)",
                p
            )
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
    /// Parse a literal, which is just a grounded symbol or ungrounded symbol
    fn parse(l: &str) -> Result<Self, anyhow::Error> {
        eprintln!("Literal::parse({})", l);
        Ok(Literal::Grounded(Symbol::from_str(l)?))
    }
}

impl Parse for Example {
    /// Parse an example, which is a predicate surrounded in `pos( )` or `neg( )`
    fn parse(p: &str) -> Result<Self, anyhow::Error> {
        eprintln!("Example::parse({})", p);
        let p = p.trim().trim_end_matches('.');
        if (p.starts_with("pos(") || p.starts_with("neg(")) && p.ends_with(")") {
            let kind = &p[..3];
            let pred_str = &p[4..p.len() - 1];
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
