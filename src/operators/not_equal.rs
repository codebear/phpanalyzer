use crate::{
    types::union::{DiscreteType, PHPType},
    value::PHPValue,
    Range,
};

use super::{
    binary::{BinaryOperator, BinaryOperatorOperandAccess},
    operator::Operator,
};
#[derive(Clone, Debug)]

pub struct NotEqualOperator(pub Range);

impl Operator for NotEqualOperator {
    fn brief_desc(&self) -> String {
        todo!()
    }

    fn range(&self) -> Range {
        self.0
    }

    fn operator(&self) -> &'static str {
        "!="
    }
}

impl BinaryOperator for NotEqualOperator {
    fn get_operator_utype(
        &self,
        _operands: &impl BinaryOperatorOperandAccess,
        _state: &mut crate::analysis::state::AnalysisState,
        _emitter: &dyn crate::issue::IssueEmitter,
    ) -> Option<PHPType> {
        Some(DiscreteType::Bool.into())
    }

    fn get_operator_php_value(
        &self,
        operands: &impl BinaryOperatorOperandAccess,
        state: &mut crate::analysis::state::AnalysisState,
        _emitter: &dyn crate::issue::IssueEmitter,
    ) -> Option<crate::value::PHPValue> {
        let left_value = operands.get_left_value(state)?;
        let right_value = operands.get_right_value(state)?;
        let bool_value = left_value.equal_to(&right_value)?;
        Some(PHPValue::Boolean(!bool_value))
    }
}
