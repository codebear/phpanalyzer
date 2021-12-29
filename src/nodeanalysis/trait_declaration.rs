use crate::{
    analysis::state::{AnalysisState, ClassState},
    autonodes::{any::AnyNodeRef, trait_declaration::TraitDeclarationNode},
    issue::IssueEmitter,
    symboldata::{
        class::{ClassName, ClassType, TraitData},
        FileLocation,
    },
    symbols::Name,
    types::union::UnionType,
};

use super::{
    analysis::{AnalyzeableNode, AnalyzeableRoundTwoNode},
    class::{AnalysisOfClassBaseLikeNode, AnalysisOfDeclaredNameNode},
};

use crate::autotree::NodeAccess;

impl TraitDeclarationNode {
    pub fn read_from(&self, _state: &mut AnalysisState, _emitter: &dyn IssueEmitter) {
        // void
    }

    pub fn get_php_value(
        &self,
        _state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<crate::value::PHPValue> {
        None
    }

    pub fn get_utype(
        &self,
        _state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        None
    }

    pub fn get_trait_name(&self, state: &mut AnalysisState) -> ClassName {
        let decl_trait_name = self.get_declared_name();
        ClassName::new_with_analysis_state_without_aliasing(&decl_trait_name, state)
    }
}
///
/// TRAITS
///
impl AnalysisOfDeclaredNameNode for TraitDeclarationNode {
    fn get_declared_name(&self) -> Name {
        self.name.get_name()
    }
}
impl AnalysisOfClassBaseLikeNode for TraitDeclarationNode {}

impl AnalyzeableNode for TraitDeclarationNode {
    fn analyze_round_one(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        let trait_name = self.get_trait_name(state);
        let base_name = self.get_declared_base_class_name(state, emitter);

        let mut trait_data =
            TraitData::new(FileLocation::new(self.name.pos(state)), trait_name.clone());
        trait_data.base_name = base_name;

        let symbol_data = state.symbol_data.get_or_create_class(&trait_name);
        {
            let mut unlocked = symbol_data.write().unwrap();
            match *unlocked {
                ClassType::None => {
                    *unlocked = ClassType::Trait(trait_data);
                }
                _ => panic!("NOE DUPS?"),
            }
        }

        state.last_doc_comment = None;
        state.in_class = Some(ClassState::Trait(trait_name));
        self.analyze_round_one_children(&self.as_any(), state, emitter);
        state.in_class = None;
    }
}

impl AnalyzeableRoundTwoNode for TraitDeclarationNode {
    fn analyze_round_two(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
        path: &Vec<AnyNodeRef>,
    ) -> bool {
        let trait_name = self.get_trait_name(state);
        state.last_doc_comment = None;
        state.in_class = Some(ClassState::Trait(trait_name));
        let carry_on = self.analyze_round_two_children(&self.as_any(), state, emitter, path);
        state.in_class = None;

        carry_on
    }
}
