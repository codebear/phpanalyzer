use crate::analysis::state::AnalysisState;
use crate::autonodes::_type::_TypeNode;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::attribute_list::AttributeListNode;
use crate::autonodes::class_interface_clause::ClassInterfaceClauseNode;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::enum_declaration_list::EnumDeclarationListNode;
use crate::autonodes::name::NameNode;
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
pub enum EnumDeclarationChildren {
    _Type(Box<_TypeNode>),
    ClassInterfaceClause(Box<ClassInterfaceClauseNode>),
    Comment(Box<CommentNode>),
    TextInterpolation(Box<TextInterpolationNode>),
    Error(Box<ErrorNode>),
}

impl EnumDeclarationChildren {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => {
                EnumDeclarationChildren::Comment(Box::new(CommentNode::parse(node, source)?))
            }
            "text_interpolation" => EnumDeclarationChildren::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            )),
            "ERROR" => EnumDeclarationChildren::Error(Box::new(ErrorNode::parse(node, source)?)),
            "class_interface_clause" => EnumDeclarationChildren::ClassInterfaceClause(Box::new(
                ClassInterfaceClauseNode::parse(node, source)?,
            )),

            _ => {
                if let Some(x) = _TypeNode::parse_opt(node, source)?
                    .map(|x| Box::new(x))
                    .map(|y| EnumDeclarationChildren::_Type(y))
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
            "comment" => {
                EnumDeclarationChildren::Comment(Box::new(CommentNode::parse(node, source)?))
            }
            "text_interpolation" => EnumDeclarationChildren::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            )),
            "ERROR" => EnumDeclarationChildren::Error(Box::new(ErrorNode::parse(node, source)?)),
            "class_interface_clause" => EnumDeclarationChildren::ClassInterfaceClause(Box::new(
                ClassInterfaceClauseNode::parse(node, source)?,
            )),

            _ => {
                return Ok(
                    if let Some(x) = _TypeNode::parse_opt(node, source)?
                        .map(|x| Box::new(x))
                        .map(|y| EnumDeclarationChildren::_Type(y))
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
            EnumDeclarationChildren::Comment(x) => x.get_utype(state, emitter),
            EnumDeclarationChildren::TextInterpolation(x) => x.get_utype(state, emitter),
            EnumDeclarationChildren::Error(x) => x.get_utype(state, emitter),
            EnumDeclarationChildren::_Type(x) => x.get_utype(state, emitter),
            EnumDeclarationChildren::ClassInterfaceClause(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            EnumDeclarationChildren::Comment(x) => x.get_php_value(state, emitter),
            EnumDeclarationChildren::TextInterpolation(x) => x.get_php_value(state, emitter),
            EnumDeclarationChildren::Error(x) => x.get_php_value(state, emitter),
            EnumDeclarationChildren::_Type(x) => x.get_php_value(state, emitter),
            EnumDeclarationChildren::ClassInterfaceClause(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            EnumDeclarationChildren::Comment(x) => x.read_from(state, emitter),
            EnumDeclarationChildren::TextInterpolation(x) => x.read_from(state, emitter),
            EnumDeclarationChildren::Error(x) => x.read_from(state, emitter),
            EnumDeclarationChildren::_Type(x) => x.read_from(state, emitter),
            EnumDeclarationChildren::ClassInterfaceClause(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for EnumDeclarationChildren {
    fn brief_desc(&self) -> String {
        match self {
            EnumDeclarationChildren::Comment(x) => {
                format!("EnumDeclarationChildren::comment({})", x.brief_desc())
            }
            EnumDeclarationChildren::TextInterpolation(x) => format!(
                "EnumDeclarationChildren::text_interpolation({})",
                x.brief_desc()
            ),
            EnumDeclarationChildren::Error(x) => {
                format!("EnumDeclarationChildren::ERROR({})", x.brief_desc())
            }
            EnumDeclarationChildren::_Type(x) => {
                format!("EnumDeclarationChildren::_type({})", x.brief_desc())
            }
            EnumDeclarationChildren::ClassInterfaceClause(x) => format!(
                "EnumDeclarationChildren::class_interface_clause({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            EnumDeclarationChildren::Comment(x) => x.as_any(),
            EnumDeclarationChildren::TextInterpolation(x) => x.as_any(),
            EnumDeclarationChildren::Error(x) => x.as_any(),
            EnumDeclarationChildren::_Type(x) => x.as_any(),
            EnumDeclarationChildren::ClassInterfaceClause(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            EnumDeclarationChildren::Comment(x) => x.children_any(),
            EnumDeclarationChildren::TextInterpolation(x) => x.children_any(),
            EnumDeclarationChildren::Error(x) => x.children_any(),
            EnumDeclarationChildren::_Type(x) => x.children_any(),
            EnumDeclarationChildren::ClassInterfaceClause(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            EnumDeclarationChildren::Comment(x) => x.range(),
            EnumDeclarationChildren::TextInterpolation(x) => x.range(),
            EnumDeclarationChildren::Error(x) => x.range(),
            EnumDeclarationChildren::_Type(x) => x.range(),
            EnumDeclarationChildren::ClassInterfaceClause(x) => x.range(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct EnumDeclarationNode {
    pub range: Range,
    pub attributes: Option<AttributeListNode>,
    pub body: EnumDeclarationListNode,
    pub name: NameNode,
    pub children: Vec<Box<EnumDeclarationChildren>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl EnumDeclarationNode {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
        if node.kind() != "enum_declaration" {
            return Err(ParseError::new(
                range,
                format!(
                    "Node is of the wrong kind [{}] vs expected [enum_declaration] on pos {}:{}",
                    node.kind(),
                    range.start_point.row + 1,
                    range.start_point.column
                ),
            ));
        }
        let mut skip_nodes: Vec<usize> = vec![];
        let attributes: Option<AttributeListNode> = node
            .children_by_field_name("attributes", &mut node.walk())
            .map(|chnode| {
                skip_nodes.push(chnode.id());
                chnode
            })
            .map(|chnode1| AttributeListNode::parse(chnode1, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .next();
        let body: EnumDeclarationListNode = node
            .children_by_field_name("body", &mut node.walk())
            .map(|chnode| {
                skip_nodes.push(chnode.id());
                chnode
            })
            .map(|chnode1| EnumDeclarationListNode::parse(chnode1, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .next()
            .expect("Field body should exist");
        let name: NameNode = node
            .children_by_field_name("name", &mut node.walk())
            .map(|chnode| {
                skip_nodes.push(chnode.id());
                chnode
            })
            .map(|chnode1| NameNode::parse(chnode1, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .next()
            .expect("Field name should exist");
        Ok(Self {
            range,
            attributes,
            body,
            name,
            children: EnumDeclarationChildren::parse_vec(
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
        "enum_declaration"
    }
}

impl NodeAccess for EnumDeclarationNode {
    fn brief_desc(&self) -> String {
        "EnumDeclarationNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::EnumDeclaration(self)
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        if let Some(x) = &self.attributes {
            child_vec.push(x.as_any());
        }
        child_vec.push(self.body.as_any());
        child_vec.push(self.name.as_any());
        child_vec.extend(self.children.iter().map(|n| n.as_any()));
        child_vec.extend(self.extras.iter().map(|n| n.as_any()));

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
