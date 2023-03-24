use std::cell::OnceCell;

use oxc_ast::Span;

#[derive(Debug, Clone, Copy)]
pub enum JsDocTagKind {
    Deprecated,
}

#[derive(Debug, Clone, Copy)]
pub struct JsDocTag<'a> {
    pub kind: JsDocTagKind,
    pub value: &'a str,
}

impl<'a> JsDocTag<'a> {
    pub fn is_deprecated(&self) -> bool {
        matches!(self.kind, JsDocTagKind::Deprecated)
    }
}

pub type JsDocParser<'a> = fn(&'a str) -> Vec<JsDocTag<'a>>;

#[derive(Debug, Clone, Copy)]
pub struct JsDoc<'a> {
    parser: OnceCell<JsDocParser<'a>>,
}

impl<'a> JsDoc<'a> {
    pub fn new(span: Span) -> Self {
        let parser: OnceCell<JsDocParser<'a>> = OnceCell::new();
        parser.set(|source_text| Self::parse_comments(source_text, span));

        Self { parser }
    }

    fn parse_comments(source_text: &'a str, span: Span) -> Vec<JsDocTag> {
        let mut tags = Vec::new();
        let comment = &source_text[span.start as usize..span.end as usize];

        for line in comment.lines() {
            let line = line.trim().trim_start_matches("* ");
            if line.starts_with("@deprecated") {
                let value = line.trim_start_matches("@deprecated").trim();
                tags.push(JsDocTag { kind: JsDocTagKind::Deprecated, value });
            }
        }

        tags
    }

    pub fn tags(self, source_text: &str) -> Vec<JsDocTag> {
        self.parser.get().map(|get_tags| get_tags(source_text)).unwrap_or_default()
    }
}
