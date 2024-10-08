use crate::{types::union::DiscreteType, value::PHPValue, Range};

use super::{
    binary::{BinaryOperator, BinaryOperatorOperandAccess},
    operator::Operator,
};
#[derive(Clone, Debug)]

pub struct GreaterThanOperator(pub Range);

impl Operator for GreaterThanOperator {
    fn brief_desc(&self) -> String {
        "GreaterThanOperator".into()
    }

    fn operator(&self) -> &'static str {
        ">"
    }

    fn range(&self) -> Range {
        self.0
    }
}

impl BinaryOperator for GreaterThanOperator {
    fn get_operator_utype(
        &self,
        _operands: &impl BinaryOperatorOperandAccess,
        _state: &mut crate::analysis::state::AnalysisState,
        _emitter: &dyn crate::issue::IssueEmitter,
    ) -> Option<crate::types::union::PHPType> {
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
        match (left_value, right_value) {
            (PHPValue::Int(lint), PHPValue::Int(rint)) => Some(PHPValue::Boolean(lint > rint)),
            (PHPValue::Float(lint), PHPValue::Float(rint)) => Some(PHPValue::Boolean(lint > rint)),
            (left, right) => crate::missing_none!(
                "{}[{:?} {} {:?}].get_operator_php_value(..)",
                self.brief_desc(),
                left.get_utype(),
                self.operator(),
                right.get_utype()
            ),
        }
    }
}
