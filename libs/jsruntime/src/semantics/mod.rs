mod scope;

use jsparser::syntax::AssignmentOperator;
use jsparser::syntax::BinaryOperator;
use jsparser::syntax::LogicalOperator;
use jsparser::syntax::Node;
use jsparser::syntax::NodeHandler;
use jsparser::syntax::UnaryOperator;
use jsparser::syntax::UpdateOperator;
use jsparser::Error;
use jsparser::Symbol;
use jsparser::SymbolRegistry;

use super::logger;
use super::FunctionId;
use super::FunctionRegistry;

use scope::BindingKind;
use scope::ScopeKind;
use scope::ScopeManager;
use scope::ScopeRef;

pub struct Program {
    pub functions: Vec<FunctionRecipe>,
}

pub struct FunctionRecipe {
    pub symbol: Symbol,
    pub id: FunctionId,
    pub commands: Vec<CompileCommand>,
}

pub struct Analyzer<'r> {
    symbol_registry: &'r mut SymbolRegistry,
    function_registry: &'r mut FunctionRegistry,
    context_stack: Vec<FunctionContext>,
    functions: Vec<FunctionRecipe>,
    scope_manager: ScopeManager,
    references: Vec<Reference>,
    use_global_bindings: bool,
}

impl<'r> Analyzer<'r> {
    pub fn new(
        symbol_registry: &'r mut SymbolRegistry,
        function_registry: &'r mut FunctionRegistry,
    ) -> Self {
        let id = function_registry.create_native_function(vec![]);
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

    fn handle_node(&mut self, node: Node<'_>) {
        logger::debug!(event = "handle_node", ?node);
        match node {
            Node::Null => self.handle_null(),
            Node::Boolean(value) => self.handle_boolean(value),
            Node::Number(value, ..) => self.handle_number(value),
            Node::String(value, ..) => self.handle_string(value),
            Node::IdentifierReference(symbol) => self.handle_identifier_reference(symbol),
            Node::BindingIdentifier(symbol) => self.handle_binding_identifier(symbol),
            Node::LabelIdentifier(symbol) => self.handle_label_identifier(symbol),
            Node::ArgumentListHead(empty, spread) => self.handle_argument_list_head(empty, spread),
            Node::ArgumentListItem(spread) => self.handle_argument_list_item(spread),
            Node::Arguments => (),
            Node::CallExpression => self.handle_call_expression(),
            Node::UpdateExpression(op) => self.handle_operator(op),
            Node::UnaryExpression(op) => self.handle_operator(op),
            Node::BinaryExpression(op) => self.handle_operator(op),
            Node::LogicalExpression(op) => self.handle_operator(op),
            Node::ConditionalExpression => self.handle_conditional_expression(),
            Node::AssignmentExpression(op) => self.handle_operator(op),
            Node::BlockStatement => (),
            Node::LexicalBinding(init) => self.handle_lexical_binding(init),
            Node::LetDeclaration(n) => self.handle_let_declaration(n),
            Node::ConstDeclaration(n) => self.handle_const_declaration(n),
            Node::BindingElement(init) => self.handle_binding_element(init),
            Node::EmptyStatement => (),
            Node::ExpressionStatement => self.handle_expression_statement(),
            Node::IfElseStatement => self.handle_if_else_statement(),
            Node::IfStatement => self.handle_if_statement(),
            Node::ReturnStatement(n) => self.handle_return_statement(n),
            Node::FormalParameter => self.handle_formal_parameter(),
            Node::FormalParameters(n) => self.handle_formal_parameters(n),
            Node::FunctionDeclaration => self.handle_function_declaration(),
            Node::ThenBlock => self.handle_then_block(),
            Node::ElseBlock => self.handle_else_block(),
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
            func_index: context.func_index,
            scope_ref: self.scope_manager.current(),
            command_index,
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
                func_index: context.func_index,
                scope_ref: self.scope_manager.current(),
                command_index,
                symbol,
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

    fn handle_label_identifier(&mut self, _symbol: Symbol) {
        // TODO
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

    fn handle_conditional_expression(&mut self) {
        self.context_stack
            .last_mut()
            .unwrap()
            .put_command(CompileCommand::ConditionalTernary);
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

    fn handle_return_statement(&mut self, n: u32) {
        self.context_stack
            .last_mut()
            .unwrap()
            .put_command(CompileCommand::Return(n));
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
        let func_index = context.func_index as usize;
        self.functions[func_index].commands = context.commands;

        self.context_stack
            .last_mut()
            .unwrap()
            .process_function_declaration(self.functions[func_index].id);
    }

    fn handle_then_block(&mut self) {
        let context = self.context_stack.last_mut().unwrap();
        context.put_command(CompileCommand::Test);
        context.put_command(CompileCommand::Then);
    }

    fn handle_else_block(&mut self) {
        self.context_stack
            .last_mut()
            .unwrap()
            .put_command(CompileCommand::Else);
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
        let func_index = self.functions.len() as u32;
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
            id: FunctionId::native(0),
            commands: vec![],
        });
        self.scope_manager.push(ScopeKind::Function);
    }

    fn handle_function_signature(&mut self, symbol: Symbol) {
        let context = self.context_stack.last_mut().unwrap();
        let id = self
            .function_registry
            .create_native_function(context.formal_parameters.clone());
        let i = context.func_index as usize;
        self.functions[i].symbol = symbol;
        self.functions[i].id = id;
        context.in_body = true;
    }

    // TODO: global object
    fn put_global_bindings(&mut self) {
        let context = self.context_stack.last_mut().unwrap();

        // Register `undefined`.
        let symbol = SymbolRegistry::UNDEFINED;
        // The locator will be computed in `resolve_locators()`.
        let command_index = context.put_command(CompileCommand::Reference(symbol, Locator::NONE));
        context.put_lexical_binding(false);
        context.process_immutable_bindings(1);
        self.scope_manager
            .add_binding(symbol, BindingKind::Immutable);
        self.references.push(Reference {
            symbol,
            func_index: context.func_index,
            scope_ref: self.scope_manager.current(),
            command_index,
        });

        // Register `Infinity`.
        let symbol = SymbolRegistry::INFINITY;
        // The locator will be computed in `resolve_locators()`.
        let command_index = context.put_command(CompileCommand::Reference(symbol, Locator::NONE));
        context.put_number(f64::INFINITY);
        context.put_lexical_binding(true);
        context.process_immutable_bindings(1);
        self.scope_manager
            .add_binding(symbol, BindingKind::Immutable);
        self.references.push(Reference {
            symbol,
            func_index: context.func_index,
            scope_ref: self.scope_manager.current(),
            command_index,
        });

        // Register `NaN`.
        let symbol = SymbolRegistry::NAN;
        // The locator will be computed in `resolve_locators()`.
        let command_index = context.put_command(CompileCommand::Reference(symbol, Locator::NONE));
        context.put_number(f64::NAN);
        context.put_lexical_binding(true);
        context.process_immutable_bindings(1);
        self.scope_manager
            .add_binding(symbol, BindingKind::Immutable);
        self.references.push(Reference {
            symbol,
            func_index: context.func_index,
            scope_ref: self.scope_manager.current(),
            command_index,
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
                func_index: context.func_index,
                scope_ref: self.scope_manager.current(),
                command_index,
            });
        }
    }

    fn resolve_locators(&mut self) {
        for reference in self.references.iter() {
            let locator = self.scope_manager.compute_locator(reference);
            logger::debug!(event = "resolve-locator", ?reference.symbol, ?locator);
            self.functions[reference.func_index as usize].commands[reference.command_index] =
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
        self.functions[context.func_index as usize].commands = context.commands;
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

#[derive(Default)]
struct FunctionContext {
    commands: Vec<CompileCommand>,
    pending_lexical_bindings: Vec<usize>,
    formal_parameters: Vec<Symbol>,
    scope_stack: Vec<Scope>,
    nargs_stack: Vec<(usize, u16)>,
    max_bindings: usize,
    func_index: u32,
    in_body: bool,
}

impl FunctionContext {
    #[inline(always)]
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
    Plus,
    Negation,
    BitwiseNot,
    LogicalNot,

    // binary operators
    Equality,
    Inequality,
    StrictEquality,
    StrictInequality,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LeftShift,
    RightShift,
    UnsignedRightShift,
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Remainder,
    BitwiseOr,
    BitwiseXor,
    BitwiseAnd,
    In,
    Instanceof,
    Exponentiation,

    // logical operators
    LogicalAnd,
    LogicalOr,
    Nullish,

    // assignment operators
    Assignment,
    MultiplicationAssignment,
    DivisionAssignment,
    RemainderAssignment,
    AdditionAssignment,
    SubtractionAssignment,
    LeftShiftAssignment,
    RightShiftAssignment,
    UnsignedRightShiftAssignment,
    BitwiseAndAssignment,
    BitwiseXorAssignment,
    BitwiseOrAssignment,
    ExponentiationAssignment,
    LogicalAndAssignment,
    LogicalOrAssignment,
    NullishCoalescingAssignment,

    // conditional
    Test,
    Then,
    Else,
    ConditionalTernary,
    IfElseStatement,
    IfStatement,

    Return(u32),

    Discard,
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
            UnaryOperator::Plus => Self::Plus,
            UnaryOperator::Negation => Self::Negation,
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
            BinaryOperator::RightShift => Self::RightShift,
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

impl From<LogicalOperator> for CompileCommand {
    fn from(value: LogicalOperator) -> Self {
        match value {
            LogicalOperator::LogicalAnd => Self::LogicalAnd,
            LogicalOperator::LogicalOr => Self::LogicalOr,
            LogicalOperator::Nullish => Self::Nullish,
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
            AssignmentOperator::RightShiftAssignment => Self::RightShiftAssignment,
            AssignmentOperator::UnsignedRightShiftAssignment => Self::UnsignedRightShiftAssignment,
            AssignmentOperator::BitwiseAndAssignment => Self::BitwiseAndAssignment,
            AssignmentOperator::BitwiseXorAssignment => Self::BitwiseXorAssignment,
            AssignmentOperator::BitwiseOrAssignment => Self::BitwiseOrAssignment,
            AssignmentOperator::ExponentiationAssignment => Self::ExponentiationAssignment,
            AssignmentOperator::LogicalAndAssignment => Self::LogicalAndAssignment,
            AssignmentOperator::LogicalOrAssignment => Self::LogicalOrAssignment,
            AssignmentOperator::NullishCoalescingAssignment => Self::NullishCoalescingAssignment,
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Locator(u32);

impl Locator {
    pub const NONE: Self = Self(0);

    const ARGUMENT_BIT: u32 = 0x00010000;
    const LOCAL_BIT: u32 = 0x00020000;

    const MAX_OFFSET: usize = u8::MAX as usize;
    const MAX_INDEX: usize = u16::MAX as usize;

    #[inline(always)]
    pub fn is_argument(&self) -> bool {
        (self.0 & Self::ARGUMENT_BIT) != 0
    }

    #[inline(always)]
    pub fn is_local(&self) -> bool {
        (self.0 & Self::LOCAL_BIT) != 0
    }

    #[inline(always)]
    pub fn offset(&self) -> u8 {
        (self.0 >> 24) as u8
    }

    #[inline(always)]
    pub fn flags(&self) -> u8 {
        ((self.0 >> 16) & 0x000000FF) as u8
    }

    #[inline(always)]
    pub fn index(&self) -> u16 {
        (self.0 & 0x0000FFFF) as u16
    }

    pub fn checked_argument(offset: usize, index: usize) -> Option<Self> {
        Self::checked_new(Self::ARGUMENT_BIT, offset, index)
    }

    #[allow(dead_code)]
    pub const fn local(offset: usize, index: usize) -> Self {
        Self::new(Self::LOCAL_BIT, offset, index)
    }

    pub fn checked_local(offset: usize, index: usize) -> Option<Self> {
        Self::checked_new(Self::LOCAL_BIT, offset, index)
    }

    const fn new(flags: u32, offset: usize, index: usize) -> Self {
        Self(flags | (offset as u32) << 24 | index as u32)
    }

    fn checked_new(flags: u32, offset: usize, index: usize) -> Option<Self> {
        if offset > Self::MAX_OFFSET {
            logger::error!(err = "too large", offset);
            return None;
        }
        if index > Self::MAX_INDEX {
            logger::error!(err = "too large", index);
            return None;
        }
        Some(Self::new(flags, offset, index))
    }
}

impl From<u32> for Locator {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl std::fmt::Debug for Locator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let offset = self.offset();
        let index = self.index();
        if self.is_argument() {
            write!(f, "Locator::Argument({offset}, {index})")
        } else if self.is_local() {
            write!(f, "Locator::Local({offset}, {index})")
        } else {
            write!(f, "Locator::None")
        }
    }
}

#[derive(Debug)]
struct Reference {
    symbol: Symbol,
    func_index: u32,
    scope_ref: ScopeRef,
    command_index: usize,
}

#[cfg(test)]
mod tests {
    use jsparser::Parser;
    use jsparser::Processor;

    use super::*;

    #[test]
    fn test_locator_size() {
        assert_eq!(std::mem::size_of::<Locator>(), 4);
    }

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
