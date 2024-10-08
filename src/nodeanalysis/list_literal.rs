use crate::{
    analysis::state::AnalysisState,
    autonodes::list_literal::{ListLiteralChildren, ListLiteralNode},
    issue::IssueEmitter,
    types::union::PHPType,
    value::PHPValue,
};

impl ListLiteralNode {
    pub fn read_from(&self, _state: &mut AnalysisState, _emitter: &dyn IssueEmitter) {
        crate::missing!("{}.read_from(..)", self.kind());
    }

    pub fn get_php_value(
        &self,
        _state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<crate::value::PHPValue> {
        crate::missing_none!("{}.get_php_value(..)", self.kind())
    }

    pub fn get_utype(
        &self,
        _state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<PHPType> {
        crate::missing_none!("{}.get_utype(..)", self.kind())
    }

    pub fn write_to(
        &self,
        state: &mut crate::analysis::state::AnalysisState,
        emitter: &dyn IssueEmitter,
        val_type: Option<PHPType>,
        value: Option<PHPValue>,
    ) {
        if val_type.is_some() {
            crate::missing!("list(..) = .. har type, som blir ignorert");
        }
        if value.is_some() {
            crate::missing!("list(..) = .. har verdi, som blir ignorert");
        }
        for (idx, child) in self.children.iter().enumerate() {
            match &**child {
                ListLiteralChildren::_Expression(_) => {
                    crate::missing!("list({:?}) write to", child.kind())
                }
                ListLiteralChildren::ByRef(_) => {
                    crate::missing!("list({:?}) write to", child.kind())
                }
                ListLiteralChildren::DynamicVariableName(_) => {
                    crate::missing!("list({:?}) write to", child.kind())
                }
                ListLiteralChildren::FunctionCallExpression(_) => {
                    crate::missing!("list({:?}) write to", child.kind())
                }
                ListLiteralChildren::ListLiteral(_) => {
                    crate::missing!("list({:?}) write to", child.kind())
                }
                ListLiteralChildren::MemberAccessExpression(_) => {
                    crate::missing!("list({:?}) write to", child.kind())
                }
                ListLiteralChildren::MemberCallExpression(_) => {
                    crate::missing!("list({:?}) write to", child.kind())
                }
                ListLiteralChildren::NullsafeMemberAccessExpression(_) => {
                    crate::missing!("list({:?}) write to", child.kind())
                }
                ListLiteralChildren::NullsafeMemberCallExpression(_) => {
                    crate::missing!("list({:?}) write to", child.kind())
                }
                ListLiteralChildren::ScopedCallExpression(_) => {
                    crate::missing!("list({:?}) write to", child.kind())
                }
                ListLiteralChildren::ScopedPropertyAccessExpression(_) => {
                    crate::missing!("list({:?}) write to", child.kind())
                }
                ListLiteralChildren::SubscriptExpression(_) => {
                    crate::missing!("list({:?}) write to", child.kind())
                }
                ListLiteralChildren::VariableName(vname) => {
                    let mut sub_val_type = None;
                    let mut sub_value = None;
                    match &value {
                        Some(PHPValue::Array(a)) => {
                            let php_idx = PHPValue::Int(
                                idx.try_into()
                                    .expect("This loop can't be so long that it overflows an i64"),
                            );
                            sub_value = a.get_value_by_key(php_idx.clone());
                            sub_val_type = a.get_type_by_key(php_idx);
                        }
                        Some(v) => {
                            crate::missing!("Extract something fra value in list(...) = {:?}", v);
                        }
                        None => match &val_type {
                            Some(x) => {
                                crate::missing!("list(..) = type: {:?}", x);
                            }
                            None => (),
                        },
                    }
                    vname.write_to(state, emitter, sub_val_type, sub_value);
                }

                ListLiteralChildren::Extra(_) => (),
            }
        }
        crate::missing!("list literal write_to");
    }
}
