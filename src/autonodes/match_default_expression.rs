use crate::autonodes::_expression::_ExpressionNode;
use crate::autonodes::any::AnyNodeRef;
use crate::autotree::ChildNodeParser;
use crate::autotree::NodeAccess;
use crate::autotree::NodeParser;
use crate::autotree::ParseError;
use crate::extra::ExtraChild;
use tree_sitter::Node;
use tree_sitter::Range;

#[derive(Debug, Clone)]
pub struct MatchDefaultExpressionNode {
    pub range: Range,
    pub return_expression: _ExpressionNode,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for MatchDefaultExpressionNode {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
        if node.kind() != "match_default_expression" {
            return Err(ParseError::new(range, format!("Node is of the wrong kind [{}] vs expected [match_default_expression] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }
        let return_expression: _ExpressionNode =
            Result::from(node.parse_child("return_expression", source).into())?;
        Ok(Self {
            range,
            return_expression,
            extras: ExtraChild::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| node.kind() == "comment"),
                source,
            )
            .unwrap(),
        })
    }
}

impl MatchDefaultExpressionNode {
    pub fn kind(&self) -> &'static str {
        "match_default_expression"
    }
}

impl NodeAccess for MatchDefaultExpressionNode {
    fn brief_desc(&self) -> String {
        "MatchDefaultExpressionNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::MatchDefaultExpression(self)
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        child_vec.push(self.return_expression.as_any());

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
