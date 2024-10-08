use std::sync::{Arc, RwLock};

use crate::{
    analysis::state::AnalysisState,
    autonodes::{
        property_declaration::{PropertyDeclarationModifiers, PropertyDeclarationNode},
        property_element::PropertyElementNode,
    },
    autotree::NodeAccess,
    issue::{Issue, IssueEmitter},
    phpdoc::types::{PHPDocComment, PHPDocEntry},
    symboldata::{
        class::{ClassMemberVisibility, ClassModifier, PropertyData},
        FileLocation,
    },
    symbols::Name,
    types::{type_parser::TypeParser, union::PHPType},
};

impl PropertyElementNode {
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

    pub fn get_property_name(&self) -> Name {
        self.name.get_variable_name()
    }

    pub fn get_property_data(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<Arc<RwLock<PropertyData>>> {
        let property_name = self.get_property_name();
        let class = if let Some(c) = &state.in_class {
            c
        } else {
            emitter.emit(Issue::ParseAnomaly(
                state.pos_from_range(self.range()),
                "Property outside of class".into(),
            ));
            return None;
        };
        state
            .symbol_data
            .get_or_create_property(
                &class.get_name(),
                &property_name,
                FileLocation::new(self.pos(state)),
            )
            .clone()
    }

    pub fn analyze_round_one_with_declaration(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
        declaration: &PropertyDeclarationNode,
    ) {
        let data_handle = if let Some(handle) = self.get_property_data(state, emitter) {
            handle
        } else {
            emitter.emit(Issue::ParseAnomaly(
                self.pos(state),
                "Unable to get property data".into(),
            ));
            crate::missing!("Unable to get property data, maybe an internal bug");
            return;
        };

        let mut modifier = ClassModifier::None;
        let mut visibility = ClassMemberVisibility::Public;
        let mut is_static = false;
        let mut readonly = false;
        for mods in &declaration.modifiers {
            match &**mods {
                PropertyDeclarationModifiers::AbstractModifier(_) => {
                    modifier = ClassModifier::Abstract;
                }
                PropertyDeclarationModifiers::FinalModifier(_) => {
                    modifier = ClassModifier::Final;
                }
                PropertyDeclarationModifiers::StaticModifier(_) => is_static = true,
                PropertyDeclarationModifiers::VarModifier(_) => {
                    visibility = ClassMemberVisibility::Public
                }
                PropertyDeclarationModifiers::VisibilityModifier(v) => {
                    visibility = v.get_visibility()
                }
                PropertyDeclarationModifiers::ReadonlyModifier(_) => {
                    readonly = true;
                }

                PropertyDeclarationModifiers::Extra(_) => (),
            }
        }
        /*eprintln!(
            "declared type: {:?}",
            declaration
                .type_
                .as_ref()
                .and_then(|x| x.get_utype(state, emitter))
        );
        eprintln!(
            "phpdoc type: {:?}",
            self.get_doc_comment_type(state, emitter)
        );*/
        // FIXME se på doc-comment for type
        // FIXME doc_comment ska kanskje ha precedence fremfor declarated_type

        let declared_type = declaration
            .type_
            .as_ref()
            .and_then(|x| x.get_utype(state, emitter));

        let doc_comment =
            if let Some((raw_doc_comment, doc_comment_range)) = state.last_doc_comment.clone() {
                match PHPDocComment::parse(&raw_doc_comment, &doc_comment_range) {
                    Ok(doc_comment) => Some(doc_comment),
                    Err(_) => {
                        emitter.emit(Issue::PHPDocParseError(
                            state.pos_from_range(doc_comment_range),
                        ));
                        None
                    }
                }
            } else {
                None
            };

        let mut comment_type = None;

        if let Some(doc_comment) = &doc_comment {
            for entry in &doc_comment.entries {
                // void
                match entry {
                    PHPDocEntry::Var(range, property_type, _opt_name, _opt_desc) => {
                        comment_type = TypeParser::from_parsed_type(
                            property_type.clone(),
                            state,
                            emitter,
                            None,
                        )
                        .map(|x| (x, *range))
                    }
                    PHPDocEntry::Anything(range, comment) if doc_comment.entries.len() == 1 => {
                        comment_type = TypeParser::parse(comment.clone(), *range, state, emitter)
                            .map(|x| (x, *range));
                    }
                    _ => (),
                }
            }
        };

        {
            let mut data = data_handle.write().unwrap();
            data.declared_type = declared_type;
            data.comment_type = comment_type;
            data.is_static = is_static;
            data.readonly = readonly;
            data.modifier = modifier;
            data.visibility = visibility;
            data.phpdoc = doc_comment;
        }
    }

    pub fn analyze_third_pass_with_declaration(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
        _declaration: &PropertyDeclarationNode,
    ) {
        // eprintln!("round three property element");
        let data_handle = if let Some(handle) = self.get_property_data(state, emitter) {
            handle
        } else {
            emitter.emit(Issue::ParseAnomaly(
                self.pos(state),
                "Unable to get property data".into(),
            ));
            crate::missing!("Unable to get property data, maybe an internal bug");
            return;
        };

        let mut data = data_handle.write().unwrap();

        data.default_value = if let Some(init) = &self.child {
            init.get_php_value(state, emitter)
        } else {
            None
        };
        if let Some(x) = data.comment_type.as_ref() {
            x.0.check_type_casing(x.1, state, emitter)
        }
        if let Some(x) = data.declared_type.as_ref() {
            x.check_type_casing(self.range(), state, emitter)
        }
    }
}
