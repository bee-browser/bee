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

pub struct Program {
    pub functions: Vec<FunctionRecipe>,
}

pub struct FunctionRecipe {
    pub symbol: Symbol,
    pub formal_parameters: Vec<Symbol>,
    pub commands: Vec<CompileCommand>,
}

pub struct Analyzer<'r> {
    symbol_registry: &'r mut SymbolRegistry,
    context_stack: Vec<FunctionContext>,
    functions: Vec<FunctionRecipe>,
}

impl<'r> Analyzer<'r> {
    pub fn new(symbol_registry: &'r mut SymbolRegistry) -> Self {
        Self {
            symbol_registry,
            context_stack: vec![Default::default()],
            functions: vec![FunctionRecipe {
                symbol: 0.into(),
                formal_parameters: vec![],
                commands: vec![],
            }],
        }
    }

    fn handle_node<'s>(&mut self, node: Node<'s>) {
        logger::debug!(event = "handle_node", ?node);
        match node {
            Node::Null => self.handle_null(),
            Node::Boolean(value) => self.handle_boolean(value),
            Node::Number(value, ..) => self.handle_number(value),
            Node::String(value, ..) => self.handle_string(value),
            Node::IdentifierReference(symbol) => self.handle_identifier_reference(symbol),
            Node::BindingIdentifier(symbol) => self.handle_binding_identifier(symbol),
            Node::LabelIdentifier(_symbol) => {
                // TODO
            }
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
            Node::LetDeclaration(_n) => (),
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
            Node::StartScope => self.handle_start_scope(),
            Node::EndScope => self.handle_end_scope(),
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
        self.context_stack.last_mut().unwrap().put_reference(symbol);
    }

    fn handle_binding_identifier(&mut self, symbol: Symbol) {
        self.context_stack.last_mut().unwrap().put_binding(symbol);
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
            .put_mutable_binding(init);
    }

    fn handle_const_declaration(&mut self, n: u32) {
        self.context_stack
            .last_mut()
            .unwrap()
            .into_immutable_bindings(n);
    }

    fn handle_binding_element(&mut self, init: bool) {
        self.context_stack
            .last_mut()
            .unwrap()
            .put_binding_element(init);
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
        self.context_stack
            .last_mut()
            .unwrap()
            .into_formal_parameter();
    }

    fn handle_formal_parameters(&mut self, _n: u32) {
        // TODO
    }

    fn handle_function_declaration(&mut self) {
        let mut context = self.context_stack.pop().unwrap();
        context.end_scope(true);
        // TODO: remaining references must be handled as var bindings w/ undefined value.
        context.commands[0] = CompileCommand::Bindings(context.max_bindings as u16);
        self.functions[context.func_id as usize].commands = context.commands;
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

    fn handle_start_scope(&mut self) {
        self.context_stack.last_mut().unwrap().start_scope();
    }

    fn handle_end_scope(&mut self) {
        self.context_stack.last_mut().unwrap().end_scope(false);
    }

    fn handle_function_context(&mut self) {
        let func_id = self.functions.len() as u32;
        let mut context = FunctionContext {
            func_id,
            ..Default::default()
        };
        context.start_scope();
        self.context_stack.push(context);
        self.functions.push(FunctionRecipe {
            symbol: 0.into(),
            formal_parameters: vec![],
            commands: vec![CompileCommand::Nop],
        });
    }

    fn handle_function_signature(&mut self, symbol: Symbol) {
        let context = self.context_stack.last().unwrap();
        let func_id = context.func_id as usize;
        self.functions[func_id].symbol = symbol;
        self.functions[func_id].formal_parameters = context.formal_parameters.clone();
    }
}

impl<'r, 's> NodeHandler<'s> for Analyzer<'r> {
    type Artifact = Program;

    fn start(&mut self) {
        logger::debug!(event = "start");
        let context = self.context_stack.last_mut().unwrap();
        // Push `Nop` as a placeholder.
        // It will be replaced with `Bindings` in `accept()`.
        context.commands.push(CompileCommand::Nop);
        context.start_scope();
    }

    fn accept(&mut self) -> Result<Self::Artifact, Error> {
        logger::debug!(event = "accept");
        let mut context = self.context_stack.pop().unwrap();
        context.end_scope(true);
        // TODO: remaining references must be handled as var bindings w/ undefined value.
        context.commands[0] = CompileCommand::Bindings(context.max_bindings as u16);
        self.functions[context.func_id as usize].commands = context.commands;
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
    func_id: u32,
    commands: Vec<CompileCommand>,
    bindings: Vec<(Symbol, usize)>,
    references: Vec<Reference>,
    formal_parameters: Vec<Symbol>,
    scope_stack: Vec<Scope>,
    nargs_stack: Vec<(usize, u16)>,
    max_bindings: usize,
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

    fn put_reference(&mut self, symbol: Symbol) {
        // The location may be updated later.
        let command_index = self.put_command(CompileCommand::Reference(symbol, Locator::Global));
        self.references.push(Reference {
            command_index,
            symbol,
            stack_offset: 0,
        });
    }

    fn put_binding(&mut self, symbol: Symbol) {
        // The location may be updated later.
        let command_index = self.put_command(CompileCommand::Reference(symbol, Locator::Global));
        self.references.push(Reference {
            command_index,
            symbol,
            stack_offset: 0,
        });
        // The index of the command will be set later.
        self.bindings.push((symbol, 0));
    }

    fn into_formal_parameter(&mut self) {
        let (symbol, _) = self.bindings.pop().unwrap();
        self.formal_parameters.push(symbol);
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

    fn put_mutable_binding(&mut self, init: bool) {
        let binding = self.bindings.last_mut().unwrap();
        if !init {
            // Set undefined as the initial value.
            self.commands.push(CompileCommand::Undefined);
        }
        binding.1 = self.commands.len();
        // The offset will be set later.
        self.commands.push(CompileCommand::MutableBinding);
        // TODO: type info
    }

    fn into_immutable_bindings(&mut self, n: u32) {
        let n = n as usize;
        debug_assert!(self.bindings.len() >= n);
        let i = self.bindings.len() - n;
        for (_, index) in self.bindings[i..].iter().cloned() {
            debug_assert!(matches!(
                self.commands[index],
                CompileCommand::MutableBinding
            ));
            self.commands[index] = CompileCommand::ImmutableBinding;
        }
    }

    fn put_binding_element(&mut self, init: bool) {
        if init {
            // TODO
        } else {
            debug_assert!(matches!(
                self.commands.last(),
                Some(CompileCommand::Reference(..))
            ));
            self.commands.pop();
        }
    }

    fn start_scope(&mut self) {
        let index = self.commands.len();
        // Push `Nop` as a placeholder.
        // It will be replaced with `ResetBindings` in `end_scope()`.
        self.commands.push(CompileCommand::Nop);
        self.scope_stack.push(Scope {
            command_base_index: index,
            binding_base_index: self.bindings.len(),
            reference_base_index: self.references.len(),
        });
    }

    fn end_scope(&mut self, last: bool) {
        let scope = self.scope_stack.pop().unwrap();
        let bindings = &mut self.bindings[scope.binding_base_index..];

        let num_bindings = bindings.len() as u16;
        self.commands.push(CompileCommand::EndScope(num_bindings));
        self.commands[scope.command_base_index] = CompileCommand::StartScope(num_bindings);

        // Sort the list by symbol for binary search.
        bindings.sort_unstable_by_key(|(symbol, _)| *symbol);

        // Set the offset field of each reference.
        let references = &mut self.references[scope.reference_base_index..];
        let mut i = 0;
        for j in 0..references.len() {
            let reference = &references[j];
            match bindings.binary_search_by_key(&reference.symbol, |(symbol, _)| *symbol) {
                Ok(offset) => {
                    let offset = offset as u16;
                    match self.commands[reference.command_index] {
                        CompileCommand::Reference(symbol, ref mut locator) => {
                            debug_assert_eq!(symbol, reference.symbol);
                            *locator = Locator::Local(reference.stack_offset, offset);
                        }
                        _ => unreachable!(),
                    }
                }
                Err(_) => {
                    references.swap(i, j);
                    references[i].stack_offset += 1;
                    i += 1;
                }
            }
        }

        if last {
            let n = i;
            i = 0;
            for j in 0..n {
                let reference = &references[j];
                match self
                    .formal_parameters
                    .iter()
                    .position(|ref symbol| reference.symbol.eq(symbol))
                {
                    Some(index) => {
                        let index = index as u16;
                        match self.commands[reference.command_index] {
                            CompileCommand::Reference(symbol, ref mut locator) => {
                                debug_assert_eq!(symbol, reference.symbol);
                                *locator = Locator::Argument(index);
                            }
                            _ => unreachable!(),
                        }
                    }
                    None => {
                        references.swap(i, j);
                        i += 1;
                    }
                }
            }
        }

        self.max_bindings = self.max_bindings.max(self.bindings.len());

        self.bindings.truncate(scope.binding_base_index);
        self.references.truncate(scope.reference_base_index + i);
    }
}

struct Reference {
    command_index: usize,
    symbol: Symbol,
    stack_offset: u16,
}

struct Scope {
    command_base_index: usize,
    binding_base_index: usize,
    reference_base_index: usize,
}

#[derive(Debug, PartialEq)]
pub enum CompileCommand {
    Nop,
    Undefined,
    Null,
    Boolean(bool),
    Number(f64),
    String(Vec<u16>),

    Reference(Symbol, Locator),
    Bindings(u16),
    MutableBinding,
    ImmutableBinding,
    Arguments(u16),
    Argument(u16),
    Call(u16),
    StartScope(u16),
    EndScope(u16),

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

#[derive(Debug, PartialEq)]
pub enum Locator {
    Global,
    Argument(u16),
    Local(u16, u16),
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

    macro_rules! locator {
        (g) => {
            Locator::Global
        };
        (argument: $index:expr) => {
            Locator::Argument($index)
        };
        (local: $stack:expr, $offset:expr) => {
            Locator::Local($stack, $offset)
        };
    }

    #[test]
    fn test_lexical_declarations() {
        test("let a, b = 2; const c = 3, d = 4;", |reg, program| {
            assert_eq!(
                program.functions[0].commands,
                [
                    CompileCommand::Bindings(4),
                    CompileCommand::StartScope(4),
                    CompileCommand::Reference(symbol!(reg, "a"), locator!(local: 0, 0)),
                    CompileCommand::Undefined,
                    CompileCommand::MutableBinding,
                    CompileCommand::Reference(symbol!(reg, "b"), locator!(local: 0, 1)),
                    CompileCommand::Number(2.0),
                    CompileCommand::MutableBinding,
                    CompileCommand::Reference(symbol!(reg, "c"), locator!(local: 0, 2)),
                    CompileCommand::Number(3.0),
                    CompileCommand::ImmutableBinding,
                    CompileCommand::Reference(symbol!(reg, "d"), locator!(local: 0, 3)),
                    CompileCommand::Number(4.0),
                    CompileCommand::ImmutableBinding,
                    CompileCommand::EndScope(4),
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
                    CompileCommand::StartScope(1),
                    CompileCommand::Reference(symbol!(reg, "a"), locator!(local: 0, 0)),
                    CompileCommand::Undefined,
                    CompileCommand::MutableBinding,
                    CompileCommand::StartScope(1),
                    CompileCommand::Reference(symbol!(reg, "a"), locator!(local: 0, 0)),
                    CompileCommand::Undefined,
                    CompileCommand::MutableBinding,
                    CompileCommand::EndScope(1),
                    CompileCommand::StartScope(2),
                    CompileCommand::Reference(symbol!(reg, "a"), locator!(local: 0, 0)),
                    CompileCommand::Undefined,
                    CompileCommand::MutableBinding,
                    CompileCommand::Reference(symbol!(reg, "b"), locator!(local: 0, 1)),
                    CompileCommand::Undefined,
                    CompileCommand::MutableBinding,
                    CompileCommand::EndScope(2),
                    CompileCommand::EndScope(1),
                ]
            );
        });
    }

    fn test(regc: &str, validate: fn(symbol_registry: &SymbolRegistry, program: &Program)) {
        let mut symbol_registry = Default::default();
        let result = Parser::for_script(
            regc,
            Processor::new(Analyzer::new(&mut symbol_registry), false),
        )
        .parse();
        assert!(result.is_ok());
        if let Ok(program) = result {
            validate(&symbol_registry, &program)
        }
    }
}
