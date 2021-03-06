use crate::autonodes::_expression::_ExpressionNode;
use crate::autonodes::_type::_TypeNode;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::attribute_list::AttributeListNode;
use crate::autonodes::reference_modifier::ReferenceModifierNode;
use crate::autonodes::variable_name::VariableNameNode;
use crate::autotree::NodeAccess;
use crate::autotree::ParseError;
use crate::extra::ExtraChild;
use tree_sitter::Node;
use tree_sitter::Range;

#[derive(Debug, Clone)]
pub struct SimpleParameterNode {
    pub range: Range,
    pub attributes: Option<AttributeListNode>,
    pub default_value: Option<_ExpressionNode>,
    pub name: VariableNameNode,
    pub reference_modifier: Option<ReferenceModifierNode>,
    pub type_: Option<_TypeNode>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl SimpleParameterNode {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
        if node.kind() != "simple_parameter" {
            return Err(ParseError::new(
                range,
                format!(
                    "Node is of the wrong kind [{}] vs expected [simple_parameter] on pos {}:{}",
                    node.kind(),
                    range.start_point.row + 1,
                    range.start_point.column
                ),
            ));
        }
        let attributes: Option<AttributeListNode> = node
            .children_by_field_name("attributes", &mut node.walk())
            .map(|chnode1| AttributeListNode::parse(chnode1, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .next();
        let default_value: Option<_ExpressionNode> = node
            .children_by_field_name("default_value", &mut node.walk())
            .map(|chnode1| _ExpressionNode::parse(chnode1, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .next();
        let name: VariableNameNode = node
            .children_by_field_name("name", &mut node.walk())
            .map(|chnode1| VariableNameNode::parse(chnode1, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .next()
            .expect("Field name should exist");
        let reference_modifier: Option<ReferenceModifierNode> = node
            .children_by_field_name("reference_modifier", &mut node.walk())
            .map(|chnode1| ReferenceModifierNode::parse(chnode1, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .next();
        let type_: Option<_TypeNode> = node
            .children_by_field_name("type", &mut node.walk())
            .map(|chnode1| _TypeNode::parse(chnode1, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .next();
        Ok(Self {
            range,
            attributes,
            default_value,
            name,
            reference_modifier,
            type_,
            extras: ExtraChild::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| node.kind() == "comment"),
                source,
            )
            .unwrap(),
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
        "simple_parameter"
    }
}

impl NodeAccess for SimpleParameterNode {
    fn brief_desc(&self) -> String {
        "SimpleParameterNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::SimpleParameter(self)
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        if let Some(x) = &self.attributes {
            child_vec.push(x.as_any());
        }
        if let Some(x) = &self.default_value {
            child_vec.push(x.as_any());
        }
        child_vec.push(self.name.as_any());
        if let Some(x) = &self.reference_modifier {
            child_vec.push(x.as_any());
        }
        if let Some(x) = &self.type_ {
            child_vec.push(x.as_any());
        }

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
