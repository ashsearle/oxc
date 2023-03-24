use std::{borrow::Cow, str::FromStr};

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
    #[must_use]
    pub fn is_deprecated(&self) -> bool {
        matches!(self.kind, JsDocTagKind::Deprecated)
    }
}

#[derive(Debug)]
pub struct JsDocParser<'a> {
    source_text: &'a str,
    current: usize,
}

impl<'a> JsDocParser<'a> {
    pub fn new(source_text: &'a str) -> Self {
        Self { source_text, current: 0 }
    }

    pub fn parse(mut self) -> Vec<JsDocTag<'a>> {
        self.parse_comment(self.source_text)
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

    use super::JsDocParser;
    use crate::jsdoc::parser::{JsDocTag, JsDocTagKind};

    #[test]
    fn parses_single_line_jsdoc() {
        let source = "/** @deprecated */";

        let tags = JsDocParser::new(source).parse();
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
        "#;

        let tags = JsDocParser::new(source).parse();
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
       "#;

        let tags = JsDocParser::new(source).parse();
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
       "#;

        let tags = JsDocParser::new(source).parse();
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
