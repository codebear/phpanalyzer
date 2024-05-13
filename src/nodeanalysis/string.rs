use std::{
    ffi::{OsStr, OsString},
    os::unix::prelude::OsStrExt,
};

use crate::{
    analysis::state::AnalysisState,
    autonodes::string::{StringChildren, StringNode},
    issue::IssueEmitter,
    types::union::{DiscreteType, UnionType},
    value::PHPValue,
};

impl StringNode {
    pub fn read_from(&self, _state: &mut AnalysisState, _emitter: &dyn IssueEmitter) {
        ()
    }

    pub fn get_php_value(
        &self,
        _state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        let mut buf = OsString::new();
        for part in &self.children {
            buf.push(OsStr::from_bytes(part.get_string_value()?));
        }

        Some(PHPValue::String(buf))
    }

    pub fn get_utype(
        &self,
        _state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        Some(DiscreteType::String.into())
    }
}

impl StringChildren {
    pub fn get_string_value(&self) -> Option<&[u8]> {
        match self {
            StringChildren::Extra(y) => todo!(),
            StringChildren::EscapeSequence(y) => todo!(),
            StringChildren::StringValue(y) => {
                let len = y.raw.len();
                if len < 2 {
                    return None;
                }
                let raw = &y.raw[1..len - 1];
                Some(&raw)
            }
        }
    }
}
