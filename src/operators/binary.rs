use crate::{
    analysis::state::AnalysisState,
    autonodes::{_expression::_ExpressionNode, binary_expression::BinaryExpressionRight},
    issue::IssueEmitter,
    symbols::FullyQualifiedName,
    types::union::UnionType,
    value::PHPValue,
};

#[derive(Debug)]
pub enum InstanceOfSymbol {
    FullyQualifiedName(FullyQualifiedName),
}

pub trait BinaryOperatorOperandAccess {
    fn get_left_value(&self, state: &mut AnalysisState) -> Option<PHPValue>;
    fn get_right_value(&self, state: &mut AnalysisState) -> Option<PHPValue>;
    fn get_left_type(&self, state: &mut AnalysisState) -> Option<UnionType>;
    fn get_right_type(&self, state: &mut AnalysisState) -> Option<UnionType>;

    ///
    /// Instanceof has some special needs that we chose to handle with a separate
    /// support-method in the trait.
    ///
    fn get_right_symbol(&self, state: &mut AnalysisState) -> Option<InstanceOfSymbol> {
        None
    }
}

pub trait BinaryOperator {
    fn get_operator_utype(
        &self,
        _operands: &impl BinaryOperatorOperandAccess,
        _state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        None
    }

    fn get_operator_php_value(
        &self,
        _operands: &impl BinaryOperatorOperandAccess,
        _state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        None
    }
}

pub trait BinaryAssignmentOperator {
    fn get_operator_utype(
        &self,
        _operands: &impl BinaryOperatorOperandAccess,
        _state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        crate::missing_none!("BinaryAssignmentOperator.get_operator_utype(..)")
    }
}

pub trait BinaryOperatorBranchTypeHardening {
    fn branch_with_hardened_types_base_on_conditional_node(
        &self,
        left: &_ExpressionNode,
        right: &BinaryExpressionRight,
        scope: std::sync::Arc<std::sync::RwLock<crate::analysis::scope::Scope>>,
        branch_side: crate::analysis::scope::BranchSide,
        state: &mut crate::analysis::state::AnalysisState,
    ) -> Option<std::sync::Arc<std::sync::RwLock<crate::analysis::scope::Scope>>>;
}
