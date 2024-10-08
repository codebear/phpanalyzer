use crate::analysis::state::AnalysisState;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::catch_clause::CatchClauseNode;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::compound_statement::CompoundStatementNode;
use crate::autonodes::finally_clause::FinallyClauseNode;
use crate::autonodes::text_interpolation::TextInterpolationNode;
use crate::autotree::ChildNodeParser;
use crate::autotree::NodeAccess;
use crate::autotree::NodeParser;
use crate::autotree::ParseError;
use crate::errornode::ErrorNode;
use crate::extra::ExtraChild;
use crate::issue::IssueEmitter;
use crate::parser::Range;
use crate::types::union::PHPType;
use crate::value::PHPValue;
use tree_sitter::Node;

#[derive(Debug, Clone)]
pub enum TryStatementChildren {
    CatchClause(Box<CatchClauseNode>),
    FinallyClause(Box<FinallyClauseNode>),
    Extra(ExtraChild),
}

impl NodeParser for TryStatementChildren {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => TryStatementChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => TryStatementChildren::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => TryStatementChildren::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(
                node, source,
            )?))),
            "catch_clause" => {
                TryStatementChildren::CatchClause(Box::new(CatchClauseNode::parse(node, source)?))
            }
            "finally_clause" => TryStatementChildren::FinallyClause(Box::new(
                FinallyClauseNode::parse(node, source)?,
            )),

            _ => {
                return Err(ParseError::new(
                    node.range(),
                    format!(
                        "TryStatementChildren: Parse error, unexpected node-type: {}",
                        node.kind()
                    ),
                ))
            }
        })
    }
}

impl TryStatementChildren {
    pub fn parse_opt(node: Node, source: &[u8]) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => TryStatementChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => TryStatementChildren::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => TryStatementChildren::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(
                node, source,
            )?))),
            "catch_clause" => {
                TryStatementChildren::CatchClause(Box::new(CatchClauseNode::parse(node, source)?))
            }
            "finally_clause" => TryStatementChildren::FinallyClause(Box::new(
                FinallyClauseNode::parse(node, source)?,
            )),

            _ => return Ok(None),
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            TryStatementChildren::Extra(y) => y.kind(),
            TryStatementChildren::CatchClause(y) => y.kind(),
            TryStatementChildren::FinallyClause(y) => y.kind(),
        }
    }

    pub fn parse_vec<'a, I>(children: I, source: &[u8]) -> Result<Vec<Box<Self>>, ParseError>
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
    ) -> Option<PHPType> {
        match self {
            TryStatementChildren::Extra(x) => x.get_utype(state, emitter),
            TryStatementChildren::CatchClause(x) => x.get_utype(state, emitter),
            TryStatementChildren::FinallyClause(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            TryStatementChildren::Extra(x) => x.get_php_value(state, emitter),
            TryStatementChildren::CatchClause(x) => x.get_php_value(state, emitter),
            TryStatementChildren::FinallyClause(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            TryStatementChildren::Extra(x) => x.read_from(state, emitter),
            TryStatementChildren::CatchClause(x) => x.read_from(state, emitter),
            TryStatementChildren::FinallyClause(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for TryStatementChildren {
    fn brief_desc(&self) -> String {
        match self {
            TryStatementChildren::Extra(x) => {
                format!("TryStatementChildren::extra({})", x.brief_desc())
            }
            TryStatementChildren::CatchClause(x) => {
                format!("TryStatementChildren::catch_clause({})", x.brief_desc())
            }
            TryStatementChildren::FinallyClause(x) => {
                format!("TryStatementChildren::finally_clause({})", x.brief_desc())
            }
        }
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        match self {
            TryStatementChildren::Extra(x) => x.as_any(),
            TryStatementChildren::CatchClause(x) => x.as_any(),
            TryStatementChildren::FinallyClause(x) => x.as_any(),
        }
    }

    fn children_any(&self) -> Vec<AnyNodeRef<'_>> {
        match self {
            TryStatementChildren::Extra(x) => x.children_any(),
            TryStatementChildren::CatchClause(x) => x.children_any(),
            TryStatementChildren::FinallyClause(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            TryStatementChildren::Extra(x) => x.range(),
            TryStatementChildren::CatchClause(x) => x.range(),
            TryStatementChildren::FinallyClause(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TryStatementNode {
    pub range: Range,
    pub body: CompoundStatementNode,
    pub children: Vec<Box<TryStatementChildren>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for TryStatementNode {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "try_statement" {
            return Err(ParseError::new(range, format!("TryStatementNode: Node is of the wrong kind [{}] vs expected [try_statement] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }
        let mut skip_nodes: Vec<usize> = vec![];
        let body: CompoundStatementNode = Into::<Result<_, _>>::into(
            node.parse_child("body", source)
                .mark_skipped_node(&mut skip_nodes),
        )?;
        Ok(Self {
            range,
            body,
            children: TryStatementChildren::parse_vec(
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
}

impl TryStatementNode {
    pub fn kind(&self) -> &'static str {
        "try_statement"
    }
}

impl NodeAccess for TryStatementNode {
    fn brief_desc(&self) -> String {
        "TryStatementNode".into()
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        AnyNodeRef::TryStatement(self)
    }

    #[allow(clippy::vec_init_then_push)]
    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        child_vec.push(self.body.as_any());
        child_vec.extend(self.children.iter().map(|n| n.as_any()));
        child_vec.extend(self.extras.iter().map(|n| n.as_any()));

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
