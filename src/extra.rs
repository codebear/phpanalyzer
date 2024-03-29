use tree_sitter::Node;

use crate::{
    analysis::state::AnalysisState,
    autonodes::{any::AnyNodeRef, comment::CommentNode, text_interpolation::TextInterpolationNode},
    autotree::{NodeAccess, ParseError},
    errornode::ErrorNode,
    issue::IssueEmitter,
    types::union::UnionType,
    value::PHPValue,
};

#[derive(Debug, Clone)]
pub enum ExtraChild {
    Comment(Box<CommentNode>),
    TextInterpolation(Box<TextInterpolationNode>),
    Error(Box<ErrorNode>),
}

impl ExtraChild {
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

    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => ExtraChild::Comment(Box::new(CommentNode::parse(node, source)?)),
            "text_interpolation" => {
                ExtraChild::TextInterpolation(Box::new(TextInterpolationNode::parse(node, source)?))
            }
            "ERROR" => ExtraChild::Error(Box::new(ErrorNode::parse(node, source)?)),
            _ => {
                return Err(ParseError::new(
                    node.range(),
                    format!("Parse error, unexpected node-type: {}", node.kind()),
                ))
            }
        })
    }
    pub fn kind(&self) -> &'static str {
        return match self {
            ExtraChild::Comment(c) => c.kind(),
            ExtraChild::TextInterpolation(t) => t.kind(),
            ExtraChild::Error(e) => e.kind(),
        };
    }
}
impl NodeAccess for ExtraChild {
    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            ExtraChild::Comment(c) => c.as_any(),
            ExtraChild::TextInterpolation(t) => t.as_any(),
            ExtraChild::Error(e) => e.as_any(),
        }
    }

    fn brief_desc(&self) -> String {
        match self {
            ExtraChild::Comment(c) => c.brief_desc(),
            ExtraChild::TextInterpolation(t) => t.brief_desc(),
            ExtraChild::Error(e) => e.brief_desc(),
        }
    }

    fn range(&self) -> tree_sitter::Range {
        match self {
            ExtraChild::Comment(c) => c.range(),
            ExtraChild::TextInterpolation(t) => t.range(),
            ExtraChild::Error(e) => e.range(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            ExtraChild::Comment(c) => c.children_any(),
            ExtraChild::TextInterpolation(t) => t.children_any(),
            ExtraChild::Error(e) => e.children_any(),
        }
    }
}

impl ExtraChild {
    pub fn get_utype(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        match self {
            ExtraChild::Comment(c) => c.get_utype(state, emitter),
            ExtraChild::TextInterpolation(t) => t.get_utype(state, emitter),
            ExtraChild::Error(e) => e.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            ExtraChild::Comment(c) => c.get_php_value(state, emitter),
            ExtraChild::TextInterpolation(t) => t.get_php_value(state, emitter),
            ExtraChild::Error(e) => e.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            ExtraChild::Comment(c) => c.read_from(state, emitter),
            ExtraChild::TextInterpolation(t) => t.read_from(state, emitter),
            ExtraChild::Error(e) => e.read_from(state, emitter),
        }
    }
}
