use crate::analysis::state::AnalysisState;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::name::NameNode;
use crate::autonodes::qualified_name::QualifiedNameNode;
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
pub enum ClassInterfaceClauseChildren {
    Name(Box<NameNode>),
    QualifiedName(Box<QualifiedNameNode>),
    Comment(Box<CommentNode>),
    TextInterpolation(Box<TextInterpolationNode>),
    Error(Box<ErrorNode>),
}

impl ClassInterfaceClauseChildren {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => {
                ClassInterfaceClauseChildren::Comment(Box::new(CommentNode::parse(node, source)?))
            }
            "text_interpolation" => ClassInterfaceClauseChildren::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            )),
            "ERROR" => {
                ClassInterfaceClauseChildren::Error(Box::new(ErrorNode::parse(node, source)?))
            }
            "name" => ClassInterfaceClauseChildren::Name(Box::new(NameNode::parse(node, source)?)),
            "qualified_name" => ClassInterfaceClauseChildren::QualifiedName(Box::new(
                QualifiedNameNode::parse(node, source)?,
            )),

            _ => {
                return Err(ParseError::new(
                    node.range(),
                    format!("Parse error, unexpected node-type: {}", node.kind()),
                ))
            }
        })
    }

    pub fn parse_opt(node: Node, source: &Vec<u8>) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => {
                ClassInterfaceClauseChildren::Comment(Box::new(CommentNode::parse(node, source)?))
            }
            "text_interpolation" => ClassInterfaceClauseChildren::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            )),
            "ERROR" => {
                ClassInterfaceClauseChildren::Error(Box::new(ErrorNode::parse(node, source)?))
            }
            "name" => ClassInterfaceClauseChildren::Name(Box::new(NameNode::parse(node, source)?)),
            "qualified_name" => ClassInterfaceClauseChildren::QualifiedName(Box::new(
                QualifiedNameNode::parse(node, source)?,
            )),

            _ => return Ok(None),
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
            ClassInterfaceClauseChildren::Comment(x) => x.get_utype(state, emitter),
            ClassInterfaceClauseChildren::TextInterpolation(x) => x.get_utype(state, emitter),
            ClassInterfaceClauseChildren::Error(x) => x.get_utype(state, emitter),
            ClassInterfaceClauseChildren::Name(x) => x.get_utype(state, emitter),
            ClassInterfaceClauseChildren::QualifiedName(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            ClassInterfaceClauseChildren::Comment(x) => x.get_php_value(state, emitter),
            ClassInterfaceClauseChildren::TextInterpolation(x) => x.get_php_value(state, emitter),
            ClassInterfaceClauseChildren::Error(x) => x.get_php_value(state, emitter),
            ClassInterfaceClauseChildren::Name(x) => x.get_php_value(state, emitter),
            ClassInterfaceClauseChildren::QualifiedName(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            ClassInterfaceClauseChildren::Comment(x) => x.read_from(state, emitter),
            ClassInterfaceClauseChildren::TextInterpolation(x) => x.read_from(state, emitter),
            ClassInterfaceClauseChildren::Error(x) => x.read_from(state, emitter),
            ClassInterfaceClauseChildren::Name(x) => x.read_from(state, emitter),
            ClassInterfaceClauseChildren::QualifiedName(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for ClassInterfaceClauseChildren {
    fn brief_desc(&self) -> String {
        match self {
            ClassInterfaceClauseChildren::Comment(x) => {
                format!("ClassInterfaceClauseChildren::comment({})", x.brief_desc())
            }
            ClassInterfaceClauseChildren::TextInterpolation(x) => format!(
                "ClassInterfaceClauseChildren::text_interpolation({})",
                x.brief_desc()
            ),
            ClassInterfaceClauseChildren::Error(x) => {
                format!("ClassInterfaceClauseChildren::ERROR({})", x.brief_desc())
            }
            ClassInterfaceClauseChildren::Name(x) => {
                format!("ClassInterfaceClauseChildren::name({})", x.brief_desc())
            }
            ClassInterfaceClauseChildren::QualifiedName(x) => format!(
                "ClassInterfaceClauseChildren::qualified_name({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            ClassInterfaceClauseChildren::Comment(x) => x.as_any(),
            ClassInterfaceClauseChildren::TextInterpolation(x) => x.as_any(),
            ClassInterfaceClauseChildren::Error(x) => x.as_any(),
            ClassInterfaceClauseChildren::Name(x) => x.as_any(),
            ClassInterfaceClauseChildren::QualifiedName(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            ClassInterfaceClauseChildren::Comment(x) => x.children_any(),
            ClassInterfaceClauseChildren::TextInterpolation(x) => x.children_any(),
            ClassInterfaceClauseChildren::Error(x) => x.children_any(),
            ClassInterfaceClauseChildren::Name(x) => x.children_any(),
            ClassInterfaceClauseChildren::QualifiedName(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            ClassInterfaceClauseChildren::Comment(x) => x.range(),
            ClassInterfaceClauseChildren::TextInterpolation(x) => x.range(),
            ClassInterfaceClauseChildren::Error(x) => x.range(),
            ClassInterfaceClauseChildren::Name(x) => x.range(),
            ClassInterfaceClauseChildren::QualifiedName(x) => x.range(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct ClassInterfaceClauseNode {
    pub range: Range,
    pub children: Vec<Box<ClassInterfaceClauseChildren>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl ClassInterfaceClauseNode {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
        if node.kind() != "class_interface_clause" {
            return Err(ParseError::new(range, format!("Node is of the wrong kind [{}] vs expected [class_interface_clause] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }

        Ok(Self {
            range,
            children: ClassInterfaceClauseChildren::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| node.kind() != "comment"),
                source,
            )?,
            extras: ExtraChild::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| node.kind() == "comment"),
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
        "class_interface_clause"
    }
}

impl NodeAccess for ClassInterfaceClauseNode {
    fn brief_desc(&self) -> String {
        "ClassInterfaceClauseNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::ClassInterfaceClause(self)
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        child_vec.extend(self.children.iter().map(|n| n.as_any()));
        child_vec.extend(self.extras.iter().map(|n| n.as_any()));

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
