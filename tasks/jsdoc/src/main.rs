use std::rc::Rc;

use oxc_allocator::Allocator;
use oxc_ast::SourceType;
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

    let ctx = SemanticBuilder::new(source_type).build(program, &trivias);

    for node in ctx.semantic.nodes().iter() {
        let parsed_jsdoc = ctx.semantic.jsdoc().get(node);
        if let Some(parsed_jsdoc) = parsed_jsdoc {
            println!("{parsed_jsdoc:?}");
        }
    }
}
