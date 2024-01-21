use crate::parser::semver::SemVer;
use crate::parser::Rule;
use pest::iterators::Pair;

pub(crate) fn namespace(pair: Pair<Rule>) -> Result<Namespace, Box<dyn std::error::Error>> {
    let mut ns = Namespace::default();

    match pair.into_inner().next() {
        Some(qualified_namespace) => match qualified_namespace.into_inner().next() {
            Some(namespace_declaration) => match namespace_declaration.as_rule() {
                Rule::VersionedQualifiedNamespace => {
                    for part in namespace_declaration.into_inner() {
                        match part.as_rule() {
                            Rule::QualifiedName => ns.name = part.as_str().to_string(),
                            Rule::SemVer => ns.version = Some(crate::parser::semver::semver(part)?),
                            _ => unreachable!(),
                        }
                    }
                }
                Rule::QualifiedName => ns.name = namespace_declaration.as_str().to_string(),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        },
        _ => unreachable!(),
    };
    Ok(ns)
}

#[derive(Default, Debug, PartialEq)]
pub struct Namespace {
    pub name: String,
    pub version: Option<SemVer>,
}

#[cfg(test)]
mod test {
    use crate::parser::{semver::SemVer, ConcertoParser, Rule};
    use pest::Parser;

    fn parse<'a>(input: &'a str) -> super::Namespace {
        super::namespace(
            ConcertoParser::parse(Rule::Namespace, input)
                .unwrap()
                .next()
                .unwrap(),
        )
        .unwrap()
    }

    #[test]
    fn test_namespace_unversioned() {
        assert_eq!(
            parse("namespace com.foo.bar"),
            super::Namespace {
                name: String::from("com.foo.bar"),
                version: None
            }
        )
    }

    #[test]
    fn test_namespace_versioned() {
        assert_eq!(
            parse("namespace com.example.foo@1.0.42"),
            super::Namespace {
                name: String::from("com.example.foo"),
                version: Some(SemVer {
                    major: 1,
                    minor: 0,
                    patch: 42,
                    prerelease: String::new(),
                    build: String::new()
                })
            }
        )
    }
}
