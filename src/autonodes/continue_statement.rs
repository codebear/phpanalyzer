use crate::autonodes::_expression::_ExpressionNode;
use crate::autonodes::any::AnyNodeRef;
use crate::autotree::NodeAccess;
use crate::autotree::NodeParser;
use crate::autotree::ParseError;
use crate::extra::ExtraChild;
use crate::parser::Range;
use tree_sitter::Node;

#[derive(Debug, Clone)]
pub struct ContinueStatementNode {
    pub range: Range,
    pub child: Option<Box<_ExpressionNode>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for ContinueStatementNode {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "continue_statement" {
            return Err(ParseError::new(range, format!("ContinueStatementNode: Node is of the wrong kind [{}] vs expected [continue_statement] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }

        Ok(Self {
            range,
            child: node
                .named_children(&mut node.walk())
                .filter(|node| node.kind() != "comment")
                .map(|k| _ExpressionNode::parse(k, source))
                .collect::<Result<Vec<_ExpressionNode>, ParseError>>()?
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

impl ContinueStatementNode {
    pub fn kind(&self) -> &'static str {
        "continue_statement"
    }
}

impl NodeAccess for ContinueStatementNode {
    fn brief_desc(&self) -> String {
        "ContinueStatementNode".into()
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        AnyNodeRef::ContinueStatement(self)
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
