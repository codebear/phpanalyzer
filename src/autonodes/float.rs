use crate::autonodes::any::AnyNodeRef;
use crate::autotree::NodeAccess;
use crate::autotree::NodeParser;
use crate::autotree::ParseError;
use crate::parser::Range;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::os::unix::ffi::OsStrExt;
use tree_sitter::Node;

#[derive(Debug, Clone)]
pub struct FloatNode {
    pub range: Range,
    pub raw: Vec<u8>,
}

impl NodeParser for FloatNode {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "float" {
            return Err(ParseError::new(
                range,
                format!(
                    "FloatNode: Node is of the wrong kind [{}] vs expected [float] on pos {}:{}",
                    node.kind(),
                    range.start_point.row + 1,
                    range.start_point.column
                ),
            ));
        }

        Ok(Self {
            range,
            raw: source[range.start_byte..range.end_byte].to_vec(),
        })
    }
}

impl FloatNode {
    pub fn kind(&self) -> &'static str {
        "float"
    }

    pub fn get_raw(&self) -> OsString {
        OsStr::from_bytes(&self.raw).to_os_string()
    }
}

impl NodeAccess for FloatNode {
    fn brief_desc(&self) -> String {
        "FloatNode".into()
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        AnyNodeRef::Float(self)
    }

    fn children_any(&self) -> Vec<AnyNodeRef<'_>> {
        vec![]
    }

    fn range(&self) -> Range {
        self.range
    }
}
