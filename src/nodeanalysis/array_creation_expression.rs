use std::{convert::TryInto, os::unix::prelude::OsStrExt};

use crate::autonodes::array_element_initializer::ArrayElementInitializerValue;
// use crate::autonodes::array_element_initializer::ArrayElementInitializerChildren;
use crate::autotree::NodeAccess;
use crate::types::union::PHPType;
use crate::value::PHPArray;
use crate::{
    analysis::state::AnalysisState,
    autonodes::array_creation_expression::ArrayCreationExpressionNode,
    issue::IssueEmitter,
    missing_none,
    types::union::{DiscreteType, UnionType},
    value::PHPValue,
};

impl ArrayCreationExpressionNode {
    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        if self.children.is_empty() {
            return;
        }
        for child in &self.children {
            child.read_from(state, emitter);
        }
    }

    pub fn get_utype(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPType> {
        if self.children.is_empty() {
            return Some(DiscreteType::Array.into());
        }

        let mut has_some_keys = false;

        let mut value_types = vec![];
        let mut key_types = vec![];
        for child in &self.children {
            if child.spread.is_some() {
                // Noen barn har spread, da gir vi opp
                return missing_none!("{}: Finn ut av array-type med spread", self.brief_desc());
            }
            if let Some(key) = &child.key {
                has_some_keys = true;
                let key_type = key.get_utype(state, emitter)?;
                key_types.push(key_type);
            }
            let value_type = child.value.as_ref()?.get_utype(state, emitter)?;
            value_types.push(value_type);
        }
        let value_types = UnionType::flatten(value_types).into();
        if has_some_keys {
            let key_types = UnionType::flatten(key_types).into();
            Some(DiscreteType::HashMap(key_types, value_types).into())
        } else {
            Some(DiscreteType::Vector(value_types).into())
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        let mut entries: Vec<(PHPValue, PHPValue)> = vec![];
        let mut idx: usize = 0;
        // Increment index for each element
        // If key is numeric and positive, set the index to key,
        // else don't increment index
        let mut hashmap = false;
        for child in &self.children {
            // idx += 1;
            let entry_key = if let Some(key) = &child.key {
                if let Some(key_val) = key.get_php_value(state, emitter) {
                    hashmap = true;
                    match key_val {
                        PHPValue::Int(ival) if ival > 0 && idx < ival.try_into().unwrap() => {
                            // FIXME this should probably not unwrap, but result in a None return
                            idx = (ival + 1).try_into().unwrap();
                            key_val
                        }
                        PHPValue::String(ref s) => {
                            let s_bytes = s.as_bytes();
                            let is_numeric = if s.len() == 0 {
                                false
                            } else if s_bytes[0] >= b'0' && s_bytes[0] <= b'9' {
                                return crate::missing_none!("Sjekk om string er numerisk");
                            } else {
                                false
                            };

                            if is_numeric {
                                return crate::missing_none!("korriger index-teller");
                            }
                            key_val
                        }
                        _ => key_val,
                    }
                } else {
                    // If one of the values are unknown, we can't reliably build the array
                    return None;
                }
            } else {
                let v = PHPValue::Int(idx.try_into().unwrap());
                idx += 1;
                v
            };
            let entry_val = if let Some(val_ref) = &child.value {
                match &**val_ref {
                    ArrayElementInitializerValue::_Expression(e) => e.get_php_value(state, emitter),
                    ArrayElementInitializerValue::ByRef(v) => v.get_php_value(state, emitter),

                    ArrayElementInitializerValue::Extra(_) => todo!(),
                }
            } else {
                None
            };
            if let Some(entry_val) = entry_val {
                entries.push((entry_key, entry_val));
            } else {
                // If we're missing a value, we'll abandon the whole array
                return None;
            }
        }
        if !entries.is_empty() {
            if hashmap {
                Some(PHPValue::Array(PHPArray::HashMap(entries)))
            } else {
                Some(PHPValue::Array(PHPArray::Vector(
                    entries.iter().map(|x| x.1.clone()).collect(),
                )))
            }
        } else {
            Some(PHPValue::Array(PHPArray::Empty))
        }
    }
}
