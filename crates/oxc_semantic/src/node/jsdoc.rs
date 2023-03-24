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

#[derive(Debug, Clone, Copy)]
pub struct JsDoc {
    pub span: Span,
}

impl JsDoc {
    pub fn new(span: Span) -> Self {
        Self { span }
    }

    pub fn tags(self, source_text: &str) -> Vec<JsDocTag> {
        let mut tags = Vec::new();
        let comment = &source_text[self.span.start as usize..self.span.end as usize];

        for line in comment.lines() {
            let line = line.trim().trim_start_matches("* ");
            if line.starts_with("@deprecated") {
                let value = line.trim_start_matches("@deprecated").trim();
                tags.push(JsDocTag { kind: JsDocTagKind::Deprecated, value });
            }
        }

        tags
    }
}
