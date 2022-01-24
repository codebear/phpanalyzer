use std::{
    ffi::{OsStr, OsString},
    os::unix::prelude::OsStrExt,
};

use nom::Finish;
use tree_sitter::Range;

use crate::types::parse_types::UnionOfTypes;

use super::phpdoc::parse_phpdoc;
use super::position::PHPDocInput;

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum PHPDocEntry {
    /// *  .0 type
    /// *  .1 Name
    /// *  .2 Description (The first word of descripton might be misinterpreted as name)
    Var(Range, UnionOfTypes, Option<OsString>, Option<OsString>),
    /// https://docs.phpdoc.org/guide/references/phpdoc/tags/param.html
    /// *  .0 type
    /// *  .1 Name Not actually optional, but declared as such to allow to parse badly declared params
    /// *  .2 Description  
    Param(Range, UnionOfTypes, Option<OsString>, Option<OsString>),
    /// *  .0 type
    /// *  .2 Description (The first word of descripton might be misinterpreted as name)
    Return(Range, UnionOfTypes, Option<OsString>),
    Description(Range, OsString),
    General(Range, OsString),
    GeneralWithParam(Range, OsString, OsString),

    Anything(Range, OsString),
    EmptyLine(Range),
}

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub struct PHPDocComment {
    pub raw: OsString,
    pub entries: Vec<PHPDocEntry>,
}

impl PHPDocComment {
    pub fn parse(input: &OsString, range: &Range) -> Result<Self, OsString> {
        // -> IResult<&[u8], Vec<PHPDocEntry>> {
        let parse_result = parse_phpdoc(PHPDocInput(input.as_bytes(), range.clone()))
            .map_err(|e| e.map_input(|i| OsStr::from_bytes(i.0)))
            .finish();
        match parse_result {
            Ok((_remainder, entries)) => Ok(Self {
                // FIXME assert that remainder is empty?
                raw: input.clone(),
                entries,
            }),
            Err(parse_err) => {
                todo!("ERR: {:?}", parse_err)
            }
        }
    }
}
