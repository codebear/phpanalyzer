use crate::{
    autonodes::_expression::_ExpressionNode, issue::IssueEmitter, missing, types::union::PHPType,
    value::PHPValue,
};

impl _ExpressionNode {
    pub fn write_to(
        &self,
        state: &mut crate::analysis::state::AnalysisState,
        emitter: &dyn IssueEmitter,
        val_type: Option<PHPType>,
        value: Option<PHPValue>,
    ) {
        match self {
            _ExpressionNode::_PrimaryExpression(pe) => pe.write_to(state, emitter, val_type, value),
            _ExpressionNode::AssignmentExpression(_) => missing!(),
            _ExpressionNode::AugmentedAssignmentExpression(_) => missing!(),
            _ExpressionNode::BinaryExpression(_) => missing!(),
            _ExpressionNode::CastExpression(_) => missing!(),
            _ExpressionNode::CloneExpression(_) => missing!(),
            _ExpressionNode::ConditionalExpression(_) => missing!(),
            _ExpressionNode::IncludeExpression(_) => missing!(),
            _ExpressionNode::IncludeOnceExpression(_) => missing!(),
            _ExpressionNode::MatchExpression(_) => missing!(),
            _ExpressionNode::ReferenceAssignmentExpression(_) => missing!(),
            _ExpressionNode::RequireExpression(_) => missing!(),
            _ExpressionNode::RequireOnceExpression(_) => missing!(),
            _ExpressionNode::ErrorSuppressionExpression(_) => missing!(),
            _ExpressionNode::UnaryOpExpression(_) => missing!(),
            _ExpressionNode::YieldExpression(_) => missing!(),

            _ExpressionNode::Extra(_) => missing!(),
        }
    }
}
