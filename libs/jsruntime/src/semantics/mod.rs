mod scope;

use jsparser::syntax::AssignmentOperator;
use jsparser::syntax::BinaryOperator;
use jsparser::syntax::LoopFlags;
use jsparser::syntax::Node;
use jsparser::syntax::NodeHandler;
use jsparser::syntax::UnaryOperator;
use jsparser::syntax::UpdateOperator;
use jsparser::Error;
use jsparser::Symbol;
use jsparser::SymbolRegistry;

use super::bridge::Locator;
use super::logger;
use super::FunctionId;
use super::FunctionRegistry;

use scope::BindingKind;
use scope::ScopeKind;
use scope::ScopeManager;
use scope::ScopeRef;

/// A type representing a JavaScript program after the semantic analysis.
pub struct Program {
    pub functions: Vec<FunctionRecipe>,
}

/// A type representing a JavaScript function after the semantic analysis.
pub struct FunctionRecipe {
    pub symbol: Symbol,
    pub id: FunctionId,
    pub commands: Vec<CompileCommand>,
}

/// A semantic analyzer.
///
/// A semantic analyzer analyzes semantics of a JavaScript program.
pub struct Analyzer<'r> {
    /// A mutable reference to a symbol registry.
    symbol_registry: &'r mut SymbolRegistry,

    /// A mutable reference to a function registry.
    function_registry: &'r mut FunctionRegistry,

    /// A stack to keep the analysis data for outer JavaScript functions when analyzing nested
    /// JavaScript functions.
    context_stack: Vec<FunctionContext>,

    /// A list of [`FunctionRecipe`]s.
    functions: Vec<FunctionRecipe>,

    /// A scope manager used for building the scope tree of the JavaScript program.
    scope_manager: ScopeManager,

    /// Holds references in a JavaScript program.
    ///
    /// All of the references are resolved in [`Analyzer::accept()`] at once.
    references: Vec<Reference>,

    use_global_bindings: bool,
}

impl<'r> Analyzer<'r> {
    /// Creates a semantic analyzer.
    pub fn new(
        symbol_registry: &'r mut SymbolRegistry,
        function_registry: &'r mut FunctionRegistry,
    ) -> Self {
        let id = function_registry.create_native_function();
        Self {
            symbol_registry,
            function_registry,
            context_stack: vec![FunctionContext {
                in_body: true,
                ..Default::default()
            }],
            functions: vec![FunctionRecipe {
                symbol: Symbol::NONE,
                id,
                commands: vec![],
            }],
            scope_manager: Default::default(),
            references: vec![],
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
            Node::UpdateExpression(op) => self.handle_operator(op),
            Node::UnaryExpression(op) => self.handle_operator(op),
            Node::BinaryExpression(op) => self.handle_operator(op),
            Node::LogicalExpression(_op) => self.handle_conditional_expression(),
            Node::ConditionalExpression => self.handle_conditional_expression(),
            Node::AssignmentExpression(AssignmentOperator::LogicalAndAssignment) => {
                self.handle_conditional_assignment()
            }
            Node::AssignmentExpression(AssignmentOperator::LogicalOrAssignment) => {
                self.handle_conditional_assignment()
            }
            Node::AssignmentExpression(AssignmentOperator::NullishCoalescingAssignment) => {
                self.handle_conditional_assignment()
            }
            Node::AssignmentExpression(op) => self.handle_operator(op),
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
        let context = self.context_stack.last_mut().unwrap();
        // The locator will be updated later.
        let command_index = context.put_command(CompileCommand::Reference(symbol, Locator::NONE));
        self.references.push(Reference {
            symbol,
            scope_ref: self.scope_manager.current(),
            command_locator: (context.func_index, command_index),
        });
    }

    fn handle_binding_identifier(&mut self, symbol: Symbol) {
        let context = self.context_stack.last_mut().unwrap();
        if context.in_body {
            // The locator will be updated later.
            let command_index =
                context.put_command(CompileCommand::Reference(symbol, Locator::NONE));
            self.scope_manager.add_binding(symbol, BindingKind::Mutable);
            self.references.push(Reference {
                symbol,
                scope_ref: self.scope_manager.current(),
                command_locator: (context.func_index, command_index),
            })
        } else {
            // TODO: the compilation should fail if the following condition is unmet.
            assert!(context.formal_parameters.len() < u16::MAX as usize);
            let i = context.formal_parameters.len();
            context.formal_parameters.push(symbol);
            self.scope_manager
                .add_binding(symbol, BindingKind::FormalParameter(i));
        }
    }

    fn handle_argument_list_head(&mut self, empty: bool, spread: bool) {
        self.context_stack
            .last_mut()
            .unwrap()
            .put_arguments(empty, spread)
    }

    fn handle_argument_list_item(&mut self, spread: bool) {
        self.context_stack.last_mut().unwrap().put_argument(spread);
    }

    fn handle_call_expression(&mut self) {
        self.context_stack.last_mut().unwrap().put_call();
    }

    fn handle_operator<T>(&mut self, op: T)
    where
        T: Into<CompileCommand>,
    {
        self.context_stack
            .last_mut()
            .unwrap()
            .put_command(op.into());
    }

    fn handle_sequence_expression(&mut self) {
        self.context_stack
            .last_mut()
            .unwrap()
            .process_sequence_expression();
    }

    fn handle_conditional_expression(&mut self) {
        self.context_stack
            .last_mut()
            .unwrap()
            .put_command(CompileCommand::ConditionalTernary);
    }

    fn handle_conditional_assignment(&mut self) {
        let context = self.context_stack.last_mut().unwrap();
        context.put_command(CompileCommand::ConditionalTernary);
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
        self.scope_manager.set_immutable(n);
        self.context_stack
            .last_mut()
            .unwrap()
            .process_immutable_bindings(n);
    }

    fn handle_binding_element(&mut self, _init: bool) {
        // TODO
    }

    fn handle_expression_statement(&mut self) {
        self.context_stack
            .last_mut()
            .unwrap()
            .put_command(CompileCommand::Discard);
    }

    fn handle_if_else_statement(&mut self) {
        self.context_stack
            .last_mut()
            .unwrap()
            .put_command(CompileCommand::IfElseStatement);
    }

    fn handle_if_statement(&mut self) {
        self.context_stack
            .last_mut()
            .unwrap()
            .put_command(CompileCommand::IfStatement);
    }

    fn handle_do_while_statement(&mut self) {
        // See handle_loop_start() for the reason why we always pop the lexical scope here.
        self.scope_manager.pop();
        self.context_stack
            .last_mut()
            .unwrap()
            .process_do_while_statement();
    }

    fn handle_while_statement(&mut self) {
        // See handle_loop_start() for the reason why we always pop the lexical scope here.
        self.scope_manager.pop();
        self.context_stack
            .last_mut()
            .unwrap()
            .process_while_statement();
    }

    fn handle_for_statement(&mut self, flags: LoopFlags) {
        // See handle_loop_start() for the reason why we always pop the lexical scope here.
        self.scope_manager.pop();
        self.context_stack
            .last_mut()
            .unwrap()
            .process_for_statement(flags);
    }

    fn handle_continue_statement(&mut self, symbol: Symbol) {
        self.context_stack
            .last_mut()
            .unwrap()
            .put_command(CompileCommand::Continue(symbol));
    }

    fn handle_break_statement(&mut self, symbol: Symbol) {
        self.context_stack
            .last_mut()
            .unwrap()
            .put_command(CompileCommand::Break(symbol));
    }

    fn handle_return_statement(&mut self, n: u32) {
        self.context_stack
            .last_mut()
            .unwrap()
            .put_command(CompileCommand::Return(n));
    }

    fn handle_switch_statement(&mut self) {
        self.context_stack
            .last_mut()
            .unwrap()
            .process_switch_statement();
    }

    fn handle_case_block(&mut self) {
        self.context_stack.last_mut().unwrap().process_case_block();
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
        self.context_stack.last_mut().unwrap().process_catch_block();
    }

    fn handle_finally_block(&mut self) {
        self.context_stack
            .last_mut()
            .unwrap()
            .process_finally_block();
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
        self.scope_manager.pop();

        let mut context = self.context_stack.pop().unwrap();
        context.end_scope(true);
        // TODO: remaining references must be handled as var bindings w/ undefined value.
        context.commands[0] = CompileCommand::Bindings(context.max_bindings as u16);
        let func_index = context.func_index;
        self.functions[func_index].commands = context.commands;

        self.context_stack
            .last_mut()
            .unwrap()
            .process_function_declaration(self.functions[func_index].id);
    }

    // TODO: reduce code clone took from handle_function_declaration().
    fn handle_function_expression(&mut self, named: bool) {
        self.scope_manager.pop();

        let mut context = self.context_stack.pop().unwrap();
        context.end_scope(true);
        // TODO: remaining references must be handled as var bindings w/ undefined value.
        context.commands[0] = CompileCommand::Bindings(context.max_bindings as u16);
        let func_index = context.func_index;
        self.functions[func_index].commands = context.commands;

        self.context_stack
            .last_mut()
            .unwrap()
            .process_function_expression(self.functions[func_index].id, named);
    }

    fn handle_arrow_function(&mut self) {
        // TODO: An ArrowFunction does not define local bindings for arguments, super, this, or
        // new.target.  Any reference to arguments, super, this, or new.target within an
        // ArrowFunction must resolve to a binding in a lexically enclosing environment.

        self.scope_manager.pop();

        let mut context = self.context_stack.pop().unwrap();
        context.end_scope(true);
        // TODO: remaining references must be handled as var bindings w/ undefined value.
        context.commands[0] = CompileCommand::Bindings(context.max_bindings as u16);
        let func_index = context.func_index;
        self.functions[func_index].commands = context.commands;

        self.context_stack
            .last_mut()
            .unwrap()
            .process_function_expression(self.functions[func_index].id, false);
    }

    fn handle_then_block(&mut self) {
        let context = self.context_stack.last_mut().unwrap();
        context.put_command(CompileCommand::Truthy);
        context.put_command(CompileCommand::Then);
    }

    fn handle_else_block(&mut self) {
        self.context_stack
            .last_mut()
            .unwrap()
            .put_command(CompileCommand::Else);
    }

    fn handle_falsy_short_circuit(&mut self) {
        self.context_stack
            .last_mut()
            .unwrap()
            .put_command(CompileCommand::FalsyShortCircuit);
    }

    fn handle_truthy_short_circuit(&mut self) {
        self.context_stack
            .last_mut()
            .unwrap()
            .put_command(CompileCommand::TruthyShortCircuit);
    }

    fn handle_nullish_short_circuit(&mut self) {
        self.context_stack
            .last_mut()
            .unwrap()
            .put_command(CompileCommand::NullishShortCircuit);
    }

    fn handle_falsy_short_circuit_assignment(&mut self) {
        self.context_stack
            .last_mut()
            .unwrap()
            .put_command(CompileCommand::FalsyShortCircuitAssignment);
    }

    fn handle_truthy_short_circuit_assignment(&mut self) {
        self.context_stack
            .last_mut()
            .unwrap()
            .put_command(CompileCommand::TruthyShortCircuitAssignment);
    }

    fn handle_nullish_short_circuit_assignment(&mut self) {
        self.context_stack
            .last_mut()
            .unwrap()
            .put_command(CompileCommand::NullishShortCircuitAssignment);
    }

    fn handle_loop_start(&mut self) {
        self.context_stack.last_mut().unwrap().process_loop_start();
        self.scope_manager.push(ScopeKind::Block);
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
        self.context_stack.last_mut().unwrap().start_scope();
        self.scope_manager.push(ScopeKind::Block);
    }

    fn handle_end_block_scope(&mut self) {
        self.scope_manager.pop();
        self.context_stack.last_mut().unwrap().end_scope(false);
    }

    fn handle_function_context(&mut self) {
        // TODO: the compilation should fail if the following condition is unmet.
        assert!(self.functions.len() < u32::MAX as usize);
        let func_index = self.functions.len();
        let mut context = FunctionContext {
            func_index,
            commands: vec![CompileCommand::Nop],
            ..Default::default()
        };
        context.start_scope();
        self.context_stack.push(context);
        // Push a placeholder data which will be filled later.
        self.functions.push(FunctionRecipe {
            symbol: Symbol::NONE,
            id: FunctionId::MAIN,
            commands: vec![],
        });
        self.scope_manager.push(ScopeKind::Function);
    }

    fn handle_function_signature(&mut self, symbol: Symbol) {
        let context = self.context_stack.last_mut().unwrap();
        let id = self.function_registry.create_native_function();
        let func_index = context.func_index;
        self.functions[func_index].symbol = symbol;
        self.functions[func_index].id = id;
        context.in_body = true;
    }

    // TODO: global object
    fn put_global_bindings(&mut self) {
        let context = self.context_stack.last_mut().unwrap();

        // Register `undefined`.
        let symbol = Symbol::UNDEFINED;
        // The locator will be computed in `resolve_locators()`.
        let command_index = context.put_command(CompileCommand::Reference(symbol, Locator::NONE));
        context.put_lexical_binding(false);
        context.process_immutable_bindings(1);
        self.scope_manager
            .add_binding(symbol, BindingKind::Immutable);
        self.references.push(Reference {
            symbol,
            scope_ref: self.scope_manager.current(),
            command_locator: (context.func_index, command_index),
        });

        // Register `Infinity`.
        let symbol = Symbol::INFINITY;
        // The locator will be computed in `resolve_locators()`.
        let command_index = context.put_command(CompileCommand::Reference(symbol, Locator::NONE));
        context.put_number(f64::INFINITY);
        context.put_lexical_binding(true);
        context.process_immutable_bindings(1);
        self.scope_manager
            .add_binding(symbol, BindingKind::Immutable);
        self.references.push(Reference {
            symbol,
            scope_ref: self.scope_manager.current(),
            command_locator: (context.func_index, command_index),
        });

        // Register `NaN`.
        let symbol = Symbol::NAN;
        // The locator will be computed in `resolve_locators()`.
        let command_index = context.put_command(CompileCommand::Reference(symbol, Locator::NONE));
        context.put_number(f64::NAN);
        context.put_lexical_binding(true);
        context.process_immutable_bindings(1);
        self.scope_manager
            .add_binding(symbol, BindingKind::Immutable);
        self.references.push(Reference {
            symbol,
            scope_ref: self.scope_manager.current(),
            command_locator: (context.func_index, command_index),
        });
    }

    // TODO: global object
    fn register_host_functions(&mut self) {
        let context = self.context_stack.last_mut().unwrap();

        for (func_id, host_func) in self.function_registry.enumerate_host_function() {
            let symbol = self.symbol_registry.intern_cstr(&host_func.name);
            // The locator will be computed in `resolve_locators()`.
            let command_index =
                context.put_command(CompileCommand::Reference(symbol, Locator::NONE));
            context.process_function_declaration(func_id);
            self.scope_manager
                .add_binding(symbol, BindingKind::Immutable);
            self.references.push(Reference {
                symbol,
                scope_ref: self.scope_manager.current(),
                command_locator: (context.func_index, command_index),
            });
        }
    }

    fn resolve_locators(&mut self) {
        for reference in self.references.iter() {
            let locator = self.scope_manager.compute_locator(reference);
            logger::debug!(event = "resolve-locator", ?reference.symbol, ?locator);
            let (func_index, command_index) = reference.command_locator;
            self.functions[func_index].commands[command_index] =
                CompileCommand::Reference(reference.symbol, locator);
        }
    }
}

impl<'r, 's> NodeHandler<'s> for Analyzer<'r> {
    type Artifact = Program;

    fn start(&mut self) {
        logger::debug!(event = "start");
        let context = self.context_stack.last_mut().unwrap();
        // Push `Nop` as a placeholder.
        // It will be replaced with `Bindings(n)` in `accept()`.
        context.commands.push(CompileCommand::Nop);
        context.start_scope();
        self.scope_manager.push(ScopeKind::Function);

        if self.use_global_bindings {
            self.put_global_bindings();
        }

        self.register_host_functions();
    }

    fn accept(&mut self) -> Result<Self::Artifact, Error> {
        logger::debug!(event = "accept");
        let mut context = self.context_stack.pop().unwrap();

        self.scope_manager.pop();
        //self.scope_manager.dump(ScopeRef(1));
        context.end_scope(true);

        // TODO: remaining references must be handled as var bindings w/ undefined value.
        context.commands[0] = CompileCommand::Bindings(context.max_bindings as u16);
        context.commands.push(CompileCommand::Return(0));
        self.functions[context.func_index].commands = context.commands;
        self.resolve_locators();
        Ok(Program {
            functions: std::mem::take(&mut self.functions),
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

    /// A list of indexes of commands that have to be updated while analyzing.
    pending_lexical_bindings: Vec<usize>,

    /// A list of symbols in the formal parameters of this function.
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

    /// A variable to hold the current maximum number of bindings in this function.
    max_bindings: usize,

    /// The index of this function in [`Analyzer::functions`].
    func_index: usize,

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

    fn put_arguments(&mut self, empty: bool, _spread: bool) {
        // TODO: spread
        let index = self.put_command(CompileCommand::Nop);
        let nargs = if empty {
            0
        } else {
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

    fn put_call(&mut self) {
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
        self.scope_stack.last_mut().unwrap().num_bindings += self.pending_lexical_bindings.len();
        self.pending_lexical_bindings.clear();
    }

    fn process_immutable_bindings(&mut self, n: u32) {
        debug_assert_eq!(n as usize, self.pending_lexical_bindings.len());
        for i in self.pending_lexical_bindings.iter().cloned() {
            debug_assert!(matches!(self.commands[i], CompileCommand::Nop));
            self.commands[i] = CompileCommand::ImmutableBinding;
        }
        self.scope_stack.last_mut().unwrap().num_bindings += self.pending_lexical_bindings.len();
        self.pending_lexical_bindings.clear();
    }

    fn process_function_declaration(&mut self, func_id: FunctionId) {
        self.commands.push(CompileCommand::Function(func_id));
        self.commands.push(CompileCommand::DeclareFunction);
        self.scope_stack.last_mut().unwrap().num_bindings += 1;
    }

    fn process_function_expression(&mut self, func_id: FunctionId, named: bool) {
        if named {
            // Remove the BindingIdentifier of the function.
            self.put_command(CompileCommand::Discard);
        }
        self.commands.push(CompileCommand::Function(func_id));
    }

    fn process_loop_start(&mut self) {
        // Push `Nop` as a placeholder.
        // It will be replaced with an appropriate command in process_loop_end().
        let start_index = self.put_command(CompileCommand::Nop);
        self.loop_stack.push(LoopContext { start_index });
        // NOTE: This doesn't follow the specification, but we create a new lexical scope for the
        // iteration statement.  This is needed for the for-let/const statements, but not for
        // others.  We believe that this change does not compromise conformance to the
        // specification and does not cause security problems.
        self.start_scope();
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
        self.end_scope(false);
        self.put_command(CompileCommand::LoopEnd);
        let LoopContext { start_index } = self.loop_stack.pop().unwrap();
        self.commands[start_index] = command;
    }

    fn process_do_while_statement(&mut self) {
        self.put_command(CompileCommand::LoopTest);
        self.process_loop_end(CompileCommand::DoWhileLoop);
    }

    fn process_while_statement(&mut self) {
        self.put_command(CompileCommand::LoopBody);
        self.process_loop_end(CompileCommand::WhileLoop);
    }

    fn process_for_statement(&mut self, flags: LoopFlags) {
        self.put_command(CompileCommand::LoopBody);
        self.process_loop_end(CompileCommand::ForLoop(flags));
    }

    fn process_case_block(&mut self) {
        let case_block_index = self.put_command(CompileCommand::CaseBlock(0));
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
        self.switch_stack.last_mut().unwrap().num_clauses += 1;
    }

    fn process_default_selector(&mut self) {
        self.put_command(CompileCommand::Discard);
        // TODO: refactoring
        self.put_command(CompileCommand::Then);
    }

    fn process_default_clause(&mut self, has_statement: bool) {
        self.put_command(CompileCommand::DefaultClause(has_statement));
        let context = self.switch_stack.last_mut().unwrap();
        context.default_index = Some(context.num_clauses);
        context.num_clauses += 1;
    }

    fn process_switch_statement(&mut self) {
        let context = self.switch_stack.pop().unwrap();

        // TODO: the compilation should fail if the following condition is unmet.
        assert!(context.num_clauses <= u32::MAX as usize);
        let n = context.num_clauses as u32;

        if n == 0 {
            // empty case block
            // Discard the `switchValue`.
            self.commands[context.case_block_index] = CompileCommand::Discard;
        } else {
            self.commands[context.case_block_index] = CompileCommand::CaseBlock(n);
            let default_index = context.default_index.map(|i| i as u32);
            self.put_command(CompileCommand::Switch(n, default_index));
        }
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

    fn process_catch_block(&mut self) {
        // Push a *nominal* `Catch` command.
        // It will be replaced with a *substantial* `Catch` command in `process_catch_clause()`
        // if a catch clause exists.
        let index = self.put_command(CompileCommand::Catch(true));
        self.try_stack.last_mut().unwrap().catch_index = index;
        // In the specification, a new lexical scope is created only when the catch parameter
        // exists, but we always create a scope here for simplicity.  In our processing model,
        // the catch and finally clauses are always created even if there is no corresponding
        // node in the AST.
        self.start_scope();
    }

    fn process_catch_clause(&mut self, _has_parameter: bool) {
        let index = self.try_stack.last().unwrap().catch_index;
        self.commands[index] = CompileCommand::Catch(false); // substantial
    }

    fn process_finally_block(&mut self) {
        // Remove the scope created for the catch clause.
        self.end_scope(false);
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

    fn start_scope(&mut self) {
        // Push `Nop` as a placeholder.
        // It will be replaced with `AllocateBindings(n)` in `end_scope()`.
        let index = self.put_command(CompileCommand::Nop);
        self.scope_stack.push(Scope {
            command_base_index: index,
            num_bindings: 0,
            max_child_bindings: 0,
        });
    }

    fn end_scope(&mut self, function_scope: bool) {
        let scope = self.scope_stack.pop().unwrap();

        // TODO: the compilation should fail if the following conditions are unmet.
        assert!(scope.num_bindings <= ScopeManager::MAX_LOCAL_BINDINGS);
        assert!(scope.num_bindings + scope.max_child_bindings <= ScopeManager::MAX_LOCAL_BINDINGS);

        let n = scope.num_bindings as u16;
        if n > 0 {
            self.commands[scope.command_base_index] =
                CompileCommand::AllocateBindings(n, function_scope);
            self.commands.push(CompileCommand::ReleaseBindings(n));
        }

        let n = scope.num_bindings + scope.max_child_bindings;
        match self.scope_stack.last_mut() {
            Some(scope) => scope.max_child_bindings = scope.max_child_bindings.max(n),
            None => self.max_bindings = n,
        }
    }
}

struct Scope {
    command_base_index: usize,
    num_bindings: usize,
    max_child_bindings: usize,
}

struct LoopContext {
    start_index: usize,
}

#[derive(Default)]
struct SwitchContext {
    case_block_index: usize,
    num_clauses: usize,
    default_index: Option<usize>,
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
    Reference(Symbol, Locator),
    Exception,

    Bindings(u16),
    MutableBinding,
    ImmutableBinding,
    DeclareFunction,
    Arguments(u16),
    Argument(u16),
    Call(u16),
    AllocateBindings(u16, bool),
    ReleaseBindings(u16),

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

    // conditional operator
    ConditionalTernary,

    // assignment operators
    Assignment,
    MultiplicationAssignment,
    DivisionAssignment,
    RemainderAssignment,
    AdditionAssignment,
    SubtractionAssignment,
    LeftShiftAssignment,
    SignedRightShiftAssignment,
    UnsignedRightShiftAssignment,
    BitwiseAndAssignment,
    BitwiseXorAssignment,
    BitwiseOrAssignment,
    ExponentiationAssignment,

    // short-circuit
    FalsyShortCircuit,
    TruthyShortCircuit,
    NullishShortCircuit,
    FalsyShortCircuitAssignment,
    TruthyShortCircuitAssignment,
    NullishShortCircuitAssignment,

    // conditional
    Truthy,
    Then,
    Else,
    IfElseStatement,
    IfStatement,

    // loop
    WhileLoop,
    DoWhileLoop,
    ForLoop(LoopFlags),
    LoopInit,
    LoopTest,
    LoopNext,
    LoopBody,
    LoopEnd,

    // switch
    CaseBlock(u32),
    CaseClause(bool),
    DefaultClause(bool),
    Switch(u32, Option<u32>),

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

impl From<AssignmentOperator> for CompileCommand {
    fn from(value: AssignmentOperator) -> Self {
        match value {
            AssignmentOperator::Assignment => Self::Assignment,
            AssignmentOperator::MultiplicationAssignment => Self::MultiplicationAssignment,
            AssignmentOperator::DivisionAssignment => Self::DivisionAssignment,
            AssignmentOperator::RemainderAssignment => Self::RemainderAssignment,
            AssignmentOperator::AdditionAssignment => Self::AdditionAssignment,
            AssignmentOperator::SubtractionAssignment => Self::SubtractionAssignment,
            AssignmentOperator::LeftShiftAssignment => Self::LeftShiftAssignment,
            AssignmentOperator::SignedRightShiftAssignment => Self::SignedRightShiftAssignment,
            AssignmentOperator::UnsignedRightShiftAssignment => Self::UnsignedRightShiftAssignment,
            AssignmentOperator::BitwiseAndAssignment => Self::BitwiseAndAssignment,
            AssignmentOperator::BitwiseXorAssignment => Self::BitwiseXorAssignment,
            AssignmentOperator::BitwiseOrAssignment => Self::BitwiseOrAssignment,
            AssignmentOperator::ExponentiationAssignment => Self::ExponentiationAssignment,
            // There is no corresponding command for `&&=`, `||=` and `??=`.
            // These are converted into the corresponding conditional expression for short-circuit
            // evaluation of the LHS.
            _ => unreachable!(),
        }
    }
}

/// A type representing information needed for resolving a reference to a symbol.
#[derive(Debug)]
struct Reference {
    /// The symbol referred in this reference.
    symbol: Symbol,

    /// The reference to the current (function or block) scope when this reference happens.
    scope_ref: ScopeRef,

    /// Indicates a location of the [`CompileCommand::Reference`] command for this reference.
    ///
    /// This is a tuple of two indexes.  The first one is the index of a function where this
    /// reference happens.  The second one is the index of the command in
    /// [`FunctionContext::commands`].
    ///
    /// This variable is used to update the locator field in the command once the locator is
    /// resolved.  When a function uses free variables, locators of the free variables are resolved
    /// outside the function scope of the function.
    command_locator: (usize, usize),
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

    macro_rules! local {
        ($index:expr) => {
            local!(0, $index)
        };
        ($func:expr, $index:expr) => {
            Locator::local($func, $index)
        };
    }

    #[test]
    fn test_lexical_declarations() {
        test("let a, b = 2; const c = 3, d = 4;", |reg, program| {
            assert_eq!(
                program.functions[0].commands,
                [
                    CompileCommand::Bindings(4),
                    CompileCommand::AllocateBindings(4, true),
                    CompileCommand::Reference(symbol!(reg, "a"), local!(0)),
                    CompileCommand::Undefined,
                    CompileCommand::MutableBinding,
                    CompileCommand::Reference(symbol!(reg, "b"), local!(1)),
                    CompileCommand::Number(2.0),
                    CompileCommand::MutableBinding,
                    CompileCommand::Reference(symbol!(reg, "c"), local!(2)),
                    CompileCommand::Number(3.0),
                    CompileCommand::ImmutableBinding,
                    CompileCommand::Reference(symbol!(reg, "d"), local!(3)),
                    CompileCommand::Number(4.0),
                    CompileCommand::ImmutableBinding,
                    CompileCommand::ReleaseBindings(4),
                    CompileCommand::Return(0),
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
                    CompileCommand::Bindings(3),
                    CompileCommand::AllocateBindings(1, true),
                    CompileCommand::Reference(symbol!(reg, "a"), local!(0)),
                    CompileCommand::Undefined,
                    CompileCommand::MutableBinding,
                    CompileCommand::AllocateBindings(1, false),
                    CompileCommand::Reference(symbol!(reg, "a"), local!(1)),
                    CompileCommand::Undefined,
                    CompileCommand::MutableBinding,
                    CompileCommand::ReleaseBindings(1),
                    CompileCommand::AllocateBindings(2, false),
                    CompileCommand::Reference(symbol!(reg, "a"), local!(1)),
                    CompileCommand::Undefined,
                    CompileCommand::MutableBinding,
                    CompileCommand::Reference(symbol!(reg, "b"), local!(2)),
                    CompileCommand::Undefined,
                    CompileCommand::MutableBinding,
                    CompileCommand::ReleaseBindings(2),
                    CompileCommand::ReleaseBindings(1),
                    CompileCommand::Return(0),
                ]
            );
        });
    }

    fn test(regc: &str, validate: fn(symbol_registry: &SymbolRegistry, program: &Program)) {
        let mut symbol_registry = Default::default();
        let mut function_registry = FunctionRegistry::new();
        let result = Parser::for_script(
            regc,
            Processor::new(
                Analyzer::new(&mut symbol_registry, &mut function_registry),
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
