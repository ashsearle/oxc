use std::{borrow::Cow, str::FromStr};

use oxc_ast::Span;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JsDocTagKind {
    Deprecated,
    Param,
}

impl FromStr for JsDocTagKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "deprecated" => Ok(Self::Deprecated),
            "param" => Ok(Self::Param),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsDocTag<'a> {
    pub kind: JsDocTagKind,
    pub description: Cow<'a, str>,
}

impl<'a> JsDocTag<'a> {
    pub fn is_deprecated(&self) -> bool {
        matches!(self.kind, JsDocTagKind::Deprecated)
    }
}

#[derive(Debug, Clone)]
pub struct JsDocParser<'a> {
    source_text: &'a str,
    current: usize,
}

impl<'a> JsDocParser<'a> {
    pub fn new(source_text: &'a str) -> Self {
        Self { source_text, current: 0 }
    }

    // Given the span of a node, find the start of the comment that precedes it.
    fn get_comment_span(&self, span: Span) -> Option<Span> {
        let mut in_comment = false;
        let mut end = span.start as usize;
        let mut index = span.start as usize;

        while index > 0 {
            let Some(c) = self.source_text.chars().nth(index) else { return None };

            match c {
                '*' => {
                    match (
                        self.source_text.chars().nth(index - 1),
                        self.source_text.chars().nth(index - 2),
                    ) {
                        (Some('*'), Some('/')) => {
                            index -= 2;
                            break;
                        }
                        _ => {
                            index -= 1;
                        }
                    }
                }
                '/' => {
                    if !in_comment {
                        in_comment = true;
                        end = index;
                        index -= 1;
                    }
                }
                _ => {
                    index -= 1;
                }
            }
        }

        Some(Span::new(u32::try_from(index).unwrap(), u32::try_from(end).unwrap()))
    }

    fn take_until(&mut self, s: &'a str, predicate: fn(char) -> bool) -> &'a str {
        let start = self.current;
        while let Some(c) = s.chars().nth(self.current) {
            if predicate(c) {
                break;
            }
            self.current += 1;
        }
        &s[start..self.current]
    }

    fn skip_whitespace(&mut self, s: &'a str) {
        while let Some(c) = s.chars().nth(self.current) {
            if c != ' ' {
                break;
            }
            self.current += 1;
        }
    }

    pub fn parse(&mut self, span: Span) -> Vec<JsDocTag<'a>> {
        let Some(comment_span) = self.get_comment_span(span) else { return vec![] };
        self.parse_comment(
            &self.source_text[comment_span.start as usize..=comment_span.end as usize],
        )
    }

    fn parse_comment(&mut self, comment: &'a str) -> Vec<JsDocTag<'a>> {
        let mut tags = vec![];

        while let Some(c) = comment.chars().nth(self.current) {
            match c {
                '@' => {
                    self.current += 1;
                    let Some(tag) = self.parse_tag(comment) else { break };
                    self.current += tag.description.len();
                    tags.push(tag);
                }
                _ => {
                    self.current += 1;
                }
            }
        }

        tags
    }

    fn parse_tag(&mut self, comment: &'a str) -> Option<JsDocTag<'a>> {
        let tag = self.take_until(comment, |c| c == ' ' || c == '\n');
        let Ok(kind) = JsDocTagKind::from_str(tag) else { return None };
        self.skip_whitespace(comment);
        let description = self.take_until(comment, |c| c == '\n' || c == '*');

        return Some(JsDocTag { kind, description: Cow::Borrowed(description) });
    }
}

#[cfg(test)]
mod test {
    use std::borrow::Cow;

    use oxc_ast::Span;

    use super::JsDocParser;
    use crate::jsdoc::parser::{JsDocTag, JsDocTagKind};

    #[test]
    fn gets_comment_span() {
        let source = r#"/**
        * @deprecated
        */
        function foo() {}"#;

        let parser = JsDocParser::new(source);
        let function_span = Span::new(
            u32::try_from(source.find("function").unwrap()).unwrap(),
            u32::try_from(source.find('}').unwrap()).unwrap(),
        );

        let comment_span = Span::new(
            u32::try_from(source.find("/*").unwrap()).unwrap(),
            u32::try_from(source.match_indices('/').last().unwrap().0).unwrap(),
        );
        assert_eq!(parser.get_comment_span(function_span).unwrap(), comment_span);
    }

    #[test]
    fn parses_single_line_jsdoc() {
        let source = r#"/** @deprecated */
        function foo() {}"#;

        let mut parser = JsDocParser::new(source);
        let tags = parser.parse(Span::new(
            u32::try_from(source.find("function").unwrap()).unwrap(),
            u32::try_from(source.find('}').unwrap()).unwrap(),
        ));
        assert_eq!(tags.len(), 1);
        assert_eq!(
            tags,
            vec![JsDocTag { kind: JsDocTagKind::Deprecated, description: Cow::Borrowed("") }]
        );
    }

    #[test]
    fn parses_multi_line_disjoint_jsdoc() {
        let source = r#"/** @deprecated
        */
        function foo() {}"#;

        let mut parser = JsDocParser::new(source);
        let tags = parser.parse(Span::new(
            u32::try_from(source.find("function").unwrap()).unwrap(),
            u32::try_from(source.find('}').unwrap()).unwrap(),
        ));
        assert_eq!(tags.len(), 1);
        assert_eq!(
            tags,
            vec![JsDocTag { kind: JsDocTagKind::Deprecated, description: Cow::Borrowed("") }]
        );
    }

    #[test]
    fn parses_multiline_jsdoc() {
        let source = r#"/**
        * @param a
        * @deprecated
        */
       function foo(a) {}"#;

        let start = source.find("function").unwrap();
        let end = source.find('}').unwrap();

        let mut parser = JsDocParser::new(source);
        let tags =
            parser.parse(Span::new(u32::try_from(start).unwrap(), u32::try_from(end).unwrap()));
        assert_eq!(tags.len(), 2);
        assert_eq!(
            tags,
            vec![
                JsDocTag { kind: JsDocTagKind::Param, description: Cow::Borrowed("a") },
                JsDocTag { kind: JsDocTagKind::Deprecated, description: Cow::Borrowed("") },
            ]
        );
    }

    #[test]
    fn parses_multiline_jsdoc_with_descriptions() {
        let source = r#"/**
        * @param a
        * @deprecated since version 1.0
        */
       function foo(a) {}"#;

        let start = source.find("function").unwrap();
        let end = source.find('}').unwrap();

        let mut parser = JsDocParser::new(source);
        let tags =
            parser.parse(Span::new(u32::try_from(start).unwrap(), u32::try_from(end).unwrap()));
        assert_eq!(tags.len(), 2);
        assert_eq!(
            tags,
            vec![
                JsDocTag { kind: JsDocTagKind::Param, description: Cow::Borrowed("a") },
                JsDocTag {
                    kind: JsDocTagKind::Deprecated,
                    description: Cow::Borrowed("since version 1.0")
                },
            ]
        );
    }
}
