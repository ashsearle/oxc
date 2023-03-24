use std::{borrow::Cow, cell::RefCell};

use oxc_ast::{GetSpan, Span};
use rustc_hash::FxHashMap;

use crate::AstNode;

#[derive(Debug, Clone, Copy)]
pub enum JsDocTagKind {
    Deprecated,
}

#[derive(Debug, Clone)]
pub struct JsDocTag<'a> {
    pub kind: JsDocTagKind,
    pub value: Cow<'a, str>,
}

impl<'a> JsDocTag<'a> {
    pub fn is_deprecated(&self) -> bool {
        matches!(self.kind, JsDocTagKind::Deprecated)
    }
}

#[derive(Debug, Clone)]
pub struct JsDocParser<'a> {
    source_text: &'a str,
}

impl<'a> JsDocParser<'a> {
    pub fn new(source_text: &'a str) -> Self {
        Self { source_text }
    }

    fn parse(&mut self, span: Span) -> Vec<JsDocTag<'a>> {
        todo!()
    }
}

#[derive(Default, Debug, Clone)]
pub struct JsDoc<'a> {
    source_text: &'a str,

    // Cached parsed jsdoc, keyed by span start
    parsed: RefCell<FxHashMap<u32, Vec<JsDocTag<'a>>>>,
}

impl<'a> JsDoc<'a> {
    pub fn get(&self, node: &AstNode<'a>) -> Option<Vec<JsDocTag<'a>>> {
        if !node.get().has_jsdoc() {
            return None;
        }

        let span = node.get().kind().span();
        if let Some(parsed) = self.parsed.borrow().get(&span.start) {
            Some(parsed.to_vec())
        } else {
            let mut parser = JsDocParser::new(self.source_text);
            let parsed_jsdoc = parser.parse(span);
            self.parsed.borrow_mut().insert(span.start, parsed_jsdoc.to_vec());
            Some(parsed_jsdoc)
        }
    }
}

#[cfg(test)]
mod test {
    use oxc_ast::Span;

    use super::JsDocParser;

    #[test]
    fn parses_single_line_jsdoc() {
        let source = r#"/** @deprecated */
        function foo() {}"#;

        let mut parser = JsDocParser::new(source);
        let tags = parser.parse(Span::new(20, 37));
        assert_eq!(tags.len(), 1);
    }
}
