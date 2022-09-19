use crate::analysis::state::AnalysisState;
use crate::autonodes::_expression::_ExpressionNode;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::array_creation_expression::ArrayCreationExpressionNode;
use crate::autonodes::cast_expression::CastExpressionNode;
use crate::autonodes::class_constant_access_expression::ClassConstantAccessExpressionNode;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::dynamic_variable_name::DynamicVariableNameNode;
use crate::autonodes::encapsed_string::EncapsedStringNode;
use crate::autonodes::function_call_expression::FunctionCallExpressionNode;
use crate::autonodes::heredoc::HeredocNode;
use crate::autonodes::integer::IntegerNode;
use crate::autonodes::member_access_expression::MemberAccessExpressionNode;
use crate::autonodes::member_call_expression::MemberCallExpressionNode;
use crate::autonodes::name::NameNode;
use crate::autonodes::nowdoc::NowdocNode;
use crate::autonodes::nullsafe_member_access_expression::NullsafeMemberAccessExpressionNode;
use crate::autonodes::nullsafe_member_call_expression::NullsafeMemberCallExpressionNode;
use crate::autonodes::parenthesized_expression::ParenthesizedExpressionNode;
use crate::autonodes::qualified_name::QualifiedNameNode;
use crate::autonodes::scoped_call_expression::ScopedCallExpressionNode;
use crate::autonodes::scoped_property_access_expression::ScopedPropertyAccessExpressionNode;
use crate::autonodes::string::StringNode;
use crate::autonodes::text_interpolation::TextInterpolationNode;
use crate::autonodes::unary_op_expression::UnaryOpExpressionNode;
use crate::autonodes::variable_name::VariableNameNode;
use crate::autotree::NodeAccess;
use crate::autotree::ParseError;
use crate::errornode::ErrorNode;
use crate::extra::ExtraChild;
use crate::issue::IssueEmitter;
use crate::types::union::UnionType;
use crate::value::PHPValue;
use tree_sitter::Node;
use tree_sitter::Range;

#[derive(Debug, Clone)]
pub enum SubscriptExpressionDereferenceable {
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
    Comment(Box<CommentNode>),
    TextInterpolation(Box<TextInterpolationNode>),
    Error(Box<ErrorNode>),
}

impl SubscriptExpressionDereferenceable {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => SubscriptExpressionDereferenceable::Comment(Box::new(CommentNode::parse(
                node, source,
            )?)),
            "text_interpolation" => SubscriptExpressionDereferenceable::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            ),
            "ERROR" => {
                SubscriptExpressionDereferenceable::Error(Box::new(ErrorNode::parse(node, source)?))
            }
            "array_creation_expression" => {
                SubscriptExpressionDereferenceable::ArrayCreationExpression(Box::new(
                    ArrayCreationExpressionNode::parse(node, source)?,
                ))
            }
            "cast_expression" => SubscriptExpressionDereferenceable::CastExpression(Box::new(
                CastExpressionNode::parse(node, source)?,
            )),
            "class_constant_access_expression" => {
                SubscriptExpressionDereferenceable::ClassConstantAccessExpression(Box::new(
                    ClassConstantAccessExpressionNode::parse(node, source)?,
                ))
            }
            "dynamic_variable_name" => SubscriptExpressionDereferenceable::DynamicVariableName(
                Box::new(DynamicVariableNameNode::parse(node, source)?),
            ),
            "encapsed_string" => SubscriptExpressionDereferenceable::EncapsedString(Box::new(
                EncapsedStringNode::parse(node, source)?,
            )),
            "function_call_expression" => {
                SubscriptExpressionDereferenceable::FunctionCallExpression(Box::new(
                    FunctionCallExpressionNode::parse(node, source)?,
                ))
            }
            "heredoc" => SubscriptExpressionDereferenceable::Heredoc(Box::new(HeredocNode::parse(
                node, source,
            )?)),
            "member_access_expression" => {
                SubscriptExpressionDereferenceable::MemberAccessExpression(Box::new(
                    MemberAccessExpressionNode::parse(node, source)?,
                ))
            }
            "member_call_expression" => SubscriptExpressionDereferenceable::MemberCallExpression(
                Box::new(MemberCallExpressionNode::parse(node, source)?),
            ),
            "name" => {
                SubscriptExpressionDereferenceable::Name(Box::new(NameNode::parse(node, source)?))
            }
            "nowdoc" => SubscriptExpressionDereferenceable::Nowdoc(Box::new(NowdocNode::parse(
                node, source,
            )?)),
            "nullsafe_member_access_expression" => {
                SubscriptExpressionDereferenceable::NullsafeMemberAccessExpression(Box::new(
                    NullsafeMemberAccessExpressionNode::parse(node, source)?,
                ))
            }
            "nullsafe_member_call_expression" => {
                SubscriptExpressionDereferenceable::NullsafeMemberCallExpression(Box::new(
                    NullsafeMemberCallExpressionNode::parse(node, source)?,
                ))
            }
            "parenthesized_expression" => {
                SubscriptExpressionDereferenceable::ParenthesizedExpression(Box::new(
                    ParenthesizedExpressionNode::parse(node, source)?,
                ))
            }
            "qualified_name" => SubscriptExpressionDereferenceable::QualifiedName(Box::new(
                QualifiedNameNode::parse(node, source)?,
            )),
            "scoped_call_expression" => SubscriptExpressionDereferenceable::ScopedCallExpression(
                Box::new(ScopedCallExpressionNode::parse(node, source)?),
            ),
            "scoped_property_access_expression" => {
                SubscriptExpressionDereferenceable::ScopedPropertyAccessExpression(Box::new(
                    ScopedPropertyAccessExpressionNode::parse(node, source)?,
                ))
            }
            "string" => SubscriptExpressionDereferenceable::String(Box::new(StringNode::parse(
                node, source,
            )?)),
            "subscript_expression" => SubscriptExpressionDereferenceable::SubscriptExpression(
                Box::new(SubscriptExpressionNode::parse(node, source)?),
            ),
            "variable_name" => SubscriptExpressionDereferenceable::VariableName(Box::new(
                VariableNameNode::parse(node, source)?,
            )),

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
            "comment" => SubscriptExpressionDereferenceable::Comment(Box::new(CommentNode::parse(
                node, source,
            )?)),
            "text_interpolation" => SubscriptExpressionDereferenceable::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            ),
            "ERROR" => {
                SubscriptExpressionDereferenceable::Error(Box::new(ErrorNode::parse(node, source)?))
            }
            "array_creation_expression" => {
                SubscriptExpressionDereferenceable::ArrayCreationExpression(Box::new(
                    ArrayCreationExpressionNode::parse(node, source)?,
                ))
            }
            "cast_expression" => SubscriptExpressionDereferenceable::CastExpression(Box::new(
                CastExpressionNode::parse(node, source)?,
            )),
            "class_constant_access_expression" => {
                SubscriptExpressionDereferenceable::ClassConstantAccessExpression(Box::new(
                    ClassConstantAccessExpressionNode::parse(node, source)?,
                ))
            }
            "dynamic_variable_name" => SubscriptExpressionDereferenceable::DynamicVariableName(
                Box::new(DynamicVariableNameNode::parse(node, source)?),
            ),
            "encapsed_string" => SubscriptExpressionDereferenceable::EncapsedString(Box::new(
                EncapsedStringNode::parse(node, source)?,
            )),
            "function_call_expression" => {
                SubscriptExpressionDereferenceable::FunctionCallExpression(Box::new(
                    FunctionCallExpressionNode::parse(node, source)?,
                ))
            }
            "heredoc" => SubscriptExpressionDereferenceable::Heredoc(Box::new(HeredocNode::parse(
                node, source,
            )?)),
            "member_access_expression" => {
                SubscriptExpressionDereferenceable::MemberAccessExpression(Box::new(
                    MemberAccessExpressionNode::parse(node, source)?,
                ))
            }
            "member_call_expression" => SubscriptExpressionDereferenceable::MemberCallExpression(
                Box::new(MemberCallExpressionNode::parse(node, source)?),
            ),
            "name" => {
                SubscriptExpressionDereferenceable::Name(Box::new(NameNode::parse(node, source)?))
            }
            "nowdoc" => SubscriptExpressionDereferenceable::Nowdoc(Box::new(NowdocNode::parse(
                node, source,
            )?)),
            "nullsafe_member_access_expression" => {
                SubscriptExpressionDereferenceable::NullsafeMemberAccessExpression(Box::new(
                    NullsafeMemberAccessExpressionNode::parse(node, source)?,
                ))
            }
            "nullsafe_member_call_expression" => {
                SubscriptExpressionDereferenceable::NullsafeMemberCallExpression(Box::new(
                    NullsafeMemberCallExpressionNode::parse(node, source)?,
                ))
            }
            "parenthesized_expression" => {
                SubscriptExpressionDereferenceable::ParenthesizedExpression(Box::new(
                    ParenthesizedExpressionNode::parse(node, source)?,
                ))
            }
            "qualified_name" => SubscriptExpressionDereferenceable::QualifiedName(Box::new(
                QualifiedNameNode::parse(node, source)?,
            )),
            "scoped_call_expression" => SubscriptExpressionDereferenceable::ScopedCallExpression(
                Box::new(ScopedCallExpressionNode::parse(node, source)?),
            ),
            "scoped_property_access_expression" => {
                SubscriptExpressionDereferenceable::ScopedPropertyAccessExpression(Box::new(
                    ScopedPropertyAccessExpressionNode::parse(node, source)?,
                ))
            }
            "string" => SubscriptExpressionDereferenceable::String(Box::new(StringNode::parse(
                node, source,
            )?)),
            "subscript_expression" => SubscriptExpressionDereferenceable::SubscriptExpression(
                Box::new(SubscriptExpressionNode::parse(node, source)?),
            ),
            "variable_name" => SubscriptExpressionDereferenceable::VariableName(Box::new(
                VariableNameNode::parse(node, source)?,
            )),

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
            SubscriptExpressionDereferenceable::Comment(x) => x.get_utype(state, emitter),
            SubscriptExpressionDereferenceable::TextInterpolation(x) => x.get_utype(state, emitter),
            SubscriptExpressionDereferenceable::Error(x) => x.get_utype(state, emitter),
            SubscriptExpressionDereferenceable::ArrayCreationExpression(x) => {
                x.get_utype(state, emitter)
            }
            SubscriptExpressionDereferenceable::CastExpression(x) => x.get_utype(state, emitter),
            SubscriptExpressionDereferenceable::ClassConstantAccessExpression(x) => {
                x.get_utype(state, emitter)
            }
            SubscriptExpressionDereferenceable::DynamicVariableName(x) => {
                x.get_utype(state, emitter)
            }
            SubscriptExpressionDereferenceable::EncapsedString(x) => x.get_utype(state, emitter),
            SubscriptExpressionDereferenceable::FunctionCallExpression(x) => {
                x.get_utype(state, emitter)
            }
            SubscriptExpressionDereferenceable::Heredoc(x) => x.get_utype(state, emitter),
            SubscriptExpressionDereferenceable::MemberAccessExpression(x) => {
                x.get_utype(state, emitter)
            }
            SubscriptExpressionDereferenceable::MemberCallExpression(x) => {
                x.get_utype(state, emitter)
            }
            SubscriptExpressionDereferenceable::Name(x) => x.get_utype(state, emitter),
            SubscriptExpressionDereferenceable::Nowdoc(x) => x.get_utype(state, emitter),
            SubscriptExpressionDereferenceable::NullsafeMemberAccessExpression(x) => {
                x.get_utype(state, emitter)
            }
            SubscriptExpressionDereferenceable::NullsafeMemberCallExpression(x) => {
                x.get_utype(state, emitter)
            }
            SubscriptExpressionDereferenceable::ParenthesizedExpression(x) => {
                x.get_utype(state, emitter)
            }
            SubscriptExpressionDereferenceable::QualifiedName(x) => x.get_utype(state, emitter),
            SubscriptExpressionDereferenceable::ScopedCallExpression(x) => {
                x.get_utype(state, emitter)
            }
            SubscriptExpressionDereferenceable::ScopedPropertyAccessExpression(x) => {
                x.get_utype(state, emitter)
            }
            SubscriptExpressionDereferenceable::String(x) => x.get_utype(state, emitter),
            SubscriptExpressionDereferenceable::SubscriptExpression(x) => {
                x.get_utype(state, emitter)
            }
            SubscriptExpressionDereferenceable::VariableName(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            SubscriptExpressionDereferenceable::Comment(x) => x.get_php_value(state, emitter),
            SubscriptExpressionDereferenceable::TextInterpolation(x) => {
                x.get_php_value(state, emitter)
            }
            SubscriptExpressionDereferenceable::Error(x) => x.get_php_value(state, emitter),
            SubscriptExpressionDereferenceable::ArrayCreationExpression(x) => {
                x.get_php_value(state, emitter)
            }
            SubscriptExpressionDereferenceable::CastExpression(x) => {
                x.get_php_value(state, emitter)
            }
            SubscriptExpressionDereferenceable::ClassConstantAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            SubscriptExpressionDereferenceable::DynamicVariableName(x) => {
                x.get_php_value(state, emitter)
            }
            SubscriptExpressionDereferenceable::EncapsedString(x) => {
                x.get_php_value(state, emitter)
            }
            SubscriptExpressionDereferenceable::FunctionCallExpression(x) => {
                x.get_php_value(state, emitter)
            }
            SubscriptExpressionDereferenceable::Heredoc(x) => x.get_php_value(state, emitter),
            SubscriptExpressionDereferenceable::MemberAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            SubscriptExpressionDereferenceable::MemberCallExpression(x) => {
                x.get_php_value(state, emitter)
            }
            SubscriptExpressionDereferenceable::Name(x) => x.get_php_value(state, emitter),
            SubscriptExpressionDereferenceable::Nowdoc(x) => x.get_php_value(state, emitter),
            SubscriptExpressionDereferenceable::NullsafeMemberAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            SubscriptExpressionDereferenceable::NullsafeMemberCallExpression(x) => {
                x.get_php_value(state, emitter)
            }
            SubscriptExpressionDereferenceable::ParenthesizedExpression(x) => {
                x.get_php_value(state, emitter)
            }
            SubscriptExpressionDereferenceable::QualifiedName(x) => x.get_php_value(state, emitter),
            SubscriptExpressionDereferenceable::ScopedCallExpression(x) => {
                x.get_php_value(state, emitter)
            }
            SubscriptExpressionDereferenceable::ScopedPropertyAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            SubscriptExpressionDereferenceable::String(x) => x.get_php_value(state, emitter),
            SubscriptExpressionDereferenceable::SubscriptExpression(x) => {
                x.get_php_value(state, emitter)
            }
            SubscriptExpressionDereferenceable::VariableName(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            SubscriptExpressionDereferenceable::Comment(x) => x.read_from(state, emitter),
            SubscriptExpressionDereferenceable::TextInterpolation(x) => x.read_from(state, emitter),
            SubscriptExpressionDereferenceable::Error(x) => x.read_from(state, emitter),
            SubscriptExpressionDereferenceable::ArrayCreationExpression(x) => {
                x.read_from(state, emitter)
            }
            SubscriptExpressionDereferenceable::CastExpression(x) => x.read_from(state, emitter),
            SubscriptExpressionDereferenceable::ClassConstantAccessExpression(x) => {
                x.read_from(state, emitter)
            }
            SubscriptExpressionDereferenceable::DynamicVariableName(x) => {
                x.read_from(state, emitter)
            }
            SubscriptExpressionDereferenceable::EncapsedString(x) => x.read_from(state, emitter),
            SubscriptExpressionDereferenceable::FunctionCallExpression(x) => {
                x.read_from(state, emitter)
            }
            SubscriptExpressionDereferenceable::Heredoc(x) => x.read_from(state, emitter),
            SubscriptExpressionDereferenceable::MemberAccessExpression(x) => {
                x.read_from(state, emitter)
            }
            SubscriptExpressionDereferenceable::MemberCallExpression(x) => {
                x.read_from(state, emitter)
            }
            SubscriptExpressionDereferenceable::Name(x) => x.read_from(state, emitter),
            SubscriptExpressionDereferenceable::Nowdoc(x) => x.read_from(state, emitter),
            SubscriptExpressionDereferenceable::NullsafeMemberAccessExpression(x) => {
                x.read_from(state, emitter)
            }
            SubscriptExpressionDereferenceable::NullsafeMemberCallExpression(x) => {
                x.read_from(state, emitter)
            }
            SubscriptExpressionDereferenceable::ParenthesizedExpression(x) => {
                x.read_from(state, emitter)
            }
            SubscriptExpressionDereferenceable::QualifiedName(x) => x.read_from(state, emitter),
            SubscriptExpressionDereferenceable::ScopedCallExpression(x) => {
                x.read_from(state, emitter)
            }
            SubscriptExpressionDereferenceable::ScopedPropertyAccessExpression(x) => {
                x.read_from(state, emitter)
            }
            SubscriptExpressionDereferenceable::String(x) => x.read_from(state, emitter),
            SubscriptExpressionDereferenceable::SubscriptExpression(x) => {
                x.read_from(state, emitter)
            }
            SubscriptExpressionDereferenceable::VariableName(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for SubscriptExpressionDereferenceable {
    fn brief_desc(&self) -> String {
        match self {
            SubscriptExpressionDereferenceable::Comment(x) => format!(
                "SubscriptExpressionDereferenceable::comment({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferenceable::TextInterpolation(x) => format!(
                "SubscriptExpressionDereferenceable::text_interpolation({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferenceable::Error(x) => format!(
                "SubscriptExpressionDereferenceable::ERROR({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferenceable::ArrayCreationExpression(x) => format!(
                "SubscriptExpressionDereferenceable::array_creation_expression({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferenceable::CastExpression(x) => format!(
                "SubscriptExpressionDereferenceable::cast_expression({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferenceable::ClassConstantAccessExpression(x) => format!(
                "SubscriptExpressionDereferenceable::class_constant_access_expression({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferenceable::DynamicVariableName(x) => format!(
                "SubscriptExpressionDereferenceable::dynamic_variable_name({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferenceable::EncapsedString(x) => format!(
                "SubscriptExpressionDereferenceable::encapsed_string({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferenceable::FunctionCallExpression(x) => format!(
                "SubscriptExpressionDereferenceable::function_call_expression({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferenceable::Heredoc(x) => format!(
                "SubscriptExpressionDereferenceable::heredoc({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferenceable::MemberAccessExpression(x) => format!(
                "SubscriptExpressionDereferenceable::member_access_expression({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferenceable::MemberCallExpression(x) => format!(
                "SubscriptExpressionDereferenceable::member_call_expression({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferenceable::Name(x) => format!(
                "SubscriptExpressionDereferenceable::name({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferenceable::Nowdoc(x) => format!(
                "SubscriptExpressionDereferenceable::nowdoc({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferenceable::NullsafeMemberAccessExpression(x) => format!(
                "SubscriptExpressionDereferenceable::nullsafe_member_access_expression({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferenceable::NullsafeMemberCallExpression(x) => format!(
                "SubscriptExpressionDereferenceable::nullsafe_member_call_expression({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferenceable::ParenthesizedExpression(x) => format!(
                "SubscriptExpressionDereferenceable::parenthesized_expression({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferenceable::QualifiedName(x) => format!(
                "SubscriptExpressionDereferenceable::qualified_name({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferenceable::ScopedCallExpression(x) => format!(
                "SubscriptExpressionDereferenceable::scoped_call_expression({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferenceable::ScopedPropertyAccessExpression(x) => format!(
                "SubscriptExpressionDereferenceable::scoped_property_access_expression({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferenceable::String(x) => format!(
                "SubscriptExpressionDereferenceable::string({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferenceable::SubscriptExpression(x) => format!(
                "SubscriptExpressionDereferenceable::subscript_expression({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferenceable::VariableName(x) => format!(
                "SubscriptExpressionDereferenceable::variable_name({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            SubscriptExpressionDereferenceable::Comment(x) => x.as_any(),
            SubscriptExpressionDereferenceable::TextInterpolation(x) => x.as_any(),
            SubscriptExpressionDereferenceable::Error(x) => x.as_any(),
            SubscriptExpressionDereferenceable::ArrayCreationExpression(x) => x.as_any(),
            SubscriptExpressionDereferenceable::CastExpression(x) => x.as_any(),
            SubscriptExpressionDereferenceable::ClassConstantAccessExpression(x) => x.as_any(),
            SubscriptExpressionDereferenceable::DynamicVariableName(x) => x.as_any(),
            SubscriptExpressionDereferenceable::EncapsedString(x) => x.as_any(),
            SubscriptExpressionDereferenceable::FunctionCallExpression(x) => x.as_any(),
            SubscriptExpressionDereferenceable::Heredoc(x) => x.as_any(),
            SubscriptExpressionDereferenceable::MemberAccessExpression(x) => x.as_any(),
            SubscriptExpressionDereferenceable::MemberCallExpression(x) => x.as_any(),
            SubscriptExpressionDereferenceable::Name(x) => x.as_any(),
            SubscriptExpressionDereferenceable::Nowdoc(x) => x.as_any(),
            SubscriptExpressionDereferenceable::NullsafeMemberAccessExpression(x) => x.as_any(),
            SubscriptExpressionDereferenceable::NullsafeMemberCallExpression(x) => x.as_any(),
            SubscriptExpressionDereferenceable::ParenthesizedExpression(x) => x.as_any(),
            SubscriptExpressionDereferenceable::QualifiedName(x) => x.as_any(),
            SubscriptExpressionDereferenceable::ScopedCallExpression(x) => x.as_any(),
            SubscriptExpressionDereferenceable::ScopedPropertyAccessExpression(x) => x.as_any(),
            SubscriptExpressionDereferenceable::String(x) => x.as_any(),
            SubscriptExpressionDereferenceable::SubscriptExpression(x) => x.as_any(),
            SubscriptExpressionDereferenceable::VariableName(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            SubscriptExpressionDereferenceable::Comment(x) => x.children_any(),
            SubscriptExpressionDereferenceable::TextInterpolation(x) => x.children_any(),
            SubscriptExpressionDereferenceable::Error(x) => x.children_any(),
            SubscriptExpressionDereferenceable::ArrayCreationExpression(x) => x.children_any(),
            SubscriptExpressionDereferenceable::CastExpression(x) => x.children_any(),
            SubscriptExpressionDereferenceable::ClassConstantAccessExpression(x) => {
                x.children_any()
            }
            SubscriptExpressionDereferenceable::DynamicVariableName(x) => x.children_any(),
            SubscriptExpressionDereferenceable::EncapsedString(x) => x.children_any(),
            SubscriptExpressionDereferenceable::FunctionCallExpression(x) => x.children_any(),
            SubscriptExpressionDereferenceable::Heredoc(x) => x.children_any(),
            SubscriptExpressionDereferenceable::MemberAccessExpression(x) => x.children_any(),
            SubscriptExpressionDereferenceable::MemberCallExpression(x) => x.children_any(),
            SubscriptExpressionDereferenceable::Name(x) => x.children_any(),
            SubscriptExpressionDereferenceable::Nowdoc(x) => x.children_any(),
            SubscriptExpressionDereferenceable::NullsafeMemberAccessExpression(x) => {
                x.children_any()
            }
            SubscriptExpressionDereferenceable::NullsafeMemberCallExpression(x) => x.children_any(),
            SubscriptExpressionDereferenceable::ParenthesizedExpression(x) => x.children_any(),
            SubscriptExpressionDereferenceable::QualifiedName(x) => x.children_any(),
            SubscriptExpressionDereferenceable::ScopedCallExpression(x) => x.children_any(),
            SubscriptExpressionDereferenceable::ScopedPropertyAccessExpression(x) => {
                x.children_any()
            }
            SubscriptExpressionDereferenceable::String(x) => x.children_any(),
            SubscriptExpressionDereferenceable::SubscriptExpression(x) => x.children_any(),
            SubscriptExpressionDereferenceable::VariableName(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            SubscriptExpressionDereferenceable::Comment(x) => x.range(),
            SubscriptExpressionDereferenceable::TextInterpolation(x) => x.range(),
            SubscriptExpressionDereferenceable::Error(x) => x.range(),
            SubscriptExpressionDereferenceable::ArrayCreationExpression(x) => x.range(),
            SubscriptExpressionDereferenceable::CastExpression(x) => x.range(),
            SubscriptExpressionDereferenceable::ClassConstantAccessExpression(x) => x.range(),
            SubscriptExpressionDereferenceable::DynamicVariableName(x) => x.range(),
            SubscriptExpressionDereferenceable::EncapsedString(x) => x.range(),
            SubscriptExpressionDereferenceable::FunctionCallExpression(x) => x.range(),
            SubscriptExpressionDereferenceable::Heredoc(x) => x.range(),
            SubscriptExpressionDereferenceable::MemberAccessExpression(x) => x.range(),
            SubscriptExpressionDereferenceable::MemberCallExpression(x) => x.range(),
            SubscriptExpressionDereferenceable::Name(x) => x.range(),
            SubscriptExpressionDereferenceable::Nowdoc(x) => x.range(),
            SubscriptExpressionDereferenceable::NullsafeMemberAccessExpression(x) => x.range(),
            SubscriptExpressionDereferenceable::NullsafeMemberCallExpression(x) => x.range(),
            SubscriptExpressionDereferenceable::ParenthesizedExpression(x) => x.range(),
            SubscriptExpressionDereferenceable::QualifiedName(x) => x.range(),
            SubscriptExpressionDereferenceable::ScopedCallExpression(x) => x.range(),
            SubscriptExpressionDereferenceable::ScopedPropertyAccessExpression(x) => x.range(),
            SubscriptExpressionDereferenceable::String(x) => x.range(),
            SubscriptExpressionDereferenceable::SubscriptExpression(x) => x.range(),
            SubscriptExpressionDereferenceable::VariableName(x) => x.range(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum SubscriptExpressionChildren {
    Integer(Box<IntegerNode>),
    Name(Box<NameNode>),
    UnaryOpExpression(Box<UnaryOpExpressionNode>),
    VariableName(Box<VariableNameNode>),
    Comment(Box<CommentNode>),
    TextInterpolation(Box<TextInterpolationNode>),
    Error(Box<ErrorNode>),
}

impl SubscriptExpressionChildren {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => {
                SubscriptExpressionChildren::Comment(Box::new(CommentNode::parse(node, source)?))
            }
            "text_interpolation" => SubscriptExpressionChildren::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            )),
            "ERROR" => {
                SubscriptExpressionChildren::Error(Box::new(ErrorNode::parse(node, source)?))
            }
            "integer" => {
                SubscriptExpressionChildren::Integer(Box::new(IntegerNode::parse(node, source)?))
            }
            "name" => SubscriptExpressionChildren::Name(Box::new(NameNode::parse(node, source)?)),
            "unary_op_expression" => SubscriptExpressionChildren::UnaryOpExpression(Box::new(
                UnaryOpExpressionNode::parse(node, source)?,
            )),
            "variable_name" => SubscriptExpressionChildren::VariableName(Box::new(
                VariableNameNode::parse(node, source)?,
            )),

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
                SubscriptExpressionChildren::Comment(Box::new(CommentNode::parse(node, source)?))
            }
            "text_interpolation" => SubscriptExpressionChildren::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            )),
            "ERROR" => {
                SubscriptExpressionChildren::Error(Box::new(ErrorNode::parse(node, source)?))
            }
            "integer" => {
                SubscriptExpressionChildren::Integer(Box::new(IntegerNode::parse(node, source)?))
            }
            "name" => SubscriptExpressionChildren::Name(Box::new(NameNode::parse(node, source)?)),
            "unary_op_expression" => SubscriptExpressionChildren::UnaryOpExpression(Box::new(
                UnaryOpExpressionNode::parse(node, source)?,
            )),
            "variable_name" => SubscriptExpressionChildren::VariableName(Box::new(
                VariableNameNode::parse(node, source)?,
            )),

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
            SubscriptExpressionChildren::Comment(x) => x.get_utype(state, emitter),
            SubscriptExpressionChildren::TextInterpolation(x) => x.get_utype(state, emitter),
            SubscriptExpressionChildren::Error(x) => x.get_utype(state, emitter),
            SubscriptExpressionChildren::Integer(x) => x.get_utype(state, emitter),
            SubscriptExpressionChildren::Name(x) => x.get_utype(state, emitter),
            SubscriptExpressionChildren::UnaryOpExpression(x) => x.get_utype(state, emitter),
            SubscriptExpressionChildren::VariableName(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            SubscriptExpressionChildren::Comment(x) => x.get_php_value(state, emitter),
            SubscriptExpressionChildren::TextInterpolation(x) => x.get_php_value(state, emitter),
            SubscriptExpressionChildren::Error(x) => x.get_php_value(state, emitter),
            SubscriptExpressionChildren::Integer(x) => x.get_php_value(state, emitter),
            SubscriptExpressionChildren::Name(x) => x.get_php_value(state, emitter),
            SubscriptExpressionChildren::UnaryOpExpression(x) => x.get_php_value(state, emitter),
            SubscriptExpressionChildren::VariableName(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            SubscriptExpressionChildren::Comment(x) => x.read_from(state, emitter),
            SubscriptExpressionChildren::TextInterpolation(x) => x.read_from(state, emitter),
            SubscriptExpressionChildren::Error(x) => x.read_from(state, emitter),
            SubscriptExpressionChildren::Integer(x) => x.read_from(state, emitter),
            SubscriptExpressionChildren::Name(x) => x.read_from(state, emitter),
            SubscriptExpressionChildren::UnaryOpExpression(x) => x.read_from(state, emitter),
            SubscriptExpressionChildren::VariableName(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for SubscriptExpressionChildren {
    fn brief_desc(&self) -> String {
        match self {
            SubscriptExpressionChildren::Comment(x) => {
                format!("SubscriptExpressionChildren::comment({})", x.brief_desc())
            }
            SubscriptExpressionChildren::TextInterpolation(x) => format!(
                "SubscriptExpressionChildren::text_interpolation({})",
                x.brief_desc()
            ),
            SubscriptExpressionChildren::Error(x) => {
                format!("SubscriptExpressionChildren::ERROR({})", x.brief_desc())
            }
            SubscriptExpressionChildren::Integer(x) => {
                format!("SubscriptExpressionChildren::integer({})", x.brief_desc())
            }
            SubscriptExpressionChildren::Name(x) => {
                format!("SubscriptExpressionChildren::name({})", x.brief_desc())
            }
            SubscriptExpressionChildren::UnaryOpExpression(x) => format!(
                "SubscriptExpressionChildren::unary_op_expression({})",
                x.brief_desc()
            ),
            SubscriptExpressionChildren::VariableName(x) => format!(
                "SubscriptExpressionChildren::variable_name({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            SubscriptExpressionChildren::Comment(x) => x.as_any(),
            SubscriptExpressionChildren::TextInterpolation(x) => x.as_any(),
            SubscriptExpressionChildren::Error(x) => x.as_any(),
            SubscriptExpressionChildren::Integer(x) => x.as_any(),
            SubscriptExpressionChildren::Name(x) => x.as_any(),
            SubscriptExpressionChildren::UnaryOpExpression(x) => x.as_any(),
            SubscriptExpressionChildren::VariableName(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            SubscriptExpressionChildren::Comment(x) => x.children_any(),
            SubscriptExpressionChildren::TextInterpolation(x) => x.children_any(),
            SubscriptExpressionChildren::Error(x) => x.children_any(),
            SubscriptExpressionChildren::Integer(x) => x.children_any(),
            SubscriptExpressionChildren::Name(x) => x.children_any(),
            SubscriptExpressionChildren::UnaryOpExpression(x) => x.children_any(),
            SubscriptExpressionChildren::VariableName(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            SubscriptExpressionChildren::Comment(x) => x.range(),
            SubscriptExpressionChildren::TextInterpolation(x) => x.range(),
            SubscriptExpressionChildren::Error(x) => x.range(),
            SubscriptExpressionChildren::Integer(x) => x.range(),
            SubscriptExpressionChildren::Name(x) => x.range(),
            SubscriptExpressionChildren::UnaryOpExpression(x) => x.range(),
            SubscriptExpressionChildren::VariableName(x) => x.range(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct SubscriptExpressionNode {
    pub range: Range,
    pub dereferenceable: Option<Box<SubscriptExpressionDereferenceable>>,
    pub index: Option<_ExpressionNode>,
    pub children: Vec<Box<SubscriptExpressionChildren>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl SubscriptExpressionNode {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
        if node.kind() != "subscript_expression" {
            return Err(ParseError::new(range, format!("Node is of the wrong kind [{}] vs expected [subscript_expression] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }
        let mut skip_nodes: Vec<usize> = vec![];
        let dereferenceable: Option<Box<SubscriptExpressionDereferenceable>> = node
            .children_by_field_name("dereferenceable", &mut node.walk())
            .map(|chnode| {
                skip_nodes.push(chnode.id());
                chnode
            })
            .map(|chnode2| SubscriptExpressionDereferenceable::parse(chnode2, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .map(|z| Box::new(z))
            .next()
            .into();
        let index: Option<_ExpressionNode> = node
            .children_by_field_name("index", &mut node.walk())
            .map(|chnode| {
                skip_nodes.push(chnode.id());
                chnode
            })
            .map(|chnode1| _ExpressionNode::parse(chnode1, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .next();
        Ok(Self {
            range,
            dereferenceable,
            index,
            children: SubscriptExpressionChildren::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| !skip_nodes.contains(&node.id()))
                    .filter(|node| node.kind() != "comment"),
                source,
            )?,
            extras: ExtraChild::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| node.kind() == "comment")
                    .filter(|node| !skip_nodes.contains(&node.id())),
                source,
            )?,
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
        "subscript_expression"
    }
}

impl NodeAccess for SubscriptExpressionNode {
    fn brief_desc(&self) -> String {
        "SubscriptExpressionNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::SubscriptExpression(self)
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        if let Some(x) = &self.dereferenceable {
            child_vec.push(x.as_any());
        }
        if let Some(x) = &self.index {
            child_vec.push(x.as_any());
        }
        child_vec.extend(self.children.iter().map(|n| n.as_any()));
        child_vec.extend(self.extras.iter().map(|n| n.as_any()));

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
