use std::cell::RefCell;

use oxc_ast::GetSpan;
use rustc_hash::FxHashMap;

use self::parser::{JsDocParser, JsDocTag};
use crate::AstNode;

mod parser;

#[derive(Debug, Clone)]
pub struct JsDoc<'a> {
    source_text: &'a str,

    // Cached parsed jsdoc, keyed by span start
    parsed: RefCell<FxHashMap<u32, Vec<JsDocTag<'a>>>>,
}

impl<'a> JsDoc<'a> {
    pub fn new(source_text: &'a str) -> Self {
        Self { source_text, parsed: RefCell::new(FxHashMap::default()) }
    }

    pub fn get(&self, node: &AstNode<'a>) -> Option<Vec<JsDocTag<'a>>> {
        if !node.get().has_jsdoc() {
            return None;
        }

        let mut parsed = self.parsed.borrow_mut();
        let span = node.get().kind().span();

        #[allow(clippy::option_if_let_else)]
        if let Some(parsed) = parsed.get(&span.start) {
            Some(parsed.clone())
        } else {
            let mut parser = JsDocParser::new(self.source_text);
            let parsed_jsdoc = parser.parse(span);
            parsed.insert(span.start, parsed_jsdoc.clone());
            Some(parsed_jsdoc)
        }
    }
}
