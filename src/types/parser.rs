use std::{
    cell::Cell,
    ffi::{OsStr, OsString},
    os::unix::prelude::OsStrExt,
    sync::atomic::{AtomicBool, Ordering},
};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{digit1, multispace0, space0},
    combinator::opt,
    error::{Error, ErrorKind},
    multi::{many1, separated_list1},
    sequence::{delimited, preceded, terminated},
    Err, IResult,
};

use crate::symbols::{FullyQualifiedName, Name};

use super::parse_types::{
    ArgumentVector, ConcreteType, ParsedType, ReturnType, ShapeEntry, ShapeKey, TypeName,
    TypeStruct, UnionOfTypes,
};

///
/// Make sure that all parse-function only accepts whitespace _before_ expected content. We don't want to
/// parse trailing whitespace, at it might have semantic meaning to others
///

fn nullable(input: &[u8]) -> IResult<&[u8], bool> {
    let (input, _) = tag(b"?")(input)?;
    Ok((input, true))
}

fn type_name(input: &[u8]) -> IResult<&[u8], TypeName> {
    alt((relative_name, qualified_name, only_simple_type_name))(input)
}

fn only_simple_type_name(input: &[u8]) -> IResult<&[u8], TypeName> {
    let (input, name) = simple_type_name(input)?;
    Ok((input, TypeName::Name(name)))
}

fn qualified_name(input: &[u8]) -> IResult<&[u8], TypeName> {
    let (input, names) = many1(preceded(tag(b"\\"), simple_type_name))(input)?;

    Ok((input, TypeName::FQName(FullyQualifiedName::from(names))))
}

fn relative_name(input: &[u8]) -> IResult<&[u8], TypeName> {
    let mut path = vec![];
    let (input, name) = simple_type_name(input)?;
    path.push(name);
    let (input, names) = many1(preceded(tag(b"\\"), simple_type_name))(input)?;
    path.extend(names);
    Ok((input, TypeName::RelativeName(path)))
}

fn simple_type_name(input: &[u8]) -> IResult<&[u8], Name> {
    let second = AtomicBool::new(false);
    let (input, result) = take_while1(move |x: u8| {
        let sec = second.load(Ordering::Relaxed);
        second.store(true, Ordering::Relaxed);
        x == b'_'
            || if sec {
                // We allow dash in type names, to allow for `class-string` and similar
                x.is_ascii_alphanumeric() || x == b'-'
            } else {
                x.is_ascii_alphabetic()
            }
    })(input)?;

    Ok((input, Name::from(result)))
}

fn php_var_name(input: &[u8]) -> IResult<&[u8], OsString> {
    #[derive(Clone, Copy)]
    enum State {
        First,
        Dollar,
        Alpha,
    }
    let state = Cell::new(State::First);

    let (input, result) = take_while1(move |x: u8| match state.get() {
        State::First if x == b'$' => {
            state.set(State::Dollar);
            true
        }
        State::First => false,
        State::Dollar if x.is_ascii_alphabetic() => {
            state.set(State::Alpha);
            true
        }
        State::Dollar => false,
        State::Alpha => x.is_ascii_alphanumeric(),
    })(input)?;
    Ok((input, OsStr::from_bytes(result).into()))
}

pub fn union_type_with_colon(multiline: bool) -> impl Fn(&[u8]) -> IResult<&[u8], UnionOfTypes> {
    move |input| {
        let (input, _) = ourspace0(multiline)(input)?;
        let (input, _) = tag(":")(input)?;
        let (input, _) = ourspace0(multiline)(input)?;
        separated_list1(union_separator(multiline), concrete_type(multiline))(input)
        //      union_type(multiline)
    }
}

pub fn union_type(multiline: bool) -> impl Fn(&[u8]) -> IResult<&[u8], UnionOfTypes> {
    move |input| {
        let (input, _) = ourspace0(multiline)(input)?;
        separated_list1(union_separator(multiline), concrete_type(multiline))(input)
    }
}

pub fn only_generic_args(multiline: bool) -> impl Fn(&[u8]) -> IResult<&[u8], Vec<UnionOfTypes>> {
    move |input| {
        let (input, _) = ourspace0(multiline)(input)?;
        generic_args(multiline)(input)
    }
}

fn union_separator(multiline: bool) -> impl Fn(&[u8]) -> IResult<&[u8], ()> {
    move |input| {
        let (input, _) = ourspace0(multiline)(input)?;
        let (input, _) = tag(b"|")(input)?;
        Ok((input, ()))
    }
}

fn normal_type(multiline: bool) -> impl Fn(&[u8]) -> IResult<&[u8], ParsedType> {
    move |input| {
        let (input, _) = ourspace0(multiline)(input)?;
        let (input, type_name) = type_name(input)?;

        let (input, generics) = opt(generic_args(multiline))(input)?;

        let type_struct = TypeStruct {
            type_name,
            generics,
        };

        Ok((input, ParsedType::Type(type_struct)))
    }
}

fn shape_type(multiline: bool) -> impl Fn(&[u8]) -> IResult<&[u8], ParsedType> {
    move |input| {
        let (input, _) = tag(b"array")(input)?;
        let (input, _) = ourspace0(multiline)(input)?;
        let (input, _) = tag(b"{")(input)?;
        let (input, _) = ourspace0(multiline)(input)?;
        let (input, entries) =
            separated_list1(generic_separator(multiline), shape_entry(multiline))(input)?;
        let (input, _) = ourspace0(multiline)(input)?;
        let (input, _) = tag(b"}")(input)?;
        Ok((input, ParsedType::Shape(entries)))
    }
}

fn shape_key_num(input: &[u8]) -> IResult<&[u8], ShapeKey> {
    let (input, digits) = digit1(input)?;

    // digits should only contain ... digits, so this should be perfectly safe
    let str: String = unsafe { String::from_utf8_unchecked(digits.to_vec()) };
    let ival: i64 = if let Ok(i) = str.parse::<i64>() {
        i
    } else {
        return Err(Err::Error(Error {
            input,
            code: ErrorKind::Digit,
        }));
    };

    Ok((input, ShapeKey::Num(ival)))
}

fn shape_key_str(input: &[u8]) -> IResult<&[u8], ShapeKey> {
    let (input, str_key) = simple_type_name(input)?;

    Ok((input, ShapeKey::String(str_key)))
}

fn shape_key(multiline: bool) -> impl Fn(&[u8]) -> IResult<&[u8], (ShapeKey, bool)> {
    move |input| {
        let (input, _) = ourspace0(multiline)(input)?;

        let (input, key) = alt((shape_key_num, shape_key_str))(input)?;
        let (input, _) = ourspace0(multiline)(input)?;
        let (input, optional_questionmark) = opt(tag("?"))(input)?;
        let optional = optional_questionmark.is_some();
        let (input, _) = ourspace0(multiline)(input)?;
        let (input, _) = tag(b":")(input)?;
        Ok((input, (key, optional)))
    }
}

fn shape_entry(multiline: bool) -> impl Fn(&[u8]) -> IResult<&[u8], ShapeEntry> {
    move |input| {
        let (input, _) = ourspace0(multiline)(input)?;
        let (input, key) = opt(shape_key(multiline))(input)?;
        let (input, _) = ourspace0(multiline)(input)?;
        let (input, ctype) = union_type(multiline)(input)?;
        Ok((input, ShapeEntry(key, ctype)))
    }
}

fn concrete_type(multiline: bool) -> impl Fn(&[u8]) -> IResult<&[u8], ConcreteType> {
    move |input| {
        let (input, _) = ourspace0(multiline)(input)?;
        let (input, nullable) = opt(nullable)(input)?;
        let (input, mut parsed_type) = one_type(multiline)(input)?;
        // Handle any `thing[]`-declarations
        let mut iter_input = input;
        loop {
            let (input, _) = ourspace0(multiline)(iter_input)?;

            let (input, post_decl_array) = opt(many1(tag(b"[]")))(input)?;
            if let Some(levels) = post_decl_array {
                for _ in levels {
                    // Wrap parsed_type in an array, converting from `thing[]` to `array<thing>`
                    parsed_type = ParsedType::Type(TypeStruct {
                        type_name: TypeName::Name(Name::from("array")),
                        generics: Some(vec![vec![ConcreteType {
                            nullable: false,
                            ptype: parsed_type,
                        }]]),
                    })
                }
                // prepare for next iteration
                iter_input = input;
            } else {
                break;
            }
        }
        let input = iter_input;
        let nullable = nullable.unwrap_or(false);

        let concrete_type = ConcreteType {
            nullable,
            ptype: parsed_type,
        };
        Ok((input, concrete_type))
    }
}
fn one_type(multiline: bool) -> impl Fn(&[u8]) -> IResult<&[u8], ParsedType> {
    move |input| {
        alt((
            class_type(multiline),
            shape_type(multiline),
            callable_type(multiline),
            tuple_type(multiline),
            normal_type(multiline),
        ))(input)
    }
}

fn generic_separator(multiline: bool) -> impl Fn(&[u8]) -> IResult<&[u8], ()> {
    move |input| {
        let (input, _) = ourspace0(multiline)(input)?;
        let (input, _) = tag(b",")(input)?;
        Ok((input, ()))
    }
}

fn ourspace0(multiline: bool) -> impl Fn(&[u8]) -> IResult<&[u8], &[u8]> {
    move |input| {
        if multiline {
            multispace0(input)
        } else {
            space0(input)
        }
    }
}

fn generic_args(multiline: bool) -> impl Fn(&[u8]) -> IResult<&[u8], Vec<Vec<ConcreteType>>> {
    move |input| {
        let (input, _) = ourspace0(multiline)(input)?;
        let (input, _) = tag(b"<")(input)?;
        let (input, _) = ourspace0(multiline)(input)?;
        let (input, types) = alt((
            move |inp| -> IResult<&[u8], Vec<Vec<ConcreteType>>> {
                // Special-case to map `<*>` to `<mixed>`...
                let (inp, _) = tag(b"*")(inp)?;

                let ptype = ParsedType::Type(TypeStruct {
                    type_name: TypeName::Name(Name::from("mixed")),
                    generics: None,
                });
                let concrete_type = ConcreteType {
                    nullable: false,
                    ptype,
                };
                Ok((inp, vec![vec![concrete_type]]))
            },
            separated_list1(generic_separator(multiline), union_type(multiline)),
        ))(input)?;
        let (input, _) = ourspace0(multiline)(input)?;
        let (input, _) = tag(b">")(input)?;
        Ok((input, types))
    }
}

fn callable_type(multiline: bool) -> impl Fn(&[u8]) -> IResult<&[u8], ParsedType> {
    move |input| {
        let (input, _) = ourspace0(multiline)(input)?;
        let (input, _) = tag(b"callable")(input)?;
        let (input, details) = opt(callable_details(multiline))(input)?;
        let ptype = if let Some((types, return_type)) = details {
            ParsedType::Callable(types, return_type)
        } else {
            ParsedType::CallableUntyped
        };
        Ok((input, ptype))
    }
}

fn callable_details(
    multiline: bool,
) -> impl Fn(&[u8]) -> IResult<&[u8], (ArgumentVector, Option<ReturnType>)> {
    move |input| {
        let (input, _) = ourspace0(multiline)(input)?;
        let (input, _) = tag(b"(")(input)?;
        let (input, _) = ourspace0(multiline)(input)?;
        let (input, types) = separated_list1(
            generic_separator(multiline),
            terminated(
                union_type(multiline),
                opt(preceded(ourspace0(multiline), php_var_name)),
            ),
        )(input)?;
        let (input, _) = ourspace0(multiline)(input)?;
        let (input, _) = tag(b")")(input)?;
        let (input, return_type) = opt(callable_return_type(multiline))(input)?;

        Ok((input, (types, return_type)))
    }
}

fn callable_return_type(multiline: bool) -> impl Fn(&[u8]) -> IResult<&[u8], ReturnType> {
    move |input: &[u8]| -> IResult<&[u8], ReturnType> {
        let (input, _) = ourspace0(multiline)(input)?;
        let (input, _) = tag(b":")(input)?;
        let (input, _) = ourspace0(multiline)(input)?;
        callable_return_type_details(multiline)(input)
    }
}

fn callable_return_type_details(multiline: bool) -> impl Fn(&[u8]) -> IResult<&[u8], ReturnType> {
    move |input: &[u8]| -> IResult<&[u8], ReturnType> {
        if let (input, Some(return_type)) =
            opt(delimited(tag(b"("), union_type(multiline), tag(b")")))(input)?
        {
            return Ok((input, return_type));
        };
        let (input, ctype) = concrete_type(multiline)(input)?;
        Ok((input, vec![ctype]))
    }
}

fn class_type(multiline: bool) -> impl Fn(&[u8]) -> IResult<&[u8], ParsedType> {
    move |input| {
        let (input, _) = ourspace0(multiline)(input)?;
        // FIXME type-name, tillater dash i "klasse"-navn, det er uheldig
        let (input, cname) = type_name(input)?;
        let (input, _) = ourspace0(multiline)(input)?;
        let (input, _) = tag(b"::")(input)?;
        let (input, _) = ourspace0(multiline)(input)?;
        let (input, tname) = simple_type_name(input)?;
        Ok((input, ParsedType::ClassType(cname, tname)))
    }
}

fn tuple_type(multiline: bool) -> impl Fn(&[u8]) -> IResult<&[u8], ParsedType> {
    move |input| {
        let (input, mut noe) = delimited(
            tag(b"("),
            separated_list1(tag(b","), union_type(multiline)),
            tag(b")"),
        )(input)?;

        let shape_entries: Vec<ShapeEntry> = noe
            .drain(..)
            .map(|vtypes| ShapeEntry(None, vtypes))
            .collect();

        Ok((input, ParsedType::Shape(shape_entries)))
    }
}
