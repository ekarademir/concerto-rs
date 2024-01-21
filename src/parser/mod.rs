pub(crate) mod namespace;
pub(crate) mod semver;

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser/concerto.pest"]
pub struct ConcertoParser;

pub fn parse<'a>(input: &'a str) -> Result<Model, Box<dyn std::error::Error>> {
    let mut model = Model::default();
    match ConcertoParser::parse(Rule::Model, input)?.next() {
        Some(model_file) => {
            for part in model_file.into_inner() {
                match part.as_rule() {
                    Rule::Namespace => model.namespace = namespace::namespace(part)?,
                    Rule::EOI => (),
                    _ => unreachable!(),
                }
            }
        }
        None => (),
    };
    Ok(model)
}

#[derive(Default, Debug, PartialEq)]
pub struct Model {
    pub namespace: namespace::Namespace,
}
