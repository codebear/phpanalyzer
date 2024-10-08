use crate::analysis::state::AnalysisState;
use crate::autonodes::_type::_TypeNode;
use crate::autonodes::anonymous_function_use_clause::AnonymousFunctionUseClauseNode;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::attribute_list::AttributeListNode;
use crate::autonodes::bottom_type::BottomTypeNode;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::compound_statement::CompoundStatementNode;
use crate::autonodes::formal_parameters::FormalParametersNode;
use crate::autonodes::reference_modifier::ReferenceModifierNode;
use crate::autonodes::text_interpolation::TextInterpolationNode;
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
pub enum AnonymousFunctionCreationExpressionReturnType {
    _Type(Box<_TypeNode>),
    BottomType(Box<BottomTypeNode>),
    Extra(ExtraChild),
}

impl NodeParser for AnonymousFunctionCreationExpressionReturnType {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => AnonymousFunctionCreationExpressionReturnType::Extra(ExtraChild::Comment(
                Box::new(CommentNode::parse(node, source)?),
            )),
            "text_interpolation" => {
                AnonymousFunctionCreationExpressionReturnType::Extra(ExtraChild::TextInterpolation(
                    Box::new(TextInterpolationNode::parse(node, source)?),
                ))
            }
            "ERROR" => AnonymousFunctionCreationExpressionReturnType::Extra(ExtraChild::Error(
                Box::new(ErrorNode::parse(node, source)?),
            )),
            "bottom_type" => AnonymousFunctionCreationExpressionReturnType::BottomType(Box::new(
                BottomTypeNode::parse(node, source)?,
            )),

            _ => {
                if let Some(x) = _TypeNode::parse_opt(node, source)?
                    .map(Box::new)
                    .map(AnonymousFunctionCreationExpressionReturnType::_Type)
                {
                    x
                } else {
                    return Err(ParseError::new(node.range(), format!("AnonymousFunctionCreationExpressionReturnType: Parse error, unexpected node-type: {}", node.kind())));
                }
            }
        })
    }
}

impl AnonymousFunctionCreationExpressionReturnType {
    pub fn parse_opt(node: Node, source: &[u8]) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => AnonymousFunctionCreationExpressionReturnType::Extra(ExtraChild::Comment(
                Box::new(CommentNode::parse(node, source)?),
            )),
            "text_interpolation" => {
                AnonymousFunctionCreationExpressionReturnType::Extra(ExtraChild::TextInterpolation(
                    Box::new(TextInterpolationNode::parse(node, source)?),
                ))
            }
            "ERROR" => AnonymousFunctionCreationExpressionReturnType::Extra(ExtraChild::Error(
                Box::new(ErrorNode::parse(node, source)?),
            )),
            "bottom_type" => AnonymousFunctionCreationExpressionReturnType::BottomType(Box::new(
                BottomTypeNode::parse(node, source)?,
            )),

            _ => {
                return Ok(_TypeNode::parse_opt(node, source)?
                    .map(Box::new)
                    .map(AnonymousFunctionCreationExpressionReturnType::_Type))
            }
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            AnonymousFunctionCreationExpressionReturnType::Extra(y) => y.kind(),
            AnonymousFunctionCreationExpressionReturnType::_Type(y) => y.kind(),
            AnonymousFunctionCreationExpressionReturnType::BottomType(y) => y.kind(),
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
            AnonymousFunctionCreationExpressionReturnType::Extra(x) => x.get_utype(state, emitter),
            AnonymousFunctionCreationExpressionReturnType::_Type(x) => x.get_utype(state, emitter),
            AnonymousFunctionCreationExpressionReturnType::BottomType(x) => {
                x.get_utype(state, emitter)
            }
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            AnonymousFunctionCreationExpressionReturnType::Extra(x) => {
                x.get_php_value(state, emitter)
            }
            AnonymousFunctionCreationExpressionReturnType::_Type(x) => {
                x.get_php_value(state, emitter)
            }
            AnonymousFunctionCreationExpressionReturnType::BottomType(x) => {
                x.get_php_value(state, emitter)
            }
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            AnonymousFunctionCreationExpressionReturnType::Extra(x) => x.read_from(state, emitter),
            AnonymousFunctionCreationExpressionReturnType::_Type(x) => x.read_from(state, emitter),
            AnonymousFunctionCreationExpressionReturnType::BottomType(x) => {
                x.read_from(state, emitter)
            }
        }
    }
}

impl NodeAccess for AnonymousFunctionCreationExpressionReturnType {
    fn brief_desc(&self) -> String {
        match self {
            AnonymousFunctionCreationExpressionReturnType::Extra(x) => format!(
                "AnonymousFunctionCreationExpressionReturnType::extra({})",
                x.brief_desc()
            ),
            AnonymousFunctionCreationExpressionReturnType::_Type(x) => format!(
                "AnonymousFunctionCreationExpressionReturnType::_type({})",
                x.brief_desc()
            ),
            AnonymousFunctionCreationExpressionReturnType::BottomType(x) => format!(
                "AnonymousFunctionCreationExpressionReturnType::bottom_type({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        match self {
            AnonymousFunctionCreationExpressionReturnType::Extra(x) => x.as_any(),
            AnonymousFunctionCreationExpressionReturnType::_Type(x) => x.as_any(),
            AnonymousFunctionCreationExpressionReturnType::BottomType(x) => x.as_any(),
        }
    }

    fn children_any(&self) -> Vec<AnyNodeRef<'_>> {
        match self {
            AnonymousFunctionCreationExpressionReturnType::Extra(x) => x.children_any(),
            AnonymousFunctionCreationExpressionReturnType::_Type(x) => x.children_any(),
            AnonymousFunctionCreationExpressionReturnType::BottomType(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            AnonymousFunctionCreationExpressionReturnType::Extra(x) => x.range(),
            AnonymousFunctionCreationExpressionReturnType::_Type(x) => x.range(),
            AnonymousFunctionCreationExpressionReturnType::BottomType(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AnonymousFunctionCreationExpressionNode {
    pub range: Range,
    pub attributes: Option<AttributeListNode>,
    pub body: CompoundStatementNode,
    pub parameters: FormalParametersNode,
    pub reference_modifier: Option<ReferenceModifierNode>,
    pub return_type: Option<Box<AnonymousFunctionCreationExpressionReturnType>>,
    pub child: Option<Box<AnonymousFunctionUseClauseNode>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for AnonymousFunctionCreationExpressionNode {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "anonymous_function_creation_expression" {
            return Err(ParseError::new(range, format!("AnonymousFunctionCreationExpressionNode: Node is of the wrong kind [{}] vs expected [anonymous_function_creation_expression] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }
        let mut skip_nodes: Vec<usize> = vec![];
        let attributes: Option<AttributeListNode> = Into::<Result<_, _>>::into(
            node.parse_child("attributes", source)
                .mark_skipped_node(&mut skip_nodes),
        )?;
        let body: CompoundStatementNode = Into::<Result<_, _>>::into(
            node.parse_child("body", source)
                .mark_skipped_node(&mut skip_nodes),
        )?;
        let parameters: FormalParametersNode = Into::<Result<_, _>>::into(
            node.parse_child("parameters", source)
                .mark_skipped_node(&mut skip_nodes),
        )?;
        let reference_modifier: Option<ReferenceModifierNode> = Into::<Result<_, _>>::into(
            node.parse_child("reference_modifier", source)
                .mark_skipped_node(&mut skip_nodes),
        )?;
        let return_type: Option<Box<AnonymousFunctionCreationExpressionReturnType>> =
            Into::<Result<_, _>>::into(
                node.parse_child("return_type", source)
                    .mark_skipped_node(&mut skip_nodes),
            )?;
        Ok(Self {
            range,
            attributes,
            body,
            parameters,
            reference_modifier,
            return_type,
            child: node
                .named_children(&mut node.walk())
                .filter(|node| !skip_nodes.contains(&node.id()))
                .filter(|node| node.kind() != "comment")
                .map(|k| AnonymousFunctionUseClauseNode::parse(k, source))
                .collect::<Result<Vec<AnonymousFunctionUseClauseNode>, ParseError>>()?
                .drain(..)
                .map(Box::new)
                .next(),
            extras: ExtraChild::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| node.kind() == "comment")
                    .filter(|node| !skip_nodes.contains(&node.id())),
                source,
            )?,
        })
    }
}

impl AnonymousFunctionCreationExpressionNode {
    pub fn kind(&self) -> &'static str {
        "anonymous_function_creation_expression"
    }
}

impl NodeAccess for AnonymousFunctionCreationExpressionNode {
    fn brief_desc(&self) -> String {
        "AnonymousFunctionCreationExpressionNode".into()
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        AnyNodeRef::AnonymousFunctionCreationExpression(self)
    }

    #[allow(clippy::vec_init_then_push)]
    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        if let Some(x) = &self.attributes {
            child_vec.push(x.as_any());
        }
        child_vec.push(self.body.as_any());
        child_vec.push(self.parameters.as_any());
        if let Some(x) = &self.reference_modifier {
            child_vec.push(x.as_any());
        }
        if let Some(x) = &self.return_type {
            child_vec.push(x.as_any());
        }
        if let Some(x) = &self.child {
            child_vec.push(x.as_any());
        }
        child_vec.extend(self.extras.iter().map(|n| n.as_any()));

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
