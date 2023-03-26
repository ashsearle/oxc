use std::rc::Rc;

use oxc_ast::{AstKind, GetSpan, Span, Trivias};
use rustc_hash::FxHashMap;

use super::JsDoc;

pub struct JsDocBuilder<'a> {
    source_text: &'a str,

    trivias: Rc<Trivias>,

    /// AstKind Span -> Comment string
    span_map: FxHashMap<Span, &'a str>,
}

impl<'a> JsDocBuilder<'a> {
    pub fn new(source_text: &'a str, trivias: &Rc<Trivias>) -> Self {
        Self { source_text, trivias: Rc::clone(trivias), span_map: FxHashMap::default() }
    }

    pub fn build(self) -> JsDoc<'a> {
        JsDoc::new(self.span_map)
    }

    pub fn retrieved_jsdoc_comment(&mut self, kind: AstKind<'a>) -> bool {
        let comment_text = match kind {
            AstKind::Function(func) if func.is_declaration() => self.find(func.span),
            _ => None,
        };
        if let Some(comment_text) = comment_text {
            self.span_map.insert(kind.span(), comment_text);
        }
        comment_text.is_some()
    }

    #[allow(clippy::cast_possible_truncation)]
    fn find(&self, span: Span) -> Option<&'a str> {
        // Find the line offset above the current span
        // Using `lines()` will not work because it consumes final line ending
        let mut prev_lines = span.leading_text(self.source_text).rsplit('\n');
        let prev_line = prev_lines.next()?;
        let prev_line_end = span.start - prev_line.len() as u32;

        // Find the comment before this line
        let (start, comment) = self.trivias.comments().range(..prev_line_end).next()?;

        let prev_line = prev_lines.next()?;
        let prev_line_start = prev_line_end - prev_line.len() as u32;

        // Check if this comment belongs to this line
        if !(prev_line_start..prev_line_end).contains(&comment.end()) {
            return None;
        }

        let comment_span = Span::new(*start, comment.end());
        let comment_text = comment_span.source_text(self.source_text);

        // It is a jsdoc comment if it starts with `*`
        // /** jsdoc comment */
        // ^^ These two characters are not part of the comment span
        if !comment_text.starts_with('*') {
            return None;
        }

        Some(comment_text)
    }
}
