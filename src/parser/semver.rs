use crate::parser::Rule;
use pest::iterators::Pair;

pub(crate) fn semver(pair: Pair<Rule>) -> Result<SemVer, Box<dyn std::error::Error>> {
    let mut semver = SemVer::default();

    for part in pair.into_inner() {
        match part.as_rule() {
            Rule::Version => match part.into_inner().next() {
                None => (),
                Some(version) => match version.as_rule() {
                    Rule::MajorMinorPatchVersion => {
                        let ver: Vec<_> = version
                            .into_inner()
                            .map(|r| {
                                let version_part: u32 = r.as_str().parse().unwrap_or(0);
                                version_part
                            })
                            .collect();
                        // ver will definitely have 3 members, since matching MajorMinorPatchVersion
                        semver.major = ver[0];
                        semver.minor = ver[1];
                        semver.patch = ver[2];
                    }
                    Rule::MajorMinorVersion => {
                        let ver: Vec<_> = version
                            .into_inner()
                            .map(|r| {
                                let version_part: u32 = r.as_str().parse().unwrap_or(0);
                                version_part
                            })
                            .collect();
                        // ver will definitely have 2 members, since matching MajorMinorVersion
                        semver.major = ver[0];
                        semver.minor = ver[1];
                    }
                    Rule::MajorVersion => {
                        let ver: Vec<_> = version
                            .into_inner()
                            .map(|r| {
                                let version_part: u32 = r.as_str().parse().unwrap_or(0);
                                version_part
                            })
                            .collect();
                        // ver will definitely have 1 member, since matching MajorVersion
                        semver.major = ver[0];
                    }
                    _ => unreachable!(),
                },
            },
            Rule::Prerelease => match part.into_inner().next() {
                None => (),
                Some(prerelease) => semver.prerelease = prerelease.as_str().to_string(),
            },
            Rule::Build => match part.into_inner().next() {
                None => (),
                Some(build) => semver.build = build.as_str().to_string(),
            },
            _ => unreachable!(),
        }
    }
    Ok(semver)
}

#[derive(Default, Debug, PartialEq)]
pub(crate) struct SemVer {
    major: u32,
    minor: u32,
    patch: u32,
    prerelease: String,
    build: String,
}

#[cfg(test)]
mod test {
    use crate::parser::{ConcertoParser, Rule};
    use pest::Parser;

    fn parse<'a>(input: &'a str) -> super::SemVer {
        super::semver(
            ConcertoParser::parse(Rule::SemVer, input)
                .unwrap()
                .next()
                .unwrap(),
        )
        .unwrap()
    }

    #[test]
    fn test_full_semver() {
        assert_eq!(
            parse("12.13.14-pre123+a"),
            super::SemVer {
                major: 12,
                minor: 13,
                patch: 14,
                prerelease: String::from("pre123"),
                build: String::from("a")
            }
        );
        assert_eq!(
            parse("1.0.0-alpha+001"),
            super::SemVer {
                major: 1,
                minor: 0,
                patch: 0,
                prerelease: String::from("alpha"),
                build: String::from("001")
            }
        );
        assert_eq!(
            parse("1.0.0+21AF26D3----117B344092BD"),
            super::SemVer {
                major: 1,
                minor: 0,
                patch: 0,
                prerelease: String::from(""),
                build: String::from("21AF26D3----117B344092BD")
            }
        );
    }

    #[test]
    fn test_major_minor() {
        assert_eq!(
            parse("12.13-pre123+a"),
            super::SemVer {
                major: 12,
                minor: 13,
                patch: 0,
                prerelease: String::from("pre123"),
                build: String::from("a")
            }
        );
        assert_eq!(
            parse("12.13+a"),
            super::SemVer {
                major: 12,
                minor: 13,
                patch: 0,
                prerelease: String::from(""),
                build: String::from("a")
            }
        );
        assert_eq!(
            parse("12.13"),
            super::SemVer {
                major: 12,
                minor: 13,
                patch: 0,
                prerelease: String::from(""),
                build: String::from("")
            }
        );
    }

    #[test]
    fn test_major() {
        assert_eq!(
            parse("12-pre123+a"),
            super::SemVer {
                major: 12,
                minor: 0,
                patch: 0,
                prerelease: String::from("pre123"),
                build: String::from("a")
            }
        );
        assert_eq!(
            parse("12+a"),
            super::SemVer {
                major: 12,
                minor: 0,
                patch: 0,
                prerelease: String::from(""),
                build: String::from("a")
            }
        );
        assert_eq!(
            parse("12"),
            super::SemVer {
                major: 12,
                minor: 0,
                patch: 0,
                prerelease: String::from(""),
                build: String::from("")
            }
        );
    }
}
