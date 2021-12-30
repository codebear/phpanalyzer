use std::sync::{Arc, RwLock};

use crate::{
    analysis::data::VarData, autonodes::any::AnyNodeRef, autotree::NodeAccess, issue::Issue,
    symbols::Name,
};
use crate::{
    analysis::state::AnalysisState,
    autonodes::variable_name::VariableNameNode,
    issue::IssueEmitter,
    types::union::{DiscreteType, UnionType},
    value::PHPValue,
};

use super::{analysis::AnalyzeableRoundTwoNode, lang::AnalysisOfType};

impl VariableNameNode {
    pub fn get_variable_name(&self) -> Name {
        self.child.get_name()
    }

    pub fn get_utype(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        let lock = if let Some(lock) = self.get_var_data(state) {
            lock.clone()
        } else {
            return None;
        };
        let data = lock.read().unwrap();

        if let Some(t) = &data.comment_declared_type {
            return Some(t.clone());
        }
        if let Some(t) = &data.php_declared_type {
            return Some(t.clone());
        }
        if let Some(x) = self.get_inferred_type(state, emitter) {
            Some(x)
        } else {
            data.get_inferred_type()
        }
    }

    pub fn get_var_data(&self, state: &mut AnalysisState) -> Option<Arc<RwLock<VarData>>> {
        let var_name = self.get_variable_name();
        let scope = state.current_scope();
        let read = scope.read().unwrap();
        read.get_var(&var_name)
    }

    fn get_or_create_var_data(&self, state: &mut AnalysisState) -> Arc<RwLock<VarData>> {
        let var_name = self.get_variable_name();
        let scope = state.current_scope();
        let mut write = scope.write().unwrap();
        write.get_or_create_var(var_name).clone()
    }

    ///
    /// Can be called in contexts where the content of the variabel will be extracted.
    /// Will emit if the variable is empty (not written anything to)
    pub fn check_can_be_read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        let var_name = self.get_variable_name();
        let readable = if let Some(v) = self.get_var_data(state) {
            let data = v.read().unwrap();
            if data.is_partial {
                emitter.emit(Issue::VariableNotInitializedInAllBranhces(
                    self.pos(state),
                    var_name.clone(),
                ));
            }
            data.written_to > 0
        } else {
            false
        };
        if !readable {
            emitter.emit(Issue::UnknownVariable(self.pos(state), var_name));
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, _emitter: &dyn IssueEmitter) {
        let lock = if let Some(lock) = self.get_var_data(state) {
            lock.clone()
        } else {
            return;
        };
        let mut data = lock.write().unwrap();
        data.read_from += 1;
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<crate::value::PHPValue> {
        let val_data_handle = self.get_var_data(state)?;
        let val_data = val_data_handle.read().unwrap();
        let data_iter = val_data.written_data.iter();
        let (_, data) = data_iter.as_ref().last()?;
        data.clone()
    }

    pub fn write_to(
        &self,
        state: &mut crate::analysis::state::AnalysisState,
        _emitter: &dyn IssueEmitter,
        val_type: Option<UnionType>,
        value: Option<PHPValue>,
    ) {
        let val_data = self.get_or_create_var_data(state);
        {
            let mut wr_val_data = val_data.write().unwrap();

            wr_val_data.written_data.push((
                val_type.unwrap_or_else(|| DiscreteType::Unknown.into()),
                if state.in_conditional_branch {
                    None
                } else {
                    value
                },
            ));
            wr_val_data.written_to += 1;
        }
    }
}

impl AnalyzeableRoundTwoNode for VariableNameNode {
    fn analyze_round_two(
        &self,
        state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
        _path: &Vec<AnyNodeRef>,
    ) -> bool {
        let vname = self.get_variable_name();
        let curr_scope = state.current_scope();
        let mut scope = curr_scope.write().unwrap();
        let var_ref = scope.get_or_create_var(vname);
        let mut var_data = var_ref.write().unwrap();
        (*var_data).referenced_ranges.push(self.range);
        true
    }
}

impl AnalysisOfType for VariableNameNode {
    fn get_declared_type(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        if state.pass == 1 {
            todo!("Hvorfor kommer vi hit på pass 1");
        }
        let var_name = self.child.get_name();
        let scope = state.current_scope();

        let read = scope.read().unwrap();
        if let Some(var_data) = read.vars.get(&var_name) {
            var_data.read().unwrap().get_declared_type()
        } else {
            emitter.emit(Issue::UnknownVariable(self.pos(state), var_name.clone()));
            None
        }
    }

    fn get_inferred_type(
        &self,
        state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        let scope_handle = state.current_scope();
        let scope = scope_handle.read().ok()?;

        let var_name = self.get_variable_name();
        let var_data_handle = scope.get_var(&var_name)?;
        let var_data = var_data_handle.read().ok()?;
        if let Some((vtype, _)) = var_data.written_data.last() {
            Some(vtype.clone())
        } else {
            return None;
        }
    }
}