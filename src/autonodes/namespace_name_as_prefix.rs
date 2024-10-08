use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::namespace_name::NamespaceNameNode;
use crate::autotree::NodeAccess;
use crate::autotree::NodeParser;
use crate::autotree::ParseError;
use crate::extra::ExtraChild;
use crate::parser::Range;
use tree_sitter::Node;

#[derive(Debug, Clone)]
pub struct NamespaceNameAsPrefixNode {
    pub range: Range,
    pub child: Option<Box<NamespaceNameNode>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for NamespaceNameAsPrefixNode {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "namespace_name_as_prefix" {
            return Err(ParseError::new(range, format!("NamespaceNameAsPrefixNode: Node is of the wrong kind [{}] vs expected [namespace_name_as_prefix] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }

        Ok(Self {
            range,
            child: node
                .named_children(&mut node.walk())
                .filter(|node| node.kind() != "comment")
                .map(|k| NamespaceNameNode::parse(k, source))
                .collect::<Result<Vec<NamespaceNameNode>, ParseError>>()?
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

impl NamespaceNameAsPrefixNode {
    pub fn kind(&self) -> &'static str {
        "namespace_name_as_prefix"
    }
}

impl NodeAccess for NamespaceNameAsPrefixNode {
    fn brief_desc(&self) -> String {
        "NamespaceNameAsPrefixNode".into()
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        AnyNodeRef::NamespaceNameAsPrefix(self)
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
