use oxc_allocator::{Allocator, Vec};
#[allow(clippy::wildcard_imports)]
use oxc_ast::{ast::*, visit_mut::VisitMut, AstBuilder, Span};

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Copy)]
pub struct CompressOptions {
    /// Various optimizations for boolean context, for example `!!a ? b : c` → `a ? b : c`
    /// Default true
    pub booleans: bool,

    /// Remove `debugger;` statements
    /// Default true
    pub drop_debugger: bool,

    /// Join consecutive var statements
    /// Default true
    pub join_vars: bool,

    /// Optimizations for do, while and for loops when we can statically determine the condition
    /// Default: true
    pub loops: bool,

    /// Transforms `typeof foo == "undefined" into `foo === void 0`
    /// Default true
    pub typeofs: bool,
}

impl Default for CompressOptions {
    fn default() -> Self {
        Self { booleans: true, drop_debugger: true, join_vars: true, loops: true, typeofs: true }
    }
}

pub struct Compressor<'a> {
    ast: AstBuilder<'a>,
    options: CompressOptions,
}

const SPAN: Span = Span::new(0, 0);

impl<'a> Compressor<'a> {
    pub fn new(allocator: &'a Allocator, options: CompressOptions) -> Self {
        Self { ast: AstBuilder::new(allocator), options }
    }

    pub fn build<'b>(mut self, program: &'b mut Program<'a>) {
        self.visit_program(program);
    }

    /* Utilities */

    fn create_void_0(&self) -> Expression<'a> {
        let num = self.ast.literal_number_expression(NumberLiteral::new(
            SPAN,
            0.0,
            "0",
            NumberBase::Decimal,
        ));
        self.ast.unary_expression(SPAN, UnaryOperator::Void, true, num)
    }

    /* Statements */

    #[allow(clippy::unused_self)]
    fn drop_empty<'b>(&mut self, stmt: &'b Statement<'a>) -> bool {
        matches!(stmt, Statement::EmptyStatement(_))
    }

    /// Drop `drop_debugger` statement.
    /// Enabled by `compress.drop_debugger`
    fn drop_debugger<'b>(&mut self, stmt: &'b Statement<'a>) -> bool {
        matches!(stmt, Statement::DebuggerStatement(_)) && self.options.drop_debugger
    }

    /// Join consecutive var statements
    fn join_vars<'b>(&mut self, stmts: &'b mut Vec<'a, Statement<'a>>) {
        // Collect all the consecutive ranges that contain joinable vars.
        // This is required because Rust prevents in-place vec mutation.
        let mut ranges = vec![];
        let mut range = 0..0;
        let mut i = 1usize;
        let mut capacity = 0usize;
        for window in stmts.windows(2) {
            let [prev, cur] = window else { unreachable!() };
            if let Statement::Declaration(Declaration::VariableDeclaration(cur_decl)) = cur
                && let Statement::Declaration(Declaration::VariableDeclaration(prev_decl)) = prev
                && cur_decl.kind == prev_decl.kind {
                if i - 1 != range.end  {
                    range.start = i - 1;
                }
                range.end = i + 1;
            }
            if (range.end != i || i == stmts.len() - 1) && range.start < range.end {
                capacity += range.end - range.start - 1;
                ranges.push(range.clone());
                range = 0..0;
            }
            i += 1;
        }

        if ranges.is_empty() {
            return;
        }

        // Reconstruct the stmts array by joining consecutive ranges
        let mut new_stmts = self.ast.new_vec_with_capacity(stmts.len() - capacity);
        for (i, stmt) in stmts.drain(..).enumerate() {
            if i > 0
                && ranges.iter().any(|range| range.contains(&i))
                && let Statement::Declaration(Declaration::VariableDeclaration(prev_decl)) = new_stmts.last_mut().unwrap()
                && let Statement::Declaration(Declaration::VariableDeclaration(mut cur_decl)) = stmt {
                prev_decl.declarations.append(&mut cur_decl.declarations);
            } else {
                new_stmts.push(stmt);
            }
        }
        *stmts = new_stmts;
    }

    /// Transforms `while(expr)` to `for(;expr;)`
    fn compress_while<'b>(&mut self, stmt: &'b mut Statement<'a>) {
        if let Statement::WhileStatement(while_stmt) = stmt
            && self.options.loops {
            let dummy_test = self.ast.this_expression(SPAN);
            let test = std::mem::replace(&mut while_stmt.test, dummy_test);
            let dummy_body = self.ast.empty_statement(SPAN);
            let body = std::mem::replace(&mut while_stmt.body, dummy_body);
            *stmt = self.ast.for_statement(SPAN, None, Some(test), None, body);
        }
    }

    /* Expressions */

    /// Transforms `undefined` => `void 0`
    fn compress_undefined<'b>(&mut self, expr: &'b mut Expression<'a>) -> bool {
        if expr.is_undefined() {
            *expr = self.create_void_0();
            return true;
        }
        false
    }

    /// Transforms boolean expression `true` => `!0` `false` => `!1`
    /// Enabled by `compress.booleans`
    fn compress_boolean<'b>(&mut self, expr: &'b mut Expression<'a>) -> bool {
        if let Expression::BooleanLiteral(lit) = expr
        && self.options.booleans {
            let num = self.ast.literal_number_expression(NumberLiteral::new(
                SPAN,
                if lit.value { 0.0 } else { 1.0 },
                if lit.value { "0" } else { "1" },
                NumberBase::Decimal,
            ));
            *expr = self.ast.unary_expression(SPAN, UnaryOperator::LogicalNot, true, num);
            return true;
        }
        false
    }

    /// Transforms `typeof foo == "undefined"` into `foo === void 0`
    /// Enabled by `compress.typeofs`
    fn compress_typeof_undefined<'b>(&mut self, expr: &'b mut BinaryExpression<'a>) -> bool {
        if expr.operator.is_equality()
            && self.options.typeofs
            && let Expression::UnaryExpression(unary_expr) = &expr.left
            && unary_expr.operator == UnaryOperator::Typeof
            && let Expression::Identifier(ident) = &unary_expr.argument
            && let Expression::StringLiteral(s) = &expr.right
            && s.value == "undefined" {
            let left = self.ast.identifier_expression((*ident).clone());
            let right = self.create_void_0();
            let operator = BinaryOperator::StrictEquality;
            *expr = BinaryExpression {span: SPAN, left, operator, right};
            return true
        }
        false
    }
}

impl<'a, 'b> VisitMut<'a, 'b> for Compressor<'a> {
    fn visit_statements(&mut self, stmts: &'b mut Vec<'a, Statement<'a>>) {
        stmts.retain(|stmt| !self.drop_empty(stmt) && !self.drop_debugger(stmt));

        self.join_vars(stmts);

        for stmt in stmts.iter_mut() {
            self.visit_statement(stmt);
        }
    }

    fn visit_statement(&mut self, stmt: &'b mut Statement<'a>) {
        self.compress_while(stmt);
        self.visit_statement_match(stmt);
    }

    fn visit_expression(&mut self, expr: &'b mut Expression<'a>) {
        if self.compress_undefined(expr) {
            return;
        }
        if self.compress_boolean(expr) {
            return;
        }
        self.visit_expression_match(expr);
    }

    fn visit_binary_expression(&mut self, expr: &'b mut BinaryExpression<'a>) {
        if self.compress_typeof_undefined(expr) {
            return;
        }
        self.visit_expression(&mut expr.left);
        self.visit_expression(&mut expr.right);
    }
}
