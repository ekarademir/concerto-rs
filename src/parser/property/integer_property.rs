use nom::{
    branch::alt,
    character::complete::{char, space0, space1},
    combinator::into,
    error::context,
    multi::fold_many_m_n,
    sequence::{preceded, tuple},
    Parser,
};
use serde_derive::Serialize;

use crate::parser::{
    common::{keywords, numeric::integer_value},
    property::internal::{primitive_property, ranged_parser, PrimitiveType, Ranged},
    CResult,
};

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct IntegerProperty {
    #[serde(rename = "$class")]
    pub class: String,
    pub name: String,
    #[serde(rename = "isOptional")]
    pub is_optional: bool,
    #[serde(rename = "isArray")]
    pub is_array: bool,
    #[serde(rename = "default")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<i32>,
    #[serde(rename = "range")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_validator: Option<IntegerDomainValidator>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IntegerDomainValidator {
    pub lower: Option<i32>,
    pub upper: Option<i32>,
}

impl serde::Serialize for IntegerDomainValidator {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&String::from(self))
    }
}

impl From<&IntegerDomainValidator> for String {
    fn from(value: &IntegerDomainValidator) -> Self {
        match (value.lower, value.upper) {
            (None, None) => Self::from(""),
            (Some(lower), Some(upper)) => format!("[{}, {}]", lower, upper),
            (None, Some(upper)) => format!("[, {}]", upper),
            (Some(lower), None) => format!("[{},]", lower),
        }
    }
}

impl From<Ranged<i32>> for IntegerDomainValidator {
    fn from(value: Ranged<i32>) -> Self {
        Self {
            lower: value.start,
            upper: value.end,
        }
    }
}
enum IntegerMetaProperty {
    Default(i32),
    Domain(IntegerDomainValidator),
    Optional,
}

/// Parses a primitive IntegerProperty with its default meta properties.
/// If a meta property is defined twice, second one will overwrite the first.
/// Meta property parser will only run three times.
pub fn integer_property<'a>(input: &'a str) -> CResult<&'a str, IntegerProperty> {
    let domain = context(
        "IntegerDomainValidator",
        preceded(space1, integer_domain_validator),
    )
    .map(|x| IntegerMetaProperty::Domain(x));
    let default = preceded(space1, integer_default_value).map(|x| IntegerMetaProperty::Default(x));
    let optional = preceded(space1, keywords::optional).map(|_| IntegerMetaProperty::Optional);

    let property_meta = context("PropertyMeta", alt((domain, default, optional)));

    context(
        "IntegerProperty",
        primitive_property(PrimitiveType::IntegerPropertyType)
            .and(fold_many_m_n(
                0,
                3,
                property_meta,
                Vec::new,
                |mut acc: Vec<_>, meta_prop| {
                    acc.push(meta_prop);
                    acc
                },
            ))
            .map(|((property_name, is_array), meta_props)| {
                let mut prop = IntegerProperty {
                    class: String::from("IntegerProperty"),
                    name: property_name.to_string(),
                    default_value: None,
                    domain_validator: None,
                    is_optional: false,
                    is_array,
                };

                for meta_prop in meta_props {
                    use IntegerMetaProperty::*;
                    match meta_prop {
                        Default(x) => prop.default_value = Some(x),
                        Domain(x) => prop.domain_validator = Some(x),
                        Optional => prop.is_optional = true,
                    }
                }

                prop
            }),
    )(input)
}

pub fn integer_default_value<'a>(input: &'a str) -> CResult<&'a str, i32> {
    into(context(
        "IntegerDefaultValue",
        preceded(
            tuple((keywords::default, space0, char('='), space0)),
            integer_value,
        ),
    ))(input)
}

pub fn integer_domain_validator<'a>(input: &'a str) -> CResult<&'a str, IntegerDomainValidator> {
    match ranged_parser(input, keywords::range, integer_value) {
        Err(e) => Err(e),
        Ok((remains, ranged)) => Ok((remains, ranged.into())),
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_integer_property() {
        assert_eq!(
            super::integer_property("o Integer foo"),
            Ok((
                "",
                super::IntegerProperty {
                    class: String::from("IntegerProperty"),
                    name: String::from("foo"),
                    default_value: None,
                    domain_validator: None,
                    is_optional: false,
                    is_array: false,
                }
            )),
            "Should parse integer with no meta properties"
        );

        assert_eq!(
            super::integer_property("o Integer[\t] foo"),
            Ok((
                "",
                super::IntegerProperty {
                    class: String::from("IntegerProperty"),
                    name: String::from("foo"),
                    default_value: None,
                    domain_validator: None,
                    is_optional: false,
                    is_array: true,
                }
            )),
            "Should parse array flag"
        );

        assert_eq!(
            super::integer_property("o Integer baz default=42"),
            Ok((
                "",
                super::IntegerProperty {
                    class: String::from("IntegerProperty"),
                    name: String::from("baz"),
                    default_value: Some(42),
                    domain_validator: None,
                    is_optional: false,
                    is_array: false,
                }
            )),
            "Should parse integer with default value only"
        );

        assert_eq!(
            super::integer_property("o Integer baz    range   = [ 0 , 10  ]"),
            Ok((
                "",
                super::IntegerProperty {
                    class: String::from("IntegerProperty"),
                    name: String::from("baz"),
                    default_value: None,
                    domain_validator: Some(super::IntegerDomainValidator {
                        lower: Some(0),
                        upper: Some(10)
                    }),
                    is_optional: false,
                    is_array: false,
                }
            )),
            "Should parse integer with range only"
        );

        assert_eq!(
            super::integer_property("o Integer baz    range   = [ 0 , 10  ] optional"),
            Ok((
                "",
                super::IntegerProperty {
                    class: String::from("IntegerProperty"),
                    name: String::from("baz"),
                    default_value: None,
                    domain_validator: Some(super::IntegerDomainValidator {
                        lower: Some(0),
                        upper: Some(10)
                    }),
                    is_optional: true,
                    is_array: false,
                }
            )),
            "Should parse integer with optional flag"
        );

        assert_eq!(
            super::integer_property("o Integer baz \tdefault  =   -42    range=[,100]"),
            Ok((
                "",
                super::IntegerProperty {
                    class: String::from("IntegerProperty"),
                    name: String::from("baz"),
                    default_value: Some(-42),
                    domain_validator: Some(super::IntegerDomainValidator {
                        lower: None,
                        upper: Some(100)
                    }),
                    is_optional: false,
                    is_array: false,
                }
            )),
            "Should parse integer with both default and range"
        );

        assert_eq!(
            super::integer_property("o Integer baz \trange=[,  100 ] \tdefault  =   42"),
            Ok((
                "",
                super::IntegerProperty {
                    class: String::from("IntegerProperty"),
                    name: String::from("baz"),
                    default_value: Some(42),
                    domain_validator: Some(super::IntegerDomainValidator {
                        lower: None,
                        upper: Some(100)
                    }),
                    is_optional: false,
                    is_array: false,
                }
            )),
            "Should parse integer with both default and range in a different order"
        );
    }
}
