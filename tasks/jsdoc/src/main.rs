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

    let semantic = SemanticBuilder::new(source_type).build(program, &trivias).semantic;

    for node in semantic.nodes().iter() {
        let node = node.get();
        if let Some(jsdoc) = node.jsdoc() {
            for tag in jsdoc.tags(&file) {
                if tag.is_deprecated() {
                    println!("{}", tag.value);
                }
            }
        }
    }
}
