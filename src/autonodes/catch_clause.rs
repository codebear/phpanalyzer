use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::compound_statement::CompoundStatementNode;
use crate::autonodes::type_list::TypeListNode;
use crate::autonodes::variable_name::VariableNameNode;
use crate::autotree::ChildNodeParser;
use crate::autotree::NodeAccess;
use crate::autotree::NodeParser;
use crate::autotree::ParseError;
use crate::extra::ExtraChild;
use crate::parser::Range;
use tree_sitter::Node;

#[derive(Debug, Clone)]
pub struct CatchClauseNode {
    pub range: Range,
    pub body: CompoundStatementNode,
    pub name: Option<VariableNameNode>,
    pub type_: TypeListNode,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for CatchClauseNode {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "catch_clause" {
            return Err(ParseError::new(range, format!("CatchClauseNode: Node is of the wrong kind [{}] vs expected [catch_clause] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }
        let body: CompoundStatementNode =
            Into::<Result<_, _>>::into(node.parse_child("body", source))?;
        let name: Option<VariableNameNode> =
            Into::<Result<_, _>>::into(node.parse_child("name", source))?;
        let type_: TypeListNode = Into::<Result<_, _>>::into(node.parse_child("type", source))?;
        Ok(Self {
            range,
            body,
            name,
            type_,
            extras: ExtraChild::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| node.kind() == "comment"),
                source,
            )
            .unwrap(),
        })
    }
}

impl CatchClauseNode {
    pub fn kind(&self) -> &'static str {
        "catch_clause"
    }
}

impl NodeAccess for CatchClauseNode {
    fn brief_desc(&self) -> String {
        "CatchClauseNode".into()
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        AnyNodeRef::CatchClause(self)
    }

    #[allow(clippy::vec_init_then_push)]
    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        child_vec.push(self.body.as_any());
        if let Some(x) = &self.name {
            child_vec.push(x.as_any());
        }
        child_vec.push(self.type_.as_any());

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
