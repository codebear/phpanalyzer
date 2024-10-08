use crate::analysis::state::AnalysisState;
use crate::autonodes::_expression::_ExpressionNode;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::array_element_initializer::ArrayElementInitializerNode;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::text_interpolation::TextInterpolationNode;
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
pub enum YieldExpressionChildren {
    _Expression(Box<_ExpressionNode>),
    ArrayElementInitializer(Box<ArrayElementInitializerNode>),
    Extra(ExtraChild),
}

impl NodeParser for YieldExpressionChildren {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => YieldExpressionChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => YieldExpressionChildren::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => YieldExpressionChildren::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "array_element_initializer" => YieldExpressionChildren::ArrayElementInitializer(
                Box::new(ArrayElementInitializerNode::parse(node, source)?),
            ),

            _ => {
                if let Some(x) = _ExpressionNode::parse_opt(node, source)?
                    .map(Box::new)
                    .map(YieldExpressionChildren::_Expression)
                {
                    x
                } else {
                    return Err(ParseError::new(
                        node.range(),
                        format!(
                            "YieldExpressionChildren: Parse error, unexpected node-type: {}",
                            node.kind()
                        ),
                    ));
                }
            }
        })
    }
}

impl YieldExpressionChildren {
    pub fn parse_opt(node: Node, source: &[u8]) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => YieldExpressionChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => YieldExpressionChildren::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => YieldExpressionChildren::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "array_element_initializer" => YieldExpressionChildren::ArrayElementInitializer(
                Box::new(ArrayElementInitializerNode::parse(node, source)?),
            ),

            _ => {
                return Ok(_ExpressionNode::parse_opt(node, source)?
                    .map(Box::new)
                    .map(YieldExpressionChildren::_Expression))
            }
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            YieldExpressionChildren::Extra(y) => y.kind(),
            YieldExpressionChildren::_Expression(y) => y.kind(),
            YieldExpressionChildren::ArrayElementInitializer(y) => y.kind(),
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
            YieldExpressionChildren::Extra(x) => x.get_utype(state, emitter),
            YieldExpressionChildren::_Expression(x) => x.get_utype(state, emitter),
            YieldExpressionChildren::ArrayElementInitializer(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            YieldExpressionChildren::Extra(x) => x.get_php_value(state, emitter),
            YieldExpressionChildren::_Expression(x) => x.get_php_value(state, emitter),
            YieldExpressionChildren::ArrayElementInitializer(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            YieldExpressionChildren::Extra(x) => x.read_from(state, emitter),
            YieldExpressionChildren::_Expression(x) => x.read_from(state, emitter),
            YieldExpressionChildren::ArrayElementInitializer(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for YieldExpressionChildren {
    fn brief_desc(&self) -> String {
        match self {
            YieldExpressionChildren::Extra(x) => {
                format!("YieldExpressionChildren::extra({})", x.brief_desc())
            }
            YieldExpressionChildren::_Expression(x) => {
                format!("YieldExpressionChildren::_expression({})", x.brief_desc())
            }
            YieldExpressionChildren::ArrayElementInitializer(x) => format!(
                "YieldExpressionChildren::array_element_initializer({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        match self {
            YieldExpressionChildren::Extra(x) => x.as_any(),
            YieldExpressionChildren::_Expression(x) => x.as_any(),
            YieldExpressionChildren::ArrayElementInitializer(x) => x.as_any(),
        }
    }

    fn children_any(&self) -> Vec<AnyNodeRef<'_>> {
        match self {
            YieldExpressionChildren::Extra(x) => x.children_any(),
            YieldExpressionChildren::_Expression(x) => x.children_any(),
            YieldExpressionChildren::ArrayElementInitializer(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            YieldExpressionChildren::Extra(x) => x.range(),
            YieldExpressionChildren::_Expression(x) => x.range(),
            YieldExpressionChildren::ArrayElementInitializer(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct YieldExpressionNode {
    pub range: Range,
    pub child: Option<Box<YieldExpressionChildren>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for YieldExpressionNode {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "yield_expression" {
            return Err(ParseError::new(range, format!("YieldExpressionNode: Node is of the wrong kind [{}] vs expected [yield_expression] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }

        Ok(Self {
            range,
            child: node
                .named_children(&mut node.walk())
                .filter(|node| node.kind() != "comment")
                .map(|k| YieldExpressionChildren::parse(k, source))
                .collect::<Result<Vec<YieldExpressionChildren>, ParseError>>()?
                .drain(..)
                .map(Box::new)
                .next(),
            extras: ExtraChild::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| node.kind() == "comment"),
                source,
            )?,
        })
    }
}

impl YieldExpressionNode {
    pub fn kind(&self) -> &'static str {
        "yield_expression"
    }
}

impl NodeAccess for YieldExpressionNode {
    fn brief_desc(&self) -> String {
        "YieldExpressionNode".into()
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        AnyNodeRef::YieldExpression(self)
    }

    #[allow(clippy::vec_init_then_push)]
    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
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
