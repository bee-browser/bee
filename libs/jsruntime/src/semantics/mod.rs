mod scope;

use indexmap::IndexMap;

use jsparser::syntax::AssignmentOperator;
use jsparser::syntax::BinaryOperator;
use jsparser::syntax::LoopFlags;
use jsparser::syntax::Node;
use jsparser::syntax::NodeHandler;
use jsparser::syntax::UnaryOperator;
use jsparser::syntax::UpdateOperator;
use jsparser::Error;
use jsparser::Parser;
use jsparser::Processor;
use jsparser::Symbol;
use jsparser::SymbolRegistry;

use super::logger;
use super::FunctionId;
use super::FunctionRegistry;
use super::Runtime;
use super::RuntimePref;
use scope::ScopeTreeBuilder;

pub use scope::BindingRef;
pub use scope::ScopeRef;
pub use scope::ScopeTree;

impl<X> Runtime<X> {
    /// Parses a given source text as a script.
    pub fn parse_script(&mut self, source: &str) -> Result<Program, Error> {
        logger::debug!(event = "parse", source_kind = "script");
        let mut analyzer = Analyzer::new(
            &self.pref,
            &mut self.symbol_registry,
            &mut self.function_registry,
        );
        analyzer.use_global_bindings();
        let processor = Processor::new(analyzer, false);
        Parser::for_script(source, processor).parse()
    }
}

/// A type representing a JavaScript program after the semantic analysis.
pub struct Program {
    pub functions: Vec<FunctionRecipe>,
    pub scope_tree: ScopeTree,
}

impl Program {
    pub fn print_functions(&self, indent: &str) {
        for func in self.functions.iter() {
            func.print(indent);
        }
    }

    pub fn print_scope_tree(&self, indent: &str) {
        self.scope_tree.print(indent);
    }
}

/// A type representing a JavaScript function after the semantic analysis.
pub struct FunctionRecipe {
    /// TODO: remove?
    pub symbol: Symbol,

    /// The function ID of the function.
    pub id: FunctionId,

    /// A list of [`CompileCommand`]s generated from the function definition.
    pub commands: Vec<CompileCommand>,

    /// A list of free variables in the function.
    pub captures: Vec<Capture>,
}

impl FunctionRecipe {
    pub fn print(&self, indent: &str) {
        println!("{indent}function: {:?}", self.id);
        if !self.commands.is_empty() {
            println!("{indent} commands:");
            for command in self.commands.iter() {
                println!("{indent}  {command:?}");
            }
        }
        if !self.captures.is_empty() {
            println!("{indent} captures:");
            for capture in self.captures.iter() {
                println!("{indent}  {capture:?}");
            }
        }
    }
}

/// Represents a free variable of a function.
#[derive(Debug)]
pub struct Capture {
    /// The symbol of the captured variable defined outside the function.
    pub symbol: Symbol,

    pub target: Locator,
}

/// A semantic analyzer.
///
/// A semantic analyzer analyzes semantics of a JavaScript program.
struct Analyzer<'r> {
    runtime_pref: &'r RuntimePref,

    /// A mutable reference to a symbol registry.
    symbol_registry: &'r mut SymbolRegistry,

    /// A mutable reference to a function registry.
    function_registry: &'r mut FunctionRegistry,

    /// A stack to keep the analysis data for outer JavaScript functions when analyzing nested
    /// JavaScript functions.
    context_stack: Vec<FunctionContext>,

    /// A list of [`FunctionRecipe`]s.
    functions: Vec<FunctionRecipe>,

    /// A scope tree builder used for building the scope tree of the JavaScript program.
    scope_tree_builder: ScopeTreeBuilder,

    use_global_bindings: bool,
}

impl<'r> Analyzer<'r> {
    /// Creates a semantic analyzer.
    pub fn new(
        runtime_pref: &'r RuntimePref,
        symbol_registry: &'r mut SymbolRegistry,
        function_registry: &'r mut FunctionRegistry,
    ) -> Self {
        let id = function_registry.create_native_function();
        Self {
            runtime_pref,
            symbol_registry,
            function_registry,
            context_stack: vec![FunctionContext {
                // `commands[0]` will be replaced with `AllocateLocals` in `accept()` if the
                // function has local variables.
                commands: vec![CompileCommand::Nop],
                in_body: true,
                ..Default::default()
            }],
            functions: vec![FunctionRecipe {
                symbol: Symbol::NONE,
                id,
                commands: vec![],
                captures: vec![],
            }],
            scope_tree_builder: Default::default(),
            use_global_bindings: false,
        }
    }

    pub fn use_global_bindings(&mut self) {
        self.use_global_bindings = true;
    }

    /// Handles an AST node coming from a parser.
    fn handle_node(&mut self, node: Node<'_>) {
        logger::debug!(event = "handle_node", ?node);
        match node {
            Node::Null => self.handle_null(),
            Node::Boolean(value) => self.handle_boolean(value),
            Node::Number(value, ..) => self.handle_number(value),
            Node::String(value, ..) => self.handle_string(value),
            Node::IdentifierReference(symbol) => self.handle_identifier_reference(symbol),
            Node::BindingIdentifier(symbol) => self.handle_binding_identifier(symbol),
            Node::ArgumentListHead(empty, spread) => self.handle_argument_list_head(empty, spread),
            Node::ArgumentListItem(spread) => self.handle_argument_list_item(spread),
            Node::Arguments => (), // nop
            Node::CallExpression => self.handle_call_expression(),
            Node::UpdateExpression(op) => self.handle_operator(op.into()),
            Node::UnaryExpression(op) => self.handle_operator(op.into()),
            Node::BinaryExpression(op) => self.handle_binary_expression(op),
            Node::LogicalExpression(_op) => self.handle_conditional_expression(),
            Node::ConditionalExpression => self.handle_conditional_expression(),
            Node::AssignmentExpression(AssignmentOperator::Assignment) => {
                self.handle_operator(CompileCommand::Assignment)
            }
            Node::AssignmentExpression(AssignmentOperator::LogicalAndAssignment) => {
                self.handle_conditional_assignment()
            }
            Node::AssignmentExpression(AssignmentOperator::LogicalOrAssignment) => {
                self.handle_conditional_assignment()
            }
            Node::AssignmentExpression(AssignmentOperator::NullishCoalescingAssignment) => {
                self.handle_conditional_assignment()
            }
            Node::AssignmentExpression(op) => self.handle_shorthand_assignment_expression(op),
            Node::SequenceExpression => self.handle_sequence_expression(),
            Node::BlockStatement => (), // nop
            Node::LexicalBinding(init) => self.handle_lexical_binding(init),
            Node::LetDeclaration(n) => self.handle_let_declaration(n),
            Node::ConstDeclaration(n) => self.handle_const_declaration(n),
            Node::BindingElement(init) => self.handle_binding_element(init),
            Node::EmptyStatement => (), // nop
            Node::ExpressionStatement => self.handle_expression_statement(),
            Node::IfElseStatement => self.handle_if_else_statement(),
            Node::IfStatement => self.handle_if_statement(),
            Node::DoWhileStatement => self.handle_do_while_statement(),
            Node::WhileStatement => self.handle_while_statement(),
            Node::ForStatement(flags) => self.handle_for_statement(flags),
            Node::ContinueStatement(symbol) => self.handle_continue_statement(symbol),
            Node::BreakStatement(symbol) => self.handle_break_statement(symbol),
            Node::ReturnStatement(n) => self.handle_return_statement(n),
            Node::SwitchStatement => self.handle_switch_statement(),
            Node::CaseBlock => self.handle_case_block(),
            Node::CaseSelector => self.handle_case_selector(),
            Node::CaseClause(has_statement) => self.handle_case_clause(has_statement),
            Node::DefaultSelector => self.handle_default_selector(),
            Node::DefaultClause(has_statement) => self.handle_default_clause(has_statement),
            Node::LabelledStatement(symbol, is_iteration_statement) => {
                self.handle_labelled_statement(symbol, is_iteration_statement)
            }
            Node::Label(symbol) => self.handle_label(symbol),
            Node::ThrowStatement => self.handle_throw_statement(),
            Node::TryStatement => self.handle_try_statement(),
            Node::CatchClause(has_parameter) => self.handle_catch_clause(has_parameter),
            Node::FinallyClause => self.handle_finally_clause(),
            Node::TryBlock => self.handle_try_block(),
            Node::CatchBlock => self.handle_catch_block(),
            Node::FinallyBlock => self.handle_finally_block(),
            Node::CatchParameter => self.handle_catch_parameter(),
            Node::FormalParameter => self.handle_formal_parameter(),
            Node::FormalParameters(n) => self.handle_formal_parameters(n),
            Node::FunctionDeclaration => self.handle_function_declaration(),
            Node::FunctionExpression(named) => self.handle_function_expression(named),
            Node::ArrowFunction => self.handle_arrow_function(),
            Node::ThenBlock => self.handle_then_block(),
            Node::ElseBlock => self.handle_else_block(),
            Node::FalsyShortCircuit => self.handle_falsy_short_circuit(),
            Node::TruthyShortCircuit => self.handle_truthy_short_circuit(),
            Node::NullishShortCircuit => self.handle_nullish_short_circuit(),
            Node::FalsyShortCircuitAssignment => self.handle_falsy_short_circuit_assignment(),
            Node::TruthyShortCircuitAssignment => self.handle_truthy_short_circuit_assignment(),
            Node::NullishShortCircuitAssignment => self.handle_nullish_short_circuit_assignment(),
            Node::LoopStart => self.handle_loop_start(),
            Node::LoopInitExpression => self.handle_loop_init_expression(),
            Node::LoopInitVarDeclaration => self.handle_loop_init_var_declaration(),
            Node::LoopInitLexicalDeclaration => self.handle_loop_init_lexical_declaration(),
            Node::LoopTest => self.handle_loop_test(),
            Node::LoopNext => self.handle_loop_next(),
            Node::LoopBody => self.handle_loop_body(),
            Node::StartBlockScope => self.handle_start_block_scope(),
            Node::EndBlockScope => self.handle_end_block_scope(),
            Node::FunctionContext => self.handle_function_context(),
            Node::FunctionSignature(symbol) => self.handle_function_signature(symbol),
        }
    }

    fn handle_null(&mut self) {
        self.context_stack.last_mut().unwrap().put_null();
    }

    fn handle_boolean(&mut self, value: bool) {
        self.context_stack.last_mut().unwrap().put_boolean(value);
    }

    fn handle_number(&mut self, value: f64) {
        self.context_stack.last_mut().unwrap().put_number(value);
    }

    fn handle_string(&mut self, value: Vec<u16>) {
        self.context_stack.last_mut().unwrap().put_string(value);
    }

    fn handle_identifier_reference(&mut self, symbol: Symbol) {
        let scope_ref = self.scope_tree_builder.current();
        self.context_stack
            .last_mut()
            .unwrap()
            .process_identifier_reference(symbol, scope_ref);
    }

    fn handle_binding_identifier(&mut self, symbol: Symbol) {
        self.context_stack
            .last_mut()
            .unwrap()
            .process_binding_identifier(symbol, &mut self.scope_tree_builder);
    }

    fn handle_argument_list_head(&mut self, empty: bool, spread: bool) {
        self.context_stack
            .last_mut()
            .unwrap()
            .process_argument_list_head(empty, spread)
    }

    fn handle_argument_list_item(&mut self, spread: bool) {
        self.context_stack.last_mut().unwrap().put_argument(spread);
    }

    fn handle_call_expression(&mut self) {
        self.context_stack
            .last_mut()
            .unwrap()
            .process_call_expression();
    }

    fn handle_operator(&mut self, op: CompileCommand) {
        self.put_command(op);
    }

    fn handle_binary_expression(&mut self, op: BinaryOperator) {
        self.context_stack
            .last_mut()
            .unwrap()
            .process_binary_expression(op);
    }

    fn handle_shorthand_assignment_expression(&mut self, op: AssignmentOperator) {
        self.context_stack
            .last_mut()
            .unwrap()
            .process_shorthand_assignment_expression(op);
    }

    fn handle_sequence_expression(&mut self) {
        self.context_stack
            .last_mut()
            .unwrap()
            .process_sequence_expression();
    }

    fn handle_conditional_expression(&mut self) {
        self.put_command(CompileCommand::Ternary);
    }

    fn handle_conditional_assignment(&mut self) {
        let context = self.context_stack.last_mut().unwrap();
        context.put_command(CompileCommand::Ternary);
        context.put_command(CompileCommand::Assignment);
    }

    fn handle_lexical_binding(&mut self, init: bool) {
        self.context_stack
            .last_mut()
            .unwrap()
            .put_lexical_binding(init);
    }

    fn handle_let_declaration(&mut self, n: u32) {
        self.context_stack
            .last_mut()
            .unwrap()
            .process_mutable_bindings(n);
    }

    fn handle_const_declaration(&mut self, n: u32) {
        self.scope_tree_builder.set_immutable(n);
        self.context_stack
            .last_mut()
            .unwrap()
            .process_immutable_bindings(n);
    }

    fn handle_binding_element(&mut self, _init: bool) {
        // TODO
    }

    fn handle_expression_statement(&mut self) {
        self.put_command(CompileCommand::Discard);
    }

    fn handle_if_else_statement(&mut self) {
        self.put_command(CompileCommand::IfElseStatement);
    }

    fn handle_if_statement(&mut self) {
        self.put_command(CompileCommand::IfStatement);
    }

    fn handle_do_while_statement(&mut self) {
        // See handle_loop_start() for the reason why we always pop the lexical scope here.
        self.scope_tree_builder.pop();
        self.context_stack
            .last_mut()
            .unwrap()
            .process_do_while_statement();
    }

    fn handle_while_statement(&mut self) {
        // See handle_loop_start() for the reason why we always pop the lexical scope here.
        self.scope_tree_builder.pop();
        self.context_stack
            .last_mut()
            .unwrap()
            .process_while_statement();
    }

    fn handle_for_statement(&mut self, flags: LoopFlags) {
        // See handle_loop_start() for the reason why we always pop the lexical scope here.
        self.scope_tree_builder.pop();
        self.context_stack
            .last_mut()
            .unwrap()
            .process_for_statement(flags);
    }

    fn handle_continue_statement(&mut self, symbol: Symbol) {
        self.put_command(CompileCommand::Continue(symbol));
    }

    fn handle_break_statement(&mut self, symbol: Symbol) {
        self.put_command(CompileCommand::Break(symbol));
    }

    fn handle_return_statement(&mut self, n: u32) {
        self.put_command(CompileCommand::Return(n));
    }

    fn handle_switch_statement(&mut self) {
        self.context_stack
            .last_mut()
            .unwrap()
            .process_switch_statement();
        self.scope_tree_builder.pop();
    }

    fn handle_case_block(&mut self) {
        let scope_ref = self.scope_tree_builder.push_block("switch");
        self.context_stack
            .last_mut()
            .unwrap()
            .process_case_block(scope_ref);
    }

    fn handle_case_selector(&mut self) {
        self.context_stack
            .last_mut()
            .unwrap()
            .process_case_selector();
    }

    fn handle_case_clause(&mut self, has_statement: bool) {
        self.context_stack
            .last_mut()
            .unwrap()
            .process_case_clause(has_statement);
    }

    fn handle_default_selector(&mut self) {
        self.context_stack
            .last_mut()
            .unwrap()
            .process_default_selector();
    }

    fn handle_default_clause(&mut self, has_statement: bool) {
        self.context_stack
            .last_mut()
            .unwrap()
            .process_default_clause(has_statement);
    }

    fn handle_labelled_statement(&mut self, symbol: Symbol, is_iteration_statement: bool) {
        self.context_stack
            .last_mut()
            .unwrap()
            .process_labelled_statement(symbol, is_iteration_statement);
    }

    fn handle_label(&mut self, symbol: Symbol) {
        self.context_stack.last_mut().unwrap().process_label(symbol);
    }

    fn handle_throw_statement(&mut self) {
        self.context_stack
            .last_mut()
            .unwrap()
            .process_throw_statement();
    }

    fn handle_try_statement(&mut self) {
        self.context_stack.last_mut().unwrap().process_try_end();
    }

    fn handle_catch_clause(&mut self, has_parameter: bool) {
        self.context_stack
            .last_mut()
            .unwrap()
            .process_catch_clause(has_parameter);
    }

    fn handle_finally_clause(&mut self) {
        self.context_stack
            .last_mut()
            .unwrap()
            .process_finally_clause();
    }

    fn handle_try_block(&mut self) {
        self.context_stack.last_mut().unwrap().process_try_block();
    }

    fn handle_catch_block(&mut self) {
        // In the specification, a new lexical scope is created only when the catch parameter
        // exists, but we always create a scope here for simplicity.  In our processing model,
        // the catch and finally clauses are always created even if there is no corresponding
        // node in the AST.
        let scope_ref = self.scope_tree_builder.push_block("catch");
        self.context_stack
            .last_mut()
            .unwrap()
            .process_catch_block(scope_ref);
    }

    fn handle_finally_block(&mut self) {
        self.context_stack
            .last_mut()
            .unwrap()
            .process_finally_block();
        self.scope_tree_builder.pop();
    }

    fn handle_catch_parameter(&mut self) {
        self.context_stack
            .last_mut()
            .unwrap()
            .process_catch_parameter();
    }

    fn handle_formal_parameter(&mut self) {
        // TODO
    }

    fn handle_formal_parameters(&mut self, _n: u32) {
        // TODO
    }

    fn handle_function_declaration(&mut self) {
        let mut context = self.context_stack.pop().unwrap();
        context.end_scope();

        self.scope_tree_builder.pop();
        self.resolve_references(&mut context);

        if context.num_locals > 0 {
            context.commands[0] = CompileCommand::AllocateLocals(context.num_locals);
        }

        if self.runtime_pref.enable_scope_cleanup_checker {
            let stack_size = self.scope_tree_builder.max_stack_size(context.scope_ref);
            debug_assert!(stack_size > 0);
            context.commands[1] = CompileCommand::PrepareScopeCleanupChecker(stack_size);
        }

        let func_index = context.func_index;
        let func = &mut self.functions[func_index];
        func.commands = context.commands;
        func.captures = context.captures.into_values().collect();

        self.context_stack
            .last_mut()
            .unwrap()
            .process_closure_declaration(
                self.scope_tree_builder.current(),
                func.id,
                &func.captures,
            );
    }

    // TODO: reduce code clone took from handle_function_declaration().
    fn handle_function_expression(&mut self, named: bool) {
        let mut context = self.context_stack.pop().unwrap();
        context.end_scope();

        self.scope_tree_builder.pop();
        self.resolve_references(&mut context);

        if context.num_locals > 0 {
            context.commands[0] = CompileCommand::AllocateLocals(context.num_locals);
        }

        let func_index = context.func_index;
        let func = &mut self.functions[func_index];
        func.commands = context.commands;
        func.captures = context.captures.into_values().collect();

        self.context_stack
            .last_mut()
            .unwrap()
            .process_closure_expression(
                self.scope_tree_builder.current(),
                func.id,
                &func.captures,
                named,
            );
    }

    fn handle_arrow_function(&mut self) {
        // TODO: An ArrowFunction does not define local bindings for arguments, super, this, or
        // new.target.  Any reference to arguments, super, this, or new.target within an
        // ArrowFunction must resolve to a binding in a lexically enclosing environment.

        let mut context = self.context_stack.pop().unwrap();
        context.end_scope();

        self.scope_tree_builder.pop();
        self.resolve_references(&mut context);

        if context.num_locals > 0 {
            context.commands[0] = CompileCommand::AllocateLocals(context.num_locals);
        }

        let func_index = context.func_index;
        let func = &mut self.functions[func_index];
        func.commands = context.commands;
        func.captures = context.captures.into_values().collect();

        self.context_stack
            .last_mut()
            .unwrap()
            .process_closure_expression(
                self.scope_tree_builder.current(),
                func.id,
                &func.captures,
                false,
            );
    }

    fn handle_then_block(&mut self) {
        let context = self.context_stack.last_mut().unwrap();
        context.put_command(CompileCommand::Truthy);
        context.put_command(CompileCommand::Then);
    }

    fn handle_else_block(&mut self) {
        self.put_command(CompileCommand::Else);
    }

    fn handle_falsy_short_circuit(&mut self) {
        self.put_command(CompileCommand::FalsyShortCircuit);
    }

    fn handle_truthy_short_circuit(&mut self) {
        self.put_command(CompileCommand::TruthyShortCircuit);
    }

    fn handle_nullish_short_circuit(&mut self) {
        self.put_command(CompileCommand::NullishShortCircuit);
    }

    fn handle_falsy_short_circuit_assignment(&mut self) {
        let context = self.context_stack.last_mut().unwrap();
        context.put_command(CompileCommand::Duplicate(0));
        context.put_command(CompileCommand::FalsyShortCircuit);
    }

    fn handle_truthy_short_circuit_assignment(&mut self) {
        let context = self.context_stack.last_mut().unwrap();
        context.put_command(CompileCommand::Duplicate(0));
        context.put_command(CompileCommand::TruthyShortCircuit);
    }

    fn handle_nullish_short_circuit_assignment(&mut self) {
        let context = self.context_stack.last_mut().unwrap();
        context.put_command(CompileCommand::Duplicate(0));
        context.put_command(CompileCommand::NullishShortCircuit);
    }

    fn handle_loop_start(&mut self) {
        // NOTE: This doesn't follow the specification, but we create a new lexical scope for the
        // iteration statement.  This is needed for the for-let/const statements, but not for
        // others.  We believe that this change does not compromise conformance to the
        // specification and does not cause security problems.
        let scope_ref = self.scope_tree_builder.push_block("loop");
        self.context_stack
            .last_mut()
            .unwrap()
            .process_loop_start(scope_ref);
    }

    fn handle_loop_init_expression(&mut self) {
        self.context_stack
            .last_mut()
            .unwrap()
            .process_loop_init_expression();
    }

    fn handle_loop_init_var_declaration(&mut self) {
        self.context_stack
            .last_mut()
            .unwrap()
            .process_loop_init_declaration();
    }

    fn handle_loop_init_lexical_declaration(&mut self) {
        self.context_stack
            .last_mut()
            .unwrap()
            .process_loop_init_declaration();
    }

    fn handle_loop_test(&mut self) {
        self.context_stack.last_mut().unwrap().process_loop_test();
    }

    fn handle_loop_next(&mut self) {
        self.context_stack.last_mut().unwrap().process_loop_next();
    }

    fn handle_loop_body(&mut self) {
        self.context_stack.last_mut().unwrap().process_loop_body();
    }

    fn handle_start_block_scope(&mut self) {
        let scope_ref = self.scope_tree_builder.push_block("");
        self.context_stack
            .last_mut()
            .unwrap()
            .start_scope(scope_ref);
    }

    fn handle_end_block_scope(&mut self) {
        self.context_stack.last_mut().unwrap().end_scope();
        self.scope_tree_builder.pop();
    }

    fn handle_function_context(&mut self) {
        let scope_ref = self.scope_tree_builder.push_function();

        // TODO: the compilation should fail if the following condition is unmet.
        assert!(self.functions.len() < u32::MAX as usize);
        let func_index = self.functions.len();
        let mut context = FunctionContext {
            func_index,
            scope_ref,
            // `commands[0]` will be replaced with `AllocateLocals` if the function has local
            // variables.
            commands: vec![CompileCommand::Nop],
            ..Default::default()
        };
        if self.runtime_pref.enable_scope_cleanup_checker {
            // Put a placeholder command which will be replaced with `PrepareScopeCleanupChecker`.
            let index = context.put_command(CompileCommand::Nop);
            debug_assert_eq!(index, 1);
        }
        context.start_scope(scope_ref);
        self.context_stack.push(context);
        // Push a placeholder data which will be filled later.
        self.functions.push(FunctionRecipe {
            symbol: Symbol::NONE,
            id: FunctionId::MAIN,
            commands: vec![],
            captures: Default::default(),
        });
    }

    fn handle_function_signature(&mut self, symbol: Symbol) {
        let context = self.context_stack.last_mut().unwrap();
        let id = self.function_registry.create_native_function();
        let func_index = context.func_index;
        self.functions[func_index].symbol = symbol;
        self.functions[func_index].id = id;
        context.in_body = true;
    }

    fn put_command(&mut self, command: CompileCommand) {
        self.context_stack.last_mut().unwrap().put_command(command);
    }

    // TODO: global object
    fn put_global_bindings(&mut self) {
        let context = self.context_stack.last_mut().unwrap();

        // Register `undefined`.
        let symbol = Symbol::UNDEFINED;
        // The locator will be computed in `resolve_references()`.
        let command_index = context.put_command(CompileCommand::REFERENCE_PLACEHOLDER);
        context.put_lexical_binding(false);
        context.process_immutable_bindings(1);
        self.scope_tree_builder
            .add_immutable(symbol, context.num_locals);
        context.references.push(Reference {
            symbol,
            scope_ref: self.scope_tree_builder.current(),
            from: ReferenceFrom::Command(command_index),
        });
        context.num_locals += 1;

        // Register `Infinity`.
        let symbol = Symbol::INFINITY;
        // The locator will be computed in `resolve_references()`.
        let command_index = context.put_command(CompileCommand::REFERENCE_PLACEHOLDER);
        context.put_number(f64::INFINITY);
        context.put_lexical_binding(true);
        context.process_immutable_bindings(1);
        self.scope_tree_builder
            .add_immutable(symbol, context.num_locals);
        context.references.push(Reference {
            symbol,
            scope_ref: self.scope_tree_builder.current(),
            from: ReferenceFrom::Command(command_index),
        });
        context.num_locals += 1;

        // Register `NaN`.
        let symbol = Symbol::NAN;
        // The locator will be computed in `resolve_references()`.
        let command_index = context.put_command(CompileCommand::REFERENCE_PLACEHOLDER);
        context.put_number(f64::NAN);
        context.put_lexical_binding(true);
        context.process_immutable_bindings(1);
        self.scope_tree_builder
            .add_immutable(symbol, context.num_locals);
        context.references.push(Reference {
            symbol,
            scope_ref: self.scope_tree_builder.current(),
            from: ReferenceFrom::Command(command_index),
        });
        context.num_locals += 1;
    }

    // TODO: global object
    fn register_host_functions(&mut self) {
        let context = self.context_stack.last_mut().unwrap();

        for (func_id, host_func) in self.function_registry.enumerate_host_function() {
            let symbol = self.symbol_registry.intern_cstr(&host_func.name);
            // The locator will be computed in `resolve_references()`.
            let command_index = context.put_command(CompileCommand::REFERENCE_PLACEHOLDER);
            context.process_closure_declaration(self.scope_tree_builder.current(), func_id, &[]);
            self.scope_tree_builder
                .add_immutable(symbol, context.num_locals);
            context.references.push(Reference {
                symbol,
                scope_ref: self.scope_tree_builder.current(),
                from: ReferenceFrom::Command(command_index),
            });
            context.num_locals += 1;
        }
    }

    fn resolve_references(&mut self, context: &mut FunctionContext) {
        for reference in std::mem::take(&mut context.references).iter() {
            self.resolve_reference(context, reference);
        }
    }

    // TODO: refactoring
    fn resolve_reference(&mut self, context: &mut FunctionContext, reference: &Reference) {
        let binding_ref = self.scope_tree_builder.resolve_reference(reference);
        logger::debug!(event = "resolve-reference", ?reference, ?binding_ref);

        if binding_ref == BindingRef::NONE {
            // This is a reference to a free variable.
            let capture_index = match context.captures.get_full(&reference.symbol) {
                Some((capture_index, ..)) => capture_index,
                None => {
                    let (capture_index, _) = context.captures.insert_full(
                        reference.symbol,
                        Capture {
                            symbol: reference.symbol,
                            target: Locator::None,
                        },
                    );
                    self.context_stack
                        .last_mut()
                        .unwrap()
                        .references
                        .push(Reference {
                            symbol: reference.symbol,
                            scope_ref: self.scope_tree_builder.current(),
                            from: ReferenceFrom::Capture(context.func_index, capture_index),
                        });
                    capture_index
                }
            };
            let locator = Locator::checked_capture(capture_index).unwrap();
            match reference.from {
                ReferenceFrom::Command(command_index) => {
                    context.commands[command_index] =
                        CompileCommand::Reference(reference.symbol, locator);
                }
                ReferenceFrom::Capture(func_index, capture_index) => {
                    self.functions[func_index].captures[capture_index].target = locator;
                }
            }
            return;
        }

        let locator = self.scope_tree_builder.compute_locator(binding_ref);
        match reference.from {
            ReferenceFrom::Command(command_index) => {
                context.commands[command_index] =
                    CompileCommand::Reference(reference.symbol, locator);
            }
            ReferenceFrom::Capture(func_index, capture_index) => {
                self.functions[func_index].captures[capture_index].target = locator;
                self.scope_tree_builder.set_captured(binding_ref);
            }
        }
    }
}

impl<'r, 's> NodeHandler<'s> for Analyzer<'r> {
    type Artifact = Program;

    fn start(&mut self) {
        logger::debug!(event = "start");
        let scope_ref = self.scope_tree_builder.push_function();

        let context = self.context_stack.last_mut().unwrap();
        context.scope_ref = scope_ref;
        context.start_scope(scope_ref);

        if self.use_global_bindings {
            self.put_global_bindings();
        }

        self.register_host_functions();
    }

    fn accept(&mut self) -> Result<Self::Artifact, Error> {
        logger::debug!(event = "accept");

        let mut context = self.context_stack.pop().unwrap();
        context.end_scope();

        self.scope_tree_builder.pop();
        self.resolve_references(&mut context);
        debug_assert!(context.captures.is_empty());

        if context.num_locals > 0 {
            context.commands[0] = CompileCommand::AllocateLocals(context.num_locals);
        }

        self.functions[context.func_index].commands = context.commands;
        self.functions[context.func_index].captures = context.captures.into_values().collect();

        Ok(Program {
            functions: std::mem::take(&mut self.functions),
            scope_tree: self.scope_tree_builder.build(),
        })
    }

    fn handle_nodes(&mut self, nodes: impl Iterator<Item = Node<'s>>) -> Result<(), Error> {
        for node in nodes {
            self.handle_node(node);
        }
        Ok(())
    }

    fn symbol_registry_mut(&mut self) -> &mut SymbolRegistry {
        self.symbol_registry
    }
}

/// A type representing analysis states of a JavaScript function.
///
/// This type uses a stack for each data type, instead of using a single stack that holds an
/// enumerate type having a variant for the each data type.  This way make it possible to easily
/// and efficiently access to particular elements in a stack for a data type.  While on the other
/// hand, this way is inefficient in memory usage points of view.
#[derive(Default)]
struct FunctionContext {
    /// A buffer to store [`CompileCommand`]s while analyzing.
    ///
    /// Some of commands stored in the buffer will work as placeholders and will be updated later.
    commands: Vec<CompileCommand>,

    /// Holds unresolved references in the function.
    ///
    /// All the references will be resolved at the end of the function definition in order to
    /// compute the memory layout of local variables on the stack because the computation has to be
    /// performed after all variable declarations are processed.
    references: Vec<Reference>,

    /// A list of captured variables outside the function scope.
    captures: IndexMap<Symbol, Capture>,

    /// A list of indexes of commands that have to be updated while analyzing.
    pending_lexical_bindings: Vec<usize>,

    /// A list of symbols in the formal parameters of the function.
    formal_parameters: Vec<Symbol>,

    /// A stack to hold [`Scope`]s.
    scope_stack: Vec<Scope>,

    /// A stack to hold [`LoopContext`]s.
    loop_stack: Vec<LoopContext>,

    /// A stack to hold [`SwitchContext`]s.
    switch_stack: Vec<SwitchContext>,

    /// A stack to hold [`LabelContext`]s.
    label_stack: Vec<LabelContext>,

    /// A stack to hold [`TryContext`]s.
    try_stack: Vec<TryContext>,

    /// A stack to hold the number of arguments of a function call.
    nargs_stack: Vec<(usize, u16)>,

    /// The index of the function in [`Analyzer::functions`].
    func_index: usize,

    /// A reference to a function scope in the scope tree.
    scope_ref: ScopeRef,

    num_locals: u16,
    num_do_while_statements: u16,
    num_while_statements: u16,
    num_for_statements: u16,
    num_switch_statements: u16,

    /// `false` while analyzing formal parameters, `true` while analyzing the function body.
    in_body: bool,
}

impl FunctionContext {
    fn put_command(&mut self, command: CompileCommand) -> usize {
        let index = self.commands.len();
        self.commands.push(command);
        index
    }

    fn put_null(&mut self) {
        self.commands.push(CompileCommand::Null);
        // TODO: type inference
    }

    fn put_boolean(&mut self, value: bool) {
        self.commands.push(CompileCommand::Boolean(value));
        // TODO: type inference
    }

    fn put_number(&mut self, value: f64) {
        self.commands.push(CompileCommand::Number(value));
        // TODO: type inference
    }

    fn put_string(&mut self, value: Vec<u16>) {
        self.commands.push(CompileCommand::String(value));
        // TODO: type inference
    }

    fn process_argument_list_head(&mut self, empty: bool, _spread: bool) {
        // TODO: spread
        let index = self.put_command(CompileCommand::Nop);
        let nargs = if empty {
            0
        } else {
            self.commands.push(CompileCommand::Swap);
            self.commands.push(CompileCommand::Argument(0));
            1
        };
        self.nargs_stack.push((index, nargs));
    }

    fn put_argument(&mut self, _spread: bool) {
        // TODO: spread
        let tuple = self.nargs_stack.last_mut().unwrap();
        self.commands.push(CompileCommand::Argument(tuple.1));
        self.nargs_stack.last_mut().unwrap().1 += 1;
    }

    fn process_call_expression(&mut self) {
        let (index, nargs) = self.nargs_stack.pop().unwrap();
        self.commands[index] = CompileCommand::Arguments(nargs);
        self.commands.push(CompileCommand::Call(nargs));
    }

    fn put_lexical_binding(&mut self, init: bool) {
        if !init {
            // Set undefined as the initial value.
            self.commands.push(CompileCommand::Undefined);
        }
        // The command will be set later.
        let command_index = self.put_command(CompileCommand::Nop);
        self.pending_lexical_bindings.push(command_index);
        // TODO: type info
    }

    fn process_identifier_reference(&mut self, symbol: Symbol, scope_ref: ScopeRef) {
        // The locator will be updated later.
        let command_index = self.put_command(CompileCommand::REFERENCE_PLACEHOLDER);
        self.references.push(Reference {
            symbol,
            scope_ref,
            from: ReferenceFrom::Command(command_index),
        });
    }

    fn process_binding_identifier(&mut self, symbol: Symbol, builder: &mut ScopeTreeBuilder) {
        if self.in_body {
            // The locator will be updated later.
            let command_index = self.put_command(CompileCommand::REFERENCE_PLACEHOLDER);
            self.references.push(Reference {
                symbol,
                scope_ref: builder.current(),
                from: ReferenceFrom::Command(command_index),
            });
            // The BindingKind may change later by `builder.set_immutable()`.
            builder.add_mutable(symbol, self.num_locals);
            self.num_locals += 1;
        } else {
            // TODO: the compilation should fail if the following condition is unmet.
            assert!(self.formal_parameters.len() < u16::MAX as usize);
            let i = self.formal_parameters.len();
            self.formal_parameters.push(symbol);
            builder.add_formal_parameter(symbol, i);
        }
    }

    fn process_binary_expression(&mut self, op: BinaryOperator) {
        // When a compiler processes the following command, the RHS has been placed on the LHS on
        // the stack of the compiler.  Swap them so that the LHS is evaluated before the RHS.
        self.put_command(CompileCommand::Swap);
        self.put_command(op.into());
    }

    // Every shorthand assignment operator except for short-circuit assignment operators are
    // expanded to a simple binary operator and an assignment operator.
    fn process_shorthand_assignment_expression(&mut self, op: AssignmentOperator) {
        // When a compiler processes the following command, the RHS has been placed on the LHS on
        // the stack of the compiler.  Duplicate the LHS and push it onto the stack.
        self.put_command(CompileCommand::Duplicate(1));
        // Now, the duplicate LHS has been placed on the RHS.  And the LHS will be evaluated before
        // the RHS.  We don't need to put a CompileCommand::Swap command and just put the operator.
        self.put_command(op.into());
        // Finally, we put the assignment operator.
        self.put_command(CompileCommand::Assignment);
    }

    fn process_sequence_expression(&mut self) {
        self.put_command(CompileCommand::Swap);
        self.put_command(CompileCommand::Discard);
    }

    fn process_mutable_bindings(&mut self, n: u32) {
        debug_assert_eq!(n as usize, self.pending_lexical_bindings.len());
        for i in self.pending_lexical_bindings.iter().cloned() {
            debug_assert!(matches!(self.commands[i], CompileCommand::Nop));
            self.commands[i] = CompileCommand::MutableBinding;
        }
        self.pending_lexical_bindings.clear();
    }

    fn process_immutable_bindings(&mut self, n: u32) {
        debug_assert_eq!(n as usize, self.pending_lexical_bindings.len());
        for i in self.pending_lexical_bindings.iter().cloned() {
            debug_assert!(matches!(self.commands[i], CompileCommand::Nop));
            self.commands[i] = CompileCommand::ImmutableBinding;
        }
        self.pending_lexical_bindings.clear();
    }

    fn process_closure_declaration(
        &mut self,
        scope_ref: ScopeRef,
        func_id: FunctionId,
        captures: &[Capture],
    ) {
        for capture in captures.iter().rev() {
            // `capture.target` has not been resolved at this point...
            let command_index = self.put_command(CompileCommand::REFERENCE_PLACEHOLDER);
            self.references.push(Reference {
                symbol: capture.symbol,
                scope_ref,
                from: ReferenceFrom::Command(command_index),
            });
            self.commands.push(CompileCommand::CaptureVariable(true));
        }
        self.commands.push(CompileCommand::Function(func_id));
        self.commands
            .push(CompileCommand::Closure(true, captures.len() as u16));
        self.commands.push(CompileCommand::DeclareClosure);
    }

    fn process_closure_expression(
        &mut self,
        scope_ref: ScopeRef,
        func_id: FunctionId,
        captures: &[Capture],
        named: bool,
    ) {
        if named {
            // Remove the BindingIdentifier of the function.
            self.put_command(CompileCommand::Discard);
        }
        for capture in captures.iter().rev() {
            // `capture.target` has not been resolved at this point...
            let command_index = self.put_command(CompileCommand::REFERENCE_PLACEHOLDER);
            self.references.push(Reference {
                symbol: capture.symbol,
                scope_ref,
                from: ReferenceFrom::Command(command_index),
            });
            self.commands.push(CompileCommand::CaptureVariable(false));
        }
        self.commands.push(CompileCommand::Function(func_id));
        self.commands
            .push(CompileCommand::Closure(false, captures.len() as u16));
    }

    fn process_loop_start(&mut self, scope_ref: ScopeRef) {
        self.start_scope(scope_ref);

        // Push `Nop` as a placeholder.
        // It will be replaced with an appropriate command in process_loop_end().
        let start_index = self.put_command(CompileCommand::Nop);
        self.loop_stack.push(LoopContext { start_index });
    }

    fn process_loop_init_expression(&mut self) {
        // Discard the evaluation result of the expression like as ExpressionStatement.
        self.put_command(CompileCommand::Discard);
        self.put_command(CompileCommand::LoopInit);
    }

    fn process_loop_init_declaration(&mut self) {
        self.put_command(CompileCommand::LoopInit);
    }

    fn process_loop_test(&mut self) {
        self.put_command(CompileCommand::LoopTest);
    }

    fn process_loop_next(&mut self) {
        self.put_command(CompileCommand::LoopNext);
    }

    fn process_loop_body(&mut self) {
        self.put_command(CompileCommand::LoopBody);
    }

    fn process_loop_end(&mut self, command: CompileCommand) {
        self.put_command(CompileCommand::LoopEnd);
        let LoopContext { start_index } = self.loop_stack.pop().unwrap();
        self.commands[start_index] = command;

        self.end_scope();
    }

    fn process_do_while_statement(&mut self) {
        self.put_command(CompileCommand::LoopTest);
        self.process_loop_end(CompileCommand::DoWhileLoop(self.num_do_while_statements));
        self.num_do_while_statements += 1;
    }

    fn process_while_statement(&mut self) {
        self.put_command(CompileCommand::LoopBody);
        self.process_loop_end(CompileCommand::WhileLoop(self.num_while_statements));
        self.num_while_statements += 1;
    }

    fn process_for_statement(&mut self, flags: LoopFlags) {
        self.put_command(CompileCommand::LoopBody);
        self.process_loop_end(CompileCommand::ForLoop(self.num_for_statements, flags));
        self.num_for_statements += 1;
    }

    fn process_case_block(&mut self, scope_ref: ScopeRef) {
        // Step#3..7 in 14.12.4 Runtime Semantics: Evaluation
        self.start_scope(scope_ref);

        let case_block_index = self.put_command(CompileCommand::Nop);
        self.switch_stack.push(SwitchContext {
            case_block_index,
            ..Default::default()
        });
    }

    fn process_case_selector(&mut self) {
        self.put_command(CompileCommand::StrictEquality);
        self.put_command(CompileCommand::Then);
    }

    fn process_case_clause(&mut self, has_statement: bool) {
        self.put_command(CompileCommand::CaseClause(has_statement));
        self.switch_stack.last_mut().unwrap().num_cases += 1;
    }

    fn process_default_selector(&mut self) {
        self.put_command(CompileCommand::Discard);
        // TODO: refactoring
        self.put_command(CompileCommand::Then);
    }

    fn process_default_clause(&mut self, has_statement: bool) {
        self.put_command(CompileCommand::DefaultClause(has_statement));
        let context = self.switch_stack.last_mut().unwrap();
        context.default_index = Some(context.num_cases);
        context.num_cases += 1;
    }

    fn process_switch_statement(&mut self) {
        let context = self.switch_stack.pop().unwrap();

        let id = self.num_switch_statements;
        let n = context.num_cases;

        if n == 0 {
            // empty case block
            // Discard the `switchValue`.
            self.commands[context.case_block_index] = CompileCommand::Discard;
        } else {
            self.commands[context.case_block_index] = CompileCommand::CaseBlock(id, n);
            let i = context.default_index;
            self.put_command(CompileCommand::Switch(id, n, i));
            self.num_switch_statements += 1;
        }

        // Step#8 in 14.12.4 Runtime Semantics: Evaluation
        self.end_scope();
    }

    fn process_label(&mut self, symbol: Symbol) {
        // Push `Nop` as a placeholder.
        // It will be replaced with `CompileCommand::LabelStart(..)` in
        // `process_labelled_statement()`.
        let start_index = self.put_command(CompileCommand::Nop);
        self.label_stack.push(LabelContext {
            start_index,
            symbol,
        });
    }

    fn process_labelled_statement(&mut self, symbol: Symbol, is_iteration_statement: bool) {
        let label = self.label_stack.pop().unwrap();
        debug_assert_eq!(label.symbol, symbol);
        self.commands[label.start_index] =
            CompileCommand::LabelStart(symbol, is_iteration_statement);
        self.put_command(CompileCommand::LabelEnd(symbol, is_iteration_statement));
    }

    fn process_throw_statement(&mut self) {
        self.put_command(CompileCommand::Throw);
    }

    fn process_try_block(&mut self) {
        self.put_command(CompileCommand::Try);
        self.try_stack.push(Default::default());
    }

    fn process_catch_block(&mut self, scope_ref: ScopeRef) {
        // Push a *nominal* `Catch` command.
        // It will be replaced with a *substantial* `Catch` command in `process_catch_clause()`
        // if a catch clause exists.
        let index = self.put_command(CompileCommand::Catch(true));
        self.try_stack.last_mut().unwrap().catch_index = index;
        self.start_scope(scope_ref);
    }

    fn process_catch_clause(&mut self, _has_parameter: bool) {
        let index = self.try_stack.last().unwrap().catch_index;
        self.commands[index] = CompileCommand::Catch(false); // substantial
    }

    fn process_finally_block(&mut self) {
        // Remove the scope created for the catch clause.
        self.end_scope();
        // Push a *nominal* `Finally` command.
        // It will be replaced with a *substantial* `Finally` command in `process_finally_clause()`
        // if a finally clause exists.
        let index = self.put_command(CompileCommand::Finally(true));
        self.try_stack.last_mut().unwrap().finally_index = index;
    }

    fn process_finally_clause(&mut self) {
        let index = self.try_stack.last().unwrap().finally_index;
        self.commands[index] = CompileCommand::Finally(false); // substantial
    }

    fn process_try_end(&mut self) {
        self.put_command(CompileCommand::TryEnd);
        self.try_stack.pop();
    }

    fn process_catch_parameter(&mut self) {
        self.put_command(CompileCommand::Exception);
        self.put_lexical_binding(true);
        self.process_mutable_bindings(1);
    }

    fn start_scope(&mut self, scope_ref: ScopeRef) {
        // NOTE(perf): The scope may has no binding.  In this case, we can remove the
        // PushScope command safely and reduce the number of the commands.  We can add a
        // post-process for optimization if it's needed.
        self.put_command(CompileCommand::PushScope(scope_ref));
        self.scope_stack.push(Scope { scope_ref });
    }

    fn end_scope(&mut self) {
        let scope = self.scope_stack.pop().unwrap();

        // NOTE(perf): The scope may has no binding.  In this case, we can remove the
        // PopScope command safely and reduce the number of the commands.  We can add a
        // post-process for optimization if it's needed.
        self.commands
            .push(CompileCommand::PopScope(scope.scope_ref));
    }
}

struct Scope {
    scope_ref: ScopeRef,
}

struct LoopContext {
    start_index: usize,
}

#[derive(Default)]
struct SwitchContext {
    case_block_index: usize,
    num_cases: u16,
    default_index: Option<u16>,
}

#[derive(Default)]
struct LabelContext {
    start_index: usize,
    symbol: Symbol,
}

#[derive(Default)]
struct TryContext {
    catch_index: usize,
    finally_index: usize,
}

/// A compile command.
#[derive(Debug, PartialEq)]
pub enum CompileCommand {
    Nop,

    Undefined,
    Null,
    Boolean(bool),
    Number(f64),
    String(Vec<u16>),
    Function(FunctionId),
    Closure(bool, u16),
    Reference(Symbol, Locator),
    Exception,

    AllocateLocals(u16),
    MutableBinding,
    ImmutableBinding,
    DeclareFunction,
    DeclareClosure,
    Arguments(u16),
    Argument(u16),
    Call(u16),
    PushScope(ScopeRef),
    PopScope(ScopeRef),
    CaptureVariable(bool),

    // update operators
    PostfixIncrement,
    PostfixDecrement,
    PrefixIncrement,
    PrefixDecrement,

    // unary operators
    Delete,
    Void,
    Typeof,
    UnaryPlus,
    UnaryMinus,
    BitwiseNot,
    LogicalNot,

    // exponentiation operator
    Exponentiation,

    // multiplicative operators
    Multiplication,
    Division,
    Remainder,

    // additive operators
    Addition,
    Subtraction,

    // bitwise shift operators
    LeftShift,
    SignedRightShift,
    UnsignedRightShift,

    // relational operators
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    Instanceof,
    In,

    // equality operators
    Equality,
    Inequality,
    StrictEquality,
    StrictInequality,

    // binary bitwise operators
    BitwiseAnd,
    BitwiseXor,
    BitwiseOr,

    // There is no compile command for binary logical operators.
    //
    // For the short-circuit evaluation on the LHS in a logical expression, we convert a binary
    // logical expression into a corresponding conditional expression.
    //
    // The conversion is performed in the following two steps:
    //
    //   1. Perform the short-circuit evaluation by using a special action for each logical
    //      operator in handle_falsy_short_circuit() for `&&`, handle_truthy_short_circuit() for
    //      `||` and handle_nullish_short_circuit() for `??`.
    //   2. Emit supplemental commands and CompileCommand::ConditionalTernery in
    //      handle_logical_expression()

    // ternary operator
    Ternary,

    // assignment operators
    Assignment,

    // short-circuit
    FalsyShortCircuit,
    TruthyShortCircuit,
    NullishShortCircuit,

    // conditional
    Truthy,
    Then,
    Else,
    IfElseStatement,
    IfStatement,

    // loop
    WhileLoop(u16),
    DoWhileLoop(u16),
    ForLoop(u16, LoopFlags),
    LoopInit,
    LoopTest,
    LoopNext,
    LoopBody,
    LoopEnd,

    // switch
    CaseBlock(u16, u16),
    CaseClause(bool),
    DefaultClause(bool),
    Switch(u16, u16, Option<u16>),

    // label
    LabelStart(Symbol, bool),
    LabelEnd(Symbol, bool),

    // try, catch, finally
    Try,
    Catch(bool),
    Finally(bool),
    TryEnd,

    Continue(Symbol),
    Break(Symbol),
    Return(u32),
    Throw,

    Discard,
    Swap,
    Duplicate(u8), // 0 or 1

    PrepareScopeCleanupChecker(u16),
}

impl CompileCommand {
    const REFERENCE_PLACEHOLDER: Self = Self::Reference(Symbol::NONE, Locator::None);
}

impl From<UpdateOperator> for CompileCommand {
    fn from(value: UpdateOperator) -> Self {
        match value {
            UpdateOperator::PostfixIncrement => Self::PostfixIncrement,
            UpdateOperator::PostfixDecrement => Self::PostfixDecrement,
            UpdateOperator::PrefixIncrement => Self::PrefixIncrement,
            UpdateOperator::PrefixDecrement => Self::PrefixDecrement,
        }
    }
}

impl From<UnaryOperator> for CompileCommand {
    fn from(value: UnaryOperator) -> Self {
        match value {
            UnaryOperator::Delete => Self::Delete,
            UnaryOperator::Void => Self::Void,
            UnaryOperator::Typeof => Self::Typeof,
            UnaryOperator::Plus => Self::UnaryPlus,
            UnaryOperator::Minus => Self::UnaryMinus,
            UnaryOperator::BitwiseNot => Self::BitwiseNot,
            UnaryOperator::LogicalNot => Self::LogicalNot,
        }
    }
}

impl From<BinaryOperator> for CompileCommand {
    fn from(value: BinaryOperator) -> Self {
        match value {
            BinaryOperator::Equality => Self::Equality,
            BinaryOperator::Inequality => Self::Inequality,
            BinaryOperator::StrictEquality => Self::StrictEquality,
            BinaryOperator::StrictInequality => Self::StrictInequality,
            BinaryOperator::LessThan => Self::LessThan,
            BinaryOperator::LessThanOrEqual => Self::LessThanOrEqual,
            BinaryOperator::GreaterThan => Self::GreaterThan,
            BinaryOperator::GreaterThanOrEqual => Self::GreaterThanOrEqual,
            BinaryOperator::LeftShift => Self::LeftShift,
            BinaryOperator::SignedRightShift => Self::SignedRightShift,
            BinaryOperator::UnsignedRightShift => Self::UnsignedRightShift,
            BinaryOperator::Addition => Self::Addition,
            BinaryOperator::Subtraction => Self::Subtraction,
            BinaryOperator::Multiplication => Self::Multiplication,
            BinaryOperator::Division => Self::Division,
            BinaryOperator::Remainder => Self::Remainder,
            BinaryOperator::BitwiseOr => Self::BitwiseOr,
            BinaryOperator::BitwiseXor => Self::BitwiseXor,
            BinaryOperator::BitwiseAnd => Self::BitwiseAnd,
            BinaryOperator::In => Self::In,
            BinaryOperator::Instanceof => Self::Instanceof,
            BinaryOperator::Exponentiation => Self::Exponentiation,
        }
    }
}

// Convert '=<binary-op>' into '<binary-op>'.
impl From<AssignmentOperator> for CompileCommand {
    fn from(value: AssignmentOperator) -> Self {
        match value {
            AssignmentOperator::MultiplicationAssignment => Self::Multiplication,
            AssignmentOperator::DivisionAssignment => Self::Division,
            AssignmentOperator::RemainderAssignment => Self::Remainder,
            AssignmentOperator::AdditionAssignment => Self::Addition,
            AssignmentOperator::SubtractionAssignment => Self::Subtraction,
            AssignmentOperator::LeftShiftAssignment => Self::LeftShift,
            AssignmentOperator::SignedRightShiftAssignment => Self::SignedRightShift,
            AssignmentOperator::UnsignedRightShiftAssignment => Self::UnsignedRightShift,
            AssignmentOperator::BitwiseAndAssignment => Self::BitwiseAnd,
            AssignmentOperator::BitwiseXorAssignment => Self::BitwiseXor,
            AssignmentOperator::BitwiseOrAssignment => Self::BitwiseOr,
            AssignmentOperator::ExponentiationAssignment => Self::Exponentiation,
            // There is no corresponding command for `&&=`, `||=` and `??=`.
            // These are converted into the corresponding conditional expression for short-circuit
            // evaluation of the LHS.
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Locator {
    None,
    Argument(u16),
    Local(u16),
    Capture(u16),
}

impl Locator {
    const MAX_INDEX: usize = u16::MAX as usize;

    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    pub fn is_argument(&self) -> bool {
        matches!(self, Self::Argument(_))
    }

    pub fn is_local(&self) -> bool {
        matches!(self, Self::Local(_))
    }

    pub fn is_capture(&self) -> bool {
        matches!(self, Self::Capture(_))
    }

    fn checked_capture(index: usize) -> Option<Self> {
        Self::ensure_index(index)?;
        Some(Self::Capture(index as u16))
    }

    fn ensure_index(index: usize) -> Option<()> {
        if index > Self::MAX_INDEX {
            crate::logger::error!(err = "too large", index);
            None
        } else {
            Some(())
        }
    }
}

/// A type representing information needed for resolving a reference to a symbol.
#[derive(Debug)]
struct Reference {
    /// The symbol referred.
    symbol: Symbol,

    /// The reference to a (function or block) scope where the symbol is referred.
    scope_ref: ScopeRef,

    from: ReferenceFrom,
}

#[derive(Debug)]
enum ReferenceFrom {
    /// A reference to a [`CompileCommand`] that needs to be updated.
    ///
    /// This is a tuple of two indexes.  The first one is the index of a [`FunctionContext`] in
    /// [`Analyzer::functions`].  The second one is the index of the [`CompileCommand`] in
    /// the [`FunctionContext::commands`] identified by the first index.
    Command(usize),

    /// A reference from a [`Capture`] that needs to be updated.
    ///
    /// This is a tuple of two indexes.  The first one is the index of a [`FunctionContext`] in
    /// [`Analyzer::functions`].  The second one is the index of the [`Capture`] in
    /// the [`FunctionContext::captures`] identified by the first index.
    Capture(usize, usize),
}

#[cfg(test)]
mod tests {
    use jsparser::Parser;
    use jsparser::Processor;

    use super::*;

    macro_rules! symbol {
        ($symbol_registry:expr, $name:literal) => {
            $symbol_registry.lookup($name.encode_utf16().collect::<Vec<u16>>().as_slice())
        };
    }
    macro_rules! scope_ref {
        ($index:expr) => {
            ScopeRef::new($index)
        };
    }

    macro_rules! locator {
        (local: $index:expr) => {
            Locator::Local($index)
        };
    }

    #[test]
    fn test_lexical_declarations() {
        test("let a, b = 2; const c = 3, d = 4;", |reg, program| {
            assert_eq!(
                program.functions[0].commands,
                [
                    CompileCommand::AllocateLocals(4),
                    CompileCommand::PushScope(scope_ref!(1)),
                    CompileCommand::Reference(symbol!(reg, "a"), locator!(local: 0)),
                    CompileCommand::Undefined,
                    CompileCommand::MutableBinding,
                    CompileCommand::Reference(symbol!(reg, "b"), locator!(local: 1)),
                    CompileCommand::Number(2.0),
                    CompileCommand::MutableBinding,
                    CompileCommand::Reference(symbol!(reg, "c"), locator!(local: 2)),
                    CompileCommand::Number(3.0),
                    CompileCommand::ImmutableBinding,
                    CompileCommand::Reference(symbol!(reg, "d"), locator!(local: 3)),
                    CompileCommand::Number(4.0),
                    CompileCommand::ImmutableBinding,
                    CompileCommand::PopScope(scope_ref!(1)),
                ]
            );
        });
    }

    #[test]
    fn test_lexical_declarations_in_scopes() {
        test("let a; { let a; } { let a, b; }", |reg, program| {
            assert_eq!(
                program.functions[0].commands,
                [
                    CompileCommand::AllocateLocals(4),
                    CompileCommand::PushScope(scope_ref!(1)),
                    CompileCommand::Reference(symbol!(reg, "a"), locator!(local: 0)),
                    CompileCommand::Undefined,
                    CompileCommand::MutableBinding,
                    CompileCommand::PushScope(scope_ref!(2)),
                    CompileCommand::Reference(symbol!(reg, "a"), locator!(local: 1)),
                    CompileCommand::Undefined,
                    CompileCommand::MutableBinding,
                    CompileCommand::PopScope(scope_ref!(2)),
                    CompileCommand::PushScope(scope_ref!(3)),
                    CompileCommand::Reference(symbol!(reg, "a"), locator!(local: 2)),
                    CompileCommand::Undefined,
                    CompileCommand::MutableBinding,
                    CompileCommand::Reference(symbol!(reg, "b"), locator!(local: 3)),
                    CompileCommand::Undefined,
                    CompileCommand::MutableBinding,
                    CompileCommand::PopScope(scope_ref!(3)),
                    CompileCommand::PopScope(scope_ref!(1)),
                ]
            );
        });
    }

    #[test]
    fn test_binary_operator() {
        test("1 + 2", |_reg, program| {
            assert_eq!(
                program.functions[0].commands,
                [
                    CompileCommand::Nop,
                    CompileCommand::PushScope(scope_ref!(1)),
                    CompileCommand::Number(1.0),
                    CompileCommand::Number(2.0),
                    CompileCommand::Swap,
                    CompileCommand::Addition,
                    CompileCommand::Discard,
                    CompileCommand::PopScope(scope_ref!(1)),
                ]
            );
        });
    }

    #[test]
    fn test_shorthand_assignment_operator() {
        test("let a = 1; a += 2", |reg, program| {
            assert_eq!(
                program.functions[0].commands,
                [
                    CompileCommand::AllocateLocals(1),
                    CompileCommand::PushScope(scope_ref!(1)),
                    CompileCommand::Reference(symbol!(reg, "a"), locator!(local: 0)),
                    CompileCommand::Number(1.0),
                    CompileCommand::MutableBinding,
                    CompileCommand::Reference(symbol!(reg, "a"), locator!(local: 0)),
                    CompileCommand::Number(2.0),
                    CompileCommand::Duplicate(1),
                    CompileCommand::Addition,
                    CompileCommand::Assignment,
                    CompileCommand::Discard,
                    CompileCommand::PopScope(scope_ref!(1)),
                ]
            );
        });
    }

    fn test(regc: &str, validate: fn(symbol_registry: &SymbolRegistry, program: &Program)) {
        let runtime_pref = RuntimePref {
            enable_scope_cleanup_checker: true,
            ..Default::default()
        };
        let mut symbol_registry = Default::default();
        let mut function_registry = FunctionRegistry::new();
        let result = Parser::for_script(
            regc,
            Processor::new(
                Analyzer::new(&runtime_pref, &mut symbol_registry, &mut function_registry),
                false,
            ),
        )
        .parse();
        assert!(result.is_ok());
        if let Ok(program) = result {
            validate(&symbol_registry, &program)
        }
    }
}
