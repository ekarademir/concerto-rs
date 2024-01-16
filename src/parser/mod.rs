mod semver;

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser/concerto.pest"]
pub struct ConcertoParser;

pub fn parse<'a>(input: &'a str) -> Result<Model, Box<dyn std::error::Error>> {
    let mut model = Model::default();
    match ConcertoParser::parse(Rule::Model, input)?.next() {
        Some(model) => {
            for part in model.into_inner() {
                match part.as_rule() {
                    Rule::Namespace => match part.into_inner().next() {
                        Some(namespace_identifier) => {
                            println!("{:?}", namespace_identifier.as_str());
                        }
                        None => (),
                    },
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
    //
}
