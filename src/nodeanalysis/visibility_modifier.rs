use crate::{
    analysis::state::AnalysisState, autonodes::visibility_modifier::VisibilityModifierNode,
    issue::IssueEmitter, symboldata::class::ClassMemberVisibility, types::union::UnionType,
};

impl VisibilityModifierNode {
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
    ) -> Option<UnionType> {
        crate::missing_none!("{}.get_utype(..)", self.kind())
    }

    pub fn get_visibility(&self) -> ClassMemberVisibility {
        match &self.raw[..] {
            b"public" => ClassMemberVisibility::Public,
            b"private" => ClassMemberVisibility::Private,
            b"protected" => ClassMemberVisibility::Protected,
            _ => {
                panic!("Unknown visibility: {:?}", self.get_raw());
            }
        }
    }
}
