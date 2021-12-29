use crate::analysis::state::AnalysisState;
use crate::autonodes::_statement::_StatementNode;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::colon_block::ColonBlockNode;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::parenthesized_expression::ParenthesizedExpressionNode;
use crate::autonodes::text_interpolation::TextInterpolationNode;
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
pub enum WhileStatementBody {
    _Statement(Box<_StatementNode>),
    ColonBlock(Box<ColonBlockNode>),
    Comment(Box<CommentNode>),
    TextInterpolation(Box<TextInterpolationNode>),
    Error(Box<ErrorNode>),
}

impl WhileStatementBody {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => WhileStatementBody::Comment(Box::new(CommentNode::parse(node, source)?)),
            "text_interpolation" => WhileStatementBody::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            )),
            "ERROR" => WhileStatementBody::Error(Box::new(ErrorNode::parse(node, source)?)),
            "colon_block" => {
                WhileStatementBody::ColonBlock(Box::new(ColonBlockNode::parse(node, source)?))
            }

            _ => {
                if let Some(x) = _StatementNode::parse_opt(node, source)?
                    .map(|x| Box::new(x))
                    .map(|y| WhileStatementBody::_Statement(y))
                {
                    x
                } else {
                    return Err(ParseError::new(
                        node.range(),
                        format!("Parse error, unexpected node-type: {}", node.kind()),
                    ));
                }
            }
        })
    }

    pub fn parse_opt(node: Node, source: &Vec<u8>) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => WhileStatementBody::Comment(Box::new(CommentNode::parse(node, source)?)),
            "text_interpolation" => WhileStatementBody::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            )),
            "ERROR" => WhileStatementBody::Error(Box::new(ErrorNode::parse(node, source)?)),
            "colon_block" => {
                WhileStatementBody::ColonBlock(Box::new(ColonBlockNode::parse(node, source)?))
            }

            _ => {
                return Ok(
                    if let Some(x) = _StatementNode::parse_opt(node, source)?
                        .map(|x| Box::new(x))
                        .map(|y| WhileStatementBody::_Statement(y))
                    {
                        Some(x)
                    } else {
                        None
                    },
                )
            }
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
            WhileStatementBody::Comment(x) => x.get_utype(state, emitter),
            WhileStatementBody::TextInterpolation(x) => x.get_utype(state, emitter),
            WhileStatementBody::Error(x) => x.get_utype(state, emitter),
            WhileStatementBody::_Statement(x) => x.get_utype(state, emitter),
            WhileStatementBody::ColonBlock(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            WhileStatementBody::Comment(x) => x.get_php_value(state, emitter),
            WhileStatementBody::TextInterpolation(x) => x.get_php_value(state, emitter),
            WhileStatementBody::Error(x) => x.get_php_value(state, emitter),
            WhileStatementBody::_Statement(x) => x.get_php_value(state, emitter),
            WhileStatementBody::ColonBlock(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            WhileStatementBody::Comment(x) => x.read_from(state, emitter),
            WhileStatementBody::TextInterpolation(x) => x.read_from(state, emitter),
            WhileStatementBody::Error(x) => x.read_from(state, emitter),
            WhileStatementBody::_Statement(x) => x.read_from(state, emitter),
            WhileStatementBody::ColonBlock(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for WhileStatementBody {
    fn brief_desc(&self) -> String {
        match self {
            WhileStatementBody::Comment(x) => {
                format!("WhileStatementBody::comment({})", x.brief_desc())
            }
            WhileStatementBody::TextInterpolation(x) => {
                format!("WhileStatementBody::text_interpolation({})", x.brief_desc())
            }
            WhileStatementBody::Error(x) => {
                format!("WhileStatementBody::ERROR({})", x.brief_desc())
            }
            WhileStatementBody::_Statement(x) => {
                format!("WhileStatementBody::_statement({})", x.brief_desc())
            }
            WhileStatementBody::ColonBlock(x) => {
                format!("WhileStatementBody::colon_block({})", x.brief_desc())
            }
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            WhileStatementBody::Comment(x) => x.as_any(),
            WhileStatementBody::TextInterpolation(x) => x.as_any(),
            WhileStatementBody::Error(x) => x.as_any(),
            WhileStatementBody::_Statement(x) => x.as_any(),
            WhileStatementBody::ColonBlock(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            WhileStatementBody::Comment(x) => x.children_any(),
            WhileStatementBody::TextInterpolation(x) => x.children_any(),
            WhileStatementBody::Error(x) => x.children_any(),
            WhileStatementBody::_Statement(x) => x.children_any(),
            WhileStatementBody::ColonBlock(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            WhileStatementBody::Comment(x) => x.range(),
            WhileStatementBody::TextInterpolation(x) => x.range(),
            WhileStatementBody::Error(x) => x.range(),
            WhileStatementBody::_Statement(x) => x.range(),
            WhileStatementBody::ColonBlock(x) => x.range(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct WhileStatementNode {
    pub range: Range,
    pub body: Box<WhileStatementBody>,
    pub condition: ParenthesizedExpressionNode,
    pub extras: Vec<Box<ExtraChild>>,
}

impl WhileStatementNode {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
        if node.kind() != "while_statement" {
            return Err(ParseError::new(
                range,
                format!(
                    "Node is of the wrong kind [{}] vs expected [while_statement] on pos {}:{}",
                    node.kind(),
                    range.start_point.row + 1,
                    range.start_point.column
                ),
            ));
        }
        let body: Box<WhileStatementBody> = node
            .children_by_field_name("body", &mut node.walk())
            .map(|chnode2| WhileStatementBody::parse(chnode2, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .map(|z| Box::new(z))
            .next()
            .expect("Field body should exist")
            .into();
        let condition: ParenthesizedExpressionNode = node
            .children_by_field_name("condition", &mut node.walk())
            .map(|chnode1| ParenthesizedExpressionNode::parse(chnode1, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .next()
            .expect("Field condition should exist");
        Ok(Self {
            range,
            body,
            condition,
            extras: vec![], // todo lookup unused nodes
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
        "while_statement"
    }
}

impl NodeAccess for WhileStatementNode {
    fn brief_desc(&self) -> String {
        "WhileStatementNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::WhileStatement(self)
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        child_vec.push(self.body.as_any());
        child_vec.push(self.condition.as_any());

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
