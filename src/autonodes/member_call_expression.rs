use crate::analysis::state::AnalysisState;
use crate::autonodes::_expression::_ExpressionNode;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::arguments::ArgumentsNode;
use crate::autonodes::array_creation_expression::ArrayCreationExpressionNode;
use crate::autonodes::cast_expression::CastExpressionNode;
use crate::autonodes::class_constant_access_expression::ClassConstantAccessExpressionNode;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::dynamic_variable_name::DynamicVariableNameNode;
use crate::autonodes::encapsed_string::EncapsedStringNode;
use crate::autonodes::function_call_expression::FunctionCallExpressionNode;
use crate::autonodes::heredoc::HeredocNode;
use crate::autonodes::member_access_expression::MemberAccessExpressionNode;
use crate::autonodes::name::NameNode;
use crate::autonodes::nowdoc::NowdocNode;
use crate::autonodes::nullsafe_member_access_expression::NullsafeMemberAccessExpressionNode;
use crate::autonodes::nullsafe_member_call_expression::NullsafeMemberCallExpressionNode;
use crate::autonodes::parenthesized_expression::ParenthesizedExpressionNode;
use crate::autonodes::qualified_name::QualifiedNameNode;
use crate::autonodes::scoped_call_expression::ScopedCallExpressionNode;
use crate::autonodes::scoped_property_access_expression::ScopedPropertyAccessExpressionNode;
use crate::autonodes::string::StringNode;
use crate::autonodes::subscript_expression::SubscriptExpressionNode;
use crate::autonodes::text_interpolation::TextInterpolationNode;
use crate::autonodes::variable_name::VariableNameNode;
use crate::autotree::ChildNodeParser;
use crate::autotree::NodeAccess;
use crate::autotree::NodeParser;
use crate::autotree::ParseError;
use crate::errornode::ErrorNode;
use crate::extra::ExtraChild;
use crate::issue::IssueEmitter;
use crate::parser::Range;
use crate::types::union::PHPType;
use crate::value::PHPValue;
use tree_sitter::Node;

#[derive(Debug, Clone)]
pub enum MemberCallExpressionName {
    _Expression(Box<_ExpressionNode>),
    DynamicVariableName(Box<DynamicVariableNameNode>),
    Name(Box<NameNode>),
    VariableName(Box<VariableNameNode>),
    Extra(ExtraChild),
}

impl NodeParser for MemberCallExpressionName {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => MemberCallExpressionName::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => MemberCallExpressionName::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => MemberCallExpressionName::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "dynamic_variable_name" => MemberCallExpressionName::DynamicVariableName(Box::new(
                DynamicVariableNameNode::parse(node, source)?,
            )),
            "name" => MemberCallExpressionName::Name(Box::new(NameNode::parse(node, source)?)),
            "variable_name" => MemberCallExpressionName::VariableName(Box::new(
                VariableNameNode::parse(node, source)?,
            )),

            _ => {
                if let Some(x) = _ExpressionNode::parse_opt(node, source)?
                    .map(Box::new)
                    .map(MemberCallExpressionName::_Expression)
                {
                    x
                } else {
                    return Err(ParseError::new(
                        node.range(),
                        format!(
                            "MemberCallExpressionName: Parse error, unexpected node-type: {}",
                            node.kind()
                        ),
                    ));
                }
            }
        })
    }
}

impl MemberCallExpressionName {
    pub fn parse_opt(node: Node, source: &[u8]) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => MemberCallExpressionName::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => MemberCallExpressionName::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => MemberCallExpressionName::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "dynamic_variable_name" => MemberCallExpressionName::DynamicVariableName(Box::new(
                DynamicVariableNameNode::parse(node, source)?,
            )),
            "name" => MemberCallExpressionName::Name(Box::new(NameNode::parse(node, source)?)),
            "variable_name" => MemberCallExpressionName::VariableName(Box::new(
                VariableNameNode::parse(node, source)?,
            )),

            _ => {
                return Ok(_ExpressionNode::parse_opt(node, source)?
                    .map(Box::new)
                    .map(MemberCallExpressionName::_Expression))
            }
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            MemberCallExpressionName::Extra(y) => y.kind(),
            MemberCallExpressionName::_Expression(y) => y.kind(),
            MemberCallExpressionName::DynamicVariableName(y) => y.kind(),
            MemberCallExpressionName::Name(y) => y.kind(),
            MemberCallExpressionName::VariableName(y) => y.kind(),
        }
    }

    pub fn parse_vec<'a, I>(children: I, source: &[u8]) -> Result<Vec<Box<Self>>, ParseError>
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
    ) -> Option<PHPType> {
        match self {
            MemberCallExpressionName::Extra(x) => x.get_utype(state, emitter),
            MemberCallExpressionName::_Expression(x) => x.get_utype(state, emitter),
            MemberCallExpressionName::DynamicVariableName(x) => x.get_utype(state, emitter),
            MemberCallExpressionName::Name(x) => x.get_utype(state, emitter),
            MemberCallExpressionName::VariableName(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            MemberCallExpressionName::Extra(x) => x.get_php_value(state, emitter),
            MemberCallExpressionName::_Expression(x) => x.get_php_value(state, emitter),
            MemberCallExpressionName::DynamicVariableName(x) => x.get_php_value(state, emitter),
            MemberCallExpressionName::Name(x) => x.get_php_value(state, emitter),
            MemberCallExpressionName::VariableName(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            MemberCallExpressionName::Extra(x) => x.read_from(state, emitter),
            MemberCallExpressionName::_Expression(x) => x.read_from(state, emitter),
            MemberCallExpressionName::DynamicVariableName(x) => x.read_from(state, emitter),
            MemberCallExpressionName::Name(x) => x.read_from(state, emitter),
            MemberCallExpressionName::VariableName(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for MemberCallExpressionName {
    fn brief_desc(&self) -> String {
        match self {
            MemberCallExpressionName::Extra(x) => {
                format!("MemberCallExpressionName::extra({})", x.brief_desc())
            }
            MemberCallExpressionName::_Expression(x) => {
                format!("MemberCallExpressionName::_expression({})", x.brief_desc())
            }
            MemberCallExpressionName::DynamicVariableName(x) => format!(
                "MemberCallExpressionName::dynamic_variable_name({})",
                x.brief_desc()
            ),
            MemberCallExpressionName::Name(x) => {
                format!("MemberCallExpressionName::name({})", x.brief_desc())
            }
            MemberCallExpressionName::VariableName(x) => format!(
                "MemberCallExpressionName::variable_name({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        match self {
            MemberCallExpressionName::Extra(x) => x.as_any(),
            MemberCallExpressionName::_Expression(x) => x.as_any(),
            MemberCallExpressionName::DynamicVariableName(x) => x.as_any(),
            MemberCallExpressionName::Name(x) => x.as_any(),
            MemberCallExpressionName::VariableName(x) => x.as_any(),
        }
    }

    fn children_any(&self) -> Vec<AnyNodeRef<'_>> {
        match self {
            MemberCallExpressionName::Extra(x) => x.children_any(),
            MemberCallExpressionName::_Expression(x) => x.children_any(),
            MemberCallExpressionName::DynamicVariableName(x) => x.children_any(),
            MemberCallExpressionName::Name(x) => x.children_any(),
            MemberCallExpressionName::VariableName(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            MemberCallExpressionName::Extra(x) => x.range(),
            MemberCallExpressionName::_Expression(x) => x.range(),
            MemberCallExpressionName::DynamicVariableName(x) => x.range(),
            MemberCallExpressionName::Name(x) => x.range(),
            MemberCallExpressionName::VariableName(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum MemberCallExpressionObject {
    ArrayCreationExpression(Box<ArrayCreationExpressionNode>),
    CastExpression(Box<CastExpressionNode>),
    ClassConstantAccessExpression(Box<ClassConstantAccessExpressionNode>),
    DynamicVariableName(Box<DynamicVariableNameNode>),
    EncapsedString(Box<EncapsedStringNode>),
    FunctionCallExpression(Box<FunctionCallExpressionNode>),
    Heredoc(Box<HeredocNode>),
    MemberAccessExpression(Box<MemberAccessExpressionNode>),
    MemberCallExpression(Box<MemberCallExpressionNode>),
    Name(Box<NameNode>),
    Nowdoc(Box<NowdocNode>),
    NullsafeMemberAccessExpression(Box<NullsafeMemberAccessExpressionNode>),
    NullsafeMemberCallExpression(Box<NullsafeMemberCallExpressionNode>),
    ParenthesizedExpression(Box<ParenthesizedExpressionNode>),
    QualifiedName(Box<QualifiedNameNode>),
    ScopedCallExpression(Box<ScopedCallExpressionNode>),
    ScopedPropertyAccessExpression(Box<ScopedPropertyAccessExpressionNode>),
    String(Box<StringNode>),
    SubscriptExpression(Box<SubscriptExpressionNode>),
    VariableName(Box<VariableNameNode>),
    Extra(ExtraChild),
}

impl NodeParser for MemberCallExpressionObject {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => MemberCallExpressionObject::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => {
                MemberCallExpressionObject::Extra(ExtraChild::TextInterpolation(Box::new(
                    TextInterpolationNode::parse(node, source)?,
                )))
            }
            "ERROR" => MemberCallExpressionObject::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "array_creation_expression" => MemberCallExpressionObject::ArrayCreationExpression(
                Box::new(ArrayCreationExpressionNode::parse(node, source)?),
            ),
            "cast_expression" => MemberCallExpressionObject::CastExpression(Box::new(
                CastExpressionNode::parse(node, source)?,
            )),
            "class_constant_access_expression" => {
                MemberCallExpressionObject::ClassConstantAccessExpression(Box::new(
                    ClassConstantAccessExpressionNode::parse(node, source)?,
                ))
            }
            "dynamic_variable_name" => MemberCallExpressionObject::DynamicVariableName(Box::new(
                DynamicVariableNameNode::parse(node, source)?,
            )),
            "encapsed_string" => MemberCallExpressionObject::EncapsedString(Box::new(
                EncapsedStringNode::parse(node, source)?,
            )),
            "function_call_expression" => MemberCallExpressionObject::FunctionCallExpression(
                Box::new(FunctionCallExpressionNode::parse(node, source)?),
            ),
            "heredoc" => {
                MemberCallExpressionObject::Heredoc(Box::new(HeredocNode::parse(node, source)?))
            }
            "member_access_expression" => MemberCallExpressionObject::MemberAccessExpression(
                Box::new(MemberAccessExpressionNode::parse(node, source)?),
            ),
            "member_call_expression" => MemberCallExpressionObject::MemberCallExpression(Box::new(
                MemberCallExpressionNode::parse(node, source)?,
            )),
            "name" => MemberCallExpressionObject::Name(Box::new(NameNode::parse(node, source)?)),
            "nowdoc" => {
                MemberCallExpressionObject::Nowdoc(Box::new(NowdocNode::parse(node, source)?))
            }
            "nullsafe_member_access_expression" => {
                MemberCallExpressionObject::NullsafeMemberAccessExpression(Box::new(
                    NullsafeMemberAccessExpressionNode::parse(node, source)?,
                ))
            }
            "nullsafe_member_call_expression" => {
                MemberCallExpressionObject::NullsafeMemberCallExpression(Box::new(
                    NullsafeMemberCallExpressionNode::parse(node, source)?,
                ))
            }
            "parenthesized_expression" => MemberCallExpressionObject::ParenthesizedExpression(
                Box::new(ParenthesizedExpressionNode::parse(node, source)?),
            ),
            "qualified_name" => MemberCallExpressionObject::QualifiedName(Box::new(
                QualifiedNameNode::parse(node, source)?,
            )),
            "scoped_call_expression" => MemberCallExpressionObject::ScopedCallExpression(Box::new(
                ScopedCallExpressionNode::parse(node, source)?,
            )),
            "scoped_property_access_expression" => {
                MemberCallExpressionObject::ScopedPropertyAccessExpression(Box::new(
                    ScopedPropertyAccessExpressionNode::parse(node, source)?,
                ))
            }
            "string" => {
                MemberCallExpressionObject::String(Box::new(StringNode::parse(node, source)?))
            }
            "subscript_expression" => MemberCallExpressionObject::SubscriptExpression(Box::new(
                SubscriptExpressionNode::parse(node, source)?,
            )),
            "variable_name" => MemberCallExpressionObject::VariableName(Box::new(
                VariableNameNode::parse(node, source)?,
            )),

            _ => {
                return Err(ParseError::new(
                    node.range(),
                    format!(
                        "MemberCallExpressionObject: Parse error, unexpected node-type: {}",
                        node.kind()
                    ),
                ))
            }
        })
    }
}

impl MemberCallExpressionObject {
    pub fn parse_opt(node: Node, source: &[u8]) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => MemberCallExpressionObject::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => {
                MemberCallExpressionObject::Extra(ExtraChild::TextInterpolation(Box::new(
                    TextInterpolationNode::parse(node, source)?,
                )))
            }
            "ERROR" => MemberCallExpressionObject::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "array_creation_expression" => MemberCallExpressionObject::ArrayCreationExpression(
                Box::new(ArrayCreationExpressionNode::parse(node, source)?),
            ),
            "cast_expression" => MemberCallExpressionObject::CastExpression(Box::new(
                CastExpressionNode::parse(node, source)?,
            )),
            "class_constant_access_expression" => {
                MemberCallExpressionObject::ClassConstantAccessExpression(Box::new(
                    ClassConstantAccessExpressionNode::parse(node, source)?,
                ))
            }
            "dynamic_variable_name" => MemberCallExpressionObject::DynamicVariableName(Box::new(
                DynamicVariableNameNode::parse(node, source)?,
            )),
            "encapsed_string" => MemberCallExpressionObject::EncapsedString(Box::new(
                EncapsedStringNode::parse(node, source)?,
            )),
            "function_call_expression" => MemberCallExpressionObject::FunctionCallExpression(
                Box::new(FunctionCallExpressionNode::parse(node, source)?),
            ),
            "heredoc" => {
                MemberCallExpressionObject::Heredoc(Box::new(HeredocNode::parse(node, source)?))
            }
            "member_access_expression" => MemberCallExpressionObject::MemberAccessExpression(
                Box::new(MemberAccessExpressionNode::parse(node, source)?),
            ),
            "member_call_expression" => MemberCallExpressionObject::MemberCallExpression(Box::new(
                MemberCallExpressionNode::parse(node, source)?,
            )),
            "name" => MemberCallExpressionObject::Name(Box::new(NameNode::parse(node, source)?)),
            "nowdoc" => {
                MemberCallExpressionObject::Nowdoc(Box::new(NowdocNode::parse(node, source)?))
            }
            "nullsafe_member_access_expression" => {
                MemberCallExpressionObject::NullsafeMemberAccessExpression(Box::new(
                    NullsafeMemberAccessExpressionNode::parse(node, source)?,
                ))
            }
            "nullsafe_member_call_expression" => {
                MemberCallExpressionObject::NullsafeMemberCallExpression(Box::new(
                    NullsafeMemberCallExpressionNode::parse(node, source)?,
                ))
            }
            "parenthesized_expression" => MemberCallExpressionObject::ParenthesizedExpression(
                Box::new(ParenthesizedExpressionNode::parse(node, source)?),
            ),
            "qualified_name" => MemberCallExpressionObject::QualifiedName(Box::new(
                QualifiedNameNode::parse(node, source)?,
            )),
            "scoped_call_expression" => MemberCallExpressionObject::ScopedCallExpression(Box::new(
                ScopedCallExpressionNode::parse(node, source)?,
            )),
            "scoped_property_access_expression" => {
                MemberCallExpressionObject::ScopedPropertyAccessExpression(Box::new(
                    ScopedPropertyAccessExpressionNode::parse(node, source)?,
                ))
            }
            "string" => {
                MemberCallExpressionObject::String(Box::new(StringNode::parse(node, source)?))
            }
            "subscript_expression" => MemberCallExpressionObject::SubscriptExpression(Box::new(
                SubscriptExpressionNode::parse(node, source)?,
            )),
            "variable_name" => MemberCallExpressionObject::VariableName(Box::new(
                VariableNameNode::parse(node, source)?,
            )),

            _ => return Ok(None),
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            MemberCallExpressionObject::Extra(y) => y.kind(),
            MemberCallExpressionObject::ArrayCreationExpression(y) => y.kind(),
            MemberCallExpressionObject::CastExpression(y) => y.kind(),
            MemberCallExpressionObject::ClassConstantAccessExpression(y) => y.kind(),
            MemberCallExpressionObject::DynamicVariableName(y) => y.kind(),
            MemberCallExpressionObject::EncapsedString(y) => y.kind(),
            MemberCallExpressionObject::FunctionCallExpression(y) => y.kind(),
            MemberCallExpressionObject::Heredoc(y) => y.kind(),
            MemberCallExpressionObject::MemberAccessExpression(y) => y.kind(),
            MemberCallExpressionObject::MemberCallExpression(y) => y.kind(),
            MemberCallExpressionObject::Name(y) => y.kind(),
            MemberCallExpressionObject::Nowdoc(y) => y.kind(),
            MemberCallExpressionObject::NullsafeMemberAccessExpression(y) => y.kind(),
            MemberCallExpressionObject::NullsafeMemberCallExpression(y) => y.kind(),
            MemberCallExpressionObject::ParenthesizedExpression(y) => y.kind(),
            MemberCallExpressionObject::QualifiedName(y) => y.kind(),
            MemberCallExpressionObject::ScopedCallExpression(y) => y.kind(),
            MemberCallExpressionObject::ScopedPropertyAccessExpression(y) => y.kind(),
            MemberCallExpressionObject::String(y) => y.kind(),
            MemberCallExpressionObject::SubscriptExpression(y) => y.kind(),
            MemberCallExpressionObject::VariableName(y) => y.kind(),
        }
    }

    pub fn parse_vec<'a, I>(children: I, source: &[u8]) -> Result<Vec<Box<Self>>, ParseError>
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
    ) -> Option<PHPType> {
        match self {
            MemberCallExpressionObject::Extra(x) => x.get_utype(state, emitter),
            MemberCallExpressionObject::ArrayCreationExpression(x) => x.get_utype(state, emitter),
            MemberCallExpressionObject::CastExpression(x) => x.get_utype(state, emitter),
            MemberCallExpressionObject::ClassConstantAccessExpression(x) => {
                x.get_utype(state, emitter)
            }
            MemberCallExpressionObject::DynamicVariableName(x) => x.get_utype(state, emitter),
            MemberCallExpressionObject::EncapsedString(x) => x.get_utype(state, emitter),
            MemberCallExpressionObject::FunctionCallExpression(x) => x.get_utype(state, emitter),
            MemberCallExpressionObject::Heredoc(x) => x.get_utype(state, emitter),
            MemberCallExpressionObject::MemberAccessExpression(x) => x.get_utype(state, emitter),
            MemberCallExpressionObject::MemberCallExpression(x) => x.get_utype(state, emitter),
            MemberCallExpressionObject::Name(x) => x.get_utype(state, emitter),
            MemberCallExpressionObject::Nowdoc(x) => x.get_utype(state, emitter),
            MemberCallExpressionObject::NullsafeMemberAccessExpression(x) => {
                x.get_utype(state, emitter)
            }
            MemberCallExpressionObject::NullsafeMemberCallExpression(x) => {
                x.get_utype(state, emitter)
            }
            MemberCallExpressionObject::ParenthesizedExpression(x) => x.get_utype(state, emitter),
            MemberCallExpressionObject::QualifiedName(x) => x.get_utype(state, emitter),
            MemberCallExpressionObject::ScopedCallExpression(x) => x.get_utype(state, emitter),
            MemberCallExpressionObject::ScopedPropertyAccessExpression(x) => {
                x.get_utype(state, emitter)
            }
            MemberCallExpressionObject::String(x) => x.get_utype(state, emitter),
            MemberCallExpressionObject::SubscriptExpression(x) => x.get_utype(state, emitter),
            MemberCallExpressionObject::VariableName(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            MemberCallExpressionObject::Extra(x) => x.get_php_value(state, emitter),
            MemberCallExpressionObject::ArrayCreationExpression(x) => {
                x.get_php_value(state, emitter)
            }
            MemberCallExpressionObject::CastExpression(x) => x.get_php_value(state, emitter),
            MemberCallExpressionObject::ClassConstantAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            MemberCallExpressionObject::DynamicVariableName(x) => x.get_php_value(state, emitter),
            MemberCallExpressionObject::EncapsedString(x) => x.get_php_value(state, emitter),
            MemberCallExpressionObject::FunctionCallExpression(x) => {
                x.get_php_value(state, emitter)
            }
            MemberCallExpressionObject::Heredoc(x) => x.get_php_value(state, emitter),
            MemberCallExpressionObject::MemberAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            MemberCallExpressionObject::MemberCallExpression(x) => x.get_php_value(state, emitter),
            MemberCallExpressionObject::Name(x) => x.get_php_value(state, emitter),
            MemberCallExpressionObject::Nowdoc(x) => x.get_php_value(state, emitter),
            MemberCallExpressionObject::NullsafeMemberAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            MemberCallExpressionObject::NullsafeMemberCallExpression(x) => {
                x.get_php_value(state, emitter)
            }
            MemberCallExpressionObject::ParenthesizedExpression(x) => {
                x.get_php_value(state, emitter)
            }
            MemberCallExpressionObject::QualifiedName(x) => x.get_php_value(state, emitter),
            MemberCallExpressionObject::ScopedCallExpression(x) => x.get_php_value(state, emitter),
            MemberCallExpressionObject::ScopedPropertyAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            MemberCallExpressionObject::String(x) => x.get_php_value(state, emitter),
            MemberCallExpressionObject::SubscriptExpression(x) => x.get_php_value(state, emitter),
            MemberCallExpressionObject::VariableName(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            MemberCallExpressionObject::Extra(x) => x.read_from(state, emitter),
            MemberCallExpressionObject::ArrayCreationExpression(x) => x.read_from(state, emitter),
            MemberCallExpressionObject::CastExpression(x) => x.read_from(state, emitter),
            MemberCallExpressionObject::ClassConstantAccessExpression(x) => {
                x.read_from(state, emitter)
            }
            MemberCallExpressionObject::DynamicVariableName(x) => x.read_from(state, emitter),
            MemberCallExpressionObject::EncapsedString(x) => x.read_from(state, emitter),
            MemberCallExpressionObject::FunctionCallExpression(x) => x.read_from(state, emitter),
            MemberCallExpressionObject::Heredoc(x) => x.read_from(state, emitter),
            MemberCallExpressionObject::MemberAccessExpression(x) => x.read_from(state, emitter),
            MemberCallExpressionObject::MemberCallExpression(x) => x.read_from(state, emitter),
            MemberCallExpressionObject::Name(x) => x.read_from(state, emitter),
            MemberCallExpressionObject::Nowdoc(x) => x.read_from(state, emitter),
            MemberCallExpressionObject::NullsafeMemberAccessExpression(x) => {
                x.read_from(state, emitter)
            }
            MemberCallExpressionObject::NullsafeMemberCallExpression(x) => {
                x.read_from(state, emitter)
            }
            MemberCallExpressionObject::ParenthesizedExpression(x) => x.read_from(state, emitter),
            MemberCallExpressionObject::QualifiedName(x) => x.read_from(state, emitter),
            MemberCallExpressionObject::ScopedCallExpression(x) => x.read_from(state, emitter),
            MemberCallExpressionObject::ScopedPropertyAccessExpression(x) => {
                x.read_from(state, emitter)
            }
            MemberCallExpressionObject::String(x) => x.read_from(state, emitter),
            MemberCallExpressionObject::SubscriptExpression(x) => x.read_from(state, emitter),
            MemberCallExpressionObject::VariableName(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for MemberCallExpressionObject {
    fn brief_desc(&self) -> String {
        match self {
            MemberCallExpressionObject::Extra(x) => {
                format!("MemberCallExpressionObject::extra({})", x.brief_desc())
            }
            MemberCallExpressionObject::ArrayCreationExpression(x) => format!(
                "MemberCallExpressionObject::array_creation_expression({})",
                x.brief_desc()
            ),
            MemberCallExpressionObject::CastExpression(x) => format!(
                "MemberCallExpressionObject::cast_expression({})",
                x.brief_desc()
            ),
            MemberCallExpressionObject::ClassConstantAccessExpression(x) => format!(
                "MemberCallExpressionObject::class_constant_access_expression({})",
                x.brief_desc()
            ),
            MemberCallExpressionObject::DynamicVariableName(x) => format!(
                "MemberCallExpressionObject::dynamic_variable_name({})",
                x.brief_desc()
            ),
            MemberCallExpressionObject::EncapsedString(x) => format!(
                "MemberCallExpressionObject::encapsed_string({})",
                x.brief_desc()
            ),
            MemberCallExpressionObject::FunctionCallExpression(x) => format!(
                "MemberCallExpressionObject::function_call_expression({})",
                x.brief_desc()
            ),
            MemberCallExpressionObject::Heredoc(x) => {
                format!("MemberCallExpressionObject::heredoc({})", x.brief_desc())
            }
            MemberCallExpressionObject::MemberAccessExpression(x) => format!(
                "MemberCallExpressionObject::member_access_expression({})",
                x.brief_desc()
            ),
            MemberCallExpressionObject::MemberCallExpression(x) => format!(
                "MemberCallExpressionObject::member_call_expression({})",
                x.brief_desc()
            ),
            MemberCallExpressionObject::Name(x) => {
                format!("MemberCallExpressionObject::name({})", x.brief_desc())
            }
            MemberCallExpressionObject::Nowdoc(x) => {
                format!("MemberCallExpressionObject::nowdoc({})", x.brief_desc())
            }
            MemberCallExpressionObject::NullsafeMemberAccessExpression(x) => format!(
                "MemberCallExpressionObject::nullsafe_member_access_expression({})",
                x.brief_desc()
            ),
            MemberCallExpressionObject::NullsafeMemberCallExpression(x) => format!(
                "MemberCallExpressionObject::nullsafe_member_call_expression({})",
                x.brief_desc()
            ),
            MemberCallExpressionObject::ParenthesizedExpression(x) => format!(
                "MemberCallExpressionObject::parenthesized_expression({})",
                x.brief_desc()
            ),
            MemberCallExpressionObject::QualifiedName(x) => format!(
                "MemberCallExpressionObject::qualified_name({})",
                x.brief_desc()
            ),
            MemberCallExpressionObject::ScopedCallExpression(x) => format!(
                "MemberCallExpressionObject::scoped_call_expression({})",
                x.brief_desc()
            ),
            MemberCallExpressionObject::ScopedPropertyAccessExpression(x) => format!(
                "MemberCallExpressionObject::scoped_property_access_expression({})",
                x.brief_desc()
            ),
            MemberCallExpressionObject::String(x) => {
                format!("MemberCallExpressionObject::string({})", x.brief_desc())
            }
            MemberCallExpressionObject::SubscriptExpression(x) => format!(
                "MemberCallExpressionObject::subscript_expression({})",
                x.brief_desc()
            ),
            MemberCallExpressionObject::VariableName(x) => format!(
                "MemberCallExpressionObject::variable_name({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        match self {
            MemberCallExpressionObject::Extra(x) => x.as_any(),
            MemberCallExpressionObject::ArrayCreationExpression(x) => x.as_any(),
            MemberCallExpressionObject::CastExpression(x) => x.as_any(),
            MemberCallExpressionObject::ClassConstantAccessExpression(x) => x.as_any(),
            MemberCallExpressionObject::DynamicVariableName(x) => x.as_any(),
            MemberCallExpressionObject::EncapsedString(x) => x.as_any(),
            MemberCallExpressionObject::FunctionCallExpression(x) => x.as_any(),
            MemberCallExpressionObject::Heredoc(x) => x.as_any(),
            MemberCallExpressionObject::MemberAccessExpression(x) => x.as_any(),
            MemberCallExpressionObject::MemberCallExpression(x) => x.as_any(),
            MemberCallExpressionObject::Name(x) => x.as_any(),
            MemberCallExpressionObject::Nowdoc(x) => x.as_any(),
            MemberCallExpressionObject::NullsafeMemberAccessExpression(x) => x.as_any(),
            MemberCallExpressionObject::NullsafeMemberCallExpression(x) => x.as_any(),
            MemberCallExpressionObject::ParenthesizedExpression(x) => x.as_any(),
            MemberCallExpressionObject::QualifiedName(x) => x.as_any(),
            MemberCallExpressionObject::ScopedCallExpression(x) => x.as_any(),
            MemberCallExpressionObject::ScopedPropertyAccessExpression(x) => x.as_any(),
            MemberCallExpressionObject::String(x) => x.as_any(),
            MemberCallExpressionObject::SubscriptExpression(x) => x.as_any(),
            MemberCallExpressionObject::VariableName(x) => x.as_any(),
        }
    }

    fn children_any(&self) -> Vec<AnyNodeRef<'_>> {
        match self {
            MemberCallExpressionObject::Extra(x) => x.children_any(),
            MemberCallExpressionObject::ArrayCreationExpression(x) => x.children_any(),
            MemberCallExpressionObject::CastExpression(x) => x.children_any(),
            MemberCallExpressionObject::ClassConstantAccessExpression(x) => x.children_any(),
            MemberCallExpressionObject::DynamicVariableName(x) => x.children_any(),
            MemberCallExpressionObject::EncapsedString(x) => x.children_any(),
            MemberCallExpressionObject::FunctionCallExpression(x) => x.children_any(),
            MemberCallExpressionObject::Heredoc(x) => x.children_any(),
            MemberCallExpressionObject::MemberAccessExpression(x) => x.children_any(),
            MemberCallExpressionObject::MemberCallExpression(x) => x.children_any(),
            MemberCallExpressionObject::Name(x) => x.children_any(),
            MemberCallExpressionObject::Nowdoc(x) => x.children_any(),
            MemberCallExpressionObject::NullsafeMemberAccessExpression(x) => x.children_any(),
            MemberCallExpressionObject::NullsafeMemberCallExpression(x) => x.children_any(),
            MemberCallExpressionObject::ParenthesizedExpression(x) => x.children_any(),
            MemberCallExpressionObject::QualifiedName(x) => x.children_any(),
            MemberCallExpressionObject::ScopedCallExpression(x) => x.children_any(),
            MemberCallExpressionObject::ScopedPropertyAccessExpression(x) => x.children_any(),
            MemberCallExpressionObject::String(x) => x.children_any(),
            MemberCallExpressionObject::SubscriptExpression(x) => x.children_any(),
            MemberCallExpressionObject::VariableName(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            MemberCallExpressionObject::Extra(x) => x.range(),
            MemberCallExpressionObject::ArrayCreationExpression(x) => x.range(),
            MemberCallExpressionObject::CastExpression(x) => x.range(),
            MemberCallExpressionObject::ClassConstantAccessExpression(x) => x.range(),
            MemberCallExpressionObject::DynamicVariableName(x) => x.range(),
            MemberCallExpressionObject::EncapsedString(x) => x.range(),
            MemberCallExpressionObject::FunctionCallExpression(x) => x.range(),
            MemberCallExpressionObject::Heredoc(x) => x.range(),
            MemberCallExpressionObject::MemberAccessExpression(x) => x.range(),
            MemberCallExpressionObject::MemberCallExpression(x) => x.range(),
            MemberCallExpressionObject::Name(x) => x.range(),
            MemberCallExpressionObject::Nowdoc(x) => x.range(),
            MemberCallExpressionObject::NullsafeMemberAccessExpression(x) => x.range(),
            MemberCallExpressionObject::NullsafeMemberCallExpression(x) => x.range(),
            MemberCallExpressionObject::ParenthesizedExpression(x) => x.range(),
            MemberCallExpressionObject::QualifiedName(x) => x.range(),
            MemberCallExpressionObject::ScopedCallExpression(x) => x.range(),
            MemberCallExpressionObject::ScopedPropertyAccessExpression(x) => x.range(),
            MemberCallExpressionObject::String(x) => x.range(),
            MemberCallExpressionObject::SubscriptExpression(x) => x.range(),
            MemberCallExpressionObject::VariableName(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MemberCallExpressionNode {
    pub range: Range,
    pub arguments: ArgumentsNode,
    pub name: Box<MemberCallExpressionName>,
    pub object: Box<MemberCallExpressionObject>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for MemberCallExpressionNode {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "member_call_expression" {
            return Err(ParseError::new(range, format!("MemberCallExpressionNode: Node is of the wrong kind [{}] vs expected [member_call_expression] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }
        let arguments: ArgumentsNode =
            Into::<Result<_, _>>::into(node.parse_child("arguments", source))?;
        let name: Box<MemberCallExpressionName> =
            Into::<Result<_, _>>::into(node.parse_child("name", source))?;
        let object: Box<MemberCallExpressionObject> =
            Into::<Result<_, _>>::into(node.parse_child("object", source))?;
        Ok(Self {
            range,
            arguments,
            name,
            object,
            extras: ExtraChild::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| node.kind() == "comment"),
                source,
            )
            .unwrap(),
        })
    }
}

impl MemberCallExpressionNode {
    pub fn kind(&self) -> &'static str {
        "member_call_expression"
    }
}

impl NodeAccess for MemberCallExpressionNode {
    fn brief_desc(&self) -> String {
        "MemberCallExpressionNode".into()
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        AnyNodeRef::MemberCallExpression(self)
    }

    #[allow(clippy::vec_init_then_push)]
    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        child_vec.push(self.arguments.as_any());
        child_vec.push(self.name.as_any());
        child_vec.push(self.object.as_any());

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
