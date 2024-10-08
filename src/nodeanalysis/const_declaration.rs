use crate::{
    analysis::state::AnalysisState,
    autonodes::const_declaration::{ConstDeclarationChildren, ConstDeclarationNode},
    issue::{Issue, IssueEmitter},
    symboldata::class::ClassType,
    types::union::PHPType,
};

use super::analysis::FirstPassAnalyzeableNode;
use crate::autotree::NodeAccess;

impl ConstDeclarationNode {
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
}

impl FirstPassAnalyzeableNode for ConstDeclarationNode {
    fn analyze_first_pass(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        // Finn ut av
        // self.attributes;
        // Finn ut av
        if let Some(modi) = &self.modifier {
            crate::missing!("Const har en {:?}, hva gjør vi med det?", modi.get_raw());
        }

        for child in &self.children {
            match &**child {
                ConstDeclarationChildren::ConstElement(c) => {
                    let name = c.get_const_name();
                    let maybe_value = c.get_php_value(state, emitter);
                    if maybe_value.is_none() {
                        emitter.emit(Issue::ParseAnomaly(
                            self.pos(state),
                            format!(
                                "Couldn't resolve class const content for {:?} from {}",
                                name,
                                c.kind()
                            )
                            .into(),
                        ));
                    }
                    // let value = c.get_php_value(state, emitter);

                    let Some(class_state) = &state.in_class else {
                        // Global const?
                        eprintln!("Global const decls?");
                        todo!("Const: self::{:?} = {:?} ({:?})", name, maybe_value, c);
                        continue;
                    };

                    let Some(class_data) = state.symbol_data.get_class(&class_state.get_name())
                    else {
                        eprintln!("Missing class: {:?}", class_state.get_name());
                        // Finner ikke klassen?
                        continue;
                    };

                    let mut mutable = class_data.write().unwrap();
                    match &mut (*mutable) {
                        ClassType::Class(c) => {
                            if c.constants.get(&name).is_some() {
                                emitter.emit(Issue::DuplicateClassConstant(
                                    self.pos(state),
                                    class_state.get_name().get_fq_name().clone(),
                                    name,
                                ));
                            } else {
                                /*eprintln!(
                                    "Inject Class {}::{} = {:?}",
                                    class_state.get_name(),
                                    name,
                                    value
                                );*/
                                c.constants.insert(name, maybe_value);
                            }
                        }
                        ClassType::None => todo!(),
                        ClassType::Interface(intf) => {
                            if intf.constants.get(&name).is_some() {
                                emitter.emit(Issue::DuplicateClassConstant(
                                    self.pos(state),
                                    class_state.get_name().get_fq_name().clone(),
                                    name,
                                ));
                            } else {
                                /*eprintln!(
                                    "Inject Interface {}::{} = {:?}",
                                    class_state.get_name(),
                                    name,
                                    value
                                );*/
                                intf.constants.insert(name, maybe_value);
                            }
                        }
                        ClassType::Trait(_) => todo!(),
                    }
                }
                ConstDeclarationChildren::VisibilityModifier(v) => todo!("analysere: {:?}", v),
                _ => continue,
            }
        }
    }
}
