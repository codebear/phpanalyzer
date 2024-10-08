use crate::analysis::state::AnalysisState;
use crate::autonodes::_expression::_ExpressionNode;
use crate::autonodes::_statement::_StatementNode;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::sequence_expression::SequenceExpressionNode;
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
pub enum ForStatementCondition {
    _Expression(Box<_ExpressionNode>),
    SequenceExpression(Box<SequenceExpressionNode>),
    Extra(ExtraChild),
}

impl NodeParser for ForStatementCondition {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => ForStatementCondition::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => ForStatementCondition::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => ForStatementCondition::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(
                node, source,
            )?))),
            "sequence_expression" => ForStatementCondition::SequenceExpression(Box::new(
                SequenceExpressionNode::parse(node, source)?,
            )),

            _ => {
                if let Some(x) = _ExpressionNode::parse_opt(node, source)?
                    .map(Box::new)
                    .map(ForStatementCondition::_Expression)
                {
                    x
                } else {
                    return Err(ParseError::new(
                        node.range(),
                        format!(
                            "ForStatementCondition: Parse error, unexpected node-type: {}",
                            node.kind()
                        ),
                    ));
                }
            }
        })
    }
}

impl ForStatementCondition {
    pub fn parse_opt(node: Node, source: &[u8]) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => ForStatementCondition::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => ForStatementCondition::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => ForStatementCondition::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(
                node, source,
            )?))),
            "sequence_expression" => ForStatementCondition::SequenceExpression(Box::new(
                SequenceExpressionNode::parse(node, source)?,
            )),

            _ => {
                return Ok(_ExpressionNode::parse_opt(node, source)?
                    .map(Box::new)
                    .map(ForStatementCondition::_Expression))
            }
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            ForStatementCondition::Extra(y) => y.kind(),
            ForStatementCondition::_Expression(y) => y.kind(),
            ForStatementCondition::SequenceExpression(y) => y.kind(),
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
            ForStatementCondition::Extra(x) => x.get_utype(state, emitter),
            ForStatementCondition::_Expression(x) => x.get_utype(state, emitter),
            ForStatementCondition::SequenceExpression(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            ForStatementCondition::Extra(x) => x.get_php_value(state, emitter),
            ForStatementCondition::_Expression(x) => x.get_php_value(state, emitter),
            ForStatementCondition::SequenceExpression(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            ForStatementCondition::Extra(x) => x.read_from(state, emitter),
            ForStatementCondition::_Expression(x) => x.read_from(state, emitter),
            ForStatementCondition::SequenceExpression(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for ForStatementCondition {
    fn brief_desc(&self) -> String {
        match self {
            ForStatementCondition::Extra(x) => {
                format!("ForStatementCondition::extra({})", x.brief_desc())
            }
            ForStatementCondition::_Expression(x) => {
                format!("ForStatementCondition::_expression({})", x.brief_desc())
            }
            ForStatementCondition::SequenceExpression(x) => format!(
                "ForStatementCondition::sequence_expression({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        match self {
            ForStatementCondition::Extra(x) => x.as_any(),
            ForStatementCondition::_Expression(x) => x.as_any(),
            ForStatementCondition::SequenceExpression(x) => x.as_any(),
        }
    }

    fn children_any(&self) -> Vec<AnyNodeRef<'_>> {
        match self {
            ForStatementCondition::Extra(x) => x.children_any(),
            ForStatementCondition::_Expression(x) => x.children_any(),
            ForStatementCondition::SequenceExpression(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            ForStatementCondition::Extra(x) => x.range(),
            ForStatementCondition::_Expression(x) => x.range(),
            ForStatementCondition::SequenceExpression(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ForStatementIncrement {
    _Expression(Box<_ExpressionNode>),
    SequenceExpression(Box<SequenceExpressionNode>),
    Extra(ExtraChild),
}

impl NodeParser for ForStatementIncrement {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => ForStatementIncrement::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => ForStatementIncrement::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => ForStatementIncrement::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(
                node, source,
            )?))),
            "sequence_expression" => ForStatementIncrement::SequenceExpression(Box::new(
                SequenceExpressionNode::parse(node, source)?,
            )),

            _ => {
                if let Some(x) = _ExpressionNode::parse_opt(node, source)?
                    .map(Box::new)
                    .map(ForStatementIncrement::_Expression)
                {
                    x
                } else {
                    return Err(ParseError::new(
                        node.range(),
                        format!(
                            "ForStatementIncrement: Parse error, unexpected node-type: {}",
                            node.kind()
                        ),
                    ));
                }
            }
        })
    }
}

impl ForStatementIncrement {
    pub fn parse_opt(node: Node, source: &[u8]) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => ForStatementIncrement::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => ForStatementIncrement::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => ForStatementIncrement::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(
                node, source,
            )?))),
            "sequence_expression" => ForStatementIncrement::SequenceExpression(Box::new(
                SequenceExpressionNode::parse(node, source)?,
            )),

            _ => {
                return Ok(_ExpressionNode::parse_opt(node, source)?
                    .map(Box::new)
                    .map(ForStatementIncrement::_Expression))
            }
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            ForStatementIncrement::Extra(y) => y.kind(),
            ForStatementIncrement::_Expression(y) => y.kind(),
            ForStatementIncrement::SequenceExpression(y) => y.kind(),
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
            ForStatementIncrement::Extra(x) => x.get_utype(state, emitter),
            ForStatementIncrement::_Expression(x) => x.get_utype(state, emitter),
            ForStatementIncrement::SequenceExpression(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            ForStatementIncrement::Extra(x) => x.get_php_value(state, emitter),
            ForStatementIncrement::_Expression(x) => x.get_php_value(state, emitter),
            ForStatementIncrement::SequenceExpression(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            ForStatementIncrement::Extra(x) => x.read_from(state, emitter),
            ForStatementIncrement::_Expression(x) => x.read_from(state, emitter),
            ForStatementIncrement::SequenceExpression(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for ForStatementIncrement {
    fn brief_desc(&self) -> String {
        match self {
            ForStatementIncrement::Extra(x) => {
                format!("ForStatementIncrement::extra({})", x.brief_desc())
            }
            ForStatementIncrement::_Expression(x) => {
                format!("ForStatementIncrement::_expression({})", x.brief_desc())
            }
            ForStatementIncrement::SequenceExpression(x) => format!(
                "ForStatementIncrement::sequence_expression({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        match self {
            ForStatementIncrement::Extra(x) => x.as_any(),
            ForStatementIncrement::_Expression(x) => x.as_any(),
            ForStatementIncrement::SequenceExpression(x) => x.as_any(),
        }
    }

    fn children_any(&self) -> Vec<AnyNodeRef<'_>> {
        match self {
            ForStatementIncrement::Extra(x) => x.children_any(),
            ForStatementIncrement::_Expression(x) => x.children_any(),
            ForStatementIncrement::SequenceExpression(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            ForStatementIncrement::Extra(x) => x.range(),
            ForStatementIncrement::_Expression(x) => x.range(),
            ForStatementIncrement::SequenceExpression(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ForStatementInitialize {
    _Expression(Box<_ExpressionNode>),
    SequenceExpression(Box<SequenceExpressionNode>),
    Extra(ExtraChild),
}

impl NodeParser for ForStatementInitialize {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => ForStatementInitialize::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => ForStatementInitialize::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => ForStatementInitialize::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "sequence_expression" => ForStatementInitialize::SequenceExpression(Box::new(
                SequenceExpressionNode::parse(node, source)?,
            )),

            _ => {
                if let Some(x) = _ExpressionNode::parse_opt(node, source)?
                    .map(Box::new)
                    .map(ForStatementInitialize::_Expression)
                {
                    x
                } else {
                    return Err(ParseError::new(
                        node.range(),
                        format!(
                            "ForStatementInitialize: Parse error, unexpected node-type: {}",
                            node.kind()
                        ),
                    ));
                }
            }
        })
    }
}

impl ForStatementInitialize {
    pub fn parse_opt(node: Node, source: &[u8]) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => ForStatementInitialize::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => ForStatementInitialize::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => ForStatementInitialize::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "sequence_expression" => ForStatementInitialize::SequenceExpression(Box::new(
                SequenceExpressionNode::parse(node, source)?,
            )),

            _ => {
                return Ok(_ExpressionNode::parse_opt(node, source)?
                    .map(Box::new)
                    .map(ForStatementInitialize::_Expression))
            }
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            ForStatementInitialize::Extra(y) => y.kind(),
            ForStatementInitialize::_Expression(y) => y.kind(),
            ForStatementInitialize::SequenceExpression(y) => y.kind(),
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
            ForStatementInitialize::Extra(x) => x.get_utype(state, emitter),
            ForStatementInitialize::_Expression(x) => x.get_utype(state, emitter),
            ForStatementInitialize::SequenceExpression(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            ForStatementInitialize::Extra(x) => x.get_php_value(state, emitter),
            ForStatementInitialize::_Expression(x) => x.get_php_value(state, emitter),
            ForStatementInitialize::SequenceExpression(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            ForStatementInitialize::Extra(x) => x.read_from(state, emitter),
            ForStatementInitialize::_Expression(x) => x.read_from(state, emitter),
            ForStatementInitialize::SequenceExpression(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for ForStatementInitialize {
    fn brief_desc(&self) -> String {
        match self {
            ForStatementInitialize::Extra(x) => {
                format!("ForStatementInitialize::extra({})", x.brief_desc())
            }
            ForStatementInitialize::_Expression(x) => {
                format!("ForStatementInitialize::_expression({})", x.brief_desc())
            }
            ForStatementInitialize::SequenceExpression(x) => format!(
                "ForStatementInitialize::sequence_expression({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        match self {
            ForStatementInitialize::Extra(x) => x.as_any(),
            ForStatementInitialize::_Expression(x) => x.as_any(),
            ForStatementInitialize::SequenceExpression(x) => x.as_any(),
        }
    }

    fn children_any(&self) -> Vec<AnyNodeRef<'_>> {
        match self {
            ForStatementInitialize::Extra(x) => x.children_any(),
            ForStatementInitialize::_Expression(x) => x.children_any(),
            ForStatementInitialize::SequenceExpression(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            ForStatementInitialize::Extra(x) => x.range(),
            ForStatementInitialize::_Expression(x) => x.range(),
            ForStatementInitialize::SequenceExpression(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ForStatementNode {
    pub range: Range,
    pub condition: Option<Box<ForStatementCondition>>,
    pub increment: Option<Box<ForStatementIncrement>>,
    pub initialize: Option<Box<ForStatementInitialize>>,
    pub children: Vec<Box<_StatementNode>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for ForStatementNode {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "for_statement" {
            return Err(ParseError::new(range, format!("ForStatementNode: Node is of the wrong kind [{}] vs expected [for_statement] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }
        let mut skip_nodes: Vec<usize> = vec![];
        let condition: Option<Box<ForStatementCondition>> = Into::<Result<_, _>>::into(
            node.parse_child("condition", source)
                .mark_skipped_node(&mut skip_nodes),
        )?;
        let increment: Option<Box<ForStatementIncrement>> = Into::<Result<_, _>>::into(
            node.parse_child("increment", source)
                .mark_skipped_node(&mut skip_nodes),
        )?;
        let initialize: Option<Box<ForStatementInitialize>> = Into::<Result<_, _>>::into(
            node.parse_child("initialize", source)
                .mark_skipped_node(&mut skip_nodes),
        )?;
        Ok(Self {
            range,
            condition,
            increment,
            initialize,
            children: _StatementNode::parse_vec(
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

impl ForStatementNode {
    pub fn kind(&self) -> &'static str {
        "for_statement"
    }
}

impl NodeAccess for ForStatementNode {
    fn brief_desc(&self) -> String {
        "ForStatementNode".into()
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        AnyNodeRef::ForStatement(self)
    }

    #[allow(clippy::vec_init_then_push)]
    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        if let Some(x) = &self.condition {
            child_vec.push(x.as_any());
        }
        if let Some(x) = &self.increment {
            child_vec.push(x.as_any());
        }
        if let Some(x) = &self.initialize {
            child_vec.push(x.as_any());
        }
        child_vec.extend(self.children.iter().map(|n| n.as_any()));
        child_vec.extend(self.extras.iter().map(|n| n.as_any()));

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
