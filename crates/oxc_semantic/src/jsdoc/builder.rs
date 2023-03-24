use std::rc::Rc;

#[allow(clippy::wildcard_imports)]
use oxc_ast::{ast::*, AstKind, Trivias};

pub struct JsDocBuilder {
    trivias: Rc<Trivias>,
}

impl JsDocBuilder {
    pub fn new(trivias: &Rc<Trivias>) -> Self {
        Self { trivias: Rc::clone(trivias) }
    }

    pub fn enter_kind<'a>(&mut self, kind: AstKind<'a>) {
        match kind {
            AstKind::Function(func) if func.is_declaration() => {
                dbg!(func, &self.trivias);
                let comments = self.trivias.get_leading_comment_span(func.span);
                println!("{:?}", &comments);
            }
            _ => {}
        }
    }
}
