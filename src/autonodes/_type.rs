use crate::analysis::state::AnalysisState;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::disjunctive_normal_form_type::DisjunctiveNormalFormTypeNode;
use crate::autonodes::intersection_type::IntersectionTypeNode;
use crate::autonodes::named_type::NamedTypeNode;
use crate::autonodes::optional_type::OptionalTypeNode;
use crate::autonodes::primitive_type::PrimitiveTypeNode;
use crate::autonodes::text_interpolation::TextInterpolationNode;
use crate::autonodes::union_type::UnionTypeNode;
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
pub enum _TypeNode {
    DisjunctiveNormalFormType(Box<DisjunctiveNormalFormTypeNode>),
    IntersectionType(Box<IntersectionTypeNode>),
    NamedType(Box<NamedTypeNode>),
    OptionalType(Box<OptionalTypeNode>),
    PrimitiveType(Box<PrimitiveTypeNode>),
    UnionType(Box<UnionTypeNode>),
    Extra(ExtraChild),
}

impl NodeParser for _TypeNode {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => _TypeNode::Extra(ExtraChild::Comment(Box::new(CommentNode::parse(
                node, source,
            )?))),
            "text_interpolation" => _TypeNode::Extra(ExtraChild::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            ))),
            "ERROR" => {
                _TypeNode::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(node, source)?)))
            }
            "disjunctive_normal_form_type" => _TypeNode::DisjunctiveNormalFormType(Box::new(
                DisjunctiveNormalFormTypeNode::parse(node, source)?,
            )),
            "intersection_type" => {
                _TypeNode::IntersectionType(Box::new(IntersectionTypeNode::parse(node, source)?))
            }
            "named_type" => _TypeNode::NamedType(Box::new(NamedTypeNode::parse(node, source)?)),
            "optional_type" => {
                _TypeNode::OptionalType(Box::new(OptionalTypeNode::parse(node, source)?))
            }
            "primitive_type" => {
                _TypeNode::PrimitiveType(Box::new(PrimitiveTypeNode::parse(node, source)?))
            }
            "union_type" => _TypeNode::UnionType(Box::new(UnionTypeNode::parse(node, source)?)),

            _ => {
                return Err(ParseError::new(
                    node.range(),
                    format!(
                        "_TypeNode: Parse error, unexpected node-type: {}",
                        node.kind()
                    ),
                ))
            }
        })
    }
}

impl _TypeNode {
    pub fn parse_opt(node: Node, source: &[u8]) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => _TypeNode::Extra(ExtraChild::Comment(Box::new(CommentNode::parse(
                node, source,
            )?))),
            "text_interpolation" => _TypeNode::Extra(ExtraChild::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            ))),
            "ERROR" => {
                _TypeNode::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(node, source)?)))
            }
            "disjunctive_normal_form_type" => _TypeNode::DisjunctiveNormalFormType(Box::new(
                DisjunctiveNormalFormTypeNode::parse(node, source)?,
            )),
            "intersection_type" => {
                _TypeNode::IntersectionType(Box::new(IntersectionTypeNode::parse(node, source)?))
            }
            "named_type" => _TypeNode::NamedType(Box::new(NamedTypeNode::parse(node, source)?)),
            "optional_type" => {
                _TypeNode::OptionalType(Box::new(OptionalTypeNode::parse(node, source)?))
            }
            "primitive_type" => {
                _TypeNode::PrimitiveType(Box::new(PrimitiveTypeNode::parse(node, source)?))
            }
            "union_type" => _TypeNode::UnionType(Box::new(UnionTypeNode::parse(node, source)?)),

            _ => return Ok(None),
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            _TypeNode::Extra(y) => y.kind(),
            _TypeNode::DisjunctiveNormalFormType(y) => y.kind(),
            _TypeNode::IntersectionType(y) => y.kind(),
            _TypeNode::NamedType(y) => y.kind(),
            _TypeNode::OptionalType(y) => y.kind(),
            _TypeNode::PrimitiveType(y) => y.kind(),
            _TypeNode::UnionType(y) => y.kind(),
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
            _TypeNode::Extra(x) => x.get_utype(state, emitter),
            _TypeNode::DisjunctiveNormalFormType(x) => x.get_utype(state, emitter),
            _TypeNode::IntersectionType(x) => x.get_utype(state, emitter),
            _TypeNode::NamedType(x) => x.get_utype(state, emitter),
            _TypeNode::OptionalType(x) => x.get_utype(state, emitter),
            _TypeNode::PrimitiveType(x) => x.get_utype(state, emitter),
            _TypeNode::UnionType(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            _TypeNode::Extra(x) => x.get_php_value(state, emitter),
            _TypeNode::DisjunctiveNormalFormType(x) => x.get_php_value(state, emitter),
            _TypeNode::IntersectionType(x) => x.get_php_value(state, emitter),
            _TypeNode::NamedType(x) => x.get_php_value(state, emitter),
            _TypeNode::OptionalType(x) => x.get_php_value(state, emitter),
            _TypeNode::PrimitiveType(x) => x.get_php_value(state, emitter),
            _TypeNode::UnionType(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            _TypeNode::Extra(x) => x.read_from(state, emitter),
            _TypeNode::DisjunctiveNormalFormType(x) => x.read_from(state, emitter),
            _TypeNode::IntersectionType(x) => x.read_from(state, emitter),
            _TypeNode::NamedType(x) => x.read_from(state, emitter),
            _TypeNode::OptionalType(x) => x.read_from(state, emitter),
            _TypeNode::PrimitiveType(x) => x.read_from(state, emitter),
            _TypeNode::UnionType(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for _TypeNode {
    fn brief_desc(&self) -> String {
        match self {
            _TypeNode::Extra(x) => format!("_TypeNode::extra({})", x.brief_desc()),
            _TypeNode::DisjunctiveNormalFormType(x) => format!(
                "_TypeNode::disjunctive_normal_form_type({})",
                x.brief_desc()
            ),
            _TypeNode::IntersectionType(x) => {
                format!("_TypeNode::intersection_type({})", x.brief_desc())
            }
            _TypeNode::NamedType(x) => format!("_TypeNode::named_type({})", x.brief_desc()),
            _TypeNode::OptionalType(x) => format!("_TypeNode::optional_type({})", x.brief_desc()),
            _TypeNode::PrimitiveType(x) => format!("_TypeNode::primitive_type({})", x.brief_desc()),
            _TypeNode::UnionType(x) => format!("_TypeNode::union_type({})", x.brief_desc()),
        }
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        match self {
            _TypeNode::Extra(x) => x.as_any(),
            _TypeNode::DisjunctiveNormalFormType(x) => x.as_any(),
            _TypeNode::IntersectionType(x) => x.as_any(),
            _TypeNode::NamedType(x) => x.as_any(),
            _TypeNode::OptionalType(x) => x.as_any(),
            _TypeNode::PrimitiveType(x) => x.as_any(),
            _TypeNode::UnionType(x) => x.as_any(),
        }
    }

    fn children_any(&self) -> Vec<AnyNodeRef<'_>> {
        match self {
            _TypeNode::Extra(x) => x.children_any(),
            _TypeNode::DisjunctiveNormalFormType(x) => x.children_any(),
            _TypeNode::IntersectionType(x) => x.children_any(),
            _TypeNode::NamedType(x) => x.children_any(),
            _TypeNode::OptionalType(x) => x.children_any(),
            _TypeNode::PrimitiveType(x) => x.children_any(),
            _TypeNode::UnionType(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            _TypeNode::Extra(x) => x.range(),
            _TypeNode::DisjunctiveNormalFormType(x) => x.range(),
            _TypeNode::IntersectionType(x) => x.range(),
            _TypeNode::NamedType(x) => x.range(),
            _TypeNode::OptionalType(x) => x.range(),
            _TypeNode::PrimitiveType(x) => x.range(),
            _TypeNode::UnionType(x) => x.range(),
        }
    }
}
