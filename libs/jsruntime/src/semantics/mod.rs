mod scope;

use bitflags::bitflags;
use jsparser::syntax::LiteralPropertyName;
use jsparser::syntax::MemberExpressionKind;
use jsparser::syntax::PropertyAccessKind;
use jsparser::syntax::PropertyDefinitionKind;
use rustc_hash::FxHashMap;
use rustc_hash::FxHashSet;

use jsparser::Error;
use jsparser::Parser;
use jsparser::Processor;
use jsparser::Symbol;
use jsparser::syntax::AssignmentOperator;
use jsparser::syntax::BinaryOperator;
use jsparser::syntax::LoopFlags;
use jsparser::syntax::Node;
use jsparser::syntax::NodeHandler;
use jsparser::syntax::UnaryOperator;
use jsparser::syntax::UpdateOperator;

use crate::ProgramId;
use crate::Runtime;
use crate::Value;
use crate::lambda::LambdaId;
use crate::lambda::LambdaKind;
use crate::logger;
use crate::objects::Property;

use scope::ScopeTreeBuilder;

pub use scope::ScopeRef;
pub use scope::ScopeTree;
pub use scope::VariableRef;

impl<X> Runtime<X> {
    /// Parses a given source text as a script.
    pub fn parse_script(&mut self, source: &str) -> Result<ProgramId, Error> {
        logger::debug!(event = "parse", source_kind = "script");
        let analyzer = Analyzer::new_for_script(self);
        let processor = Processor::new(analyzer, false);
        let program = Parser::for_script(source, processor).parse()?;
        Ok(self.register_program(program))
    }

    /// Parses a given source text as a module.
    pub fn parse_module(&mut self, source: &str) -> Result<ProgramId, Error> {
        logger::debug!(event = "parse", source_kind = "module");
        let analyzer = Analyzer::new_for_module(self);
        let processor = Processor::new(analyzer, true);
        let program = Parser::for_module(source, processor).parse()?;
        Ok(self.register_program(program))
    }

    fn register_program(&mut self, program: Program) -> ProgramId {
        let index = self.programs.len();
        let program_id = ProgramId::new(index);
        for (index, function) in program.functions.iter().enumerate() {
            let lambda_info = self.lambda_registry.get_mut(function.id);
            debug_assert_eq!(lambda_info.program_id, ProgramId::INVALID);
            lambda_info.program_id = program_id;
            debug_assert!(index < u32::MAX as usize);
            debug_assert_eq!(lambda_info.function_index, u32::MAX);
            lambda_info.function_index = index as u32;
        }
        self.programs.push(program);
        program_id
    }

    /// Prints functions in a program.
    pub fn print_functions(&self, program_id: ProgramId) {
        for func in self.programs[program_id.index()].functions.iter() {
            func.print("");
        }
    }

    /// Prints the scope tree of a program.
    pub fn print_scope_tree(&self, program_id: ProgramId) {
        self.programs[program_id.index()]
            .scope_tree
            .print(&self.symbol_registry, "");
    }

    /// Prints global symbols in a program.
    pub fn print_global_symbols(&self, program_id: ProgramId) {
        for symbol in self.programs[program_id.index()]
            .global_symbols
            .iter()
            .cloned()
        {
            // TODO: sort
            let utf16_str = self.symbol_registry.resolve(symbol).unwrap();
            let utf8_str = String::from_utf16_lossy(utf16_str);
            println!("{utf8_str}{symbol}");
        }
    }
}

/// A type representing a JavaScript program after the semantic analysis.
///
/// The program mainly consists of two kind of data.
///
/// 1. Compile commands for each JavaScript function
/// 2. Analysis data
///
/// Some of synthesized attributes (known as S-attributes) are computed during the semantic
/// analysis and values are embedded in each compile command.  The others will be computed in a
/// state machine that interprets the compile commands.
///
/// Inherited attributes are computed and stored into the analysis data.  And the values will be
/// used in the state machine.
///
/// In our processing model, a parser outputs stream of Nodes in the syntax tree in the buttom-up
/// order and it doesn't create the AST.  So, it's impossible to compute a inherited attribute
/// value before a parent node comes from the parser.  The computation has to be postponed.  This
/// is why we need to introduce the analysis data.
pub struct Program {
    /// Functions in the program.
    ///
    /// The functions are stored in post-order traversal on the function tree where the entry
    /// function is the root of the function tree.
    pub functions: Vec<Function>,

    /// The scope tree of the program.
    pub scope_tree: ScopeTree,

    /// The global variables used in the program.
    pub global_symbols: FxHashSet<Symbol>,

    /// `true` if the program is a JavaScript module.
    pub module: bool,
}

impl Program {
    pub fn entry_lambda_id(&self) -> LambdaId {
        // The entry function is always placed at the last.
        self.functions.last().unwrap().id
    }
}

/// A type representing a JavaScript function after the semantic analysis.
pub struct Function {
    // TODO: remove?
    pub name: Symbol,

    /// The function ID of the function.
    pub id: LambdaId,

    /// A list of [`CompileCommand`]s generated from the function definition.
    pub commands: Vec<CompileCommand>,

    /// The reference to the function scope.
    pub scope_ref: ScopeRef,

    /// The number of outer variables that the function captures.
    pub num_captures: u16,

    /// The number of formal parameters of the function.
    pub num_params: u16,

    /// The number of local variables used in the function except for temporal variables created by
    /// a compiler.
    pub num_locals: u16,

    /// Controls how the `this` binding is resolved.
    pub this_binding: ThisBinding,

    /// Flags.
    flags: FunctionFlags,
}

#[derive(Debug)]
pub enum ThisBinding {
    /// No `this` binding is used in the function body.
    None,

    /// The `this` binding in the function body is resolved to the `this` argument of the lambda
    /// function.
    ThisArgument,

    /// The `this` binding in the function body captures the `this` binding of the enclosing outer
    /// function scope.
    Capture,

    /// The `this` binding is always resolved to the global object even if the `this` argument of
    /// the lambda function is not null-ish.
    GlobalObject,

    /// The `this` binding in the function body is resolved to the global object if the `this`
    /// argument of the lambda function is null-ish.  Otherwise, it's resolved to the result of
    /// `ToObject(thisArgument)`.
    Quirk,
}

bitflags! {
    #[derive(Debug)]
    struct FunctionFlags: u8 {
        /// The `this` binding is captured by descendant closures.
        const THIS_BINDING_CAPTURED = 1 << 0;
    }
}

impl Function {
    /// Returns `true` if the `this` binding is captured.
    pub fn is_this_binding_captured(&self) -> bool {
        self.flags.contains(FunctionFlags::THIS_BINDING_CAPTURED)
    }

    pub fn print(&self, indent: &str) {
        println!("{indent}function: name={:?} id={:?}", self.name, self.id);
        if !self.commands.is_empty() {
            println!("{indent} commands:");
            for command in self.commands.iter() {
                println!("{indent}  {command:?}");
            }
        }
        println!("{indent} num_captures: {}", self.num_captures);
        println!("{indent} num_params: {}", self.num_params);
        println!("{indent} num_locals: {}", self.num_locals);
        println!("{indent} this_binding: {:?}", self.this_binding);
        println!("{indent} flags: {:?}", self.flags);
    }
}

/// A semantic analyzer.
///
/// A semantic analyzer analyzes semantics of a JavaScript program.
struct Analyzer<'r, R> {
    support: &'r mut R,

    /// Global analysis data.
    global_analysis: GlobalAnalysis,

    /// A stack to keep the analysis data for outer JavaScript functions when analyzing nested
    /// JavaScript functions.
    analysis_stack: Vec<FunctionAnalysis>,

    /// A list of [`Function`]s.
    functions: Vec<Function>,

    module: bool,
}

trait AnalyzerSupport {
    fn make_symbol(&mut self, lexeme: &str) -> Symbol;
    fn register_lambda(&mut self, kind: LambdaKind) -> LambdaId;
    fn define_global_property(&mut self, key: Symbol, property: Property) -> Result<bool, Value>;
}

impl<X> AnalyzerSupport for Runtime<X> {
    fn make_symbol(&mut self, lexeme: &str) -> Symbol {
        self.symbol_registry.intern_str(lexeme)
    }

    fn register_lambda(&mut self, kind: LambdaKind) -> LambdaId {
        self.lambda_registry.register(kind)
    }

    // TODO: PropertyKey::Number
    fn define_global_property(&mut self, name: Symbol, property: Property) -> Result<bool, Value> {
        self.global_object
            .define_own_property(name.into(), property)
    }
}

#[derive(Default)]
struct GlobalAnalysis {
    /// A scope tree builder used for building the scope tree of the JavaScript program.
    scope_tree_builder: ScopeTreeBuilder,
}

// We use a macro to get the mutable reference instead of use a method returning it in order to
// avoid issues caused by borrow checker.
macro_rules! analysis_mut {
    ($analyzer:expr) => {
        $analyzer.analysis_stack.last_mut().unwrap()
    };
}

macro_rules! push_commands {
    ($analyzer:expr; $($command:expr,)+) => {
        push_commands!($analyzer; $($command),+)
    };
    ($analyzer:expr; $($command:expr),+) => {
        let analysis = analysis_mut!($analyzer);
        $(analysis.push_command($command);)*
    };
}

impl<'r, R> Analyzer<'r, R>
where
    R: AnalyzerSupport,
{
    /// Creates a semantic analyzer.
    fn new_for_script(support: &'r mut R) -> Self {
        Self::new(support, false)
    }

    /// Creates a semantic analyzer.
    fn new_for_module(support: &'r mut R) -> Self {
        Self::new(support, true)
    }

    fn new(support: &'r mut R, module: bool) -> Self {
        Self {
            support,
            global_analysis: Default::default(),
            analysis_stack: vec![],
            functions: vec![],
            module,
        }
    }

    fn analysis(&self) -> &FunctionAnalysis {
        self.analysis_stack.last().unwrap()
    }

    fn analysis_mut(&mut self) -> &mut FunctionAnalysis {
        self.analysis_stack.last_mut().unwrap()
    }

    /// Handles an AST node coming from the parser.
    fn handle_node(&mut self, node: Node<'_>) {
        logger::debug!(event = "handle_node", ?node);
        match node {
            Node::Null => self.handle_null(),
            Node::Boolean(value) => self.handle_boolean(value),
            Node::Number(value, ..) => self.handle_number(value),
            Node::String(value, ..) => self.handle_string(value),
            Node::TemplateLiteral(n) => self.handle_template_literal(n),
            Node::Array => self.handle_array(),
            Node::Object => self.handle_object(),
            Node::LiteralPropertyName(name) => self.handle_literal_property_name(name),
            Node::PropertyDefinition(kind) => self.handle_property_definition(kind),
            Node::MemberExpression(kind) => self.handle_member_expression(kind),
            Node::This => self.handle_this(),
            Node::IdentifierReference(symbol) => self.handle_identifier_reference(symbol),
            Node::BindingIdentifier(symbol) => self.handle_binding_identifier(symbol),
            Node::ArgumentListHead(empty, spread) => self.handle_argument_list_head(empty, spread),
            Node::ArgumentListItem(spread) => self.handle_argument_list_item(spread),
            Node::Arguments => self.handle_arguments(),
            Node::CallExpression => self.handle_call_expression(),
            Node::NewExpression(has_args) => self.handle_new_expression(has_args),
            Node::NonNullish => self.handle_non_nullish(),
            Node::OptionalChain(kind) => self.handle_optional_chain(kind),
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
            Node::BlockStatement => self.handle_block_statement(),
            Node::LexicalBinding(init) => self.handle_lexical_binding(init),
            Node::LetDeclaration(n) => self.handle_let_declaration(n),
            Node::ConstDeclaration(n) => self.handle_const_declaration(n),
            Node::VariableDeclaration(init) => self.handle_variable_declaration(init),
            Node::VariableStatement(n) => self.handle_variable_statement(n),
            Node::BindingElement(init) => self.handle_binding_element(init),
            Node::EmptyStatement => self.handle_empty_statement(),
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
            Node::CatchParameter => self.handle_catch_parameter(),
            Node::TryBlock => self.handle_try_block(),
            Node::CatchBlock => self.handle_catch_block(),
            Node::FinallyBlock => self.handle_finally_block(),
            Node::DebuggerStatement => self.handle_debugger_statement(),
            Node::FormalParameter => self.handle_formal_parameter(),
            Node::FormalParameters(n) => self.handle_formal_parameters(n),
            Node::FunctionDeclaration => self.handle_function_declaration(),
            Node::AsyncFunctionDeclaration => self.handle_async_function_declaration(),
            Node::FunctionExpression(named) => self.handle_function_expression(named),
            Node::AsyncFunctionExpression(named) => self.handle_async_function_expression(named),
            Node::ArrowFunction => self.handle_arrow_function(),
            Node::AsyncArrowFunction => self.handle_async_arrow_function(),
            Node::AwaitExpression => self.handle_await_expression(),
            Node::Then(expr) => self.handle_then(expr),
            Node::Else(expr) => self.handle_else(expr),
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
            Node::FunctionContext(name) => self.handle_function_context(name),
            Node::AsyncFunctionContext(name) => self.handle_async_function_context(name),
            Node::ArrowFunctionContext => self.handle_arrow_function_context(),
            Node::AsyncArrowFunctionContext => self.handle_async_arrow_function_context(),
            Node::FunctionSignature => self.handle_function_signature(),
            Node::Dereference => self.handle_dereference(),
            Node::ToString => self.handle_to_string(),
        }
    }

    fn handle_null(&mut self) {
        analysis_mut!(self).put_null();
    }

    fn handle_boolean(&mut self, value: bool) {
        analysis_mut!(self).put_boolean(value);
    }

    fn handle_number(&mut self, value: f64) {
        analysis_mut!(self).put_number(value);
    }

    fn handle_string(&mut self, value: Vec<u16>) {
        analysis_mut!(self).put_string(value);
    }

    fn handle_template_literal(&mut self, n: u16) {
        // n: The number of expressions interpolated into the template literal.
        if n > 0 {
            analysis_mut!(self).put_command(CompileCommand::ConcatStrings(2 * n + 1));
        }
    }

    fn handle_array(&mut self) {
        // TODO(feat): 10.4.2.2 ArrayCreate ( length [ , proto ] )
        push_commands! {
            self;
            CompileCommand::Object,
            // object.length = 0;
            CompileCommand::PropertyReference(Symbol::LENGTH),
            CompileCommand::Number(0.0),
            CompileCommand::CreateDataProperty,
        }
    }

    fn handle_object(&mut self) {
        analysis_mut!(self).put_object();
    }

    fn handle_literal_property_name(&mut self, name: LiteralPropertyName) {
        match name {
            LiteralPropertyName::IdentifierName(key) => {
                push_commands!(self; CompileCommand::PropertyReference(key));
            }
        }
    }

    fn handle_property_definition(&mut self, kind: PropertyDefinitionKind) {
        analysis_mut!(self).process_property_definition(kind);
    }

    fn handle_member_expression(&mut self, kind: MemberExpressionKind) {
        analysis_mut!(self).process_member_expression(kind);
    }

    fn handle_this(&mut self) {
        analysis_mut!(self).process_this();
    }

    fn handle_identifier_reference(&mut self, symbol: Symbol) {
        analysis_mut!(self).process_identifier_reference(symbol);
    }

    fn handle_binding_identifier(&mut self, symbol: Symbol) {
        analysis_mut!(self).process_binding_identifier(symbol);
    }

    fn handle_argument_list_head(&mut self, empty: bool, spread: bool) {
        analysis_mut!(self).process_argument_list_head(empty, spread)
    }

    fn handle_argument_list_item(&mut self, spread: bool) {
        analysis_mut!(self).put_argument(spread);
    }

    fn handle_arguments(&mut self) {
        // nop
    }

    fn handle_call_expression(&mut self) {
        analysis_mut!(self).process_call_expression();
    }

    fn handle_new_expression(&mut self, has_args: bool) {
        analysis_mut!(self).process_new_expression(has_args);
    }

    fn handle_non_nullish(&mut self) {
        analysis_mut!(self).process_non_nullish();
    }

    fn handle_optional_chain(&mut self, kind: PropertyAccessKind) {
        analysis_mut!(self).process_optional_chain(kind);
    }

    fn handle_operator(&mut self, op: CompileCommand) {
        push_commands!(self; op);
    }

    fn handle_binary_expression(&mut self, op: BinaryOperator) {
        analysis_mut!(self).process_binary_expression(op);
    }

    fn handle_shorthand_assignment_expression(&mut self, op: AssignmentOperator) {
        analysis_mut!(self).process_shorthand_assignment_expression(op);
    }

    fn handle_sequence_expression(&mut self) {
        analysis_mut!(self).process_sequence_expression();
    }

    fn handle_conditional_expression(&mut self) {
        push_commands!(self; CompileCommand::Ternary);
    }

    fn handle_conditional_assignment(&mut self) {
        push_commands!(self; CompileCommand::Ternary, CompileCommand::Assignment);
    }

    fn handle_block_statement(&mut self) {
        // nop
    }

    fn handle_lexical_binding(&mut self, init: bool) {
        analysis_mut!(self).process_lexical_binding(init);
    }

    fn handle_let_declaration(&mut self, n: u32) {
        analysis_mut!(self).process_mutable_bindings(n, &mut self.global_analysis);
    }

    fn handle_const_declaration(&mut self, n: u32) {
        analysis_mut!(self).process_immutable_bindings(n, &mut self.global_analysis);
    }

    fn handle_variable_declaration(&mut self, init: bool) {
        analysis_mut!(self).process_variable_declaration(init);
    }

    fn handle_variable_statement(&mut self, _n: u32) {
        // nop
    }

    fn handle_binding_element(&mut self, _init: bool) {
        // TODO
    }

    fn handle_empty_statement(&mut self) {
        // nop
    }

    fn handle_expression_statement(&mut self) {
        push_commands!(self; CompileCommand::Discard);
    }

    fn handle_if_else_statement(&mut self) {
        push_commands!(self; CompileCommand::IfElseStatement);
    }

    fn handle_if_statement(&mut self) {
        push_commands!(self; CompileCommand::IfStatement);
    }

    fn handle_do_while_statement(&mut self) {
        // See handle_loop_start() for the reason why we always pop the lexical scope here.
        self.global_analysis.scope_tree_builder.pop();
        analysis_mut!(self).process_do_while_statement();
    }

    fn handle_while_statement(&mut self) {
        // See handle_loop_start() for the reason why we always pop the lexical scope here.
        self.global_analysis.scope_tree_builder.pop();
        analysis_mut!(self).process_while_statement();
    }

    fn handle_for_statement(&mut self, flags: LoopFlags) {
        // See handle_loop_start() for the reason why we always pop the lexical scope here.
        self.global_analysis.scope_tree_builder.pop();
        analysis_mut!(self).process_for_statement(flags);
    }

    fn handle_continue_statement(&mut self, symbol: Symbol) {
        push_commands!(self; CompileCommand::Continue(symbol));
    }

    fn handle_break_statement(&mut self, symbol: Symbol) {
        push_commands!(self; CompileCommand::Break(symbol));
    }

    fn handle_return_statement(&mut self, n: u32) {
        push_commands!(self; CompileCommand::Return(n));
    }

    fn handle_switch_statement(&mut self) {
        analysis_mut!(self).process_switch_statement();
        self.global_analysis.scope_tree_builder.pop();
    }

    fn handle_case_block(&mut self) {
        let scope_ref = self.global_analysis.scope_tree_builder.push_block();
        analysis_mut!(self).process_case_block(scope_ref);
    }

    fn handle_case_selector(&mut self) {
        analysis_mut!(self).process_case_selector();
    }

    fn handle_case_clause(&mut self, has_statement: bool) {
        analysis_mut!(self).process_case_clause(has_statement);
    }

    fn handle_default_selector(&mut self) {
        analysis_mut!(self).process_default_selector();
    }

    fn handle_default_clause(&mut self, has_statement: bool) {
        analysis_mut!(self).process_default_clause(has_statement);
    }

    fn handle_labelled_statement(&mut self, symbol: Symbol, is_iteration_statement: bool) {
        analysis_mut!(self).process_labelled_statement(symbol, is_iteration_statement);
    }

    fn handle_label(&mut self, symbol: Symbol) {
        analysis_mut!(self).process_label(symbol);
    }

    fn handle_throw_statement(&mut self) {
        analysis_mut!(self).process_throw_statement();
    }

    fn handle_try_statement(&mut self) {
        analysis_mut!(self).process_try_end();
    }

    fn handle_catch_clause(&mut self, has_parameter: bool) {
        analysis_mut!(self).process_catch_clause(has_parameter);
    }

    fn handle_finally_clause(&mut self) {
        analysis_mut!(self).process_finally_clause();
    }

    fn handle_catch_parameter(&mut self) {
        analysis_mut!(self).process_catch_parameter(&mut self.global_analysis);
    }

    fn handle_try_block(&mut self) {
        analysis_mut!(self).process_try_block();
    }

    fn handle_catch_block(&mut self) {
        // In the specification, a new lexical scope is created only when the catch parameter
        // exists, but we always create a scope here for simplicity.  In our processing model,
        // the catch and finally clauses are always created even if there is no corresponding
        // node in the AST.
        let scope_ref = self.global_analysis.scope_tree_builder.push_block();
        analysis_mut!(self).process_catch_block(scope_ref);
    }

    fn handle_finally_block(&mut self) {
        analysis_mut!(self).process_finally_block();
        self.global_analysis.scope_tree_builder.pop();
    }

    fn handle_debugger_statement(&mut self) {
        push_commands!(self; CompileCommand::Debugger);
    }

    fn handle_formal_parameter(&mut self) {
        analysis_mut!(self).process_formal_parameter(&mut self.global_analysis);
    }

    fn handle_formal_parameters(&mut self, _n: u32) {
        // TODO
    }

    fn end_function_scope(&mut self) {
        let mut analysis = self.analysis_stack.pop().unwrap();
        debug_assert!(analysis.symbol_stack.is_empty());

        let func_scope_ref = analysis.end_scope();
        // DO NOT CALL `self.global_analysis.scope_tree_builder.pop()` HERE.

        // Add Function-scoped variables defined by "VariableStatement"s to the function scope.
        analysis.process_function_scoped_variables(&mut self.global_analysis);

        // TODO: this binding resolution

        self.global_analysis.scope_tree_builder.pop();

        // The reference resolution must be performed after the function-scoped variables are added
        // to the function scope.
        let unresolved_references = self.resolve_references(&mut analysis);

        if analysis.is_coroutine() {
            // The local variables allocated on the heap will be passed as arguments for the
            // coroutine.  Load the local variables from the environment at first.
            analysis.set_command(0, CompileCommand::Environment(analysis.num_locals));
            debug_assert!(analysis.coroutine.state <= u16::MAX as u32);
            analysis.set_command(1, CompileCommand::JumpTable(analysis.coroutine.state + 2));
        } else {
            analysis.set_command(0, CompileCommand::AllocateLocals(analysis.num_locals));
            analysis.set_command(1, CompileCommand::Nop);
        }

        let this_local = analysis
            .flags
            .contains(FunctionAnalysisFlags::THIS_BINDING_LOCAL);
        let this_used = analysis
            .flags
            .contains(FunctionAnalysisFlags::THIS_BINDING_USED);
        let this_captured = analysis
            .flags
            .contains(FunctionAnalysisFlags::THIS_BINDING_CAPTURED);
        let this_mode_lexical = matches!(analysis.this_mode, ThisMode::Lexical);
        let outer_this_captured = this_local && (this_used || this_captured) && this_mode_lexical;

        self.apply_analysis(analysis, func_scope_ref);

        let func_index = self.functions.len() - 1;
        let analysis = self.analysis_mut();
        analysis.flags.set(
            FunctionAnalysisFlags::THIS_BINDING_CAPTURED,
            outer_this_captured,
        );
        analysis.process_unresolved_references(&unresolved_references, func_index);
    }

    fn handle_function_declaration(&mut self) {
        self.end_function_scope();

        let func = self.functions.last().unwrap();
        analysis_mut!(self).process_closure_declaration(func.scope_ref, func.id);
    }

    fn handle_async_function_declaration(&mut self) {
        self.end_coroutine_body();

        // Node::FunctionDeclaration for the outer ramp function.
        self.handle_function_declaration();
    }

    fn handle_function_expression(&mut self, named: bool) {
        self.do_handle_function_expression(named, false);
    }

    fn handle_async_function_expression(&mut self, named: bool) {
        self.end_coroutine_body();

        // Node::FunctionExpression for the outer ramp function.
        self.do_handle_function_expression(named, false);
    }

    fn do_handle_function_expression(&mut self, named: bool, coroutine: bool) {
        self.end_function_scope();

        let func = self.functions.last().unwrap();
        analysis_mut!(self).process_closure_expression(func.scope_ref, func.id, named, coroutine);
    }

    fn handle_arrow_function(&mut self) {
        self.do_handle_arrow_function(false);
    }

    fn handle_async_arrow_function(&mut self) {
        self.end_coroutine_body();

        // Node::ArrowFunction for the outer ramp function.
        self.do_handle_arrow_function(false);
    }

    fn do_handle_arrow_function(&mut self, coroutine: bool) {
        // TODO: An ArrowFunction does not define local variables for arguments, super, this, or
        // new.target.  Any reference to arguments, super, this, or new.target within an
        // ArrowFunction must resolve to a variable in a lexically enclosing environment.

        self.end_function_scope();

        let func = self.functions.last().unwrap();
        analysis_mut!(self).process_closure_expression(func.scope_ref, func.id, false, coroutine);
    }

    fn handle_await_expression(&mut self) {
        let analysis = analysis_mut!(self);
        let next_state = analysis.coroutine.state + 1;
        analysis.push_command(CompileCommand::Await(next_state));
        analysis.coroutine.state = next_state;
    }

    fn handle_then(&mut self, expr: bool) {
        push_commands!(self; CompileCommand::Truthy, CompileCommand::IfThen(expr));
    }

    fn handle_else(&mut self, expr: bool) {
        push_commands!(self; CompileCommand::Else(expr));
    }

    fn handle_falsy_short_circuit(&mut self) {
        push_commands!(self; CompileCommand::FalsyShortCircuit);
    }

    fn handle_truthy_short_circuit(&mut self) {
        push_commands!(self; CompileCommand::TruthyShortCircuit);
    }

    fn handle_nullish_short_circuit(&mut self) {
        push_commands!(self; CompileCommand::NullishShortCircuit);
    }

    fn handle_falsy_short_circuit_assignment(&mut self) {
        push_commands!(self; CompileCommand::Duplicate(0), CompileCommand::FalsyShortCircuit);
    }

    fn handle_truthy_short_circuit_assignment(&mut self) {
        push_commands!(self; CompileCommand::Duplicate(0), CompileCommand::TruthyShortCircuit);
    }

    fn handle_nullish_short_circuit_assignment(&mut self) {
        push_commands!(self; CompileCommand::Duplicate(0), CompileCommand::NullishShortCircuit);
    }

    fn handle_loop_start(&mut self) {
        // NOTE: This doesn't follow the specification, but we create a new lexical scope for the
        // iteration statement.  This is needed for the for-let/const statements, but not for
        // others.  We believe that this change does not compromise conformance to the
        // specification and does not cause security problems.
        let scope_ref = self.global_analysis.scope_tree_builder.push_block();
        analysis_mut!(self).process_loop_start(scope_ref);
    }

    fn handle_loop_init_expression(&mut self) {
        analysis_mut!(self).process_loop_init_expression();
    }

    fn handle_loop_init_var_declaration(&mut self) {
        analysis_mut!(self).process_loop_init_declaration();
    }

    fn handle_loop_init_lexical_declaration(&mut self) {
        analysis_mut!(self).process_loop_init_declaration();
    }

    fn handle_loop_test(&mut self) {
        analysis_mut!(self).process_loop_test();
    }

    fn handle_loop_next(&mut self) {
        analysis_mut!(self).process_loop_next();
    }

    fn handle_loop_body(&mut self) {
        analysis_mut!(self).process_loop_body();
    }

    fn handle_start_block_scope(&mut self) {
        let scope_ref = self.global_analysis.scope_tree_builder.push_block();
        analysis_mut!(self).start_scope(scope_ref);
    }

    fn handle_end_block_scope(&mut self) {
        analysis_mut!(self).end_scope();
        self.global_analysis.scope_tree_builder.pop();
    }

    fn start_function_scope(&mut self, name: Symbol, kind: LambdaKind, this_mode: ThisMode) {
        // TODO: the compilation should fail if the following condition is unmet.
        assert!(self.functions.len() < u32::MAX as usize);

        let is_entry_function = self.analysis_stack.is_empty();
        let lambda_id = self.support.register_lambda(kind);

        let mut analysis = FunctionAnalysis::new(name, lambda_id, this_mode);

        // `commands[0]` will be replaced with `AllocateLocals` or `Environment`.
        //
        // `commands[1]` will be replaced with `JumpTable` if the function is a coroutine.
        // Otherwise `Nop`.
        analysis.reserve_commands(2);

        let scope_ref = self.global_analysis.scope_tree_builder.push_function();
        analysis.start_scope(scope_ref);
        analysis.push_command(CompileCommand::DeclareVariables(scope_ref));

        // The `this` binding will be always resolved to the global object in the entry function.
        if !is_entry_function && matches!(this_mode, ThisMode::Strict | ThisMode::Global) {
            analysis
                .flags
                .insert(FunctionAnalysisFlags::THIS_BINDING_LOCAL);
        }

        if matches!(kind, LambdaKind::Ramp) {
            analysis.set_ramp();
        }

        if let Some(parent) = self.analysis_stack.last_mut() {
            if parent
                .flags
                .contains(FunctionAnalysisFlags::THIS_BINDING_LOCAL)
            {
                analysis
                    .flags
                    .insert(FunctionAnalysisFlags::THIS_BINDING_LOCAL);
            }
        }

        self.analysis_stack.push(analysis);
    }

    fn handle_function_context(&mut self, name: Symbol) {
        // TODO(feat): 'use strict'
        let this_mode = if self.module {
            ThisMode::Strict
        } else {
            ThisMode::Global
        };
        self.start_function_scope(name, LambdaKind::Normal, this_mode);
    }

    fn handle_async_function_context(&mut self, name: Symbol) {
        // TODO(feat): 'use strict'
        let this_mode = if self.module {
            ThisMode::Strict
        } else {
            ThisMode::Global
        };
        self.start_function_scope(name, LambdaKind::Ramp, this_mode);
    }

    fn handle_arrow_function_context(&mut self) {
        self.start_function_scope(Symbol::NONE, LambdaKind::Normal, ThisMode::Lexical);
    }

    fn handle_async_arrow_function_context(&mut self) {
        self.start_function_scope(Symbol::NONE, LambdaKind::Ramp, ThisMode::Lexical);
    }

    fn handle_function_signature(&mut self) {
        if self.analysis().is_ramp() {
            self.start_coroutine_body();
        }
    }

    // The async function is translated into a ramp function.  The ramp function creates a
    // coroutine every time it's called.  The coroutine function body is built from the async
    // function body.  It will be rewritten into a state machine for the coroutine.
    //
    // See //libs/jsruntime/docs/internals.md for details.
    //
    // TODO(perf): We never optimize an async function which has no await expression in the body.
    // Such an async function don't need to be rewritten into a state machine.
    fn start_coroutine_body(&mut self) {
        debug_assert!(self.analysis().is_ramp());
        let this_mode = self.analysis().this_mode;
        self.start_function_scope(Symbol::HIDDEN_COROUTINE, LambdaKind::Coroutine, this_mode);
        self.handle_binding_identifier(Symbol::HIDDEN_PROMISE);
        self.handle_formal_parameter();
        self.handle_binding_identifier(Symbol::HIDDEN_RESULT);
        self.handle_formal_parameter();
        self.handle_binding_identifier(Symbol::HIDDEN_ERROR);
        self.handle_formal_parameter();
        self.handle_formal_parameters(3);
        self.handle_function_signature();

        analysis_mut!(self).set_coroutine();
    }

    // Generate compile commands for the bottom-half of the coroutine.
    // See //libs/jsruntime/docs/internals.md.
    fn end_coroutine_body(&mut self) {
        // TODO(perf): Some of the local variables can be placed on the stack.
        self.do_handle_function_expression(false, true);

        let func = self.functions.last().unwrap();
        push_commands!(
            self;
            CompileCommand::Coroutine(func.id, func.num_locals),
            CompileCommand::Promise,
            CompileCommand::Duplicate(0),
            CompileCommand::Resume,
            CompileCommand::Return(1),
        );
    }

    fn handle_dereference(&mut self) {
        push_commands!(self; CompileCommand::Dereference);
    }

    fn handle_to_string(&mut self) {
        push_commands!(self; CompileCommand::ToString);
    }

    fn resolve_references(&mut self, analysis: &mut FunctionAnalysis) -> Vec<Reference> {
        if analysis
            .flags
            .contains(FunctionAnalysisFlags::THIS_BINDING_CAPTURED)
        {
            debug_assert!(
                analysis
                    .flags
                    .contains(FunctionAnalysisFlags::THIS_BINDING_LOCAL)
            );
            self.functions.last_mut().unwrap().num_captures += 1;
        }

        let mut unresolved_reference = vec![];
        for reference in analysis.references.iter() {
            let variable_ref = self
                .global_analysis
                .scope_tree_builder
                .resolve_reference(reference);
            if variable_ref != VariableRef::NONE {
                logger::debug!(event = "reference_resolved", ?reference, ?variable_ref);
                if let Some(func_index) = reference.func_index {
                    let func = &mut self.functions[func_index];
                    self.global_analysis
                        .scope_tree_builder
                        .set_captured(variable_ref);
                    self.global_analysis.scope_tree_builder.add_capture(
                        func.scope_ref,
                        reference.symbol,
                        func.num_captures,
                    );
                    func.num_captures += 1;
                }
            } else {
                logger::debug!(event = "reference_unresolved", ?reference);
                unresolved_reference.push(reference.clone());
            }
        }
        unresolved_reference
    }

    fn apply_analysis(&mut self, analysis: FunctionAnalysis, scope_ref: ScopeRef) {
        let this_used = analysis
            .flags
            .contains(FunctionAnalysisFlags::THIS_BINDING_USED);
        let this_captured = analysis
            .flags
            .contains(FunctionAnalysisFlags::THIS_BINDING_CAPTURED);
        let this_local = analysis
            .flags
            .contains(FunctionAnalysisFlags::THIS_BINDING_LOCAL);
        let this_binding = match (this_used || this_captured, this_local, analysis.this_mode) {
            (false, _, _) => ThisBinding::None,
            (_, _, ThisMode::Strict) => ThisBinding::ThisArgument,
            (_, true, ThisMode::Lexical) => ThisBinding::Capture,
            (_, false, ThisMode::Lexical) => ThisBinding::GlobalObject,
            (_, true, ThisMode::Global) => ThisBinding::Quirk,
            (_, false, ThisMode::Global) => ThisBinding::GlobalObject,
        };
        let mut flags = FunctionFlags::empty();
        // The global object is never captured.  It can be directly accessible in any scope.
        if this_captured && this_local {
            flags.insert(FunctionFlags::THIS_BINDING_CAPTURED);
        }
        self.functions.push(Function {
            name: analysis.name,
            id: analysis.id,
            commands: analysis.commands,
            scope_ref,
            num_captures: 0,
            num_params: analysis.num_params,
            num_locals: analysis.num_locals,
            this_binding,
            flags,
        });
    }
}

impl<'s, R> NodeHandler<'s> for Analyzer<'_, R>
where
    R: AnalyzerSupport,
{
    type Artifact = Program;

    fn start(&mut self) {
        logger::debug!(event = "start");

        // The global object will be specified in the `this` parameter of the lambda function
        // compiled from the top-level statements.  See `Runtime::call_entry_lambda()`.
        if self.module {
            self.start_function_scope(Symbol::NONE, LambdaKind::Ramp, ThisMode::Strict);
            // The module is always treated as an async function body.
            self.start_coroutine_body();
        } else {
            self.start_function_scope(Symbol::NONE, LambdaKind::Normal, ThisMode::Global);
        }
    }

    fn accept(&mut self) -> Result<Self::Artifact, Error> {
        logger::debug!(event = "accept");

        if self.module {
            self.end_coroutine_body();
        }

        let mut analysis = self.analysis_stack.pop().unwrap();
        debug_assert!(analysis.symbol_stack.is_empty());

        let global_scope_ref = analysis.end_scope();
        self.global_analysis.scope_tree_builder.pop();

        let unresolved_references = self.resolve_references(&mut analysis);

        analysis.set_command(0, CompileCommand::AllocateLocals(analysis.num_locals));
        analysis.set_command(1, CompileCommand::Nop);

        let mut globals_in_global_scope = FxHashSet::default();
        let mut global_symbols = FxHashSet::default();

        // References to global properties.
        for reference in unresolved_references.iter() {
            match reference.func_index {
                Some(func_index) => {
                    let func_scope_ref = self.functions[func_index].scope_ref;
                    self.global_analysis.scope_tree_builder.add_global(
                        func_scope_ref,
                        reference.symbol,
                        Default::default(),
                    );
                }
                None => {
                    if !globals_in_global_scope.contains(&reference.symbol) {
                        self.global_analysis.scope_tree_builder.add_global(
                            global_scope_ref,
                            reference.symbol,
                            Default::default(),
                        );
                        globals_in_global_scope.insert(reference.symbol);
                    }
                }
            }
            global_symbols.insert(reference.symbol);
        }

        // In the specification, global properties defined by "VariableStatement"s are created in
        // "16.1.7 GlobalDeclarationInstantiation ( script, env )".  We create them here for
        // simplicity but this still works properly for well-formed JavaScript programs.
        //
        // TODO(test): probably, the order of error handling may be different fro the
        // specification.
        for (&symbol, entry) in analysis.function_scoped_variables.iter() {
            // TODO(feat): "[[DefineOwnProperty]]()" may throw an "Error".  In this case, the
            // `function.commands` must be rewritten to throw the "Error".
            let result = self
                .support
                .define_global_property(symbol, Property::data_wec(Value::Undefined));
            debug_assert!(matches!(result, Ok(true)));
            // TODO: this
            if !globals_in_global_scope.contains(&symbol) {
                self.global_analysis.scope_tree_builder.add_global(
                    global_scope_ref,
                    symbol,
                    entry.function_declaration_batch,
                );
                globals_in_global_scope.insert(symbol);
            } else {
                // TODO(perf): rethink the algorithm.  somewhat inefficient...
                self.global_analysis
                    .scope_tree_builder
                    .set_function_declaration_batch(
                        global_scope_ref,
                        symbol,
                        entry.function_declaration_batch,
                    );
            }
            global_symbols.insert(symbol);
        }

        self.apply_analysis(analysis, global_scope_ref);

        Ok(Program {
            functions: std::mem::take(&mut self.functions),
            scope_tree: self.global_analysis.scope_tree_builder.build(),
            global_symbols,
            module: self.module,
        })
    }

    fn handle_nodes(&mut self, nodes: impl Iterator<Item = Node<'s>>) -> Result<(), Error> {
        for node in nodes {
            self.handle_node(node);
        }
        Ok(())
    }

    fn make_symbol(&mut self, lexeme: &str) -> Symbol {
        self.support.make_symbol(lexeme)
    }
}

/// A type representing analysis states of a JavaScript function.
///
/// This type uses a stack for each data type, instead of using a single stack that holds an
/// enumerate type having a variant for the each data type.  This way make it possible to easily
/// and efficiently access to particular elements in a stack for a data type.  While on the other
/// hand, this way is inefficient in memory usage points of view.
#[derive(Default)]
struct FunctionAnalysis {
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

    /// A stack to keep symbols defined as "BindingIdentifier"s.
    ///
    /// The second member of a tuple is used for keeping the index of a placeholder command.
    /// Because the type of a lexical declaration cannot be known at "LexicalBinding".
    symbol_stack: Vec<(Symbol, usize)>,

    /// A set of function-scoped variables.
    ///
    /// The set includes the following symbols used in the function body:
    ///
    ///   * Variable names declared by "VariableStatement"s
    ///   * Function names declared by "FunctionDeclaration"s
    function_scoped_variables: FxHashMap<Symbol, FunctionScopedVariableEntry>,

    /// A stack to hold [`Scope`]s.
    ///
    /// The bottom element always holds the function scope.
    scope_stack: Vec<Scope>,

    /// A stack to hold [`LoopAnalysis`]s.
    loop_stack: Vec<LoopAnalysis>,

    /// A stack to hold [`SwitchAnalysis`]s.
    switch_stack: Vec<SwitchAnalysis>,

    /// A stack to hold [`LabelAnalysis`]s.
    label_stack: Vec<LabelAnalysis>,

    /// A stack to hold [`TryAnalysis`]s.
    try_stack: Vec<TryAnalysis>,

    coroutine: CoroutineAnalysis,

    /// A stack to hold the number of arguments of a function call.
    nargs_stack: Vec<u16>,

    /// The name of the function.
    ///
    /// Its value is set to `Symbol::NONE` if the function has no name.
    name: Symbol,

    /// The Lambda ID of the function.
    id: LambdaId,

    /// The number of formal parameters.
    num_params: u16,

    /// The number of local variables.
    num_locals: u16,

    /// The number of do-while statements.
    num_do_while_statements: u16,

    /// The number of while statements.
    num_while_statements: u16,

    /// The number of for statements.
    num_for_statements: u16,

    /// The number of switch statements.
    num_switch_statements: u16,

    /// `[[ThisMode]]`.
    this_mode: ThisMode,

    flags: FunctionAnalysisFlags,
}

struct FunctionScopedVariableEntry {
    /// The index of the [`CompileCommand::Batch`] for the variable.
    ///
    /// `0` means that there is no [`CompileCommand::Batch`] for a function declaration.
    function_declaration_batch: usize,

    /// The mutability of the variable.
    mutable: bool,
}

impl FunctionScopedVariableEntry {
    fn var() -> Self {
        Self {
            function_declaration_batch: 0,
            mutable: true,
        }
    }

    fn function(declaration_batch: usize) -> Self {
        debug_assert_ne!(declaration_batch, 0);
        Self {
            function_declaration_batch: declaration_batch,
            mutable: true,
        }
    }
}

#[derive(Clone, Copy, Default)]
enum ThisMode {
    #[default]
    Strict,
    Lexical,
    Global,
}

bitflags! {
    #[derive(Debug, Default)]
    struct FunctionAnalysisFlags: u8 {
        /// Enabled if the context is the ramp function for an async function.
        const RAMP = 1 << 0;

        /// Enabled if the context is the coroutine function for an async function.
        const COROUTINE = 1 << 1;

        /// The `this` binding is used in the function body.
        ///
        /// The `this` binding must be resolved to an actual value in
        /// [`CompileCommand::DeclareVariables`] according to the [`ThisBinding`].
        const THIS_BINDING_USED = 1 << 2;

        /// The `this` binding is captured by descendant closures.
        const THIS_BINDING_CAPTURED = 1 << 3;

        /// The `this` binding will be always resolved to the global object if this flag is not
        /// set.
        const THIS_BINDING_LOCAL = 1 << 4;
    }
}

impl FunctionAnalysis {
    fn new(name: Symbol, id: LambdaId, this_mode: ThisMode) -> Self {
        Self {
            name,
            id,
            this_mode,
            ..Default::default()
        }
    }

    fn is_ramp(&self) -> bool {
        self.flags.contains(FunctionAnalysisFlags::RAMP)
    }

    fn is_coroutine(&self) -> bool {
        self.flags.contains(FunctionAnalysisFlags::COROUTINE)
    }

    fn set_ramp(&mut self) {
        self.flags.insert(FunctionAnalysisFlags::RAMP);
    }

    fn set_coroutine(&mut self) {
        self.flags.insert(FunctionAnalysisFlags::COROUTINE);
    }

    fn push_command(&mut self, command: CompileCommand) {
        self.commands.push(command);
    }

    fn reserve_commands(&mut self, n: usize) -> usize {
        let index = self.commands.len();
        for _ in 0..n {
            self.commands.push(CompileCommand::PlaceHolder);
        }
        index
    }

    fn put_command(&mut self, command: CompileCommand) -> usize {
        let index = self.commands.len();
        self.commands.push(command);
        index
    }

    fn set_command(&mut self, index: usize, command: CompileCommand) {
        self.commands[index] = command;
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

    fn put_object(&mut self) {
        self.commands.push(CompileCommand::Object);
        // TODO: type inference
    }

    fn process_argument_list_head(&mut self, empty: bool, _spread: bool) {
        // TODO: spread
        self.nargs_stack.push(if empty { 0 } else { 1 });
    }

    fn put_argument(&mut self, _spread: bool) {
        // TODO: spread
        *self.nargs_stack.last_mut().unwrap() += 1;
    }

    fn process_call_expression(&mut self) {
        let nargs = self.nargs_stack.pop().unwrap();
        self.commands.push(CompileCommand::Call(nargs));
    }

    fn process_new_expression(&mut self, has_args: bool) {
        let nargs = if has_args {
            self.nargs_stack.pop().unwrap()
        } else {
            0
        };
        self.commands.push(CompileCommand::New(nargs));
    }

    fn process_lexical_binding(&mut self, init: bool) {
        debug_assert!(!self.symbol_stack.is_empty());

        if !init {
            // Set undefined as the initial value.
            self.commands.push(CompileCommand::Undefined);
        }

        // We put placeholder commands here because we don't know whether this variable is mutable
        // or not at this point.  The placeholder commands will be replaced in
        // `process_mutable_bindings()` or `process_immutable_bindings()`.
        let command_index = self.reserve_commands(2);
        self.symbol_stack.last_mut().unwrap().1 = command_index;

        // TODO: type info
    }

    fn process_variable_declaration(&mut self, init: bool) {
        debug_assert!(!self.symbol_stack.is_empty());

        let (symbol, _) = self.symbol_stack.pop().unwrap();
        if init {
            self.commands
                .push(CompileCommand::VariableReference(symbol));
            self.commands.push(CompileCommand::Swap);
            self.commands.push(CompileCommand::Assignment);
            self.commands.push(CompileCommand::Discard);
        }

        // "VariableDeclaration"s can overwrite the variable bound to a function declared with the
        // same symbol in a "FunctionDeclaration".
        self.function_scoped_variables
            .entry(symbol)
            .or_insert(FunctionScopedVariableEntry::var());

        // TODO: type info
    }

    fn process_property_definition(&mut self, kind: PropertyDefinitionKind) {
        match kind {
            PropertyDefinitionKind::ArrayElement => {
                self.commands.push(CompileCommand::PushArrayElement);
            }
            PropertyDefinitionKind::ArrayEmptySlot => {
                // TODO(refactor): perform `array.length += 1`
                self.commands.push(CompileCommand::Undefined);
                self.commands.push(CompileCommand::PushArrayElement);
            }
            PropertyDefinitionKind::ArraySpread => {
                // 13.2.4.1 Runtime Semantics: ArrayAccumulation
                todo!("feat(jsruntime): GetIterator(spreadObj, sync)");
            }
            PropertyDefinitionKind::Reference => {
                let symbol = match self.commands.pop() {
                    Some(CompileCommand::VariableReference(symbol)) => symbol,
                    command => unreachable!("{command:?}"),
                };
                self.commands
                    .push(CompileCommand::PropertyReference(symbol));
                self.commands
                    .push(CompileCommand::VariableReference(symbol));
                self.commands.push(CompileCommand::CreateDataProperty);
            }
            PropertyDefinitionKind::KeyValue => {
                self.commands.push(CompileCommand::CreateDataProperty);
            }
            PropertyDefinitionKind::Spread => {
                self.commands.push(CompileCommand::CopyDataProperties);
            }
        }
    }

    fn process_member_expression(&mut self, kind: MemberExpressionKind) {
        match kind {
            MemberExpressionKind::PropertyAccessWithExpressionKey => {
                self.commands.push(CompileCommand::ToPropertyKey);
            }
            MemberExpressionKind::PropertyAccessWithIdentifierKey(key) => {
                self.commands.push(CompileCommand::PropertyReference(key));
            }
        }
    }

    fn process_this(&mut self) {
        // A use of the `this` keyword is implemented as a reference to a hidden immutable variable
        // in the function scope.  See the implementation of [`accept()`] for details.
        self.commands.push(CompileCommand::This);
        self.flags.insert(FunctionAnalysisFlags::THIS_BINDING_USED);
    }

    fn process_identifier_reference(&mut self, symbol: Symbol) {
        self.commands
            .push(CompileCommand::VariableReference(symbol));
        self.references
            .push(Reference::new(symbol, self.scope_ref()));
    }

    fn process_binding_identifier(&mut self, symbol: Symbol) {
        self.symbol_stack.push((symbol, 0));
    }

    fn process_non_nullish(&mut self) {
        // Convert `object?.<op>` into:
        //   (object !== undefined && object !== null) ? <op> : undefined
        self.commands.push(CompileCommand::Dereference);
        self.commands.push(CompileCommand::Duplicate(0));
        self.commands.push(CompileCommand::NonNullish);
        self.commands.push(CompileCommand::IfThen(true));
    }

    fn process_optional_chain(&mut self, kind: PropertyAccessKind) {
        match kind {
            PropertyAccessKind::Call => {
                let nargs = self.nargs_stack.pop().unwrap();
                self.commands.push(CompileCommand::Call(nargs));
                self.commands.push(CompileCommand::Else(true));
                self.commands.push(CompileCommand::Undefined);
                self.commands.push(CompileCommand::Ternary);
            }
            PropertyAccessKind::IdentifierKey(key) => {
                self.commands.push(CompileCommand::PropertyReference(key));
                self.commands.push(CompileCommand::Else(true));
                self.commands.push(CompileCommand::Undefined);
                self.commands.push(CompileCommand::Ternary);
            }
        }
    }

    fn process_binary_expression(&mut self, op: BinaryOperator) {
        // When a compiler processes the following command, the RHS has been placed on the LHS on
        // the stack of the compiler.  Swap them so that the LHS is evaluated before the RHS.
        self.commands.push(CompileCommand::Swap);
        self.commands.push(op.into());
    }

    // Every shorthand assignment operator except for short-circuit assignment operators are
    // expanded to a simple binary operator and an assignment operator.
    fn process_shorthand_assignment_expression(&mut self, op: AssignmentOperator) {
        // When a compiler processes the following command, the RHS has been placed on the LHS on
        // the stack of the compiler.  Duplicate the LHS and push it onto the stack.
        self.commands.push(CompileCommand::Duplicate(1));

        // Now, the duplicate LHS has been placed on the RHS.  And the LHS will be evaluated before
        // the RHS.  We don't need to put a CompileCommand::Swap command and just put the operator.
        self.commands.push(op.into());

        // Finally, we put the assignment operator.
        self.commands.push(CompileCommand::Assignment);
    }

    fn process_sequence_expression(&mut self) {
        self.commands.push(CompileCommand::Swap);
        self.commands.push(CompileCommand::Discard);
    }

    fn process_formal_parameter(&mut self, global_analysis: &mut GlobalAnalysis) {
        debug_assert!(!self.symbol_stack.is_empty());
        let (symbol, _) = self.symbol_stack.pop().unwrap();
        // TODO: the compilation should fail if the following condition is unmet.
        assert!(self.num_params < u16::MAX);
        global_analysis
            .scope_tree_builder
            .add_argument(symbol, self.num_params);
        self.num_params += 1;
    }

    fn process_mutable_bindings(&mut self, n: u32, global_analysis: &mut GlobalAnalysis) {
        debug_assert!(self.symbol_stack.len() >= n as usize);
        let i = self.symbol_stack.len() - n as usize;
        for (symbol, index) in self.symbol_stack[i..].iter().cloned() {
            debug_assert!(matches!(self.commands[index], CompileCommand::PlaceHolder));
            self.commands[index] = CompileCommand::VariableReference(symbol);
            debug_assert!(matches!(
                self.commands[index + 1],
                CompileCommand::PlaceHolder
            ));
            self.commands[index + 1] = CompileCommand::MutableVariable;
            global_analysis
                .scope_tree_builder
                .add_local(symbol, self.num_locals, true);
            self.num_locals += 1;
        }
        self.symbol_stack.truncate(i);
    }

    fn process_immutable_bindings(&mut self, n: u32, global_analysis: &mut GlobalAnalysis) {
        debug_assert!(self.symbol_stack.len() >= n as usize);
        let i = self.symbol_stack.len() - n as usize;
        for (symbol, index) in self.symbol_stack[i..].iter().cloned() {
            debug_assert!(matches!(self.commands[index], CompileCommand::PlaceHolder));
            self.commands[index] = CompileCommand::VariableReference(symbol);
            debug_assert!(matches!(
                self.commands[index + 1],
                CompileCommand::PlaceHolder
            ));
            self.commands[index + 1] = CompileCommand::ImmutableVariable;
            global_analysis
                .scope_tree_builder
                .add_local(symbol, self.num_locals, false);
            self.num_locals += 1;
        }
        self.symbol_stack.truncate(i);
    }

    fn process_closure_declaration(&mut self, scope_ref: ScopeRef, lambda_id: LambdaId) {
        debug_assert!(!self.symbol_stack.is_empty());
        let (symbol, _) = self.symbol_stack.pop().unwrap();

        // This is a hoistable declaration.  Commands following the `Batch` command will perform
        // by a command handler for the `DeclareVariables` command generated for the current scope.
        let index = self.commands.len();
        self.commands.push(CompileCommand::Batch(5));
        self.commands.push(CompileCommand::Lambda(lambda_id));
        self.commands.push(CompileCommand::Closure(true, scope_ref));
        self.commands.push(CompileCommand::Function(symbol));
        self.commands
            .push(CompileCommand::VariableReference(symbol));
        self.commands.push(CompileCommand::DeclareFunction);

        // "VariableStatement"s have already declared variables with the same symbol.
        // Such "VariableStatement"s can overwrite the variable.
        self.function_scoped_variables
            .entry(symbol)
            .and_modify(|entry| {
                debug_assert_eq!(
                    entry.function_declaration_batch, 0,
                    "Multiple function declarations with the same symbol are not allowed."
                );
                entry.function_declaration_batch = index;
            })
            .or_insert(FunctionScopedVariableEntry::function(index));
    }

    fn process_closure_expression(
        &mut self,
        scope_ref: ScopeRef,
        lambda_id: LambdaId,
        named: bool,
        coroutine: bool,
    ) {
        let name = if named {
            debug_assert!(!self.symbol_stack.is_empty());
            self.symbol_stack.pop().unwrap().0
        } else {
            Symbol::NONE
        };
        self.commands.push(CompileCommand::Lambda(lambda_id));
        self.commands
            .push(CompileCommand::Closure(false, scope_ref));
        if !coroutine {
            self.commands.push(CompileCommand::Function(name));
        }
    }

    fn process_loop_start(&mut self, scope_ref: ScopeRef) {
        self.start_scope(scope_ref);

        // The placeholder command will be replaced with an appropriate command in
        // `process_loop_end()`.
        let start_index = self.reserve_commands(1);
        self.loop_stack.push(LoopAnalysis { start_index });
    }

    fn process_loop_init_expression(&mut self) {
        // Discard the evaluation result of the expression like as ExpressionStatement.
        self.commands.push(CompileCommand::Discard);
        self.commands.push(CompileCommand::LoopInit);
    }

    fn process_loop_init_declaration(&mut self) {
        self.commands.push(CompileCommand::LoopInit);
    }

    fn process_loop_test(&mut self) {
        self.commands.push(CompileCommand::LoopTest);
    }

    fn process_loop_next(&mut self) {
        self.commands.push(CompileCommand::LoopNext);
    }

    fn process_loop_body(&mut self) {
        self.commands.push(CompileCommand::LoopBody);
    }

    fn process_loop_end(&mut self, command: CompileCommand) {
        self.commands.push(CompileCommand::LoopEnd);
        let LoopAnalysis { start_index } = self.loop_stack.pop().unwrap();
        debug_assert!(matches!(
            self.commands[start_index],
            CompileCommand::PlaceHolder
        ));
        self.commands[start_index] = command;

        self.end_scope();
    }

    fn process_do_while_statement(&mut self) {
        self.commands.push(CompileCommand::LoopTest);
        self.process_loop_end(CompileCommand::DoWhileLoop(self.num_do_while_statements));
        self.num_do_while_statements += 1;
    }

    fn process_while_statement(&mut self) {
        self.commands.push(CompileCommand::LoopBody);
        self.process_loop_end(CompileCommand::WhileLoop(self.num_while_statements));
        self.num_while_statements += 1;
    }

    fn process_for_statement(&mut self, flags: LoopFlags) {
        self.commands.push(CompileCommand::LoopBody);
        self.process_loop_end(CompileCommand::ForLoop(self.num_for_statements, flags));
        self.num_for_statements += 1;
    }

    fn process_case_block(&mut self, scope_ref: ScopeRef) {
        // Step#3..7 in 14.12.4 Runtime Semantics: Evaluation
        self.start_scope(scope_ref);

        // The placeholder commands will be replaced in `process_switch_statement()`.
        let case_block_index = self.reserve_commands(1);
        self.switch_stack.push(SwitchAnalysis {
            case_block_index,
            ..Default::default()
        });
    }

    fn process_case_selector(&mut self) {
        // Make a duplicate of the `switchValue` for the evaluation on the case selector.
        self.commands.push(CompileCommand::Duplicate(1));
        self.commands.push(CompileCommand::StrictEquality);
        self.reserve_command_for_case_statements();
    }

    fn reserve_command_for_case_statements(&mut self) {
        let index = self.reserve_commands(1);
        self.switch_stack.last_mut().unwrap().case_statements_index = index;
    }

    fn process_case_clause(&mut self, has_statement: bool) {
        let batch_index = self.update_command_for_case_statements(has_statement);
        self.commands
            .push(CompileCommand::CaseClause(false, batch_index));
        self.switch_stack.last_mut().unwrap().num_cases += 1;
    }

    fn update_command_for_case_statements(&mut self, has_statement: bool) -> Option<usize> {
        let index = self.switch_stack.last().unwrap().case_statements_index;
        debug_assert_eq!(self.commands[index], CompileCommand::PlaceHolder);
        if has_statement {
            let end_index = self.commands.len();
            debug_assert!(end_index - index - 1 < u16::MAX as usize);
            self.commands[index] = CompileCommand::Batch((end_index - index - 1) as u16);
            Some(index)
        } else {
            self.commands[index] = CompileCommand::Nop;
            None
        }
    }

    fn process_default_selector(&mut self) {
        self.reserve_command_for_case_statements();
    }

    fn process_default_clause(&mut self, has_statement: bool) {
        let batch_index = self.update_command_for_case_statements(has_statement);
        self.commands
            .push(CompileCommand::CaseClause(true, batch_index));
        let switch = self.switch_stack.last_mut().unwrap();
        switch.default_index = Some(switch.num_cases);
        switch.num_cases += 1;
    }

    fn process_switch_statement(&mut self) {
        let SwitchAnalysis {
            case_block_index,
            default_index,
            num_cases,
            ..
        } = self.switch_stack.pop().unwrap();

        let id = self.num_switch_statements;

        debug_assert!(matches!(
            self.commands[case_block_index],
            CompileCommand::PlaceHolder
        ));
        if num_cases == 0 {
            // An empty case block.  Just discard the `switchValue`.
            self.commands[case_block_index] = CompileCommand::Discard;
        } else {
            self.commands[case_block_index] = CompileCommand::CaseBlock(id, num_cases);
            // Discard the `switchValue` remaining on the stack.
            self.commands.push(CompileCommand::Discard);
            self.commands
                .push(CompileCommand::Switch(id, num_cases, default_index));
            self.num_switch_statements += 1;
        }

        // Step#8 in 14.12.4 Runtime Semantics: Evaluation
        self.end_scope();
    }

    fn process_label(&mut self, symbol: Symbol) {
        // The placeholder command will be replaced with `CompileCommand::LabelStart` in
        // `process_labelled_statement()`.
        let start_index = self.reserve_commands(1);
        self.label_stack.push(LabelAnalysis {
            start_index,
            symbol,
        });
    }

    fn process_labelled_statement(&mut self, symbol: Symbol, is_iteration_statement: bool) {
        let label = self.label_stack.pop().unwrap();
        debug_assert_eq!(label.symbol, symbol);
        debug_assert!(matches!(
            self.commands[label.start_index],
            CompileCommand::PlaceHolder
        ));
        self.commands[label.start_index] =
            CompileCommand::LabelStart(symbol, is_iteration_statement);
        self.commands
            .push(CompileCommand::LabelEnd(symbol, is_iteration_statement));
    }

    fn process_throw_statement(&mut self) {
        self.commands.push(CompileCommand::Throw);
    }

    fn process_try_block(&mut self) {
        self.commands.push(CompileCommand::Try);
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
        self.commands.push(CompileCommand::TryEnd);
        self.try_stack.pop();
    }

    fn process_catch_parameter(&mut self, global_analysis: &mut GlobalAnalysis) {
        self.commands.push(CompileCommand::Exception);
        self.process_lexical_binding(true);
        self.process_mutable_bindings(1, global_analysis);
    }

    fn scope_ref(&self) -> ScopeRef {
        self.scope_stack.last().unwrap().scope_ref
    }

    fn start_scope(&mut self, scope_ref: ScopeRef) {
        // NOTE(perf): The scope may has no variable.  In this case, we can remove the
        // PushScope command safely and reduce the number of the commands.  We can add a
        // post-process for optimization if it's needed.
        self.commands.push(CompileCommand::PushScope(scope_ref));
        self.scope_stack.push(Scope { scope_ref });
    }

    fn end_scope(&mut self) -> ScopeRef {
        let scope = self.scope_stack.pop().unwrap();

        // NOTE(perf): The scope may has no variable.  In this case, we can remove the
        // PopScope command safely and reduce the number of the commands.  We can add a
        // post-process for optimization if it's needed.
        self.commands
            .push(CompileCommand::PopScope(scope.scope_ref));

        scope.scope_ref
    }

    fn process_function_scoped_variables(&mut self, global_analysis: &mut GlobalAnalysis) {
        for (&symbol, entry) in self.function_scoped_variables.iter() {
            global_analysis
                .scope_tree_builder
                .add_function_scoped_variable(
                    symbol,
                    self.num_locals,
                    entry.mutable,
                    entry.function_declaration_batch,
                );
            self.num_locals += 1;
        }
    }

    fn process_unresolved_references(
        &mut self,
        unresolved_references: &[Reference],
        func_index: usize,
    ) {
        let scope_ref = self.scope_ref();
        let mut added = FxHashSet::default();
        for reference in unresolved_references.iter() {
            let symbol = reference.symbol;
            match reference.func_index {
                Some(inner_func_index) => {
                    if !added.contains(&symbol) {
                        self.references
                            .push(Reference::with_func_index(symbol, scope_ref, func_index));
                        added.insert(symbol);
                    }
                    self.references.push(Reference::with_func_index(
                        symbol,
                        scope_ref,
                        inner_func_index,
                    ));
                }
                None => {
                    if !added.contains(&symbol) {
                        self.references
                            .push(Reference::with_func_index(symbol, scope_ref, func_index));
                        added.insert(symbol);
                    }
                }
            }
        }
    }
}

struct Scope {
    scope_ref: ScopeRef,
}

struct LoopAnalysis {
    start_index: usize,
}

#[derive(Default)]
struct SwitchAnalysis {
    case_block_index: usize,
    case_statements_index: usize,
    num_cases: u16,
    default_index: Option<u16>,
}

#[derive(Default)]
struct LabelAnalysis {
    start_index: usize,
    symbol: Symbol,
}

#[derive(Default)]
struct TryAnalysis {
    catch_index: usize,
    finally_index: usize,
}

#[derive(Default)]
struct CoroutineAnalysis {
    state: u32,
}

/// A compile command.
#[derive(Debug, PartialEq)]
pub enum CompileCommand {
    // Inserting commands in the middle is an inefficient operation.  For avoiding such an
    // operation, the following commands are introduced.

    // A `Nop` performs no operation.  A `Placeholder` will be replaced with a `Nop` once it's
    // determined that command substitution is not needed.
    Nop,

    // A `Batch(n)` is inserted before `n` commands that are compile commands of a *batch*.
    // Compile commands in a batch will be skipped in an evaluation starting at the first compile
    // command.  A batch is performed at some point in the evaluation.  For example, compile
    // commands generated for a *hoistable* declaration are inserted as a batch and the compile
    // commands of the batch are performed at the `DeclareVariables` in the scope on which the
    // declaration is performed.
    Batch(u16),

    Undefined,
    Null,
    Boolean(bool),
    Number(f64),
    String(Vec<u16>),
    Object,
    Function(Symbol),
    Lambda(LambdaId),
    Closure(bool, ScopeRef),
    Coroutine(LambdaId, u16),
    Promise,
    Exception,

    // references
    This,
    VariableReference(Symbol),
    PropertyReference(Symbol),
    ToPropertyKey,

    AllocateLocals(u16),
    MutableVariable,
    ImmutableVariable,
    DeclareVariables(ScopeRef),
    DeclareFunction,
    Call(u16),
    New(u16),
    PushScope(ScopeRef),
    PopScope(ScopeRef),

    // string
    ToString,
    ConcatStrings(u16),

    // object
    CreateDataProperty,
    CopyDataProperties,
    PushArrayElement,

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
    NonNullish,
    IfThen(bool),
    Else(bool),
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
    CaseClause(bool, Option<usize>),
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

    // coroutine
    Environment(u16),
    JumpTable(u32),
    Await(u32),
    Resume,

    Discard,
    Swap,
    Duplicate(u8), // 0 or 1
    Dereference,

    // debugger
    Debugger,

    // A special command used as a placeholder in a command list, which will be replaced actual
    // command later.  The final command list must not contain placeholder commands.
    PlaceHolder,
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
    Argument(u16),
    Local(u16),
    Capture(u16),
    Global,
}

impl Locator {
    pub fn is_capture(&self) -> bool {
        matches!(self, Self::Capture(_))
    }
}

/// A type representing information needed for resolving a reference to a symbol.
#[derive(Clone, Debug)]
struct Reference {
    /// The symbol referred.
    symbol: Symbol,

    /// The reference to a (function or block) scope where the symbol is referred.
    scope_ref: ScopeRef,

    /// The index of a function in `Analyzer::functions`, that refers to the free variable.
    func_index: Option<usize>,
}

impl Reference {
    fn new(symbol: Symbol, scope_ref: ScopeRef) -> Self {
        Self {
            symbol,
            scope_ref,
            func_index: None,
        }
    }

    fn with_func_index(symbol: Symbol, scope_ref: ScopeRef, func_index: usize) -> Self {
        Self {
            symbol,
            scope_ref,
            func_index: Some(func_index),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::LambdaRegistry;
    use jsparser::Parser;
    use jsparser::Processor;
    use jsparser::SymbolRegistry;

    macro_rules! symbol {
        ($stub:expr, $name:literal) => {
            $stub
                .symbol_registry
                .lookup($name.encode_utf16().collect::<Vec<u16>>().as_slice())
        };
    }
    macro_rules! scope_ref {
        ($index:expr) => {
            ScopeRef::new($index)
        };
    }

    macro_rules! script {
        ($src:literal) => {
            Source::Script($src)
        };
    }

    macro_rules! module {
        ($src:literal) => {
            Source::Module($src)
        };
    }

    #[test]
    fn test_lexical_declarations() {
        test(
            script!("let a, b = 2; const c = 3, d = 4;"),
            |program, stub| {
                assert_eq!(
                    program.functions[0].commands,
                    [
                        CompileCommand::AllocateLocals(4),
                        CompileCommand::Nop,
                        CompileCommand::PushScope(scope_ref!(1)),
                        CompileCommand::DeclareVariables(scope_ref!(1)),
                        CompileCommand::Undefined,
                        CompileCommand::VariableReference(symbol!(stub, "a")),
                        CompileCommand::MutableVariable,
                        CompileCommand::Number(2.0),
                        CompileCommand::VariableReference(symbol!(stub, "b")),
                        CompileCommand::MutableVariable,
                        CompileCommand::Number(3.0),
                        CompileCommand::VariableReference(symbol!(stub, "c")),
                        CompileCommand::ImmutableVariable,
                        CompileCommand::Number(4.0),
                        CompileCommand::VariableReference(symbol!(stub, "d")),
                        CompileCommand::ImmutableVariable,
                        CompileCommand::PopScope(scope_ref!(1)),
                    ]
                );
            },
        );
    }

    #[test]
    fn test_lexical_declarations_in_scopes() {
        test(
            script!("let a; { let a; } { let a, b; }"),
            |program, stub| {
                assert_eq!(
                    program.functions[0].commands,
                    [
                        CompileCommand::AllocateLocals(4),
                        CompileCommand::Nop,
                        CompileCommand::PushScope(scope_ref!(1)),
                        CompileCommand::DeclareVariables(scope_ref!(1)),
                        CompileCommand::Undefined,
                        CompileCommand::VariableReference(symbol!(stub, "a")),
                        CompileCommand::MutableVariable,
                        CompileCommand::PushScope(scope_ref!(2)),
                        CompileCommand::Undefined,
                        CompileCommand::VariableReference(symbol!(stub, "a")),
                        CompileCommand::MutableVariable,
                        CompileCommand::PopScope(scope_ref!(2)),
                        CompileCommand::PushScope(scope_ref!(3)),
                        CompileCommand::Undefined,
                        CompileCommand::VariableReference(symbol!(stub, "a")),
                        CompileCommand::MutableVariable,
                        CompileCommand::Undefined,
                        CompileCommand::VariableReference(symbol!(stub, "b")),
                        CompileCommand::MutableVariable,
                        CompileCommand::PopScope(scope_ref!(3)),
                        CompileCommand::PopScope(scope_ref!(1)),
                    ]
                );
            },
        );
    }

    #[test]
    fn test_binary_operator() {
        test(script!("1 + 2"), |program, _stub| {
            assert_eq!(
                program.functions[0].commands,
                [
                    CompileCommand::AllocateLocals(0),
                    CompileCommand::Nop,
                    CompileCommand::PushScope(scope_ref!(1)),
                    CompileCommand::DeclareVariables(scope_ref!(1)),
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
        test(script!("let a = 1; a += 2"), |program, stub| {
            assert_eq!(
                program.functions[0].commands,
                [
                    CompileCommand::AllocateLocals(1),
                    CompileCommand::Nop,
                    CompileCommand::PushScope(scope_ref!(1)),
                    CompileCommand::DeclareVariables(scope_ref!(1)),
                    CompileCommand::Number(1.0),
                    CompileCommand::VariableReference(symbol!(stub, "a")),
                    CompileCommand::MutableVariable,
                    CompileCommand::VariableReference(symbol!(stub, "a")),
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

    #[test]
    fn test_await() {
        test(module!("await 0"), |program, _stub| {
            assert_eq!(program.functions.len(), 2);
            assert_eq!(
                program.functions[0].commands,
                [
                    CompileCommand::Environment(0),
                    CompileCommand::JumpTable(3),
                    CompileCommand::PushScope(scope_ref!(2)),
                    CompileCommand::DeclareVariables(scope_ref!(2)),
                    CompileCommand::Number(0.0),
                    CompileCommand::Await(1),
                    CompileCommand::Discard,
                    CompileCommand::PopScope(scope_ref!(2)),
                ],
            );
            assert_eq!(
                program.functions[1].commands,
                [
                    CompileCommand::AllocateLocals(0),
                    CompileCommand::Nop,
                    CompileCommand::PushScope(scope_ref!(1)),
                    CompileCommand::DeclareVariables(scope_ref!(1)),
                    CompileCommand::Lambda(program.functions[0].id),
                    CompileCommand::Closure(false, scope_ref!(2)),
                    CompileCommand::Coroutine(program.functions[0].id, 0),
                    CompileCommand::Promise,
                    CompileCommand::Duplicate(0),
                    CompileCommand::Resume,
                    CompileCommand::Return(1),
                    CompileCommand::PopScope(scope_ref!(1)),
                ],
            );
        });
    }

    fn test(src: Source, validate: fn(&Program, &Stub)) {
        let mut stub = Stub::default();
        let mut parser = match src {
            Source::Script(src) => Parser::for_script(
                src,
                Processor::new(Analyzer::new_for_script(&mut stub), false),
            ),
            Source::Module(src) => Parser::for_module(
                src,
                Processor::new(Analyzer::new_for_module(&mut stub), true),
            ),
        };
        let result = parser.parse();
        assert!(result.is_ok());
        if let Ok(program) = result {
            validate(&program, &stub);
        }
    }

    #[derive(Default)]
    struct Stub {
        symbol_registry: SymbolRegistry,
        lambda_registry: LambdaRegistry,
    }

    impl AnalyzerSupport for Stub {
        fn make_symbol(&mut self, lexeme: &str) -> Symbol {
            self.symbol_registry.intern_str(lexeme)
        }

        fn register_lambda(&mut self, kind: LambdaKind) -> LambdaId {
            self.lambda_registry.register(kind)
        }

        fn define_global_property(
            &mut self,
            _key: Symbol,
            _property: Property,
        ) -> Result<bool, Value> {
            Ok(true)
        }
    }

    enum Source {
        Script(&'static str),
        Module(&'static str),
    }
}
