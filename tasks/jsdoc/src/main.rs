use std::rc::Rc;

use oxc_allocator::Allocator;
use oxc_ast::{AstKind, SourceType};
use oxc_parser::Parser;
use oxc_semantic::SemanticBuilder;

fn main() {
    let mut args = std::env::args();
    args.next();
    let path = args.next().unwrap();
    let file = std::fs::read_to_string(path.clone()).unwrap();

    let allocator = Allocator::default();
    let source_type = SourceType::from_path(path).expect("incorrect {path:?}");
    let ret = Parser::new(&allocator, &file, source_type).parse();

    let program = allocator.alloc(ret.program);
    let trivias = Rc::new(ret.trivias);

    let ctx = SemanticBuilder::new(&file, source_type, &trivias).build(program);
    let jsdoc = ctx.semantic.jsdoc();

    for node in ctx.semantic.nodes().iter() {
        if let AstKind::Function(_) = node.get().kind() {
            let parsed_jsdoc = jsdoc.get(node);
            println!("{parsed_jsdoc:?}");
        }
    }
}
