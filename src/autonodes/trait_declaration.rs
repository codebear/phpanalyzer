use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::declaration_list::DeclarationListNode;
use crate::autonodes::name::NameNode;
use crate::autotree::ChildNodeParser;
use crate::autotree::NodeAccess;
use crate::autotree::NodeParser;
use crate::autotree::ParseError;
use crate::extra::ExtraChild;
use crate::parser::Range;
use tree_sitter::Node;

#[derive(Debug, Clone)]
pub struct TraitDeclarationNode {
    pub range: Range,
    pub body: DeclarationListNode,
    pub name: NameNode,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for TraitDeclarationNode {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "trait_declaration" {
            return Err(ParseError::new(
                range,
                format!(
                    "Node is of the wrong kind [{}] vs expected [trait_declaration] on pos {}:{}",
                    node.kind(),
                    range.start_point.row + 1,
                    range.start_point.column
                ),
            ));
        }
        let body: DeclarationListNode =
            Into::<Result<_, _>>::into(node.parse_child("body", source))?;
        let name: NameNode = Into::<Result<_, _>>::into(node.parse_child("name", source))?;
        Ok(Self {
            range,
            body,
            name,
            extras: ExtraChild::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| node.kind() == "comment"),
                source,
            )
            .unwrap(),
        })
    }
}

impl TraitDeclarationNode {
    pub fn kind(&self) -> &'static str {
        "trait_declaration"
    }
}

impl NodeAccess for TraitDeclarationNode {
    fn brief_desc(&self) -> String {
        "TraitDeclarationNode".into()
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        AnyNodeRef::TraitDeclaration(self)
    }

    #[allow(clippy::vec_init_then_push)]
    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        child_vec.push(self.body.as_any());
        child_vec.push(self.name.as_any());

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
