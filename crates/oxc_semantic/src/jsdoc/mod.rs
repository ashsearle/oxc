mod builder;

use std::{cell::RefCell, rc::Rc};

pub use builder::JsDocBuilder;
use oxc_ast::{GetSpan, Span};
use rustc_hash::FxHashMap;

use self::parser::JsDocParser;
pub use self::parser::JsDocTag;
use crate::AstNode;

mod parser;

#[derive(Debug)]
pub struct JsDoc<'a> {
    /// AstKind Span -> JsDoc Span
    span_map: FxHashMap<Span, &'a str>,

    /// Cached parsed jsdoc
    parsed: RefCell<FxHashMap<Span, Rc<[JsDocTag<'a>]>>>,
}

impl<'a> JsDoc<'a> {
    #[must_use]
    pub fn new(span_map: FxHashMap<Span, &'a str>) -> Self {
        Self { span_map, parsed: RefCell::new(FxHashMap::default()) }
    }

    pub fn get<'b>(&'b self, node: &AstNode<'a>) -> Option<Rc<[JsDocTag<'a>]>> {
        if !node.get().has_jsdoc() {
            return None;
        }

        let span = node.get().kind().span();

        let jsdoc_comment = self.span_map.get(&span)?;

        Some(Rc::clone(
            self.parsed
                .borrow_mut()
                .entry(span)
                .or_insert_with(|| Rc::from(JsDocParser::new(jsdoc_comment).parse())),
        ))
    }
}
