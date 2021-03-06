use crate::analysis::state::AnalysisState;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::cast_expression::CastExpressionNode;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::dynamic_variable_name::DynamicVariableNameNode;
use crate::autonodes::function_call_expression::FunctionCallExpressionNode;
use crate::autonodes::member_access_expression::MemberAccessExpressionNode;
use crate::autonodes::member_call_expression::MemberCallExpressionNode;
use crate::autonodes::nullsafe_member_access_expression::NullsafeMemberAccessExpressionNode;
use crate::autonodes::nullsafe_member_call_expression::NullsafeMemberCallExpressionNode;
use crate::autonodes::scoped_call_expression::ScopedCallExpressionNode;
use crate::autonodes::scoped_property_access_expression::ScopedPropertyAccessExpressionNode;
use crate::autonodes::subscript_expression::SubscriptExpressionNode;
use crate::autonodes::text_interpolation::TextInterpolationNode;
use crate::autonodes::variable_name::VariableNameNode;
use crate::autotree::NodeAccess;
use crate::autotree::ParseError;
use crate::errornode::ErrorNode;
use crate::extra::ExtraChild;
use crate::issue::IssueEmitter;
use crate::types::union::DiscreteType;
use crate::types::union::UnionType;
use crate::value::PHPValue;
use std::ffi::OsStr;
use tree_sitter::Node;
use tree_sitter::Range;

#[derive(Debug, Clone)]
pub enum UpdateExpressionExpr {
    CastExpression(Box<CastExpressionNode>),
    DynamicVariableName(Box<DynamicVariableNameNode>),
    FunctionCallExpression(Box<FunctionCallExpressionNode>),
    MemberAccessExpression(Box<MemberAccessExpressionNode>),
    MemberCallExpression(Box<MemberCallExpressionNode>),
    NullsafeMemberAccessExpression(Box<NullsafeMemberAccessExpressionNode>),
    NullsafeMemberCallExpression(Box<NullsafeMemberCallExpressionNode>),
    ScopedCallExpression(Box<ScopedCallExpressionNode>),
    ScopedPropertyAccessExpression(Box<ScopedPropertyAccessExpressionNode>),
    SubscriptExpression(Box<SubscriptExpressionNode>),
    VariableName(Box<VariableNameNode>),
    Comment(Box<CommentNode>),
    TextInterpolation(Box<TextInterpolationNode>),
    Error(Box<ErrorNode>),
}

impl UpdateExpressionExpr {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => UpdateExpressionExpr::Comment(Box::new(CommentNode::parse(node, source)?)),
            "text_interpolation" => UpdateExpressionExpr::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            )),
            "ERROR" => UpdateExpressionExpr::Error(Box::new(ErrorNode::parse(node, source)?)),
            "cast_expression" => UpdateExpressionExpr::CastExpression(Box::new(
                CastExpressionNode::parse(node, source)?,
            )),
            "dynamic_variable_name" => UpdateExpressionExpr::DynamicVariableName(Box::new(
                DynamicVariableNameNode::parse(node, source)?,
            )),
            "function_call_expression" => UpdateExpressionExpr::FunctionCallExpression(Box::new(
                FunctionCallExpressionNode::parse(node, source)?,
            )),
            "member_access_expression" => UpdateExpressionExpr::MemberAccessExpression(Box::new(
                MemberAccessExpressionNode::parse(node, source)?,
            )),
            "member_call_expression" => UpdateExpressionExpr::MemberCallExpression(Box::new(
                MemberCallExpressionNode::parse(node, source)?,
            )),
            "nullsafe_member_access_expression" => {
                UpdateExpressionExpr::NullsafeMemberAccessExpression(Box::new(
                    NullsafeMemberAccessExpressionNode::parse(node, source)?,
                ))
            }
            "nullsafe_member_call_expression" => {
                UpdateExpressionExpr::NullsafeMemberCallExpression(Box::new(
                    NullsafeMemberCallExpressionNode::parse(node, source)?,
                ))
            }
            "scoped_call_expression" => UpdateExpressionExpr::ScopedCallExpression(Box::new(
                ScopedCallExpressionNode::parse(node, source)?,
            )),
            "scoped_property_access_expression" => {
                UpdateExpressionExpr::ScopedPropertyAccessExpression(Box::new(
                    ScopedPropertyAccessExpressionNode::parse(node, source)?,
                ))
            }
            "subscript_expression" => UpdateExpressionExpr::SubscriptExpression(Box::new(
                SubscriptExpressionNode::parse(node, source)?,
            )),
            "variable_name" => {
                UpdateExpressionExpr::VariableName(Box::new(VariableNameNode::parse(node, source)?))
            }

            _ => {
                return Err(ParseError::new(
                    node.range(),
                    format!("Parse error, unexpected node-type: {}", node.kind()),
                ))
            }
        })
    }

    pub fn parse_opt(node: Node, source: &Vec<u8>) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => UpdateExpressionExpr::Comment(Box::new(CommentNode::parse(node, source)?)),
            "text_interpolation" => UpdateExpressionExpr::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            )),
            "ERROR" => UpdateExpressionExpr::Error(Box::new(ErrorNode::parse(node, source)?)),
            "cast_expression" => UpdateExpressionExpr::CastExpression(Box::new(
                CastExpressionNode::parse(node, source)?,
            )),
            "dynamic_variable_name" => UpdateExpressionExpr::DynamicVariableName(Box::new(
                DynamicVariableNameNode::parse(node, source)?,
            )),
            "function_call_expression" => UpdateExpressionExpr::FunctionCallExpression(Box::new(
                FunctionCallExpressionNode::parse(node, source)?,
            )),
            "member_access_expression" => UpdateExpressionExpr::MemberAccessExpression(Box::new(
                MemberAccessExpressionNode::parse(node, source)?,
            )),
            "member_call_expression" => UpdateExpressionExpr::MemberCallExpression(Box::new(
                MemberCallExpressionNode::parse(node, source)?,
            )),
            "nullsafe_member_access_expression" => {
                UpdateExpressionExpr::NullsafeMemberAccessExpression(Box::new(
                    NullsafeMemberAccessExpressionNode::parse(node, source)?,
                ))
            }
            "nullsafe_member_call_expression" => {
                UpdateExpressionExpr::NullsafeMemberCallExpression(Box::new(
                    NullsafeMemberCallExpressionNode::parse(node, source)?,
                ))
            }
            "scoped_call_expression" => UpdateExpressionExpr::ScopedCallExpression(Box::new(
                ScopedCallExpressionNode::parse(node, source)?,
            )),
            "scoped_property_access_expression" => {
                UpdateExpressionExpr::ScopedPropertyAccessExpression(Box::new(
                    ScopedPropertyAccessExpressionNode::parse(node, source)?,
                ))
            }
            "subscript_expression" => UpdateExpressionExpr::SubscriptExpression(Box::new(
                SubscriptExpressionNode::parse(node, source)?,
            )),
            "variable_name" => {
                UpdateExpressionExpr::VariableName(Box::new(VariableNameNode::parse(node, source)?))
            }

            _ => return Ok(None),
        }))
    }

    pub fn kind(&self) -> &'static str {
        self.as_any().kind()
    }

    pub fn parse_vec<'a, I>(children: I, source: &Vec<u8>) -> Result<Vec<Box<Self>>, ParseError>
    where
        I: Iterator<Item = Node<'a>>,
    {
        let mut res: Vec<Box<Self>> = vec![];
        for child in children {
            res.push(Box::new(Self::parse(child, source)?));
        }
        Ok(res)
    }

    pub fn get_utype(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        match self {
            UpdateExpressionExpr::Comment(x) => x.get_utype(state, emitter),
            UpdateExpressionExpr::TextInterpolation(x) => x.get_utype(state, emitter),
            UpdateExpressionExpr::Error(x) => x.get_utype(state, emitter),
            UpdateExpressionExpr::CastExpression(x) => x.get_utype(state, emitter),
            UpdateExpressionExpr::DynamicVariableName(x) => x.get_utype(state, emitter),
            UpdateExpressionExpr::FunctionCallExpression(x) => x.get_utype(state, emitter),
            UpdateExpressionExpr::MemberAccessExpression(x) => x.get_utype(state, emitter),
            UpdateExpressionExpr::MemberCallExpression(x) => x.get_utype(state, emitter),
            UpdateExpressionExpr::NullsafeMemberAccessExpression(x) => x.get_utype(state, emitter),
            UpdateExpressionExpr::NullsafeMemberCallExpression(x) => x.get_utype(state, emitter),
            UpdateExpressionExpr::ScopedCallExpression(x) => x.get_utype(state, emitter),
            UpdateExpressionExpr::ScopedPropertyAccessExpression(x) => x.get_utype(state, emitter),
            UpdateExpressionExpr::SubscriptExpression(x) => x.get_utype(state, emitter),
            UpdateExpressionExpr::VariableName(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            UpdateExpressionExpr::Comment(x) => x.get_php_value(state, emitter),
            UpdateExpressionExpr::TextInterpolation(x) => x.get_php_value(state, emitter),
            UpdateExpressionExpr::Error(x) => x.get_php_value(state, emitter),
            UpdateExpressionExpr::CastExpression(x) => x.get_php_value(state, emitter),
            UpdateExpressionExpr::DynamicVariableName(x) => x.get_php_value(state, emitter),
            UpdateExpressionExpr::FunctionCallExpression(x) => x.get_php_value(state, emitter),
            UpdateExpressionExpr::MemberAccessExpression(x) => x.get_php_value(state, emitter),
            UpdateExpressionExpr::MemberCallExpression(x) => x.get_php_value(state, emitter),
            UpdateExpressionExpr::NullsafeMemberAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            UpdateExpressionExpr::NullsafeMemberCallExpression(x) => {
                x.get_php_value(state, emitter)
            }
            UpdateExpressionExpr::ScopedCallExpression(x) => x.get_php_value(state, emitter),
            UpdateExpressionExpr::ScopedPropertyAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            UpdateExpressionExpr::SubscriptExpression(x) => x.get_php_value(state, emitter),
            UpdateExpressionExpr::VariableName(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            UpdateExpressionExpr::Comment(x) => x.read_from(state, emitter),
            UpdateExpressionExpr::TextInterpolation(x) => x.read_from(state, emitter),
            UpdateExpressionExpr::Error(x) => x.read_from(state, emitter),
            UpdateExpressionExpr::CastExpression(x) => x.read_from(state, emitter),
            UpdateExpressionExpr::DynamicVariableName(x) => x.read_from(state, emitter),
            UpdateExpressionExpr::FunctionCallExpression(x) => x.read_from(state, emitter),
            UpdateExpressionExpr::MemberAccessExpression(x) => x.read_from(state, emitter),
            UpdateExpressionExpr::MemberCallExpression(x) => x.read_from(state, emitter),
            UpdateExpressionExpr::NullsafeMemberAccessExpression(x) => x.read_from(state, emitter),
            UpdateExpressionExpr::NullsafeMemberCallExpression(x) => x.read_from(state, emitter),
            UpdateExpressionExpr::ScopedCallExpression(x) => x.read_from(state, emitter),
            UpdateExpressionExpr::ScopedPropertyAccessExpression(x) => x.read_from(state, emitter),
            UpdateExpressionExpr::SubscriptExpression(x) => x.read_from(state, emitter),
            UpdateExpressionExpr::VariableName(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for UpdateExpressionExpr {
    fn brief_desc(&self) -> String {
        match self {
            UpdateExpressionExpr::Comment(x) => {
                format!("UpdateExpressionExpr::comment({})", x.brief_desc())
            }
            UpdateExpressionExpr::TextInterpolation(x) => format!(
                "UpdateExpressionExpr::text_interpolation({})",
                x.brief_desc()
            ),
            UpdateExpressionExpr::Error(x) => {
                format!("UpdateExpressionExpr::ERROR({})", x.brief_desc())
            }
            UpdateExpressionExpr::CastExpression(x) => {
                format!("UpdateExpressionExpr::cast_expression({})", x.brief_desc())
            }
            UpdateExpressionExpr::DynamicVariableName(x) => format!(
                "UpdateExpressionExpr::dynamic_variable_name({})",
                x.brief_desc()
            ),
            UpdateExpressionExpr::FunctionCallExpression(x) => format!(
                "UpdateExpressionExpr::function_call_expression({})",
                x.brief_desc()
            ),
            UpdateExpressionExpr::MemberAccessExpression(x) => format!(
                "UpdateExpressionExpr::member_access_expression({})",
                x.brief_desc()
            ),
            UpdateExpressionExpr::MemberCallExpression(x) => format!(
                "UpdateExpressionExpr::member_call_expression({})",
                x.brief_desc()
            ),
            UpdateExpressionExpr::NullsafeMemberAccessExpression(x) => format!(
                "UpdateExpressionExpr::nullsafe_member_access_expression({})",
                x.brief_desc()
            ),
            UpdateExpressionExpr::NullsafeMemberCallExpression(x) => format!(
                "UpdateExpressionExpr::nullsafe_member_call_expression({})",
                x.brief_desc()
            ),
            UpdateExpressionExpr::ScopedCallExpression(x) => format!(
                "UpdateExpressionExpr::scoped_call_expression({})",
                x.brief_desc()
            ),
            UpdateExpressionExpr::ScopedPropertyAccessExpression(x) => format!(
                "UpdateExpressionExpr::scoped_property_access_expression({})",
                x.brief_desc()
            ),
            UpdateExpressionExpr::SubscriptExpression(x) => format!(
                "UpdateExpressionExpr::subscript_expression({})",
                x.brief_desc()
            ),
            UpdateExpressionExpr::VariableName(x) => {
                format!("UpdateExpressionExpr::variable_name({})", x.brief_desc())
            }
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            UpdateExpressionExpr::Comment(x) => x.as_any(),
            UpdateExpressionExpr::TextInterpolation(x) => x.as_any(),
            UpdateExpressionExpr::Error(x) => x.as_any(),
            UpdateExpressionExpr::CastExpression(x) => x.as_any(),
            UpdateExpressionExpr::DynamicVariableName(x) => x.as_any(),
            UpdateExpressionExpr::FunctionCallExpression(x) => x.as_any(),
            UpdateExpressionExpr::MemberAccessExpression(x) => x.as_any(),
            UpdateExpressionExpr::MemberCallExpression(x) => x.as_any(),
            UpdateExpressionExpr::NullsafeMemberAccessExpression(x) => x.as_any(),
            UpdateExpressionExpr::NullsafeMemberCallExpression(x) => x.as_any(),
            UpdateExpressionExpr::ScopedCallExpression(x) => x.as_any(),
            UpdateExpressionExpr::ScopedPropertyAccessExpression(x) => x.as_any(),
            UpdateExpressionExpr::SubscriptExpression(x) => x.as_any(),
            UpdateExpressionExpr::VariableName(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            UpdateExpressionExpr::Comment(x) => x.children_any(),
            UpdateExpressionExpr::TextInterpolation(x) => x.children_any(),
            UpdateExpressionExpr::Error(x) => x.children_any(),
            UpdateExpressionExpr::CastExpression(x) => x.children_any(),
            UpdateExpressionExpr::DynamicVariableName(x) => x.children_any(),
            UpdateExpressionExpr::FunctionCallExpression(x) => x.children_any(),
            UpdateExpressionExpr::MemberAccessExpression(x) => x.children_any(),
            UpdateExpressionExpr::MemberCallExpression(x) => x.children_any(),
            UpdateExpressionExpr::NullsafeMemberAccessExpression(x) => x.children_any(),
            UpdateExpressionExpr::NullsafeMemberCallExpression(x) => x.children_any(),
            UpdateExpressionExpr::ScopedCallExpression(x) => x.children_any(),
            UpdateExpressionExpr::ScopedPropertyAccessExpression(x) => x.children_any(),
            UpdateExpressionExpr::SubscriptExpression(x) => x.children_any(),
            UpdateExpressionExpr::VariableName(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            UpdateExpressionExpr::Comment(x) => x.range(),
            UpdateExpressionExpr::TextInterpolation(x) => x.range(),
            UpdateExpressionExpr::Error(x) => x.range(),
            UpdateExpressionExpr::CastExpression(x) => x.range(),
            UpdateExpressionExpr::DynamicVariableName(x) => x.range(),
            UpdateExpressionExpr::FunctionCallExpression(x) => x.range(),
            UpdateExpressionExpr::MemberAccessExpression(x) => x.range(),
            UpdateExpressionExpr::MemberCallExpression(x) => x.range(),
            UpdateExpressionExpr::NullsafeMemberAccessExpression(x) => x.range(),
            UpdateExpressionExpr::NullsafeMemberCallExpression(x) => x.range(),
            UpdateExpressionExpr::ScopedCallExpression(x) => x.range(),
            UpdateExpressionExpr::ScopedPropertyAccessExpression(x) => x.range(),
            UpdateExpressionExpr::SubscriptExpression(x) => x.range(),
            UpdateExpressionExpr::VariableName(x) => x.range(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum UpdateExpressionPostfix {
    Increment(&'static str, Range),
    Decrement(&'static str, Range),
    Comment(Box<CommentNode>),
    TextInterpolation(Box<TextInterpolationNode>),
    Error(Box<ErrorNode>),
}

impl UpdateExpressionPostfix {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => {
                UpdateExpressionPostfix::Comment(Box::new(CommentNode::parse(node, source)?))
            }
            "text_interpolation" => UpdateExpressionPostfix::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            )),
            "ERROR" => UpdateExpressionPostfix::Error(Box::new(ErrorNode::parse(node, source)?)),
            "++" => UpdateExpressionPostfix::Increment("++", node.range()),
            "--" => UpdateExpressionPostfix::Decrement("--", node.range()),

            _ => {
                return Err(ParseError::new(
                    node.range(),
                    format!("Parse error, unexpected node-type: {}", node.kind()),
                ))
            }
        })
    }

    pub fn parse_opt(node: Node, source: &Vec<u8>) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => {
                UpdateExpressionPostfix::Comment(Box::new(CommentNode::parse(node, source)?))
            }
            "text_interpolation" => UpdateExpressionPostfix::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            )),
            "ERROR" => UpdateExpressionPostfix::Error(Box::new(ErrorNode::parse(node, source)?)),
            "++" => UpdateExpressionPostfix::Increment("++", node.range()),
            "--" => UpdateExpressionPostfix::Decrement("--", node.range()),

            _ => return Ok(None),
        }))
    }

    pub fn kind(&self) -> &'static str {
        self.as_any().kind()
    }

    pub fn parse_vec<'a, I>(children: I, source: &Vec<u8>) -> Result<Vec<Box<Self>>, ParseError>
    where
        I: Iterator<Item = Node<'a>>,
    {
        let mut res: Vec<Box<Self>> = vec![];
        for child in children {
            res.push(Box::new(Self::parse(child, source)?));
        }
        Ok(res)
    }

    pub fn get_utype(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        match self {
            UpdateExpressionPostfix::Comment(x) => x.get_utype(state, emitter),
            UpdateExpressionPostfix::TextInterpolation(x) => x.get_utype(state, emitter),
            UpdateExpressionPostfix::Error(x) => x.get_utype(state, emitter),
            UpdateExpressionPostfix::Increment(_, _) => Some(DiscreteType::String.into()),
            UpdateExpressionPostfix::Decrement(_, _) => Some(DiscreteType::String.into()),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            UpdateExpressionPostfix::Comment(x) => x.get_php_value(state, emitter),
            UpdateExpressionPostfix::TextInterpolation(x) => x.get_php_value(state, emitter),
            UpdateExpressionPostfix::Error(x) => x.get_php_value(state, emitter),
            UpdateExpressionPostfix::Increment(a, _) => {
                Some(PHPValue::String(OsStr::new(a).to_os_string()))
            }
            UpdateExpressionPostfix::Decrement(a, _) => {
                Some(PHPValue::String(OsStr::new(a).to_os_string()))
            }
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            UpdateExpressionPostfix::Comment(x) => x.read_from(state, emitter),
            UpdateExpressionPostfix::TextInterpolation(x) => x.read_from(state, emitter),
            UpdateExpressionPostfix::Error(x) => x.read_from(state, emitter),
            UpdateExpressionPostfix::Increment(_, _) => (),
            UpdateExpressionPostfix::Decrement(_, _) => (),
        }
    }
}

impl NodeAccess for UpdateExpressionPostfix {
    fn brief_desc(&self) -> String {
        match self {
            UpdateExpressionPostfix::Comment(x) => {
                format!("UpdateExpressionPostfix::comment({})", x.brief_desc())
            }
            UpdateExpressionPostfix::TextInterpolation(x) => format!(
                "UpdateExpressionPostfix::text_interpolation({})",
                x.brief_desc()
            ),
            UpdateExpressionPostfix::Error(x) => {
                format!("UpdateExpressionPostfix::ERROR({})", x.brief_desc())
            }
            UpdateExpressionPostfix::Increment(a, _) => a.to_string(),
            UpdateExpressionPostfix::Decrement(a, _) => a.to_string(),
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            UpdateExpressionPostfix::Comment(x) => x.as_any(),
            UpdateExpressionPostfix::TextInterpolation(x) => x.as_any(),
            UpdateExpressionPostfix::Error(x) => x.as_any(),
            UpdateExpressionPostfix::Increment(a, b) => AnyNodeRef::StaticExpr(a, *b),
            UpdateExpressionPostfix::Decrement(a, b) => AnyNodeRef::StaticExpr(a, *b),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            UpdateExpressionPostfix::Comment(x) => x.children_any(),
            UpdateExpressionPostfix::TextInterpolation(x) => x.children_any(),
            UpdateExpressionPostfix::Error(x) => x.children_any(),
            UpdateExpressionPostfix::Increment(_, _) => todo!("Crap"),
            UpdateExpressionPostfix::Decrement(_, _) => todo!("Crap"),
        }
    }

    fn range(&self) -> Range {
        match self {
            UpdateExpressionPostfix::Comment(x) => x.range(),
            UpdateExpressionPostfix::TextInterpolation(x) => x.range(),
            UpdateExpressionPostfix::Error(x) => x.range(),
            UpdateExpressionPostfix::Increment(_, r) => *r,
            UpdateExpressionPostfix::Decrement(_, r) => *r,
        }
    }
}
#[derive(Debug, Clone)]
pub enum UpdateExpressionPrefix {
    Increment(&'static str, Range),
    Decrement(&'static str, Range),
    Comment(Box<CommentNode>),
    TextInterpolation(Box<TextInterpolationNode>),
    Error(Box<ErrorNode>),
}

impl UpdateExpressionPrefix {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => {
                UpdateExpressionPrefix::Comment(Box::new(CommentNode::parse(node, source)?))
            }
            "text_interpolation" => UpdateExpressionPrefix::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            )),
            "ERROR" => UpdateExpressionPrefix::Error(Box::new(ErrorNode::parse(node, source)?)),
            "++" => UpdateExpressionPrefix::Increment("++", node.range()),
            "--" => UpdateExpressionPrefix::Decrement("--", node.range()),

            _ => {
                return Err(ParseError::new(
                    node.range(),
                    format!("Parse error, unexpected node-type: {}", node.kind()),
                ))
            }
        })
    }

    pub fn parse_opt(node: Node, source: &Vec<u8>) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => {
                UpdateExpressionPrefix::Comment(Box::new(CommentNode::parse(node, source)?))
            }
            "text_interpolation" => UpdateExpressionPrefix::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            )),
            "ERROR" => UpdateExpressionPrefix::Error(Box::new(ErrorNode::parse(node, source)?)),
            "++" => UpdateExpressionPrefix::Increment("++", node.range()),
            "--" => UpdateExpressionPrefix::Decrement("--", node.range()),

            _ => return Ok(None),
        }))
    }

    pub fn kind(&self) -> &'static str {
        self.as_any().kind()
    }

    pub fn parse_vec<'a, I>(children: I, source: &Vec<u8>) -> Result<Vec<Box<Self>>, ParseError>
    where
        I: Iterator<Item = Node<'a>>,
    {
        let mut res: Vec<Box<Self>> = vec![];
        for child in children {
            res.push(Box::new(Self::parse(child, source)?));
        }
        Ok(res)
    }

    pub fn get_utype(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        match self {
            UpdateExpressionPrefix::Comment(x) => x.get_utype(state, emitter),
            UpdateExpressionPrefix::TextInterpolation(x) => x.get_utype(state, emitter),
            UpdateExpressionPrefix::Error(x) => x.get_utype(state, emitter),
            UpdateExpressionPrefix::Increment(_, _) => Some(DiscreteType::String.into()),
            UpdateExpressionPrefix::Decrement(_, _) => Some(DiscreteType::String.into()),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            UpdateExpressionPrefix::Comment(x) => x.get_php_value(state, emitter),
            UpdateExpressionPrefix::TextInterpolation(x) => x.get_php_value(state, emitter),
            UpdateExpressionPrefix::Error(x) => x.get_php_value(state, emitter),
            UpdateExpressionPrefix::Increment(a, _) => {
                Some(PHPValue::String(OsStr::new(a).to_os_string()))
            }
            UpdateExpressionPrefix::Decrement(a, _) => {
                Some(PHPValue::String(OsStr::new(a).to_os_string()))
            }
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            UpdateExpressionPrefix::Comment(x) => x.read_from(state, emitter),
            UpdateExpressionPrefix::TextInterpolation(x) => x.read_from(state, emitter),
            UpdateExpressionPrefix::Error(x) => x.read_from(state, emitter),
            UpdateExpressionPrefix::Increment(_, _) => (),
            UpdateExpressionPrefix::Decrement(_, _) => (),
        }
    }
}

impl NodeAccess for UpdateExpressionPrefix {
    fn brief_desc(&self) -> String {
        match self {
            UpdateExpressionPrefix::Comment(x) => {
                format!("UpdateExpressionPrefix::comment({})", x.brief_desc())
            }
            UpdateExpressionPrefix::TextInterpolation(x) => format!(
                "UpdateExpressionPrefix::text_interpolation({})",
                x.brief_desc()
            ),
            UpdateExpressionPrefix::Error(x) => {
                format!("UpdateExpressionPrefix::ERROR({})", x.brief_desc())
            }
            UpdateExpressionPrefix::Increment(a, _) => a.to_string(),
            UpdateExpressionPrefix::Decrement(a, _) => a.to_string(),
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            UpdateExpressionPrefix::Comment(x) => x.as_any(),
            UpdateExpressionPrefix::TextInterpolation(x) => x.as_any(),
            UpdateExpressionPrefix::Error(x) => x.as_any(),
            UpdateExpressionPrefix::Increment(a, b) => AnyNodeRef::StaticExpr(a, *b),
            UpdateExpressionPrefix::Decrement(a, b) => AnyNodeRef::StaticExpr(a, *b),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            UpdateExpressionPrefix::Comment(x) => x.children_any(),
            UpdateExpressionPrefix::TextInterpolation(x) => x.children_any(),
            UpdateExpressionPrefix::Error(x) => x.children_any(),
            UpdateExpressionPrefix::Increment(_, _) => todo!("Crap"),
            UpdateExpressionPrefix::Decrement(_, _) => todo!("Crap"),
        }
    }

    fn range(&self) -> Range {
        match self {
            UpdateExpressionPrefix::Comment(x) => x.range(),
            UpdateExpressionPrefix::TextInterpolation(x) => x.range(),
            UpdateExpressionPrefix::Error(x) => x.range(),
            UpdateExpressionPrefix::Increment(_, r) => *r,
            UpdateExpressionPrefix::Decrement(_, r) => *r,
        }
    }
}
#[derive(Debug, Clone)]
pub struct UpdateExpressionNode {
    pub range: Range,
    pub expr: Box<UpdateExpressionExpr>,
    pub postfix: Option<Box<UpdateExpressionPostfix>>,
    pub prefix: Option<Box<UpdateExpressionPrefix>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl UpdateExpressionNode {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
        if node.kind() != "update_expression" {
            return Err(ParseError::new(
                range,
                format!(
                    "Node is of the wrong kind [{}] vs expected [update_expression] on pos {}:{}",
                    node.kind(),
                    range.start_point.row + 1,
                    range.start_point.column
                ),
            ));
        }
        let expr: Box<UpdateExpressionExpr> = node
            .children_by_field_name("expr", &mut node.walk())
            .map(|chnode2| UpdateExpressionExpr::parse(chnode2, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .map(|z| Box::new(z))
            .next()
            .expect("Field expr should exist")
            .into();
        let postfix: Option<Box<UpdateExpressionPostfix>> = node
            .children_by_field_name("postfix", &mut node.walk())
            .map(|chnode2| UpdateExpressionPostfix::parse(chnode2, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .map(|z| Box::new(z))
            .next()
            .into();
        let prefix: Option<Box<UpdateExpressionPrefix>> = node
            .children_by_field_name("prefix", &mut node.walk())
            .map(|chnode2| UpdateExpressionPrefix::parse(chnode2, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .map(|z| Box::new(z))
            .next()
            .into();
        Ok(Self {
            range,
            expr,
            postfix,
            prefix,
            extras: ExtraChild::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| node.kind() == "comment"),
                source,
            )
            .unwrap(),
        })
    }

    pub fn parse_vec<'a, I>(children: I, source: &Vec<u8>) -> Result<Vec<Box<Self>>, ParseError>
    where
        I: Iterator<Item = Node<'a>>,
    {
        let mut res: Vec<Box<Self>> = vec![];
        for child in children {
            if child.kind() == "comment" {
                continue;
            }
            res.push(Box::new(Self::parse(child, source)?));
        }
        Ok(res)
    }

    pub fn kind(&self) -> &'static str {
        "update_expression"
    }
}

impl NodeAccess for UpdateExpressionNode {
    fn brief_desc(&self) -> String {
        "UpdateExpressionNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::UpdateExpression(self)
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        child_vec.push(self.expr.as_any());
        if let Some(x) = &self.postfix {
            child_vec.push(x.as_any());
        }
        if let Some(x) = &self.prefix {
            child_vec.push(x.as_any());
        }

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
