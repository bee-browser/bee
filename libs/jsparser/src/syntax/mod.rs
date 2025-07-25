logging::define_logger! {"bee::jsparser::syntax"}

mod actions;

use std::ops::Range;

use bitflags::bitflags;
use smallvec::SmallVec;
use smallvec::smallvec;

use crate::Error;
use crate::Location;
use crate::Parser;
use crate::ProductionRule;
use crate::Symbol;
use crate::SyntaxHandler;
use crate::Token;
use crate::TokenKind;
use crate::parser::GoalSymbol;

const MAX_ITERATION_STATEMENT_DEPTH: usize = u16::MAX as usize;
const MAX_SWITCH_STATEMENT_DEPTH: usize = u16::MAX as usize;

macro_rules! ensure {
    ($cond:expr) => {
        if !$cond {
            return Err(Error::SyntaxError);
        }
    };
}

pub trait NodeHandler<'s> {
    type Artifact;

    fn start(&mut self);
    fn accept(&mut self) -> Result<Self::Artifact, Error>;
    fn handle_nodes(&mut self, nodes: impl Iterator<Item = Node<'s>>) -> Result<(), Error>;
    fn make_symbol(&mut self, lexeme: &str) -> Symbol;
}

pub struct Processor<'s, H> {
    handler: H,
    source: &'s str,
    location: Location,
    stack: Vec<Syntax>,
    nodes: Vec<Node<'s>>,
    tokens: Vec<Token<'s>>,

    label_stack: Vec<Label>,

    // It's enough to track an *outermost* iteration/switch statement for conformance with the
    // specification, but it cannot be implemented easily.  Instead, we simply count the nesting
    // level of iteration/switch statements.
    iteration_statement_depth: usize,
    switch_statement_depth: usize,

    strict_mode: bool,
    module: bool,
}

#[derive(Debug)]
struct Syntax {
    detail: Detail,
    nodes_range: Range<usize>,
    tokens_range: Range<usize>,
    source_range: Range<usize>,
}

#[derive(Debug)]
enum Detail {
    Token(usize),
    Literal,
    TemplateString,
    TemplateLiteral(u16),
    Identifier(Symbol),
    IdentifierReference(#[allow(unused)] Symbol), // TODO: SS
    BindingIdentifier(Symbol),
    LabelIdentifier(Symbol),
    CpeaaplExpression,
    CpeaaplFormalParameters,
    CpeaaplEmpty,
    CpeaaplRestParameter,
    CpeaaplRestPattern,
    CpeaaplFormalParametersWithRestParameter,
    CpeaaplFormalParametersWithRestPattern,
    Arguments,
    ArgumentList,
    OptionalChain,
    Expression,
    ArrayLiteral,
    ElementList(u32, bool),
    ArrayInitializerElision(u32),
    SpreadElement,
    ObjectLiteral,
    PropertyDefinition(Symbol),
    PropertyDefinitionList(bool),
    CoverInitializedName,
    Initializer,
    Block,
    Binding(DeclarationSemantics),
    BindingList(DeclarationSemantics),
    LetDeclaration(#[allow(unused)] SmallVec<[Symbol; 4]>), // TODO: SS
    ConstDeclaration(#[allow(unused)] SmallVec<[Symbol; 4]>), // TODO: SS
    SingleNameBinding(Symbol, bool),
    BindingElement(BindingElement),
    CaseBlock,
    CaseClause,
    CaseClauseList,
    DefaultClause,
    CatchClause,
    FinallyClause,
    BlockStatement,
    VariableStatement(#[allow(unused)] SmallVec<[Symbol; 4]>), // TODO: SS
    EmptyStatement,
    ExpressionStatement,
    IfStatement,
    DoWhileStatement,
    WhileStatement,
    ForStatement,
    //ForInStatement,
    //ForOfStatement,
    SwitchStatement,
    ContinueStatement,
    BreakStatement,
    ReturnStatement,
    //WithStatement,
    LabelledStatement(LabelledItem),
    ThrowStatement,
    TryStatement,
    DebuggerStatement,
    Declaration,
    FormalParameters(SmallVec<[Symbol; 4]>),
    ConciseBody,
    AsyncConciseBody,
    StatementList,
    CoverCallExpressionAndAsyncArrowHead,
    ModuleItemList,
}

#[derive(Clone, Copy, Debug)]
enum LabelledItem {
    IterationStatement,
    OtherStatement,
}

#[derive(Debug)]
struct DeclarationSemantics {
    bound_names: SmallVec<[Symbol; 4]>,
    has_initializer: bool,
}

#[derive(Debug)]
struct BindingElement {
    kind: BindingElementKind,
    #[allow(unused)] // TODO: array/object patterns
    has_initializer: bool,
}

#[derive(Debug)]
enum BindingElementKind {
    SingleNameBinding(Symbol),
}

/// Represents a node in a stream of ordered nodes visited in a depth-first tree traversal on an
/// AST for a JavaScript program.
///
/// A stack machine can reconstruct the AST of the Javascript program from the stream of the nodes.
#[derive(Debug)]
pub enum Node<'s> {
    Null,
    Boolean(bool),
    Number(f64, &'s str),
    String(Vec<u16>, &'s str),
    TemplateLiteral(u16),
    Array,
    Object,
    LiteralPropertyName(LiteralPropertyName),
    PropertyDefinition(PropertyDefinitionKind),
    MemberExpression(MemberExpressionKind),
    This,
    IdentifierReference(Symbol),
    BindingIdentifier(Symbol),
    ArgumentListHead(bool, bool),
    ArgumentListItem(bool),
    Arguments,
    CallExpression,
    NewExpression(bool),
    NonNullish,
    OptionalChain(PropertyAccessKind),
    UpdateExpression(UpdateOperator),
    UnaryExpression(UnaryOperator),
    BinaryExpression(BinaryOperator),
    LogicalExpression(LogicalOperator),
    ConditionalExpression,
    AssignmentExpression(AssignmentOperator),
    SequenceExpression,
    BlockStatement,
    LexicalBinding(bool),
    LetDeclaration(u32),
    ConstDeclaration(u32),
    VariableDeclaration(bool),
    VariableStatement(u32),
    BindingElement(bool),
    EmptyStatement,
    ExpressionStatement,
    IfElseStatement,
    IfStatement,
    DoWhileStatement,
    WhileStatement,
    ForStatement(LoopFlags),
    ContinueStatement(Symbol),
    BreakStatement(Symbol),
    ReturnStatement(u32),
    SwitchStatement,
    CaseBlock,
    CaseSelector,
    CaseClause(bool),
    DefaultSelector,
    DefaultClause(bool),
    LabelledStatement(Symbol, bool),
    Label(Symbol),
    ThrowStatement,
    TryStatement,
    CatchClause(bool),
    FinallyClause,
    CatchParameter,
    TryBlock,
    CatchBlock,
    FinallyBlock,
    DebuggerStatement,
    FormalParameter,
    FormalParameters(u32),
    FunctionContext(Symbol),
    AsyncFunctionContext(Symbol),
    ArrowFunctionContext,
    AsyncArrowFunctionContext,
    FunctionSignature,
    FunctionDeclaration,
    AsyncFunctionDeclaration,
    FunctionExpression(bool),
    AsyncFunctionExpression(bool),
    ArrowFunction,
    AsyncArrowFunction,
    AwaitExpression,
    Then(bool),
    Else(bool),
    FalsyShortCircuit,
    TruthyShortCircuit,
    NullishShortCircuit,
    FalsyShortCircuitAssignment,
    TruthyShortCircuitAssignment,
    NullishShortCircuitAssignment,
    LoopStart,
    LoopInitExpression,
    LoopInitVarDeclaration,
    LoopInitLexicalDeclaration,
    LoopTest,
    LoopNext,
    LoopBody,
    StartBlockScope,
    EndBlockScope,
    Dereference,
    ToString,
}

#[derive(Clone, Debug)]
pub enum PropertyDefinitionKind {
    ArrayElement,
    ArrayEmptySlot,
    ArraySpread,
    Reference,
    KeyValue,
    Spread,
}

#[derive(Clone, Debug)]
pub enum LiteralPropertyName {
    IdentifierName(Symbol),
}

#[derive(Clone, Debug)]
pub enum MemberExpressionKind {
    PropertyAccessWithExpressionKey,
    PropertyAccessWithIdentifierKey(Symbol),
}

#[derive(Clone, Debug)]
pub enum PropertyAccessKind {
    Call,
    IdentifierKey(Symbol),
}

#[derive(Clone, Copy)]
pub enum UpdateOperator {
    PostfixIncrement,
    PostfixDecrement,
    PrefixIncrement,
    PrefixDecrement,
}

impl std::fmt::Debug for UpdateOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::PostfixIncrement => "_++",
            Self::PostfixDecrement => "_--",
            Self::PrefixIncrement => "++_",
            Self::PrefixDecrement => "--_",
        })
    }
}

#[derive(Clone, Copy)]
pub enum UnaryOperator {
    Delete,
    Void,
    Typeof,
    Plus,
    Minus,
    BitwiseNot,
    LogicalNot,
}

impl std::fmt::Debug for UnaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Delete => "delete",
            Self::Void => "void",
            Self::Typeof => "typeof",
            Self::Plus => "+",
            Self::Minus => "-",
            Self::BitwiseNot => "~",
            Self::LogicalNot => "!",
        })
    }
}

#[derive(Clone, Copy)]
pub enum BinaryOperator {
    Equality,
    Inequality,
    StrictEquality,
    StrictInequality,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LeftShift,
    SignedRightShift,
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
}

impl std::fmt::Debug for BinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Equality => "==",
            Self::Inequality => "!=",
            Self::StrictEquality => "===",
            Self::StrictInequality => "!==",
            Self::LessThan => "<",
            Self::LessThanOrEqual => "<=",
            Self::GreaterThan => ">",
            Self::GreaterThanOrEqual => ">=",
            Self::LeftShift => "<<",
            Self::SignedRightShift => ">>",
            Self::UnsignedRightShift => ">>>",
            Self::Addition => "+",
            Self::Subtraction => "-",
            Self::Multiplication => "*",
            Self::Division => "/",
            Self::Remainder => "%",
            Self::BitwiseOr => "|",
            Self::BitwiseXor => "^",
            Self::BitwiseAnd => "&",
            Self::In => "in",
            Self::Instanceof => "instanceof",
            Self::Exponentiation => "**",
        })
    }
}

#[derive(Clone, Copy)]
pub enum LogicalOperator {
    LogicalAnd,
    LogicalOr,
    NullishCoalescing,
}

impl std::fmt::Debug for LogicalOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::LogicalAnd => "&&",
            Self::LogicalOr => "||",
            Self::NullishCoalescing => "??",
        })
    }
}

#[derive(Clone, Copy)]
pub enum AssignmentOperator {
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
    LogicalAndAssignment,
    LogicalOrAssignment,
    NullishCoalescingAssignment,
}

impl std::fmt::Debug for AssignmentOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Assignment => "=",
            Self::MultiplicationAssignment => "*=",
            Self::DivisionAssignment => "/=",
            Self::RemainderAssignment => "%=",
            Self::AdditionAssignment => "+=",
            Self::SubtractionAssignment => "-=",
            Self::LeftShiftAssignment => "<<=",
            Self::SignedRightShiftAssignment => ">>=",
            Self::UnsignedRightShiftAssignment => ">>>=",
            Self::BitwiseAndAssignment => "&=",
            Self::BitwiseXorAssignment => "^=",
            Self::BitwiseOrAssignment => "|=",
            Self::ExponentiationAssignment => "**=",
            Self::LogicalAndAssignment => "&&=",
            Self::LogicalOrAssignment => "||=",
            Self::NullishCoalescingAssignment => "??=",
        })
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct LoopFlags : u16 {
        const HAS_INIT = 0b0001;
        const HAS_TEST = 0b0010;
        const HAS_NEXT = 0b0100;
    }
}

impl<'s, H> Processor<'s, H>
where
    H: NodeHandler<'s>,
{
    const INITIAL_STACK_CAPACITY: usize = 64;
    const INITIAL_QUEUE_CAPACITY: usize = 128;
    const INITIAL_TOKENS_CAPACITY: usize = 1024;
    const INITIAL_LABEL_STACK_CAPACITY: usize = 8;

    pub fn new(handler: H, module: bool) -> Self {
        Self {
            handler,
            source: "",
            location: Default::default(),
            stack: Vec::with_capacity(Self::INITIAL_STACK_CAPACITY),
            nodes: Vec::with_capacity(Self::INITIAL_QUEUE_CAPACITY),
            tokens: Vec::with_capacity(Self::INITIAL_TOKENS_CAPACITY),
            label_stack: Vec::with_capacity(Self::INITIAL_LABEL_STACK_CAPACITY),
            iteration_statement_depth: 0,
            switch_statement_depth: 0,
            strict_mode: false,
            module,
        }
    }

    fn top(&self) -> &Syntax {
        self.nth(0)
    }

    fn top_mut(&mut self) -> &mut Syntax {
        self.nth_mut(0)
    }

    fn nth(&self, n: usize) -> &Syntax {
        let len = self.stack.len();
        debug_assert!(len > n);
        &self.stack[len - n - 1]
    }

    fn nth_mut(&mut self, n: usize) -> &mut Syntax {
        let len = self.stack.len();
        debug_assert!(len > n);
        &mut self.stack[len - n - 1]
    }

    fn push(&mut self, syntax: Syntax) {
        self.stack.push(syntax);
    }

    fn pop(&mut self) -> Syntax {
        self.stack.pop().unwrap()
    }

    fn replace(&mut self, n: usize, detail: Detail) {
        debug_assert!(n > 0);
        let source_end = self.top().source_range.end;
        let nodes_end = self.nodes.len();
        let tokens_end = self.tokens.len();
        self.stack.truncate(self.stack.len() - (n - 1));
        let syntax = self.stack.last_mut().unwrap();
        syntax.detail = detail;
        syntax.nodes_range.end = nodes_end;
        syntax.tokens_range.end = tokens_end;
        syntax.source_range.end = source_end;
    }

    fn update_ends(&mut self) {
        let source_end = self.top().source_range.end;
        let nodes_end = self.nodes.len();
        let tokens_end = self.tokens.len();
        let syntax = self.top_mut();
        syntax.nodes_range.end = nodes_end;
        syntax.tokens_range.end = tokens_end;
        syntax.source_range.end = source_end;
    }

    fn enqueue(&mut self, event: Node<'s>) -> usize {
        let index = self.nodes.len();
        self.nodes.push(event);
        index
    }

    fn refine(&mut self, syntax: &Syntax, goal_symbol: GoalSymbol) -> Result<(), Error> {
        logger::debug!(
            event = "refine",
            ?syntax.detail,
            ?syntax.tokens_range,
            ?syntax.nodes_range,
            ?syntax.source_range,
            ?goal_symbol,
        );
        let src = self.src(syntax.source_range.clone());
        Parser::new(
            src,
            Refinery::new(self, syntax.source_range.start),
            goal_symbol,
        )
        .parse()
    }

    fn make_symbol(&mut self, token_index: usize) -> Symbol {
        let lexeme = self.tokens[token_index].lexeme;
        self.handler.make_symbol(lexeme)
    }

    fn src(&self, range: Range<usize>) -> &'s str {
        &self.source[range]
    }
}

// Static Semantics

impl<'s, H> Processor<'s, H>
where
    H: NodeHandler<'s>,
{
    // Commonly used actions.

    // BindingIdentifier_Yield : yield
    // BindingIdentifier_Yield_Await : yield
    // BindingIdentifier_Await : await
    // BindingIdentifier_Yield_Await : await
    fn syntax_error(&mut self) -> Result<(), Error> {
        Err(Error::SyntaxError)
    }

    // _TO_STRING_
    fn process_to_string(&mut self) -> Result<(), Error> {
        self.enqueue(Node::ToString);
        Ok(())
    }

    // _DEREFERENCE_
    fn process_dereference(&mut self) -> Result<(), Error> {
        self.enqueue(Node::Dereference);
        Ok(())
    }

    // _THEN_
    fn process_then(&mut self) -> Result<(), Error> {
        self.enqueue(Node::Then(false));
        Ok(())
    }

    // _ELSE_
    fn process_else(&mut self) -> Result<(), Error> {
        self.enqueue(Node::Else(false));
        Ok(())
    }

    // _THEN_EXPR_
    fn process_then_expr(&mut self) -> Result<(), Error> {
        self.enqueue(Node::Then(true));
        Ok(())
    }

    // _ELSE_EXPR_
    fn process_else_expr(&mut self) -> Result<(), Error> {
        self.enqueue(Node::Else(true));
        Ok(())
    }

    // _BLOCK_SCOPE_
    fn process_block_scope(&mut self) -> Result<(), Error> {
        self.enqueue(Node::StartBlockScope);
        Ok(())
    }

    // _NEW_ARRAY_
    fn process_new_array(&mut self) -> Result<(), Error> {
        self.enqueue(Node::Array);
        Ok(())
    }

    // _NEW_OBJECT_
    fn process_new_object(&mut self) -> Result<(), Error> {
        self.enqueue(Node::Object);
        Ok(())
    }

    // _NON_NULLISH_
    fn process_non_nullish(&mut self) -> Result<(), Error> {
        self.enqueue(Node::NonNullish);
        Ok(())
    }

    // _FUNCTION_CONTEXT_
    fn process_function_context(&mut self) -> Result<(), Error> {
        let name = match self.stack.last().unwrap().detail {
            Detail::BindingIdentifier(symbol) => symbol,
            Detail::Token(index) => {
                debug_assert!(matches!(self.tokens[index].kind, TokenKind::Function));
                Symbol::NONE // anonymous function
            }
            ref detail => unreachable!("{detail:?}"),
        };
        self.enqueue(Node::FunctionContext(name));
        Ok(())
    }

    // _ASYNC_FUNCTION_CONTEXT_
    fn process_async_function_context(&mut self) -> Result<(), Error> {
        let name = match self.stack.last().unwrap().detail {
            Detail::BindingIdentifier(symbol) => symbol,
            Detail::Token(index) => {
                debug_assert!(matches!(self.tokens[index].kind, TokenKind::Function));
                Symbol::NONE // anonymous function
            }
            ref detail => unreachable!("{detail:?}"),
        };
        self.enqueue(Node::AsyncFunctionContext(name));
        Ok(())
    }

    // _FUNCTION_SIGNATURE_
    fn process_function_signature(&mut self) -> Result<(), Error> {
        self.enqueue(Node::FunctionSignature);
        Ok(())
    }

    // _FALSY_SHORT_CIRCUIT_
    fn process_falsy_short_circuit(&mut self) -> Result<(), Error> {
        self.enqueue(Node::FalsyShortCircuit);
        Ok(())
    }

    // _TRUTHY_SHORT_CIRCUIT_
    fn process_truthy_short_circuit(&mut self) -> Result<(), Error> {
        self.enqueue(Node::TruthyShortCircuit);
        Ok(())
    }

    // _NULLISH_SHORT_CIRCUIT_
    fn process_nullish_short_circuit(&mut self) -> Result<(), Error> {
        self.enqueue(Node::NullishShortCircuit);
        Ok(())
    }

    // _FALSY_SHORT_CIRCUIT_ASSIGNMENT_
    fn process_falsy_short_circuit_assignment(&mut self) -> Result<(), Error> {
        self.enqueue(Node::FalsyShortCircuitAssignment);
        Ok(())
    }

    // _TRUTHY_SHORT_CIRCUIT_ASSIGNMENT_
    fn process_truthy_short_circuit_assignment(&mut self) -> Result<(), Error> {
        self.enqueue(Node::TruthyShortCircuitAssignment);
        Ok(())
    }

    // _NULLISH_SHORT_CIRCUIT_ASSIGNMENT_
    fn process_nullish_short_circuit_assignment(&mut self) -> Result<(), Error> {
        self.enqueue(Node::NullishShortCircuitAssignment);
        Ok(())
    }

    // 13.1 Identifiers

    // IdentifierReference : Identifier
    // IdentifierReference_Yield : await
    // IdentifierReference_Await : yield
    fn process_identifier_reference(&mut self) -> Result<(), Error> {
        let symbol = match self.top().detail {
            Detail::Identifier(symbol) => symbol,
            _ => unreachable!(),
        };
        self.enqueue(Node::IdentifierReference(symbol));
        self.replace(1, Detail::IdentifierReference(symbol));
        Ok(())
    }

    // IdentifierReference : yield
    fn process_identifier_reference_only_in_non_strict(&mut self) -> Result<(), Error> {
        // 13.1.1 Static Semantics: Early Errors
        ensure!(!self.strict_mode);
        self.process_identifier_reference()
    }

    // IdentifierReference : await
    fn process_identifier_reference_only_in_script(&mut self) -> Result<(), Error> {
        // 13.1.1 Static Semantics: Early Errors
        ensure!(!self.module);
        self.process_identifier_reference()
    }

    // IdentifierReference_Await : Identifier
    fn process_identifier_reference_except_for_await(&mut self) -> Result<(), Error> {
        let symbol = match self.top().detail {
            Detail::Identifier(symbol) => symbol,
            _ => unreachable!(),
        };
        // 13.1.1 Static Semantics: Early Errors
        ensure!(!matches!(symbol, Symbol::AWAIT));
        self.process_identifier_reference()
    }

    // IdentifierReference_Yield : Identifier
    fn process_identifier_reference_except_for_yield(&mut self) -> Result<(), Error> {
        let symbol = match self.top().detail {
            Detail::Identifier(symbol) => symbol,
            _ => unreachable!(),
        };
        // 13.1.1 Static Semantics: Early Errors
        ensure!(!matches!(symbol, Symbol::YIELD));
        self.process_identifier_reference()
    }

    // IdentifierReference_Yield_Await : Identifier
    fn process_identifier_reference_except_for_yield_await(&mut self) -> Result<(), Error> {
        let symbol = match self.top().detail {
            Detail::Identifier(symbol) => symbol,
            _ => unreachable!(),
        };
        // 13.1.1 Static Semantics: Early Errors
        ensure!(!matches!(symbol, Symbol::YIELD | Symbol::AWAIT));
        self.process_identifier_reference()
    }

    // BindingIdentifier : yield
    // BindingIdentifier : await
    // BindingIdentifier_YIELD : await
    // BindingIdentifier_AWAIT : yield
    fn process_binding_identifier(&mut self) -> Result<(), Error> {
        let symbol = match self.top().detail {
            Detail::Identifier(symbol) => symbol,
            _ => unreachable!(),
        };
        self.enqueue(Node::BindingIdentifier(symbol));
        self.replace(1, Detail::BindingIdentifier(symbol));
        Ok(())
    }

    // BindingIdentifier : Identifier
    fn process_binding_identifier_except_for_arguments_eval_in_strict(
        &mut self,
    ) -> Result<(), Error> {
        let symbol = match self.top().detail {
            Detail::Identifier(symbol) => symbol,
            _ => unreachable!(),
        };
        // 13.1.1 Static Semantics: Early Errors
        ensure!(!matches!(symbol, Symbol::ARGUMENTS | Symbol::EVAL if self.strict_mode));
        self.process_binding_identifier()
    }

    // BindingIdentifier : yield
    fn process_binding_identifier_only_in_non_strict(&mut self) -> Result<(), Error> {
        // 13.1.1 Static Semantics: Early Errors
        ensure!(!self.strict_mode);
        self.process_binding_identifier()
    }

    // BindingIdentifier : await
    fn process_binding_identifier_only_in_script(&mut self) -> Result<(), Error> {
        // 13.1.1 Static Semantics: Early Errors
        ensure!(!self.module);
        self.process_binding_identifier()
    }

    // BindingIdentifier_Await : Identifier
    fn process_binding_identifier_except_for_await(&mut self) -> Result<(), Error> {
        let symbol = match self.top().detail {
            Detail::Identifier(symbol) => symbol,
            _ => unreachable!(),
        };
        // 13.1.1 Static Semantics: Early Errors
        ensure!(!matches!(symbol, Symbol::AWAIT));
        self.process_binding_identifier()
    }

    // BindingIdentifier_Yield : Identifier
    fn process_binding_identifier_except_for_yield(&mut self) -> Result<(), Error> {
        let symbol = match self.top().detail {
            Detail::Identifier(symbol) => symbol,
            _ => unreachable!(),
        };
        // 13.1.1 Static Semantics: Early Errors
        ensure!(!matches!(symbol, Symbol::YIELD));
        self.process_binding_identifier()
    }

    // BindingIdentifier_Yield_Await : Identifier
    fn process_binding_identifier_except_for_yield_await(&mut self) -> Result<(), Error> {
        let symbol = match self.top().detail {
            Detail::Identifier(symbol) => symbol,
            _ => unreachable!(),
        };
        // 13.1.1 Static Semantics: Early Errors
        ensure!(!matches!(symbol, Symbol::YIELD | Symbol::AWAIT));
        self.process_binding_identifier()
    }

    // LabelIdentifier : Identifier
    // LabelIdentifier_Yield : await
    // LabelIdentifier_Await : yield
    fn process_label_identifier(&mut self) -> Result<(), Error> {
        let symbol = match self.top().detail {
            Detail::Identifier(symbol) => symbol,
            _ => unreachable!(),
        };
        self.replace(1, Detail::LabelIdentifier(symbol));
        Ok(())
    }

    // LabelIdentifier : yield
    fn process_label_identifier_only_in_non_strict(&mut self) -> Result<(), Error> {
        // 13.1.1 Static Semantics: Early Errors
        ensure!(!self.strict_mode);
        self.process_label_identifier()
    }

    // LabelIdentifier : await
    fn process_label_identifier_only_in_script(&mut self) -> Result<(), Error> {
        // 13.1.1 Static Semantics: Early Errors
        ensure!(!self.module);
        self.process_label_identifier()
    }

    // LabelIdentifier_Await : Identifier
    fn process_label_identifier_except_for_await(&mut self) -> Result<(), Error> {
        let symbol = match self.top().detail {
            Detail::Identifier(symbol) => symbol,
            _ => unreachable!(),
        };
        // 13.1.1 Static Semantics: Early Errors
        ensure!(!matches!(symbol, Symbol::AWAIT));
        self.process_label_identifier()
    }

    // LabelIdentifier_Yield : Identifier
    fn process_label_identifier_except_for_yield(&mut self) -> Result<(), Error> {
        let symbol = match self.top().detail {
            Detail::Identifier(symbol) => symbol,
            _ => unreachable!(),
        };
        // 13.1.1 Static Semantics: Early Errors
        ensure!(!matches!(symbol, Symbol::YIELD));
        self.process_label_identifier()
    }

    // LabelIdentifier_Yield_Await : Identifier
    fn process_label_identifier_except_for_yield_await(&mut self) -> Result<(), Error> {
        let symbol = match self.top().detail {
            Detail::Identifier(symbol) => symbol,
            _ => unreachable!(),
        };
        // 13.1.1 Static Semantics: Early Errors
        ensure!(!matches!(symbol, Symbol::YIELD | Symbol::AWAIT));
        self.process_label_identifier()
    }

    // Identifier :
    //   IdentifierName but not ReservedWord
    fn process_identifier(&mut self) -> Result<(), Error> {
        let token_index = self.tokens.len() - 1;
        let symbol = self.make_symbol(token_index);
        match symbol {
            // 13.1.1 Static Semantics: Early Errors
            Symbol::IMPLEMENTS
            | Symbol::LET
            | Symbol::PACKAGE
            | Symbol::PRIVATE
            | Symbol::PROTECTED
            | Symbol::PUBLIC
            | Symbol::STATIC
            | Symbol::YIELD
                if self.strict_mode =>
            {
                Err(Error::SyntaxError)
            }
            Symbol::AWAIT if self.module => Err(Error::SyntaxError),
            _ => {
                self.top_mut().detail = Detail::Identifier(symbol);
                Ok(())
            }
        }
    }

    // 13.2 Primary Expression

    // 13.2.1 The this Keyword

    // PrimaryExpression[Yield, Await] :
    //   this
    fn process_primary_expression_this(&mut self) -> Result<(), Error> {
        self.enqueue(Node::This);
        self.top_mut().detail = Detail::Expression;
        Ok(())
    }

    // 13.2.2 Identifier Reference

    // PrimaryExpression[Yield, Await] :
    //   IdentifierReference[?Yield, ?Await]
    fn process_primary_expression_identifier_reference(&mut self) -> Result<(), Error> {
        self.top_mut().detail = Detail::Expression;
        Ok(())
    }

    // PrimaryExpression[Yield, Await] :
    //   CoverParenthesizedExpressionAndArrowParameterList[?Yield, ?Await]
    fn process_primary_expression_cpeaapl(&mut self) -> Result<(), Error> {
        match self.top().detail {
            // ParenthesizedExpression[Yield, Await] :
            //   ( Expression[+In, ?Yield, ?Await] )
            Detail::CpeaaplExpression => {
                self.top_mut().detail = Detail::Expression;
                Ok(())
            }
            Detail::CpeaaplFormalParameters
            | Detail::CpeaaplEmpty
            | Detail::CpeaaplRestParameter
            | Detail::CpeaaplRestPattern
            | Detail::CpeaaplFormalParametersWithRestParameter
            | Detail::CpeaaplFormalParametersWithRestPattern => Err(Error::SyntaxError),
            _ => unreachable!(),
        }
    }

    // CoverParenthesizedExpressionAndArrowParameterList[Yield, Await] :
    //   ( Expression[+In, ?Yield, ?Await] )
    fn process_cpeaapl_expression(&mut self) -> Result<(), Error> {
        self.replace(3, Detail::CpeaaplExpression);
        Ok(())
    }

    // CoverParenthesizedExpressionAndArrowParameterList[Yield, Await] :
    //   ( Expression[+In, ?Yield, ?Await] , )
    fn process_cpeaapl_formal_parameters(&mut self) -> Result<(), Error> {
        // TODO: supplemental syntax
        self.replace(4, Detail::CpeaaplFormalParameters);
        Ok(())
    }

    // CoverParenthesizedExpressionAndArrowParameterList[Yield, Await] :
    //   ( )
    fn process_cpeaapl_empty(&mut self) -> Result<(), Error> {
        // TODO: supplemental syntax
        self.replace(2, Detail::CpeaaplEmpty);
        Ok(())
    }

    // CoverParenthesizedExpressionAndArrowParameterList[Yield, Await] :
    //   ( ... BindingIdentifier[?Yield, ?Await] )
    fn process_cpeaapl_rest_parameter(&mut self) -> Result<(), Error> {
        // TODO: supplemental syntax
        self.replace(4, Detail::CpeaaplRestParameter);
        Ok(())
    }

    // CoverParenthesizedExpressionAndArrowParameterList[Yield, Await] :
    //   ( ... BindingPattern[?Yield, ?Await] )
    fn process_cpeaapl_rest_pattern(&mut self) -> Result<(), Error> {
        // TODO: supplemental syntax
        self.replace(4, Detail::CpeaaplRestPattern);
        Ok(())
    }

    // CoverParenthesizedExpressionAndArrowParameterList[Yield, Await] :
    //   ( Expression[+In, ?Yield, ?Await] , ... BindingIdentifier[?Yield, ?Await] )
    fn process_cpeaapl_formal_parameters_with_rest_parameter(&mut self) -> Result<(), Error> {
        // TODO: supplemental syntax
        self.replace(6, Detail::CpeaaplFormalParametersWithRestParameter);
        Ok(())
    }

    // CoverParenthesizedExpressionAndArrowParameterList[Yield, Await] :
    //   ( Expression[+In, ?Yield, ?Await] , ... BindingPattern[?Yield, ?Await] )
    fn process_cpeaapl_formal_parameters_with_rest_pattern(&mut self) -> Result<(), Error> {
        // TODO: supplemental syntax
        self.replace(6, Detail::CpeaaplFormalParametersWithRestPattern);
        Ok(())
    }

    // 13.2.3 Literals

    fn process_literal(&mut self) -> Result<(), Error> {
        let token_index = self.tokens.len() - 1;
        let token = &self.tokens[token_index];
        let node_index = match token.kind {
            TokenKind::Null => self.enqueue(Node::Null),
            TokenKind::True => self.enqueue(Node::Boolean(true)),
            TokenKind::False => self.enqueue(Node::Boolean(false)),
            TokenKind::NumericLiteral => {
                // TODO: perform `NumericValue`
                let value = token.lexeme.parse::<f64>().unwrap();
                self.enqueue(Node::Number(value, token.lexeme))
            }
            TokenKind::StringLiteral => {
                // TODO: perform `SV`
                let content = &token.lexeme[1..(token.lexeme.len() - 1)];
                let value = content.encode_utf16().collect();
                self.enqueue(Node::String(value, token.lexeme))
            }
            _ => unreachable!(),
        };
        let syntax = self.top_mut();
        syntax.detail = Detail::Literal;
        syntax.nodes_range = node_index..(node_index + 1);
        Ok(())
    }

    // 13.2.4 Array Initializer

    // ArrayLiteral[Yield, Await] :
    //   [ ]
    fn process_array_literal_empty(&mut self) -> Result<(), Error> {
        self.replace(2, Detail::ArrayLiteral);
        Ok(())
    }

    // ArrayLiteral[Yield, Await] :
    //   [ ArrayInitializerElision ]
    fn process_array_literal_elision(&mut self) -> Result<(), Error> {
        self.replace(3, Detail::ArrayLiteral);
        Ok(())
    }

    // ArrayLiteral[Yield, Await] :
    //   [ ElementList[?Yield, ?Await] ]
    fn process_array_literal_list(&mut self) -> Result<(), Error> {
        self.replace(3, Detail::ArrayLiteral);
        Ok(())
    }

    // ArrayLiteral[Yield, Await] :
    //   [ ElementList[?Yield, ?Await] , ]
    fn process_array_literal_comma(&mut self) -> Result<(), Error> {
        self.replace(4, Detail::ArrayLiteral);
        Ok(())
    }

    // ArrayLiteral[Yield, Await] :
    //   [ ElementList[?Yield, ?Await] , ArrayInitializerElision ]
    fn process_array_literal_list_elision(&mut self) -> Result<(), Error> {
        self.replace(5, Detail::ArrayLiteral);
        Ok(())
    }

    // ElementList[Yield, Await] :
    //   AssignmentExpression[+In, ?Yield, ?Await]
    fn process_element_list_item(&mut self) -> Result<(), Error> {
        self.top_mut().detail = Detail::ElementList(1, false);
        self.enqueue(Node::PropertyDefinition(
            PropertyDefinitionKind::ArrayElement,
        ));
        Ok(())
    }

    // ElementList[Yield, Await] :
    //   ArrayInitializerElision AssignmentExpression[+In, ?Yield, ?Await]
    fn process_element_list_elision_item(&mut self) -> Result<(), Error> {
        let n = match self.nth(1).detail {
            Detail::ArrayInitializerElision(n) => n,
            ref detail => unreachable!("{detail:?}"),
        };
        self.replace(2, Detail::ElementList(n + 1, false));
        self.enqueue(Node::PropertyDefinition(
            PropertyDefinitionKind::ArrayElement,
        ));
        Ok(())
    }

    // ElementList[Yield, Await] :
    //   SpreadElement[?Yield, ?Await]
    fn process_element_list_spread(&mut self) -> Result<(), Error> {
        self.replace(1, Detail::ElementList(0, true));
        Ok(())
    }

    // ElementList[Yield, Await] :
    //   ArrayInitializerElision SpreadElement[?Yield, ?Await]
    fn process_element_list_elision_spread(&mut self) -> Result<(), Error> {
        let n = match self.nth(1).detail {
            Detail::ArrayInitializerElision(n) => n,
            ref detail => unreachable!("{detail:?}"),
        };
        self.replace(2, Detail::ElementList(n + 1, true));
        Ok(())
    }

    // ElementList[Yield, Await] :
    //   ElementList[?Yield, ?Await] , AssignmentExpression[+In, ?Yield, ?Await]
    fn process_element_list_list_item(&mut self) -> Result<(), Error> {
        self.pop(); // Expression
        self.pop(); // Token(,)
        match self.top_mut().detail {
            Detail::ElementList(ref mut n, _) => *n += 1,
            ref detail => unreachable!("{detail:?}"),
        }
        self.update_ends();
        self.enqueue(Node::PropertyDefinition(
            PropertyDefinitionKind::ArrayElement,
        ));
        Ok(())
    }

    // ElementList[Yield, Await] :
    //   ElementList[?Yield, ?Await] , ArrayInitializerElision
    //   AssignmentExpression[+In, ?Yield, ?Await]
    fn process_element_list_list_elision_item(&mut self) -> Result<(), Error> {
        self.pop(); // Expression
        let n = match self.pop().detail {
            Detail::ArrayInitializerElision(n) => n,
            detail => unreachable!("{detail:?}"),
        };
        self.pop(); // Token(,)
        match self.top_mut().detail {
            Detail::ElementList(ref mut m, _) => *m += n + 1,
            ref detail => unreachable!("{detail:?}"),
        }
        self.update_ends();
        self.enqueue(Node::PropertyDefinition(
            PropertyDefinitionKind::ArrayElement,
        ));
        Ok(())
    }

    // ElementList[Yield, Await] :
    //   ElementList[?Yield, ?Await] , SpreadElement[?Yield, ?Await]
    fn process_element_list_list_spread(&mut self) -> Result<(), Error> {
        self.pop(); // SpreadElement
        self.pop(); // Token(,)
        match self.top_mut().detail {
            Detail::ElementList(_, ref mut spread) => *spread = true,
            ref detail => unreachable!("{detail:?}"),
        }
        self.update_ends();
        Ok(())
    }

    //   ElementList[?Yield, ?Await] , ArrayInitializerElision SpreadElement[?Yield, ?Await]
    fn process_element_list_list_elision_spread(&mut self) -> Result<(), Error> {
        self.pop(); // SpreadElement
        let n = match self.pop().detail {
            Detail::ArrayInitializerElision(n) => n,
            detail => unreachable!("{detail:?}"),
        };
        self.pop(); // Token(,)
        match self.top_mut().detail {
            Detail::ElementList(ref mut m, ref mut spread) => {
                *m += n + 1;
                *spread = true;
            }
            ref detail => unreachable!("{detail:?}"),
        }
        self.update_ends();
        self.enqueue(Node::PropertyDefinition(
            PropertyDefinitionKind::ArrayElement,
        ));
        Ok(())
    }

    // ArrayInitializerElision :
    //   ,
    //
    // NOTE: The `Elision` production rule is used in multiple production rules.  We rename it in
    // order to perform a different action for each context.
    fn process_array_initializer_elision(&mut self) -> Result<(), Error> {
        self.replace(1, Detail::ArrayInitializerElision(1));
        self.enqueue(Node::PropertyDefinition(
            PropertyDefinitionKind::ArrayEmptySlot,
        ));
        Ok(())
    }

    // ArrayInitializerElision :
    //   ArrayInitializerElision ,
    //
    // NOTE: The `Elision` production rule is used in multiple production rules.  We rename it in
    // order to perform a different action for each context.
    fn process_array_initializer_elision_list(&mut self) -> Result<(), Error> {
        self.pop(); // Token(,)
        match self.top_mut().detail {
            Detail::ArrayInitializerElision(ref mut n) => *n += 1,
            ref detail => unreachable!("{detail:?}"),
        }
        self.update_ends();
        self.enqueue(Node::PropertyDefinition(
            PropertyDefinitionKind::ArrayEmptySlot,
        ));
        Ok(())
    }

    // SpreadElement[Yield, Await] :
    //   ... AssignmentExpression[+In, ?Yield, ?Await]
    fn process_spread_element(&mut self) -> Result<(), Error> {
        self.replace(2, Detail::SpreadElement);
        self.enqueue(Node::PropertyDefinition(
            PropertyDefinitionKind::ArraySpread,
        ));
        Ok(())
    }

    // 13.2.5 Object Initializer

    // ObjectLiteral[Yield, Await] :
    //   { }
    fn process_object_literal_empty(&mut self) -> Result<(), Error> {
        self.replace(2, Detail::ObjectLiteral);
        Ok(())
    }

    // ObjectLiteral[Yield, Await] :
    //   { PropertyDefinitionList[?Yield, ?Await] }
    fn process_object_literal_list(&mut self) -> Result<(), Error> {
        self.replace(3, Detail::ObjectLiteral);
        Ok(())
    }

    // ObjectLiteral[Yield, Await] :
    //   { PropertyDefinitionList[?Yield, ?Await] , }
    fn process_object_literal_comma(&mut self) -> Result<(), Error> {
        self.replace(4, Detail::ObjectLiteral);
        Ok(())
    }

    // PropertyDefinitionList[Yield, Await] :
    //   PropertyDefinition[?Yield, ?Await]
    fn process_property_definition_list_head(&mut self) -> Result<(), Error> {
        let mut syntax = self.pop();
        let name = match syntax.detail {
            Detail::PropertyDefinition(name) => name,
            ref detail => unreachable!("{detail:?}"),
        };
        syntax.detail = Detail::PropertyDefinitionList(name == Symbol::LEGACY_PROTO);
        self.push(syntax);
        Ok(())
    }

    // PropertyDefinitionList[Yield, Await] :
    //   PropertyDefinitionList[?Yield, ?Await] , PropertyDefinition[?Yield, ?Await]
    fn process_property_definition_list_item(&mut self) -> Result<(), Error> {
        let name = match self.pop().detail {
            Detail::PropertyDefinition(name) => name,
            detail => unreachable!("{detail:?}"),
        };
        self.pop(); // Token(,)
        let is_proto = name == Symbol::LEGACY_PROTO;
        match self.top_mut().detail {
            // 13.2.5.1 Static Semantics: Early Errors
            Detail::PropertyDefinitionList(true) if is_proto => return Err(Error::SyntaxError),
            Detail::PropertyDefinitionList(ref mut has_proto) => {
                if is_proto {
                    *has_proto = true;
                }
            }
            ref detail => unreachable!("{detail:?}"),
        }
        self.update_ends();
        Ok(())
    }

    // PropertyDefinition[Yield, Await] :
    //   IdentifierReference[?Yield, ?Await]
    fn process_property_definition_reference(&mut self) -> Result<(), Error> {
        let name = match self.top().detail {
            Detail::IdentifierReference(symbol) => symbol,
            ref detail => unreachable!("{detail:?}"),
        };
        self.enqueue(Node::PropertyDefinition(PropertyDefinitionKind::Reference));
        self.replace(1, Detail::PropertyDefinition(name));
        Ok(())
    }

    // PropertyDefinition[Yield, Await] :
    //   CoverInitializedName[?Yield, ?Await]
    fn process_property_definition_cover(&mut self) -> Result<(), Error> {
        self.syntax_error()
    }

    // PropertyDefinition[Yield, Await] :
    //   PropertyName[?Yield, ?Await] : AssignmentExpression[+In, ?Yield, ?Await]
    fn process_property_definition_key_value(&mut self) -> Result<(), Error> {
        let name = match self.nth(2).detail {
            Detail::Identifier(symbol) => symbol,
            ref detail => unreachable!("{detail:?}"),
        };
        self.enqueue(Node::PropertyDefinition(PropertyDefinitionKind::KeyValue));
        self.replace(3, Detail::PropertyDefinition(name));
        Ok(())
    }

    // PropertyDefinition[Yield, Await] :
    //   ... AssignmentExpression[+In, ?Yield, ?Await]
    fn process_property_definition_spread(&mut self) -> Result<(), Error> {
        self.enqueue(Node::PropertyDefinition(PropertyDefinitionKind::Spread));
        self.replace(2, Detail::PropertyDefinition(Symbol::NONE));
        Ok(())
    }

    // LiteralPropertyName :
    //   IdentifierName
    fn process_literal_property_name_identifier_name(&mut self) -> Result<(), Error> {
        let token_index = self.tokens.len() - 1;
        let symbol = self.make_symbol(token_index);
        self.enqueue(Node::LiteralPropertyName(
            LiteralPropertyName::IdentifierName(symbol),
        ));
        self.replace(1, Detail::Identifier(symbol));
        Ok(())
    }

    // LiteralPropertyName :
    //   NumericLiteral
    fn process_literal_property_name_numeric_literal(&mut self) -> Result<(), Error> {
        let token_index = self.tokens.len() - 1;
        // TODO: RS
        // 1. Let nbr be the NumericValue of NumericLiteral.
        // 2. Return ! ToString(nbr).
        let symbol = self.make_symbol(token_index);
        self.enqueue(Node::LiteralPropertyName(
            LiteralPropertyName::IdentifierName(symbol),
        ));
        self.replace(1, Detail::Identifier(symbol));
        Ok(())
    }

    // CoverInitializedName[Yield, Await] :
    //   IdentifierReference[?Yield, ?Await] Initializer[+In, ?Yield, ?Await]
    fn process_cover_initialized_name(&mut self) -> Result<(), Error> {
        // TODO: implementation
        self.replace(2, Detail::CoverInitializedName);
        Ok(())
    }

    // Initializer[In, Yield, Await] :
    //   = AssignmentExpression[?In, ?Yield, ?Await]
    fn process_initializer(&mut self) -> Result<(), Error> {
        self.replace(2, Detail::Initializer);
        Ok(())
    }

    // 13.2.8 Template Literals

    // TemplateLiteral[Yield, Await, Tagged] :
    //   NoSubstitutionTemplate
    fn process_template_literal_no_substitution(&mut self) -> Result<(), Error> {
        // TODO(feat): 13.2.8.1 Static Semantics: Early Errors
        // It is a Syntax Error if the [Tagged] parameter was not set and NoSubstitutionTemplate
        // Contains NotEscapeSequence.
        let token_index = self.tokens.len() - 1;
        let token = &self.tokens[token_index];
        debug_assert!(matches!(token.kind, TokenKind::NoSubstitutionTemplate));
        // TODO: perform `TV`
        let content = &token.lexeme[1..(token.lexeme.len() - 1)];
        let value = content.encode_utf16().collect();
        let start_index = self.enqueue(Node::String(value, token.lexeme));
        let end_index = self.enqueue(Node::TemplateLiteral(0));
        let syntax = self.top_mut();
        syntax.detail = Detail::TemplateLiteral(0);
        syntax.nodes_range = start_index..(end_index + 1);
        Ok(())
    }

    // TemplateLiteral[Yield, Await, Tagged] :
    //   SubstitutionTemplate[?Yield, ?Await, ?Tagged]
    fn process_template_literal(&mut self) -> Result<(), Error> {
        let n = match self.top().detail {
            Detail::TemplateLiteral(n) => n,
            _ => unreachable!(),
        };
        self.enqueue(Node::TemplateLiteral(n));
        self.replace(1, Detail::TemplateLiteral(n));
        Ok(())
    }

    // SubstitutionTemplate[Yield, Await, Tagged] :
    //   TemplateHead Expression[+In, ?Yield, ?Await] TemplateSpans[?Yield, ?Await, ?Tagged]
    fn process_substitution_template(&mut self) -> Result<(), Error> {
        let n = match self.top().detail {
            Detail::TemplateLiteral(n) => n,
            _ => unreachable!(),
        };
        debug_assert!(n < u16::MAX / 2);
        self.replace(3, Detail::TemplateLiteral(n + 1));
        Ok(())
    }

    // TemplateSpans[Yield, Await, Tagged] :
    //   TemplateTail
    fn process_template_spans_tail(&mut self) -> Result<(), Error> {
        self.replace(1, Detail::TemplateLiteral(0));
        Ok(())
    }

    // TemplateSpans[Yield, Await, Tagged] :
    //   TemplateMiddleList[?Yield, ?Await, ?Tagged] TemplateTail
    fn process_template_spans_middle_tail(&mut self) -> Result<(), Error> {
        let n = match self.nth(1).detail {
            Detail::TemplateLiteral(n) => n,
            _ => unreachable!(),
        };
        self.replace(2, Detail::TemplateLiteral(n));
        Ok(())
    }

    // TemplateMiddleList[Yield, Await, Tagged] :
    //   TemplateMiddle Expression[+In, ?Yield, ?Await]
    fn process_template_middle_list_head(&mut self) -> Result<(), Error> {
        self.replace(2, Detail::TemplateLiteral(1));
        Ok(())
    }

    // TemplateMiddleList[Yield, Await, Tagged] :
    //   TemplateMiddleList[?Yield, ?Await, ?Tagged] TemplateMiddle Expression[+In, ?Yield, ?Await]
    fn process_template_middle_list_item(&mut self) -> Result<(), Error> {
        let n = match self.nth(2).detail {
            Detail::TemplateLiteral(n) => n,
            _ => unreachable!(),
        };
        debug_assert!(n < u16::MAX / 2);
        self.replace(3, Detail::TemplateLiteral(n + 1));
        Ok(())
    }

    fn process_template_head(&mut self) -> Result<(), Error> {
        let token_index = self.tokens.len() - 1;
        let token = &self.tokens[token_index];
        debug_assert!(matches!(token.kind, TokenKind::TemplateHead));
        // TODO: perform `TV`
        let content = &token.lexeme[1..(token.lexeme.len() - 2)];
        let value = content.encode_utf16().collect();
        // The template string may be empty.
        let start_index = self.enqueue(Node::String(value, token.lexeme));
        let syntax = self.top_mut();
        syntax.detail = Detail::TemplateString;
        syntax.nodes_range = start_index..(start_index + 1);
        Ok(())
    }

    fn process_template_middle(&mut self) -> Result<(), Error> {
        let token_index = self.tokens.len() - 1;
        let token = &self.tokens[token_index];
        debug_assert!(matches!(token.kind, TokenKind::TemplateMiddle));
        // TODO: perform `TV`
        let content = &token.lexeme[1..(token.lexeme.len() - 2)];
        let value = content.encode_utf16().collect();
        // The template string may be empty.
        let start_index = self.enqueue(Node::String(value, token.lexeme));
        let syntax = self.top_mut();
        syntax.detail = Detail::TemplateString;
        syntax.nodes_range = start_index..(start_index + 1);
        Ok(())
    }

    fn process_template_tail(&mut self) -> Result<(), Error> {
        let token_index = self.tokens.len() - 1;
        let token = &self.tokens[token_index];
        debug_assert!(matches!(token.kind, TokenKind::TemplateTail));
        // TODO: perform `TV`
        let content = &token.lexeme[1..(token.lexeme.len() - 1)];
        let value = content.encode_utf16().collect();
        // The template string may be empty.
        let start_index = self.enqueue(Node::String(value, token.lexeme));
        let syntax = self.top_mut();
        syntax.detail = Detail::TemplateString;
        syntax.nodes_range = start_index..(start_index + 1);
        Ok(())
    }

    // 13.3 Left-Hand-Side Expressions

    // 13.3.2 Property Accessors

    // MemberExpression[Yield, Await] :
    //   MemberExpression[?Yield, ?Await] [ Expression[+In, ?Yield, ?Await] ]
    fn process_member_expression_bracket_notation(&mut self) -> Result<(), Error> {
        self.enqueue(Node::MemberExpression(
            MemberExpressionKind::PropertyAccessWithExpressionKey,
        ));
        self.replace(4, Detail::Expression);
        Ok(())
    }

    // MemberExpression[Yield, Await] :
    //   MemberExpression[?Yield, ?Await] . IdentifierName
    fn process_member_expression_dot_notation(&mut self) -> Result<(), Error> {
        let token_index = self.tokens.len() - 1;
        let symbol = self.make_symbol(token_index);
        self.enqueue(Node::MemberExpression(
            MemberExpressionKind::PropertyAccessWithIdentifierKey(symbol),
        ));
        self.replace(3, Detail::Expression);
        Ok(())
    }

    // CallExpression[Yield, Await] :
    //   CallExpression[?Yield, ?Await] [ Expression[+In, ?Yield, ?Await] ]
    fn process_call_expression_bracket_notation(&mut self) -> Result<(), Error> {
        self.enqueue(Node::MemberExpression(
            MemberExpressionKind::PropertyAccessWithExpressionKey,
        ));
        self.replace(4, Detail::Expression);
        Ok(())
    }

    // CallExpression[Yield, Await] :
    //   CallExpression[?Yield, ?Await] . IdentifierName
    fn process_call_expression_dot_notation(&mut self) -> Result<(), Error> {
        let token_index = self.tokens.len() - 1;
        let symbol = self.make_symbol(token_index);
        self.enqueue(Node::MemberExpression(
            MemberExpressionKind::PropertyAccessWithIdentifierKey(symbol),
        ));
        self.replace(3, Detail::Expression);
        Ok(())
    }

    // 13.3.6 Function Calls

    // CallExpression[Yield, Await] :
    //   CoverCallExpressionAndAsyncArrowHead[?Yield, ?Await]
    fn process_call_expression(&mut self) -> Result<(), Error> {
        self.enqueue(Node::CallExpression);
        self.replace(1, Detail::Expression);
        Ok(())
    }

    // CallExpression[Yield, Await] :
    //   CallExpression[?Yield, ?Await] Arguments[?Yield, ?Await]
    fn process_call_expression_call(&mut self) -> Result<(), Error> {
        self.enqueue(Node::CallExpression);
        self.replace(2, Detail::Expression);
        Ok(())
    }

    // Arguments[Yield, Await] :
    //   ( )
    fn process_arguments_empty(&mut self) -> Result<(), Error> {
        self.enqueue(Node::ArgumentListHead(true, false));
        self.enqueue(Node::Arguments);
        self.replace(2, Detail::Arguments);
        Ok(())
    }

    // Arguments[Yield, Await] :
    //   ( ArgumentList[?Yield, ?Await] )
    fn process_arguments(&mut self) -> Result<(), Error> {
        self.enqueue(Node::Arguments);
        self.replace(3, Detail::Arguments);
        Ok(())
    }

    // Arguments[Yield, Await] :
    //   ( ArgumentList[?Yield, ?Await] , )
    fn process_arguments_with_comma(&mut self) -> Result<(), Error> {
        self.enqueue(Node::Arguments);
        self.replace(4, Detail::Arguments);
        Ok(())
    }

    // ArgumentList[Yield, Await] :
    //   AssignmentExpression[+In, ?Yield, ?Await]
    fn process_argument_list_head(&mut self) -> Result<(), Error> {
        self.enqueue(Node::ArgumentListHead(false, false));
        self.replace(1, Detail::ArgumentList);
        Ok(())
    }

    // ArgumentList[Yield, Await] :
    //   ArgumentList[?Yield, ?Await] , AssignmentExpression[+In, ?Yield, ?Await]
    fn process_argument_list_item(&mut self) -> Result<(), Error> {
        self.enqueue(Node::ArgumentListItem(false));
        self.replace(3, Detail::ArgumentList);
        Ok(())
    }

    // 13.3.5 The new Operator

    // NewExpression[Yield, Await] :
    //   new NewExpression[?Yield, ?Await]
    fn process_new_expression(&mut self) -> Result<(), Error> {
        self.enqueue(Node::NewExpression(false));
        self.replace(2, Detail::Expression);
        Ok(())
    }

    // MemberExpression[Yield, Await] :
    //   new MemberExpression[?Yield, ?Await] Arguments[?Yield, ?Await]
    fn process_member_expression_new(&mut self) -> Result<(), Error> {
        self.enqueue(Node::NewExpression(true));
        self.replace(3, Detail::Expression);
        Ok(())
    }

    // 13.3.9 Optional Chains

    // OptionalExpression[Yield, Await] :
    //   MemberExpression[?Yield, ?Await] OptionalChain[?Yield, ?Await]
    fn process_optional_expression_member(&mut self) -> Result<(), Error> {
        self.replace(2, Detail::Expression);
        Ok(())
    }

    // OptionalExpression[Yield, Await] :
    //   CallExpression[?Yield, ?Await] OptionalChain[?Yield, ?Await]
    fn process_optional_expression_call(&mut self) -> Result<(), Error> {
        self.replace(2, Detail::Expression);
        Ok(())
    }

    // OptionalExpression[Yield, Await] :
    //   OptionalExpression[?Yield, ?Await] OptionalChain[?Yield, ?Await]
    fn process_optional_expression_chain(&mut self) -> Result<(), Error> {
        self.replace(2, Detail::Expression);
        Ok(())
    }

    // OptionalChain[Yield, Await] :
    //   ?. Arguments[?Yield, ?Await]
    fn process_optional_chain_call(&mut self) -> Result<(), Error> {
        self.enqueue(Node::OptionalChain(PropertyAccessKind::Call));
        self.replace(2, Detail::OptionalChain);
        Ok(())
    }

    // OptionalChain[Yield, Await] :
    //   ?. IdentifierName
    fn process_optional_chain_identifier_name(&mut self) -> Result<(), Error> {
        let token_index = self.tokens.len() - 1;
        let key = self.make_symbol(token_index);
        self.enqueue(Node::OptionalChain(PropertyAccessKind::IdentifierKey(key)));
        self.replace(2, Detail::OptionalChain);
        Ok(())
    }

    // OptionalChain[Yield, Await] :
    //   OptionalChain[?Yield, ?Await] Arguments[?Yield, ?Await]
    fn process_optional_chain_call_chain(&mut self) -> Result<(), Error> {
        self.enqueue(Node::CallExpression);
        self.replace(2, Detail::OptionalChain);
        Ok(())
    }

    // OptionalChain[Yield, Await] :
    //   OptionalChain[?Yield, ?Await] . IdentifierName
    fn process_optional_chain_identifier_name_chain(&mut self) -> Result<(), Error> {
        let token_index = self.tokens.len() - 1;
        let key = self.make_symbol(token_index);
        self.enqueue(Node::MemberExpression(
            MemberExpressionKind::PropertyAccessWithIdentifierKey(key),
        ));
        self.replace(3, Detail::OptionalChain);
        Ok(())
    }

    // 13.4 Update Expressions

    fn process_update_expression(&mut self, op: UpdateOperator) -> Result<(), Error> {
        self.enqueue(Node::UpdateExpression(op));
        self.replace(2, Detail::Expression);
        Ok(())
    }

    // UpdateExpression[Yield, Await] :
    //   LeftHandSideExpression[?Yield, ?Await] [no LineTerminator here] ++
    fn process_postfix_increment(&mut self) -> Result<(), Error> {
        // TODO: 13.4.1 Static Semantics: Early Errors
        self.process_update_expression(UpdateOperator::PostfixIncrement)
    }

    // UpdateExpression[Yield, Await] :
    //   LeftHandSideExpression[?Yield, ?Await] [no LineTerminator here] --
    fn process_postfix_decrement(&mut self) -> Result<(), Error> {
        // TODO: 13.4.1 Static Semantics: Early Errors
        self.process_update_expression(UpdateOperator::PostfixDecrement)
    }

    // UpdateExpression[Yield, Await] :
    //   ++ UnaryExpression[?Yield, ?Await]
    fn process_prefix_increment(&mut self) -> Result<(), Error> {
        // TODO: 13.4.1 Static Semantics: Early Errors
        self.process_update_expression(UpdateOperator::PrefixIncrement)
    }

    // UpdateExpression[Yield, Await] :
    //   -- UnaryExpression[?Yield, ?Await]
    fn process_prefix_decrement(&mut self) -> Result<(), Error> {
        // TODO: 13.4.1 Static Semantics: Early Errors
        self.process_update_expression(UpdateOperator::PrefixDecrement)
    }

    // 13.5 Unary Operators

    fn process_unary_expression(&mut self, op: UnaryOperator) -> Result<(), Error> {
        self.enqueue(Node::UnaryExpression(op));
        self.replace(2, Detail::Expression);
        Ok(())
    }

    // UnaryExpression[Yield, Await] :
    //   delete UnaryExpression[?Yield, ?Await]
    fn process_delete(&mut self) -> Result<(), Error> {
        // TODO: 13.5.1.1 Static Semantics: Early Errors
        self.process_unary_expression(UnaryOperator::Delete)
    }

    // UnaryExpression[Yield, Await] :
    //   void UnaryExpression[?Yield, ?Await]
    fn process_void(&mut self) -> Result<(), Error> {
        self.process_unary_expression(UnaryOperator::Void)
    }

    // UnaryExpression[Yield, Await] :
    //   typeof UnaryExpression[?Yield, ?Await]
    fn process_typeof(&mut self) -> Result<(), Error> {
        self.process_unary_expression(UnaryOperator::Typeof)
    }

    // UnaryExpression[Yield, Await] :
    //   + UnaryExpression[?Yield, ?Await]
    fn process_unary_plus(&mut self) -> Result<(), Error> {
        self.process_unary_expression(UnaryOperator::Plus)
    }

    // UnaryExpression[Yield, Await] :
    //   - UnaryExpression[?Yield, ?Await]
    fn process_unary_minus(&mut self) -> Result<(), Error> {
        self.process_unary_expression(UnaryOperator::Minus)
    }

    // UnaryExpression[Yield, Await] :
    //   ~ UnaryExpression[?Yield, ?Await]
    fn process_bitwise_not(&mut self) -> Result<(), Error> {
        self.process_unary_expression(UnaryOperator::BitwiseNot)
    }

    // UnaryExpression[Yield, Await] :
    //   ! UnaryExpression[?Yield, ?Await]
    fn process_logical_not(&mut self) -> Result<(), Error> {
        self.process_unary_expression(UnaryOperator::LogicalNot)
    }

    // 13.6 Exponentiation Operator

    #[inline(always)]
    fn process_binary_expression(&mut self, op: BinaryOperator) -> Result<(), Error> {
        self.enqueue(Node::BinaryExpression(op));
        self.replace(3, Detail::Expression);
        Ok(())
    }

    // ExponentiationExpression[Yield, Await] :
    //   UpdateExpression[?Yield, ?Await] ** ExponentiationExpression[?Yield, ?Await]
    fn process_exponentiation(&mut self) -> Result<(), Error> {
        self.process_binary_expression(BinaryOperator::Exponentiation)
    }

    // 13.7 Multiplicative Operators

    // MultiplicativeExpression[Yield, Await] :
    //   MultiplicativeExpression[?Yield, ?Await] * ExponentiationExpression[?Yield, ?Await]
    fn process_multiplication(&mut self) -> Result<(), Error> {
        self.process_binary_expression(BinaryOperator::Multiplication)
    }

    // MultiplicativeExpression[Yield, Await] :
    //   MultiplicativeExpression[?Yield, ?Await] / ExponentiationExpression[?Yield, ?Await]
    fn process_division(&mut self) -> Result<(), Error> {
        self.process_binary_expression(BinaryOperator::Division)
    }

    // MultiplicativeExpression[Yield, Await] :
    //   MultiplicativeExpression[?Yield, ?Await] % ExponentiationExpression[?Yield, ?Await]
    fn process_remainder(&mut self) -> Result<(), Error> {
        self.process_binary_expression(BinaryOperator::Remainder)
    }

    // 13.8 Additive Operators

    // AdditiveExpression[Yield, Await] :
    //   AdditiveExpression[?Yield, ?Await] + MultiplicativeExpression[?Yield, ?Await]
    fn process_addition(&mut self) -> Result<(), Error> {
        self.process_binary_expression(BinaryOperator::Addition)
    }

    // AdditiveExpression[Yield, Await] :
    //   AdditiveExpression[?Yield, ?Await] + MultiplicativeExpression[?Yield, ?Await]
    fn process_subtraction(&mut self) -> Result<(), Error> {
        self.process_binary_expression(BinaryOperator::Subtraction)
    }

    // 13.9 Bitwise Shift Operators

    // ShiftExpression[Yield, Await] :
    //   ShiftExpression[?Yield, ?Await] << AdditiveExpression[?Yield, ?Await]
    fn process_left_shift(&mut self) -> Result<(), Error> {
        self.process_binary_expression(BinaryOperator::LeftShift)
    }

    // ShiftExpression[Yield, Await] :
    //   ShiftExpression[?Yield, ?Await] >> AdditiveExpression[?Yield, ?Await]
    fn process_signed_right_shift(&mut self) -> Result<(), Error> {
        self.process_binary_expression(BinaryOperator::SignedRightShift)
    }

    // ShiftExpression[Yield, Await] :
    //   ShiftExpression[?Yield, ?Await] >>> AdditiveExpression[?Yield, ?Await]
    fn process_unsigned_right_shift(&mut self) -> Result<(), Error> {
        self.process_binary_expression(BinaryOperator::UnsignedRightShift)
    }

    // 13.10 Relational Operators

    // RelationalExpression[In, Yield, Await] :
    //   RelationalExpression[?In, ?Yield, ?Await] < ShiftExpression[?Yield, ?Await]
    fn process_less_than(&mut self) -> Result<(), Error> {
        self.process_binary_expression(BinaryOperator::LessThan)
    }

    // RelationalExpression[In, Yield, Await] :
    //   RelationalExpression[?In, ?Yield, ?Await] > ShiftExpression[?Yield, ?Await]
    fn process_greater_than(&mut self) -> Result<(), Error> {
        self.process_binary_expression(BinaryOperator::GreaterThan)
    }

    // RelationalExpression[In, Yield, Await] :
    //   RelationalExpression[?In, ?Yield, ?Await] <= ShiftExpression[?Yield, ?Await]
    fn process_less_than_or_equal(&mut self) -> Result<(), Error> {
        self.process_binary_expression(BinaryOperator::LessThanOrEqual)
    }

    // RelationalExpression[In, Yield, Await] :
    //   RelationalExpression[?In, ?Yield, ?Await] >= ShiftExpression[?Yield, ?Await]
    fn process_greater_than_or_equal(&mut self) -> Result<(), Error> {
        self.process_binary_expression(BinaryOperator::GreaterThanOrEqual)
    }

    // RelationalExpression[In, Yield, Await] :
    //   RelationalExpression[?In, ?Yield, ?Await] instanceof ShiftExpression[?Yield, ?Await]
    fn process_instanceof(&mut self) -> Result<(), Error> {
        self.process_binary_expression(BinaryOperator::Instanceof)
    }

    // RelationalExpression[In, Yield, Await] :
    //   RelationalExpression[?In, ?Yield, ?Await] in ShiftExpression[?Yield, ?Await]
    fn process_in(&mut self) -> Result<(), Error> {
        self.process_binary_expression(BinaryOperator::In)
    }

    // 13.11 Equality Operators

    // EqualityExpression[In, Yield, Await] :
    //   EqualityExpression[?In, ?Yield, ?Await] == RelationalExpression[?In, ?Yield, ?Await]
    fn process_equality(&mut self) -> Result<(), Error> {
        self.process_binary_expression(BinaryOperator::Equality)
    }

    // EqualityExpression[In, Yield, Await] :
    //   EqualityExpression[?In, ?Yield, ?Await] != RelationalExpression[?In, ?Yield, ?Await]
    fn process_inequality(&mut self) -> Result<(), Error> {
        self.process_binary_expression(BinaryOperator::Inequality)
    }

    // EqualityExpression[In, Yield, Await] :
    //   EqualityExpression[?In, ?Yield, ?Await] === RelationalExpression[?In, ?Yield, ?Await]
    fn process_strict_equality(&mut self) -> Result<(), Error> {
        self.process_binary_expression(BinaryOperator::StrictEquality)
    }

    // EqualityExpression[In, Yield, Await] :
    //   EqualityExpression[?In, ?Yield, ?Await] !== RelationalExpression[?In, ?Yield, ?Await]
    fn process_strict_inequality(&mut self) -> Result<(), Error> {
        self.process_binary_expression(BinaryOperator::StrictInequality)
    }

    // 13.12 Binary Bitwise Operators

    // BitwiseANDExpression[In, Yield, Await] :
    //   BitwiseANDExpression[?In, ?Yield, ?Await] & EqualityExpression[?In, ?Yield, ?Await]
    fn process_bitwise_and(&mut self) -> Result<(), Error> {
        self.process_binary_expression(BinaryOperator::BitwiseAnd)
    }

    // BitwiseXORExpression[In, Yield, Await] :
    //   BitwiseXORExpression[?In, ?Yield, ?Await] ^ BitwiseANDExpression[?In, ?Yield, ?Await]
    fn process_bitwise_xor(&mut self) -> Result<(), Error> {
        self.process_binary_expression(BinaryOperator::BitwiseXor)
    }

    // BitwiseORExpression[In, Yield, Await] :
    //   BitwiseORExpression[?In, ?Yield, ?Await] | BitwiseXORExpression[?In, ?Yield, ?Await]
    fn process_bitwise_or(&mut self) -> Result<(), Error> {
        self.process_binary_expression(BinaryOperator::BitwiseOr)
    }

    // 13.13 Binary Logical Operators

    #[inline(always)]
    fn process_logical_expression(&mut self, op: LogicalOperator) -> Result<(), Error> {
        self.enqueue(Node::LogicalExpression(op));
        self.replace(3, Detail::Expression);
        Ok(())
    }

    // LogicalANDExpression[In, Yield, Await] :
    //   LogicalANDExpression[?In, ?Yield, ?Await] && BitwiseORExpression[?In, ?Yield, ?Await]
    fn process_logical_and(&mut self) -> Result<(), Error> {
        self.process_logical_expression(LogicalOperator::LogicalAnd)
    }

    // LogicalORExpression[In, Yield, Await] :
    //   LogicalORExpression[?In, ?Yield, ?Await] || LogicalANDExpression[?In, ?Yield, ?Await]
    fn process_logical_or(&mut self) -> Result<(), Error> {
        self.process_logical_expression(LogicalOperator::LogicalOr)
    }

    // CoalesceExpression[In, Yield, Await] :
    //   CoalesceExpressionHead[?In, ?Yield, ?Await] ?? BitwiseORExpression[?In, ?Yield, ?Await]
    fn process_nullish_coalescing(&mut self) -> Result<(), Error> {
        self.process_logical_expression(LogicalOperator::NullishCoalescing)
    }

    // 13.14 Conditional Operator ( ? : )

    // ConditionalExpression[In, Yield, Await] :
    //   ShortCircuitExpression[?In, ?Yield, ?Await]
    //     ? AssignmentExpression[+In, ?Yield, ?Await]
    //     : AssignmentExpression[?In, ?Yield, ?Await]
    fn process_conditional_expression(&mut self) -> Result<(), Error> {
        self.enqueue(Node::ConditionalExpression);
        self.replace(5, Detail::Expression);
        Ok(())
    }

    // 13.15 Assignment Operators

    fn process_assignment_expression(&mut self, op: AssignmentOperator) -> Result<(), Error> {
        self.enqueue(Node::AssignmentExpression(op));
        self.replace(3, Detail::Expression);
        Ok(())
    }

    // AssignmentExpression[In, Yield, Await] :
    //   LeftHandSideExpression[?Yield, ?Await] = AssignmentExpression[?In, ?Yield, ?Await]
    fn process_assignment(&mut self) -> Result<(), Error> {
        // TODO: 13.15.1 Static Semantics: Early Errors
        self.process_assignment_expression(AssignmentOperator::Assignment)
    }

    // AssignmentExpression[In, Yield, Await] :
    //   LeftHandSideExpression[?Yield, ?Await] AssignmentOperator
    //     AssignmentExpression[?In, ?Yield, ?Await]
    fn process_assignment_operator(&mut self) -> Result<(), Error> {
        // TODO: 13.15.1 Static Semantics: Early Errors
        let kind = match self.stack[self.stack.len() - 2].detail {
            Detail::Token(index) => self.tokens[index].kind,
            _ => unreachable!(),
        };
        self.process_assignment_expression(match kind {
            TokenKind::MulAssign => AssignmentOperator::MultiplicationAssignment,
            TokenKind::DivAssign => AssignmentOperator::DivisionAssignment,
            TokenKind::ModAssign => AssignmentOperator::RemainderAssignment,
            TokenKind::AddAssign => AssignmentOperator::AdditionAssignment,
            TokenKind::SubAssign => AssignmentOperator::SubtractionAssignment,
            TokenKind::ShlAssign => AssignmentOperator::LeftShiftAssignment,
            TokenKind::SarAssign => AssignmentOperator::SignedRightShiftAssignment,
            TokenKind::ShrAssign => AssignmentOperator::UnsignedRightShiftAssignment,
            TokenKind::BitAndAssign => AssignmentOperator::BitwiseAndAssignment,
            TokenKind::BitXorAssign => AssignmentOperator::BitwiseXorAssignment,
            TokenKind::BitOrAssign => AssignmentOperator::BitwiseOrAssignment,
            TokenKind::ExpAssign => AssignmentOperator::ExponentiationAssignment,
            _ => unreachable!(),
        })
    }

    // AssignmentExpression[In, Yield, Await] :
    //   LeftHandSideExpression[?Yield, ?Await] &&= AssignmentExpression[?In, ?Yield, ?Await]
    fn process_logical_and_assignment(&mut self) -> Result<(), Error> {
        // TODO: 13.15.1 Static Semantics: Early Errors
        self.process_assignment_expression(AssignmentOperator::LogicalAndAssignment)
    }

    // AssignmentExpression[In, Yield, Await] :
    //   LeftHandSideExpression[?Yield, ?Await] ||= AssignmentExpression[?In, ?Yield, ?Await]
    fn process_logical_or_assignment(&mut self) -> Result<(), Error> {
        // TODO: 13.15.1 Static Semantics: Early Errors
        self.process_assignment_expression(AssignmentOperator::LogicalOrAssignment)
    }

    // AssignmentExpression[In, Yield, Await] :
    //   LeftHandSideExpression[?Yield, ?Await] ??= AssignmentExpression[?In, ?Yield, ?Await]
    fn process_nullish_coalescing_assignment(&mut self) -> Result<(), Error> {
        // TODO: 13.15.1 Static Semantics: Early Errors
        self.process_assignment_expression(AssignmentOperator::NullishCoalescingAssignment)
    }

    // 13.16 Comma Operator ( , )

    // Expression[In, Yield, Await] :
    //   Expression[?In, ?Yield, ?Await] , AssignmentExpression[?In, ?Yield, ?Await]
    fn process_comma_operator(&mut self) -> Result<(), Error> {
        self.enqueue(Node::SequenceExpression);
        self.replace(3, Detail::Expression);
        Ok(())
    }

    // 14 ECMAScript Language: Statements and Declarations

    // Statement[Yield, Await, Return] :
    //   BlockStatement[?Yield, ?Await, ?Return]
    //
    // Statement[Yield, Await, Return] :
    //   VariableStatement[?Yield, ?Await]
    //
    // Statement[Yield, Await, Return] :
    //   EmptyStatement
    //
    // Statement[Yield, Await, Return] :
    //   ExpressionStatement[?Yield, ?Await]
    //
    // Statement[Yield, Await, Return] :
    //   IfStatement[?Yield, ?Await, ?Return]
    //
    // Statement[Yield, Await, Return] :
    //   BreakableStatement[?Yield, ?Await, ?Return]
    //
    // Statement[Yield, Await, Return] :
    //   ContinueStatement[?Yield, ?Await]
    //
    // Statement[Yield, Await, Return] :
    //   BreakStatement[?Yield, ?Await]
    //
    // Statement[Yield, Await, Return] :
    //   [+Return] ReturnStatement[?Yield, ?Await]
    //
    // Statement[Yield, Await, Return] :
    //   WithStatement[?Yield, ?Await, ?Return]
    //
    // Statement[Yield, Await, Return] :
    //   LabelledStatement[?Yield, ?Await, ?Return]
    //
    // Statement[Yield, Await, Return] :
    //   ThrowStatement[?Yield, ?Await]
    //
    // Statement[Yield, Await, Return] :
    //   TryStatement[?Yield, ?Await, ?Return]
    //
    // Statement[Yield, Await, Return] :
    //   DebuggerStatement
    fn process_statement(&mut self) -> Result<(), Error> {
        // TODO
        Ok(())
    }

    // Declaration[Yield, Await] :
    //   HoistableDeclaration[?Yield, ?Await, ~Default]
    //
    // Declaration[Yield, Await] :
    //   ClassDeclaration[?Yield, ?Await, ~Default]
    //
    // Declaration[Yield, Await] :
    //   LexicalDeclaration[+In, ?Yield, ?Await]
    fn process_declaration(&mut self) -> Result<(), Error> {
        self.replace(1, Detail::Declaration);
        Ok(())
    }

    // HoistableDeclaration[Yield, Await, Default] :
    //   FunctionDeclaration[?Yield, ?Await, ?Default]
    //
    // HoistableDeclaration[Yield, Await, Default] :
    //   GeneratorDeclaration[?Yield, ?Await, ?Default]
    //
    // HoistableDeclaration[Yield, Await, Default] :
    //   AsyncFunctionDeclaration[?Yield, ?Await, ?Default]
    //
    // HoistableDeclaration[Yield, Await, Default] :
    //   AsyncGeneratorDeclaration[?Yield, ?Await, ?Default]
    fn process_hoistable_declaration(&mut self) -> Result<(), Error> {
        // TODO
        Ok(())
    }

    // BreakableStatement[Yield, Await, Return] :
    //   IterationStatement[?Yield, ?Await, ?Return]
    //
    // BreakableStatement[Yield, Await, Return] :
    //   SwitchStatement[?Yield, ?Await, ?Return]
    fn process_breakable_statement(&mut self) -> Result<(), Error> {
        // TODO
        Ok(())
    }

    // 14.2 Block

    // BlockStatement[Yield, Await, Return] :
    //   Block[?Yield, ?Await, ?Return]
    fn process_block_statement(&mut self) -> Result<(), Error> {
        self.enqueue(Node::BlockStatement);
        self.replace(1, Detail::BlockStatement);
        Ok(())
    }

    // Block[Yield, Await, Return] :
    //   { }
    fn process_empty_block(&mut self) -> Result<(), Error> {
        self.replace(2, Detail::Block);
        Ok(())
    }

    // Block[Yield, Await, Return] :
    //   { StatementList[?Yield, ?Await, ?Return] }
    fn process_block(&mut self) -> Result<(), Error> {
        self.enqueue(Node::EndBlockScope);
        self.replace(3, Detail::Block);
        Ok(())
    }

    // StatementList[Yield, Await, Return] :
    //   StatementListItem[?Yield, ?Await, ?Return]
    fn process_statement_list_head(&mut self) -> Result<(), Error> {
        self.top_mut().detail = Detail::StatementList;
        Ok(())
    }

    // StatementList[Yield, Await, Return] :
    //   StatementList[?Yield, ?Await, ?Return] StatementListItem[?Yield, ?Await, ?Return]
    fn process_statement_list_item(&mut self) -> Result<(), Error> {
        self.pop();
        self.update_ends();
        Ok(())
    }

    // 14.3 Declarations and the Variable Statement

    // 14.3.1 Let and Const Declarations

    // LexicalDeclaration[In, Yield, Await] :
    //   let BindingList[?In, ?Yield, ?Await] ;
    fn process_let_declaration(&mut self) -> Result<(), Error> {
        let index = self.stack.len() - 2;
        let bound_names = match &mut self.stack[index].detail {
            Detail::BindingList(list) => std::mem::take(&mut list.bound_names),
            detail => unreachable!("{detail:?}"),
        };
        self.enqueue(Node::LetDeclaration(bound_names.len() as u32));
        self.replace(3, Detail::LetDeclaration(bound_names));
        Ok(())
    }

    // LexicalDeclaration[In, Yield, Await] :
    //   const BindingList[?In, ?Yield, ?Await] ;
    fn process_const_declaration(&mut self) -> Result<(), Error> {
        let index = self.stack.len() - 2;
        let (bound_names, has_initializer) = match &mut self.stack[index].detail {
            Detail::BindingList(list) => {
                let bound_names = std::mem::take(&mut list.bound_names);
                (bound_names, list.has_initializer)
            }
            detail => unreachable!("{detail:?}"),
        };
        // 14.3.1.1 Static Semantics: Early Errors
        ensure!(has_initializer);
        self.enqueue(Node::ConstDeclaration(bound_names.len() as u32));
        self.replace(3, Detail::ConstDeclaration(bound_names));
        Ok(())
    }

    // BindingList[In, Yield, Await] :
    //   LexicalBinding[?In, ?Yield, ?Await]
    fn process_binding_list_head(&mut self) -> Result<(), Error> {
        let mut syntax = self.pop();
        syntax.detail = Detail::BindingList(match syntax.detail {
            Detail::Binding(decl) => decl,
            detail => unreachable!("{detail:?}"),
        });
        self.push(syntax);
        Ok(())
    }

    // BindingList[In, Yield, Await] :
    //   BindingList[?In, ?Yield, ?Await] , LexicalBinding[?In, ?Yield, ?Await]
    fn process_binding_list_item(&mut self) -> Result<(), Error> {
        let decl = match self.pop().detail {
            Detail::Binding(decl) => decl,
            detail => unreachable!("{detail:?}"),
        };
        self.pop(); // Token(,)
        match &mut self.top_mut().detail {
            Detail::BindingList(list) => {
                for name in decl.bound_names.into_iter() {
                    // 14.3.1.1 Static Semantics: Early Errors
                    ensure!(!list.bound_names.contains(&name));
                    list.bound_names.push(name);
                }
                if !decl.has_initializer {
                    list.has_initializer = false;
                }
            }
            detail => unreachable!("{detail:?}"),
        }
        self.update_ends();
        Ok(())
    }

    // LexicalBinding[In, Yield, Await] :
    //   BindingIdentifier[?Yield, ?Await]
    fn process_lexical_binding_identifier(&mut self) -> Result<(), Error> {
        let symbol = match &self.top().detail {
            Detail::BindingIdentifier(symbol) => *symbol,
            detail => unreachable!("{detail:?}"),
        };

        // 14.3.1.1 Static Semantics: Early Errors
        ensure!(!matches!(symbol, Symbol::LET));

        const HAS_INITIALIZER: bool = false;
        self.enqueue(Node::LexicalBinding(HAS_INITIALIZER));
        self.replace(
            1,
            Detail::Binding(DeclarationSemantics {
                bound_names: smallvec![symbol],
                has_initializer: HAS_INITIALIZER,
            }),
        );

        Ok(())
    }

    // LexicalBinding[In, Yield, Await] :
    //   BindingIdentifier[?Yield, ?Await] Initializer[?In, ?Yield, ?Await]
    fn process_lexical_binding_identifier_with_initializer(&mut self) -> Result<(), Error> {
        let symbol = match &self.stack[self.stack.len() - 2].detail {
            Detail::BindingIdentifier(symbol) => *symbol,
            detail => unreachable!("{detail:?}"),
        };

        // 14.3.1.1 Static Semantics: Early Errors
        ensure!(!matches!(symbol, Symbol::LET));

        const HAS_INITIALIZER: bool = true;
        self.enqueue(Node::LexicalBinding(HAS_INITIALIZER));
        self.replace(
            2,
            Detail::Binding(DeclarationSemantics {
                bound_names: smallvec![symbol],
                has_initializer: HAS_INITIALIZER,
            }),
        );

        Ok(())
    }

    // 14.3.2 Variable Statement

    // VariableStatement[Yield, Await] :
    //   var VariableDeclarationList[+In, ?Yield, ?Await] ;
    fn process_variable_statement(&mut self) -> Result<(), Error> {
        let index = self.stack.len() - 2;
        let bound_names = match &mut self.stack[index].detail {
            Detail::BindingList(list) => std::mem::take(&mut list.bound_names),
            detail => unreachable!("{detail:?}"),
        };
        self.enqueue(Node::VariableStatement(bound_names.len() as u32));
        self.replace(3, Detail::VariableStatement(bound_names));
        Ok(())
    }

    // VariableDeclarationList[In, Yield, Await] :
    //   VariableDeclaration[?In, ?Yield, ?Await]
    fn process_variable_declaration_list_head(&mut self) -> Result<(), Error> {
        self.process_binding_list_head()
    }

    // VariableDeclarationList[In, Yield, Await] :
    //   VariableDeclarationList[?In, ?Yield, ?Await] , VariableDeclaration[?In, ?Yield, ?Await]
    fn process_variable_declaration_list_item(&mut self) -> Result<(), Error> {
        self.process_binding_list_item()
    }

    // VariableDeclaration[In, Yield, Await] :
    //   BindingIdentifier[?Yield, ?Await]
    fn process_variable_declaration_no_init(&mut self) -> Result<(), Error> {
        let symbol = match &self.top().detail {
            Detail::BindingIdentifier(symbol) => *symbol,
            detail => unreachable!("{detail:?}"),
        };

        const HAS_INITIALIZER: bool = false;
        self.enqueue(Node::VariableDeclaration(HAS_INITIALIZER));
        self.replace(
            1,
            Detail::Binding(DeclarationSemantics {
                bound_names: smallvec![symbol],
                has_initializer: HAS_INITIALIZER,
            }),
        );

        Ok(())
    }

    // VariableDeclaration[In, Yield, Await] :
    //   BindingIdentifier[?Yield, ?Await] Initializer[?In, ?Yield, ?Await]
    fn process_variable_declaration(&mut self) -> Result<(), Error> {
        let symbol = match &self.stack[self.stack.len() - 2].detail {
            Detail::BindingIdentifier(symbol) => *symbol,
            detail => unreachable!("{detail:?}"),
        };

        const HAS_INITIALIZER: bool = true;
        self.enqueue(Node::VariableDeclaration(HAS_INITIALIZER));
        self.replace(
            2,
            Detail::Binding(DeclarationSemantics {
                bound_names: smallvec![symbol],
                has_initializer: HAS_INITIALIZER,
            }),
        );

        Ok(())
    }

    // VariableDeclaration[In, Yield, Await] :
    //   BindingPattern[?Yield, ?Await] Initializer[?In, ?Yield, ?Await]
    fn process_variable_declaration_pattern(&mut self) -> Result<(), Error> {
        unimplemented!();
    }

    // 14.3.3 Destructuring Binding Patterns

    // BindingElement[Yield, Await] :
    //   SingleNameBinding[?Yield, ?Await]
    fn process_binding_element(&mut self) -> Result<(), Error> {
        let (symbol, has_initializer) = match self.top().detail {
            Detail::SingleNameBinding(symbol, has_initializer) => (symbol, has_initializer),
            _ => unreachable!(),
        };
        self.replace(
            1,
            Detail::BindingElement(BindingElement {
                kind: BindingElementKind::SingleNameBinding(symbol),
                has_initializer,
            }),
        );
        self.enqueue(Node::BindingElement(has_initializer));
        Ok(())
    }

    // SingleNameBinding[Yield, Await] :
    //   BindingIdentifier[?Yield, ?Await]
    fn process_single_name_binding(&mut self) -> Result<(), Error> {
        let symbol = match self.top().detail {
            Detail::BindingIdentifier(symbol) => symbol,
            _ => unreachable!(),
        };
        self.replace(1, Detail::SingleNameBinding(symbol, false));
        Ok(())
    }

    // SingleNameBinding[Yield, Await] :
    //   BindingIdentifier[?Yield, ?Await] Initializer[+In, ?Yield, ?Await]
    fn process_single_name_binding_with_initializer(&mut self) -> Result<(), Error> {
        let symbol = match self.stack[self.stack.len() - 2].detail {
            Detail::BindingIdentifier(symbol) => symbol,
            _ => unreachable!(),
        };
        self.replace(2, Detail::SingleNameBinding(symbol, true));
        Ok(())
    }

    // 14.4 Empty Statement

    // EmptyStatement :
    //   ;
    fn process_empty_statement(&mut self) -> Result<(), Error> {
        //self.check_token(TokenKind::Semicolon);
        let node_index = self.enqueue(Node::EmptyStatement);
        let syntax = self.top_mut();
        syntax.detail = Detail::EmptyStatement;
        syntax.nodes_range = node_index..(node_index + 1);
        Ok(())
    }

    // 14.5 Expression Statement

    // ExpressionStatement[Yield, Await] :
    //   [lookahead ∉ { {, function, async [no LineTerminator here] function, class, let [ }]
    //   Expression[+In, ?Yield, ?Await] ;
    fn process_expression_statement(&mut self) -> Result<(), Error> {
        self.enqueue(Node::ExpressionStatement);
        self.replace(2, Detail::ExpressionStatement);
        Ok(())
    }

    // 14.6 The if Statement

    // IfStatement[Yield, Await, Return] :
    //   if ( Expression[+In, ?Yield, ?Await] ) Statement[?Yield, ?Await, ?Return]
    //   else Statement[?Yield, ?Await, ?Return]
    fn process_if_else_statement(&mut self) -> Result<(), Error> {
        // TODO: 14.6.1 Static Semantics: Early Errors
        self.enqueue(Node::IfElseStatement);
        self.replace(7, Detail::IfStatement);
        Ok(())
    }

    // IfStatement[Yield, Await, Return] :
    //   if ( Expression[+In, ?Yield, ?Await] ) Statement[?Yield, ?Await, ?Return]
    //   [lookahead ≠ else]
    fn process_if_statement(&mut self) -> Result<(), Error> {
        // TODO: 14.6.1 Static Semantics: Early Errors
        self.enqueue(Node::IfStatement);
        self.replace(5, Detail::IfStatement);
        Ok(())
    }

    // 14.7 Iteration Statements

    // _LOOP_START_
    fn process_loop_start(&mut self) -> Result<(), Error> {
        ensure!(self.iteration_statement_depth < MAX_ITERATION_STATEMENT_DEPTH);
        self.iteration_statement_depth += 1;
        self.enqueue(Node::LoopStart);
        Ok(())
    }

    // _LOOP_INIT_EXPRESSION_
    fn process_loop_init_expression(&mut self) -> Result<(), Error> {
        self.enqueue(Node::LoopInitExpression);
        Ok(())
    }

    // _LOOP_INIT_VAR_DECLARATION_
    fn process_loop_init_var_declaration(&mut self) -> Result<(), Error> {
        self.enqueue(Node::LoopInitVarDeclaration);
        Ok(())
    }

    // _LOOP_INIT_LEXICAL_DECLARATION_
    fn process_loop_init_lexical_declaration(&mut self) -> Result<(), Error> {
        self.enqueue(Node::LoopInitLexicalDeclaration);
        Ok(())
    }

    // _LOOP_TEST_
    fn process_loop_test(&mut self) -> Result<(), Error> {
        self.enqueue(Node::LoopTest);
        Ok(())
    }

    // _LOOP_NEXT_
    fn process_loop_next(&mut self) -> Result<(), Error> {
        self.enqueue(Node::LoopNext);
        Ok(())
    }

    // _LOOP_BODY_
    fn process_loop_body(&mut self) -> Result<(), Error> {
        self.enqueue(Node::LoopBody);
        Ok(())
    }

    // IterationStatement[Yield, Await, Return] :
    //   DoWhileStatement[?Yield, ?Await, ?Return]
    //
    // IterationStatement[Yield, Await, Return] :
    //   WhileStatement[?Yield, ?Await, ?Return]
    //
    // IterationStatement[Yield, Await, Return] :
    //   ForStatement[?Yield, ?Await, ?Return]
    //
    // IterationStatement[Yield, Await, Return] :
    //   ForInOfStatement[?Yield, ?Await, ?Return]
    fn process_iteration_statement(&mut self) -> Result<(), Error> {
        assert!(self.iteration_statement_depth > 0);
        self.iteration_statement_depth -= 1;
        Ok(())
    }

    // 14.7.2 The do-while Statement

    // DoWhileStatement[Yield, Await, Return] :
    //   do Statement[?Yield, ?Await, ?Return] while ( Expression[+In, ?Yield, ?Await] ) ;
    fn process_do_while_statement(&mut self) -> Result<(), Error> {
        self.enqueue(Node::DoWhileStatement);
        self.replace(7, Detail::DoWhileStatement);
        Ok(())
    }

    // 14.7.3 The while Statement

    // WhileStatement[Yield, Await, Return] :
    //   while ( Expression[+In, ?Yield, ?Await] ) Statement[?Yield, ?Await, ?Return]
    fn process_while_statement(&mut self) -> Result<(), Error> {
        self.enqueue(Node::WhileStatement);
        self.replace(5, Detail::WhileStatement);
        Ok(())
    }

    // 14.7.4 The for Statement

    // ForStatement[Yield, Await, Return] :
    //   for (
    //   [lookahead ≠ let [] Expression[~In, ?Yield, ?Await]opt ;
    //   Expression[+In, ?Yield, ?Await]opt ;
    //   Expression[+In, ?Yield, ?Await]opt )
    //   Statement[?Yield, ?Await, ?Return]

    fn process_for_statement_no_init_test_next(&mut self) -> Result<(), Error> {
        self.enqueue(Node::ForStatement(LoopFlags::empty()));
        self.replace(6, Detail::ForStatement);
        Ok(())
    }

    fn process_for_statement_no_test_next(&mut self) -> Result<(), Error> {
        self.enqueue(Node::ForStatement(LoopFlags::HAS_INIT));
        self.replace(7, Detail::ForStatement);
        Ok(())
    }

    fn process_for_statement_no_init_next(&mut self) -> Result<(), Error> {
        self.enqueue(Node::ForStatement(LoopFlags::HAS_TEST));
        self.replace(7, Detail::ForStatement);
        Ok(())
    }

    fn process_for_statement_no_next(&mut self) -> Result<(), Error> {
        self.enqueue(Node::ForStatement(
            LoopFlags::HAS_INIT | LoopFlags::HAS_TEST,
        ));
        self.replace(8, Detail::ForStatement);
        Ok(())
    }

    fn process_for_statement_no_init_test(&mut self) -> Result<(), Error> {
        self.enqueue(Node::ForStatement(LoopFlags::HAS_NEXT));
        self.replace(7, Detail::ForStatement);
        Ok(())
    }

    fn process_for_statement_no_test(&mut self) -> Result<(), Error> {
        self.enqueue(Node::ForStatement(
            LoopFlags::HAS_INIT | LoopFlags::HAS_NEXT,
        ));
        self.replace(8, Detail::ForStatement);
        Ok(())
    }

    fn process_for_statement_no_init(&mut self) -> Result<(), Error> {
        self.enqueue(Node::ForStatement(
            LoopFlags::HAS_TEST | LoopFlags::HAS_NEXT,
        ));
        self.replace(8, Detail::ForStatement);
        Ok(())
    }

    fn process_for_statement(&mut self) -> Result<(), Error> {
        self.enqueue(Node::ForStatement(
            LoopFlags::HAS_INIT | LoopFlags::HAS_TEST | LoopFlags::HAS_NEXT,
        ));
        self.replace(9, Detail::ForStatement);
        Ok(())
    }

    // ForStatement[Yield, Await, Return] :
    //   for (
    //   var VariableDeclarationList[~In, ?Yield, ?Await] ;
    //   Expression[+In, ?Yield, ?Await]opt ;
    //   Expression[+In, ?Yield, ?Await]opt )
    //   Statement[?Yield, ?Await, ?Return]

    fn process_for_statement_vars_no_test_next(&mut self) -> Result<(), Error> {
        self.enqueue(Node::ForStatement(LoopFlags::HAS_INIT));
        self.replace(8, Detail::ForStatement);
        Ok(())
    }

    fn process_for_statement_vars_no_next(&mut self) -> Result<(), Error> {
        self.enqueue(Node::ForStatement(
            LoopFlags::HAS_INIT | LoopFlags::HAS_TEST,
        ));
        self.replace(9, Detail::ForStatement);
        Ok(())
    }

    fn process_for_statement_vars_no_test(&mut self) -> Result<(), Error> {
        self.enqueue(Node::ForStatement(
            LoopFlags::HAS_INIT | LoopFlags::HAS_NEXT,
        ));
        self.replace(9, Detail::ForStatement);
        Ok(())
    }

    fn process_for_statement_vars(&mut self) -> Result<(), Error> {
        self.enqueue(Node::ForStatement(
            LoopFlags::HAS_INIT | LoopFlags::HAS_TEST | LoopFlags::HAS_NEXT,
        ));
        self.replace(10, Detail::ForStatement);
        Ok(())
    }

    // ForStatement[Yield, Await, Return] :
    //   for (
    //   LexicalDeclaration[~In, ?Yield, ?Await]
    //   Expression[+In, ?Yield, ?Await]opt ;
    //   Expression[+In, ?Yield, ?Await]opt )
    //   Statement[?Yield, ?Await, ?Return]

    fn process_for_statement_decl_no_test_next(&mut self) -> Result<(), Error> {
        self.enqueue(Node::ForStatement(LoopFlags::HAS_INIT));
        self.replace(6, Detail::ForStatement);
        Ok(())
    }

    fn process_for_statement_decl_no_next(&mut self) -> Result<(), Error> {
        self.enqueue(Node::ForStatement(
            LoopFlags::HAS_INIT | LoopFlags::HAS_TEST,
        ));
        self.replace(7, Detail::ForStatement);
        Ok(())
    }

    fn process_for_statement_decl_no_test(&mut self) -> Result<(), Error> {
        self.enqueue(Node::ForStatement(
            LoopFlags::HAS_INIT | LoopFlags::HAS_NEXT,
        ));
        self.replace(7, Detail::ForStatement);
        Ok(())
    }

    fn process_for_statement_decl(&mut self) -> Result<(), Error> {
        self.enqueue(Node::ForStatement(
            LoopFlags::HAS_INIT | LoopFlags::HAS_TEST | LoopFlags::HAS_NEXT,
        ));
        self.replace(8, Detail::ForStatement);
        Ok(())
    }

    // 14.8 The continue Statement

    fn is_within_iteration_statement(&self) -> bool {
        self.iteration_statement_depth > 0
    }

    // ContinueStatement[Yield, Await] :
    //   continue ;
    fn process_continue_statement(&mut self) -> Result<(), Error> {
        // 14.8.1 Static Semantics: Early Errors
        // The ContinueStatement is allowed only inside an IterationStatement.
        //
        // TODO: set details of the error like this:
        // Illegal continue statement: no surrounding iteration statement
        ensure!(self.is_within_iteration_statement());

        self.enqueue(Node::ContinueStatement(Symbol::NONE));
        self.replace(2, Detail::ContinueStatement);
        Ok(())
    }

    // ContinueStatement[Yield, Await] :
    //   continue [no LineTerminator here] LabelIdentifier[?Yield, ?Await] ;
    fn process_continue_statement_with_label(&mut self) -> Result<(), Error> {
        // 14.8.1 Static Semantics: Early Errors
        // The ContinueStatement is allowed only inside an IterationStatement.
        //
        // TODO: set details of the error like this:
        // Illegal continue statement: no surrounding iteration statement
        ensure!(self.is_within_iteration_statement());

        let symbol = match self.nth(1).detail {
            Detail::LabelIdentifier(symbol) => symbol,
            _ => unreachable!(),
        };

        // NOTE: It seems not to be described in the specification but it's a syntax error in major
        // implementations when the label is not contained in the label set.
        match self
            .label_stack
            .iter_mut()
            .rev()
            .find(|label| label.symbol == symbol)
        {
            Some(label) => label.num_continue_statements += 1,
            None => return Err(Error::SyntaxError),
        };

        // NOTE: It seems not to be described in the specification but it's a syntax error in major
        // implementations when the label does not denote an IterationStatement.

        self.enqueue(Node::ContinueStatement(symbol));
        self.replace(3, Detail::ContinueStatement);
        Ok(())
    }

    // 14.9 The break Statement

    fn is_within_breakable_statement(&self) -> bool {
        self.iteration_statement_depth > 0 || self.switch_statement_depth > 0
    }

    // BreakStatement[Yield, Await] :
    //   break ;
    fn process_break_statement(&mut self) -> Result<(), Error> {
        // 14.9.1 Static Semantics: Early Errors
        // The BreakStatement is allowed only inside an IterationStatement or a SwitchStatement.
        //
        // TODO: set details of the syntax error like this:
        // Illegal break statement
        ensure!(self.is_within_breakable_statement());

        self.enqueue(Node::BreakStatement(Symbol::NONE));
        self.replace(2, Detail::BreakStatement);
        Ok(())
    }

    // BreakStatement[Yield, Await] :
    //   break [no LineTerminator here] LabelIdentifier[?Yield, ?Await] ;
    fn process_break_statement_with_label(&mut self) -> Result<(), Error> {
        // 14.9.1 Static Semantics: Early Errors
        // The BreakStatement is allowed only inside an IterationStatement or a SwitchStatement.
        //
        // TODO: set details of the syntax error like this:
        // Illegal break statement
        ensure!(self.is_within_breakable_statement());

        let symbol = match self.nth(1).detail {
            Detail::LabelIdentifier(symbol) => symbol,
            _ => unreachable!(),
        };

        // NOTE: It seems not to be described in the specification but it's a syntax error in major
        // implementations when the label is not contained in the label set.
        match self
            .label_stack
            .iter_mut()
            .rev()
            .find(|label| label.symbol == symbol)
        {
            Some(label) => label.num_break_statements += 1,
            None => return Err(Error::SyntaxError),
        }

        // NOTE: It seems not to be described in the specification but it's a syntax error in major
        // implementations when the label does not denote an IterationStatement or a
        // SwitchStatement.

        self.enqueue(Node::BreakStatement(symbol));
        self.replace(3, Detail::BreakStatement);
        Ok(())
    }

    // 14.10 The return Statement

    // ReturnStatement[Yield, Await] :
    //   return ;
    fn process_return_statement(&mut self) -> Result<(), Error> {
        self.enqueue(Node::ReturnStatement(0));
        self.replace(2, Detail::ReturnStatement);
        Ok(())
    }

    // ReturnStatement[Yield, Await] :
    //   return [no LineTerminator here] Expression[+In, ?Yield, ?Await] ;
    fn process_return_value_statement(&mut self) -> Result<(), Error> {
        self.enqueue(Node::ReturnStatement(1));
        self.replace(3, Detail::ReturnStatement);
        Ok(())
    }

    // 14.12 The switch Statement

    // SwitchStatement[Yield, Await, Return] :
    //   switch ( Expression[+In, ?Yield, ?Await] ) CaseBlock[?Yield, ?Await, ?Return]
    fn process_switch_statement(&mut self) -> Result<(), Error> {
        assert!(self.switch_statement_depth > 0);
        self.switch_statement_depth -= 1;
        self.enqueue(Node::SwitchStatement);
        self.replace(5, Detail::SwitchStatement);
        Ok(())
    }

    // _CASE_BLOCK_
    fn process_case_block(&mut self) -> Result<(), Error> {
        ensure!(self.switch_statement_depth < MAX_SWITCH_STATEMENT_DEPTH);
        self.switch_statement_depth += 1;
        self.enqueue(Node::CaseBlock);
        Ok(())
    }

    // CaseBlock[Yield, Await, Return] :
    //   { }
    fn process_case_block_empty(&mut self) -> Result<(), Error> {
        self.replace(2, Detail::CaseBlock);
        Ok(())
    }

    // CaseBlock[Yield, Await, Return] :
    //   { CaseClauses[?Yield, ?Await, ?Return] }
    fn process_case_block_cases(&mut self) -> Result<(), Error> {
        self.replace(3, Detail::CaseBlock);
        Ok(())
    }

    // CaseBlock[Yield, Await, Return] :
    //   { DefaultClause[?Yield, ?Await, ?Return] }
    fn process_case_block_default(&mut self) -> Result<(), Error> {
        self.replace(3, Detail::CaseBlock);
        Ok(())
    }

    // CaseBlock[Yield, Await, Return] :
    //   { CaseClauses[?Yield, ?Await, ?Return] DefaultClause[?Yield, ?Await, ?Return] }
    fn process_case_block_cases_default(&mut self) -> Result<(), Error> {
        self.replace(4, Detail::CaseBlock);
        Ok(())
    }

    // CaseBlock[Yield, Await, Return] :
    //   { DefaultClause[?Yield, ?Await, ?Return] CaseClauses[?Yield, ?Await, ?Return] }
    fn process_case_block_default_cases(&mut self) -> Result<(), Error> {
        self.replace(4, Detail::CaseBlock);
        Ok(())
    }

    // CaseBlock[Yield, Await, Return] :
    //   { CaseClauses[?Yield, ?Await, ?Return] DefaultClause[?Yield, ?Await, ?Return]
    //   CaseClauses[?Yield, ?Await, ?Return] }
    fn process_case_block_cases_default_cases(&mut self) -> Result<(), Error> {
        self.replace(5, Detail::CaseBlock);
        Ok(())
    }

    // CaseClauses[Yield, Await, Return] :
    //   CaseClause[?Yield, ?Await, ?Return]
    fn process_case_clauses_head(&mut self) -> Result<(), Error> {
        self.replace(1, Detail::CaseClauseList);
        Ok(())
    }

    // CaseClauses[Yield, Await, Return] :
    //   CaseClauses[?Yield, ?Await, ?Return] CaseClause[?Yield, ?Await, ?Return]
    fn process_case_clauses(&mut self) -> Result<(), Error> {
        self.pop();
        self.update_ends();
        Ok(())
    }

    // _CASE_SELECTOR_
    fn process_case_selector(&mut self) -> Result<(), Error> {
        self.enqueue(Node::CaseSelector);
        Ok(())
    }

    // CaseClause[Yield, Await, Return] :
    //   case Expression[+In, ?Yield, ?Await] :
    fn process_case_clause_empty(&mut self) -> Result<(), Error> {
        self.enqueue(Node::CaseClause(false));
        self.replace(3, Detail::CaseClause);
        Ok(())
    }

    // CaseClause[Yield, Await, Return] :
    //   case Expression[+In, ?Yield, ?Await] : StatementList[?Yield, ?Await, ?Return]
    fn process_case_clause(&mut self) -> Result<(), Error> {
        self.enqueue(Node::CaseClause(true));
        self.replace(4, Detail::CaseClause);
        Ok(())
    }

    // _DEFAULT_SELECTOR_
    fn process_default_selector(&mut self) -> Result<(), Error> {
        self.enqueue(Node::DefaultSelector);
        Ok(())
    }

    // DefaultClause[Yield, Await, Return] :
    //   default :
    fn process_default_clause_empty(&mut self) -> Result<(), Error> {
        self.enqueue(Node::DefaultClause(false));
        self.replace(2, Detail::DefaultClause);
        Ok(())
    }

    // DefaultClause[Yield, Await, Return] :
    //   default : StatementList[?Yield, ?Await, ?Return]
    fn process_default_clause(&mut self) -> Result<(), Error> {
        self.enqueue(Node::DefaultClause(true));
        self.replace(3, Detail::DefaultClause);
        Ok(())
    }

    // 14.13 Labelled Statements

    // LabelledStatement[Yield, Await, Return] :
    //   LabelIdentifier[?Yield, ?Await] : LabelledItem[?Yield, ?Await, ?Return]
    fn process_labelled_statement(&mut self) -> Result<(), Error> {
        let labelled_item = match self.top().detail {
            Detail::DoWhileStatement | Detail::WhileStatement | Detail::ForStatement => {
                LabelledItem::IterationStatement
            }
            Detail::LabelledStatement(labelled_item) => labelled_item,
            _ => LabelledItem::OtherStatement,
        };
        let is_iteration_statement = matches!(labelled_item, LabelledItem::IterationStatement);

        let label = self.label_stack.pop().unwrap();

        // It seems not to be described in the specification but it's a syntax error in major
        // implementations when the label does not denote an iteration statement.
        ensure!(label.num_continue_statements == 0 || is_iteration_statement);

        // TODO: unused label (num_continue_statements == 0 && num_break_statements == 0) may be
        // able to be removed in the semantics analysis phase.  We can add a variable for this to
        // Node::LabelledStatement.
        self.enqueue(Node::LabelledStatement(
            label.symbol,
            is_iteration_statement,
        ));
        self.replace(3, Detail::LabelledStatement(labelled_item));
        Ok(())
    }

    // _LABEL_
    fn process_label(&mut self) -> Result<(), Error> {
        let symbol = match self.nth(1).detail {
            Detail::LabelIdentifier(symbol) => symbol,
            _ => unreachable!(),
        };
        self.enqueue(Node::Label(symbol));
        self.label_stack.push(Label {
            symbol,
            ..Default::default()
        });
        Ok(())
    }

    // 14.14 The throw Statement

    // ThrowStatement[Yield, Await] :
    //   throw [no LineTerminator here] Expression[+In, ?Yield, ?Await] ;
    fn process_throw_statement(&mut self) -> Result<(), Error> {
        self.enqueue(Node::ThrowStatement);
        self.replace(2, Detail::ThrowStatement);
        Ok(())
    }

    // 14.15 The try Statement

    // TryStatement[Yield, Await, Return] :
    //   try Block[?Yield, ?Await, ?Return] Catch[?Yield, ?Await, ?Return]
    fn process_try_statement_no_finally(&mut self) -> Result<(), Error> {
        self.enqueue(Node::TryStatement);
        self.replace(3, Detail::TryStatement);
        Ok(())
    }

    // TryStatement[Yield, Await, Return] :
    //   try Block[?Yield, ?Await, ?Return] Finally[?Yield, ?Await, ?Return]
    fn process_try_statement_no_catch(&mut self) -> Result<(), Error> {
        self.enqueue(Node::TryStatement);
        self.replace(3, Detail::TryStatement);
        Ok(())
    }

    // TryStatement[Yield, Await, Return] :
    //   try Block[?Yield, ?Await, ?Return] Catch[?Yield, ?Await, ?Return]
    //   Finally[?Yield, ?Await, ?Return]
    fn process_try_statement(&mut self) -> Result<(), Error> {
        self.enqueue(Node::TryStatement);
        self.replace(4, Detail::TryStatement);
        Ok(())
    }

    // Catch[Yield, Await, Return] :
    //   catch ( CatchParameter[?Yield, ?Await] ) Block[?Yield, ?Await, ?Return]
    fn process_catch(&mut self) -> Result<(), Error> {
        self.enqueue(Node::CatchClause(true));
        self.replace(5, Detail::CatchClause);
        Ok(())
    }

    // Catch[Yield, Await, Return] :
    //   catch Block[?Yield, ?Await, ?Return]
    fn process_catch_no_parameter(&mut self) -> Result<(), Error> {
        self.enqueue(Node::CatchClause(false));
        self.replace(2, Detail::CatchClause);
        Ok(())
    }

    // Finally[Yield, Await, Return] :
    //   finally Block[?Yield, ?Await, ?Return]
    fn process_finally(&mut self) -> Result<(), Error> {
        self.enqueue(Node::FinallyClause);
        self.replace(2, Detail::FinallyClause);
        Ok(())
    }

    // CatchParameter[Yield, Await] :
    //   BindingIdentifier[?Yield, ?Await]
    fn process_catch_parameter(&mut self) -> Result<(), Error> {
        self.enqueue(Node::CatchParameter);
        Ok(())
    }

    // CatchParameter[Yield, Await] :
    //   BindingPattern[?Yield, ?Await]

    // _TRY_BLOCK_
    fn process_try_block(&mut self) -> Result<(), Error> {
        self.enqueue(Node::TryBlock);
        Ok(())
    }

    // _CATCH_BLOCK_
    fn process_catch_block(&mut self) -> Result<(), Error> {
        self.enqueue(Node::CatchBlock);
        Ok(())
    }

    // _FINALLY_BLOCK_
    fn process_finally_block(&mut self) -> Result<(), Error> {
        self.enqueue(Node::FinallyBlock);
        Ok(())
    }

    // 14.16 The debugger Statement

    // DebuggerStatement :
    //   debugger ;
    fn process_debugger_statement(&mut self) -> Result<(), Error> {
        self.enqueue(Node::DebuggerStatement);
        self.replace(2, Detail::DebuggerStatement);
        Ok(())
    }

    // 15.1 Parameter Lists

    // FormalParameters[Yield, Await] :
    //   [empty]
    fn process_formal_parameters_empty(&mut self) -> Result<(), Error> {
        let node_index = self.enqueue(Node::FormalParameters(0));
        let token_index = self.tokens.len();
        self.push(Syntax {
            detail: Detail::FormalParameters(smallvec![]),
            nodes_range: node_index..(node_index + 1),
            tokens_range: token_index..token_index,
            source_range: self.location.offset..self.location.offset,
        });
        Ok(())
    }

    // FormalParameters[Yield, Await] :
    //   FormalParameterList[?Yield, ?Await]
    fn process_formal_parameters_list(&mut self) -> Result<(), Error> {
        let n = match self.top().detail {
            Detail::FormalParameters(ref bound_names) => bound_names.len(),
            _ => unreachable!(),
        };
        self.enqueue(Node::FormalParameters(n as u32));
        self.update_ends();
        Ok(())
    }

    // FormalParameters[Yield, Await] :
    //   FormalParameterList[?Yield, ?Await] ,
    fn process_formal_parameters_list_with_comma(&mut self) -> Result<(), Error> {
        self.pop();
        let n = match self.top().detail {
            Detail::FormalParameters(ref bound_names) => bound_names.len(),
            _ => unreachable!(),
        };
        self.enqueue(Node::FormalParameters(n as u32));
        self.update_ends();
        Ok(())
    }

    // FormalParameterList[Yield, Await] :
    //   FormalParameterList[?Yield, ?Await] , FormalParameter[?Yield, ?Await]
    fn process_formal_parameter_list(&mut self) -> Result<(), Error> {
        let bound_names = match self.pop().detail {
            Detail::FormalParameters(bound_names) => bound_names,
            _ => unreachable!(),
        };
        self.pop();
        match self.top_mut().detail {
            Detail::FormalParameters(ref mut dest) => {
                for name in bound_names.into_iter() {
                    // 15.1.1 Static Semantics: Early Errors
                    ensure!(!dest.contains(&name));
                    dest.push(name);
                }
            }
            _ => unreachable!(),
        }
        self.update_ends();
        Ok(())
    }

    // FormalParameter[Yield, Await] :
    //    BindingElement[?Yield, ?Await]
    fn process_formal_parameter(&mut self) -> Result<(), Error> {
        self.enqueue(Node::FormalParameter);
        let bound_names = match self.top().detail {
            Detail::BindingElement(ref binding) => match binding.kind {
                BindingElementKind::SingleNameBinding(symbol) => smallvec![symbol],
            },
            _ => unreachable!(),
        };
        self.replace(1, Detail::FormalParameters(bound_names));
        Ok(())
    }

    // 15.2 Function Definitions

    // FunctionDeclaration[Yield, Await, Default] :
    //   function BindingIdentifier[?Yield, ?Await] ( FormalParameters[~Yield, ~Await] )
    //   { FunctionBody[~Yield, ~Await] }
    fn process_function_declaration(&mut self) -> Result<(), Error> {
        self.enqueue(Node::FunctionDeclaration);
        self.replace(8, Detail::Declaration);
        Ok(())
    }

    // FunctionExpression :
    //   function ( FormalParameters[~Yield, ~Await] ) { FunctionBody[~Yield, ~Await] }
    fn process_anonymous_function_expression(&mut self) -> Result<(), Error> {
        self.enqueue(Node::FunctionExpression(false));
        self.replace(7, Detail::Expression);
        Ok(())
    }

    // FunctionExpression :
    //   function BindingIdentifier[~Yield, ~Await] ( FormalParameters[~Yield, ~Await] )
    //   { FunctionBody[~Yield, ~Await] }
    fn process_function_expression(&mut self) -> Result<(), Error> {
        self.enqueue(Node::FunctionExpression(true));
        self.replace(8, Detail::Expression);
        Ok(())
    }

    // FunctionStatementList[Yield, Await] :
    //   [empty]
    fn process_function_statement_list_empty(&mut self) -> Result<(), Error> {
        let node_index = self.nodes.len();
        let token_index = self.tokens.len();
        // 15.2.1 Static Semantics: Early Errors
        self.push(Syntax {
            detail: Detail::StatementList,
            nodes_range: node_index..node_index,
            tokens_range: token_index..token_index,
            source_range: self.location.offset..self.location.offset,
        });
        Ok(())
    }

    // 15.3 Arrow Function Definitions

    // ArrowFunction[In, Yield, Await] :
    //   ArrowParameters[?Yield, ?Await] [no LineTerminator here] => ConciseBody[?In]
    fn process_arrow_function(&mut self) -> Result<(), Error> {
        // TODO: 15.3.1 Static Semantics: Early Errors
        self.enqueue(Node::ArrowFunction);
        self.replace(3, Detail::Expression);
        Ok(())
    }

    // ArrowParameters[Yield, Await] :
    //   BindingIdentifier[?Yield, ?Await]
    fn process_arrow_parameters_binding_identifier(&mut self) -> Result<(), Error> {
        self.process_single_arrow_parameter(Node::ArrowFunctionContext)
    }

    fn process_single_arrow_parameter(&mut self, context_node: Node<'s>) -> Result<(), Error> {
        let i = self.enqueue(context_node);
        debug_assert!(i > 0);
        self.nodes.swap(i - 1, i); // swap BindingIdentifier and context_node.
        self.enqueue(Node::FormalParameter);
        self.enqueue(Node::FormalParameters(1));
        let bound_names = match self.top().detail {
            Detail::BindingIdentifier(symbol) => smallvec![symbol],
            _ => unreachable!(),
        };
        self.replace(1, Detail::FormalParameters(bound_names));
        Ok(())
    }

    // ArrowParameters[Yield, Await] :
    //   CoverParenthesizedExpressionAndArrowParameterList[?Yield, ?Await]

    fn process_arrow_parameters_cpeaapl(&mut self) -> Result<(), Error> {
        self.refine_arrow_parameters(GoalSymbol::ArrowFormalParameters)
    }

    fn process_arrow_parameters_cpeaapl_yield(&mut self) -> Result<(), Error> {
        self.refine_arrow_parameters(GoalSymbol::ArrowFormalParameters_Yield)
    }

    fn process_arrow_parameters_cpeaapl_await(&mut self) -> Result<(), Error> {
        self.refine_arrow_parameters(GoalSymbol::ArrowFormalParameters_Await)
    }

    fn process_arrow_parameters_cpeaapl_yield_await(&mut self) -> Result<(), Error> {
        self.refine_arrow_parameters(GoalSymbol::ArrowFormalParameters_Yield_Await)
    }

    fn refine_arrow_parameters(&mut self, goal_symbol: GoalSymbol) -> Result<(), Error> {
        let syntax = self.pop();
        self.tokens.truncate(syntax.tokens_range.start);
        self.nodes.truncate(syntax.nodes_range.start);
        self.enqueue(Node::ArrowFunctionContext);
        self.refine(&syntax, goal_symbol)
    }

    // ArrowFormalParameters[Yield, Await] :
    //   ( UniqueFormalParameters[?Yield, ?Await] )
    fn process_arrow_formal_parameters(&mut self) -> Result<(), Error> {
        let rparen = self.pop();
        let formal_parameters = self.pop();
        let tokens_end = self.tokens.len();
        let nodes_end = self.nodes.len();
        let syntax = self.top_mut();
        syntax.detail = formal_parameters.detail;
        syntax.tokens_range.end = tokens_end;
        syntax.nodes_range.end = nodes_end;
        syntax.source_range.end = rparen.source_range.end;
        Ok(())
    }

    // ConciseBody[In] :
    //   [lookahead ≠ {] ExpressionBody[?In, ~Await]
    fn process_concise_body_expression_body(&mut self) -> Result<(), Error> {
        self.enqueue(Node::ReturnStatement(1));
        self.replace(1, Detail::ConciseBody); // expression
        Ok(())
    }

    // ConciseBody[In] :
    //   { FunctionBody[~Yield, ~Await] }
    fn process_concise_body_function_body(&mut self) -> Result<(), Error> {
        self.replace(3, Detail::ConciseBody); // function body
        Ok(())
    }

    // 15.8 Async Function Definitions

    // AsyncFunctionDeclaration[Yield, Await, Default] :
    //   async [no LineTerminator here] function BindingIdentifier[?Yield, ?Await]
    //   ( FormalParameters[~Yield, +Await] ) { AsyncFunctionBody }
    fn process_async_function_declaration(&mut self) -> Result<(), Error> {
        self.enqueue(Node::AsyncFunctionDeclaration);
        self.replace(9, Detail::Declaration);
        Ok(())
    }

    // AsyncFunctionExpression :
    //   async [no LineTerminator here] function BindingIdentifier[~Yield, +Await]
    //   ( FormalParameters[~Yield, +Await] ) { AsyncFunctionBody }
    fn process_async_function_expression(&mut self) -> Result<(), Error> {
        self.enqueue(Node::AsyncFunctionExpression(true));
        self.replace(9, Detail::Expression);
        Ok(())
    }

    // AsyncFunctionExpression :
    //   async [no LineTerminator here] function
    //   ( FormalParameters[~Yield, +Await] ) { AsyncFunctionBody }
    fn process_anonymous_async_function_expression(&mut self) -> Result<(), Error> {
        self.enqueue(Node::AsyncFunctionExpression(false));
        self.replace(8, Detail::Expression);
        Ok(())
    }

    // AwaitExpression[Yield] :
    //   await UnaryExpression[?Yield, +Await]
    fn process_await(&mut self) -> Result<(), Error> {
        self.enqueue(Node::AwaitExpression);
        self.replace(2, Detail::Expression);
        Ok(())
    }

    // 15.9 Async Arrow Function Definitions

    // AsyncArrowFunction[In, Yield, Await] :
    //   async [no LineTerminator here] AsyncArrowBindingIdentifier[?Yield]
    //   [no LineTerminator here] => AsyncConciseBody[?In]
    fn process_async_arrow_function(&mut self) -> Result<(), Error> {
        self.enqueue(Node::AsyncArrowFunction);
        self.replace(4, Detail::Expression);
        Ok(())
    }

    // AsyncArrowFunction[In, Yield, Await] :
    //   AsyncArrowHeadCCEAAAH[?Yield, ?Await] [no LineTerminator here] =>
    //   AsyncConciseBody[?In]
    fn process_async_arrow_function_cceaaah(&mut self) -> Result<(), Error> {
        self.enqueue(Node::AsyncArrowFunction);
        self.replace(3, Detail::Expression);
        Ok(())
    }

    // AsyncConciseBody[In] :
    //   [lookahead ≠ {] ExpressionBody[?In, +Await]
    fn process_async_concise_body_expression_body(&mut self) -> Result<(), Error> {
        self.enqueue(Node::ReturnStatement(1));
        self.replace(1, Detail::AsyncConciseBody);
        Ok(())
    }

    // AsyncConciseBody[In] :
    //   { AsyncFunctionBody }
    fn process_async_concise_body_async_function_body(&mut self) -> Result<(), Error> {
        self.replace(3, Detail::AsyncConciseBody);
        Ok(())
    }

    // AsyncArrowBindingIdentifier[Yield] :
    //   BindingIdentifier[?Yield, +Await]
    fn process_async_arrow_binding_identifier(&mut self) -> Result<(), Error> {
        self.process_single_arrow_parameter(Node::AsyncArrowFunctionContext)
    }

    // AsyncArrowHeadCCEAAAH[Yield, Await] :
    //   CoverCallExpressionAndAsyncArrowHead[?Yield, ?Await]
    fn process_async_arrow_head_cceaaah(&mut self) -> Result<(), Error> {
        self.refine_async_arrow_head()
    }

    fn refine_async_arrow_head(&mut self) -> Result<(), Error> {
        let syntax = self.pop();
        self.tokens.truncate(syntax.tokens_range.start);
        self.nodes.truncate(syntax.nodes_range.start);
        self.enqueue(Node::AsyncArrowFunctionContext);
        self.refine(&syntax, GoalSymbol::AsyncArrowHead)
    }

    // AsyncArrowHead :
    //   async [no LineTerminator here] ArrowFormalParameters[~Yield, +Await]
    fn process_async_arrow_head(&mut self) -> Result<(), Error> {
        let formal_parameters = self.pop();
        let tokens_end = self.tokens.len();
        let nodes_end = self.nodes.len();
        let syntax = self.top_mut();
        syntax.detail = formal_parameters.detail;
        syntax.tokens_range.end = tokens_end;
        syntax.nodes_range.end = nodes_end;
        syntax.source_range.end = formal_parameters.source_range.end;
        Ok(())
    }

    // CoverCallExpressionAndAsyncArrowHead[Yield, Await] :
    //   MemberExpression[?Yield, ?Await] Arguments[?Yield, ?Await]
    fn process_cover_call_expression_and_async_arrow_head(&mut self) -> Result<(), Error> {
        self.replace(2, Detail::CoverCallExpressionAndAsyncArrowHead);
        Ok(())
    }

    // 16.1 Scripts

    // Script :
    //   [empty]
    fn process_empty_script(&mut self) -> Result<(), Error> {
        Ok(())
    }

    // Script :
    //   ScriptBody
    fn process_script(&mut self) -> Result<(), Error> {
        self.pop();
        Ok(())
    }

    // 16.2 Modules

    // Module :
    //   [empty]
    fn process_empty_module(&mut self) -> Result<(), Error> {
        Ok(())
    }

    // Module :
    //   ModuleBody
    fn process_module(&mut self) -> Result<(), Error> {
        self.pop();
        Ok(())
    }

    // ModuleItemList :
    //   ModuleItem
    fn process_module_item_list_head(&mut self) -> Result<(), Error> {
        self.top_mut().detail = Detail::ModuleItemList;
        Ok(())
    }

    // ModuleItemList :
    //   ModuleItemList ModuleItem
    fn process_module_item_list_item(&mut self) -> Result<(), Error> {
        self.pop();
        self.update_ends();
        Ok(())
    }
}

impl<'s, H> SyntaxHandler<'s> for Processor<'s, H>
where
    H: NodeHandler<'s>,
{
    type Artifact = H::Artifact;
    type Error = Error;

    fn start(&mut self) {
        logger::debug!(event = "start");
        self.handler.start();
    }

    fn source(&mut self, src: &'s str) {
        logger::debug!(event = "source");
        self.source = src;
    }

    fn accept(&mut self) -> Result<Self::Artifact, Self::Error> {
        logger::debug!(event = "accept");
        let nodes = std::mem::take(&mut self.nodes);
        self.handler.handle_nodes(nodes.into_iter())?;
        self.handler.accept()
    }

    fn shift(&mut self, token: &Token<'s>) -> Result<(), Self::Error> {
        logger::debug!(
            event = "shift",
            ?token.kind,
            inserted_automaticaly = token.inserted_automatically(),
            start = %self.location,
            end = %token.compute_end(&self.location),
        );

        let node_index = self.nodes.len();
        let token_index = self.tokens.len();

        // Tokens coming from the `parser` module are held until refinements of permissive
        // production rules in a statement are processed.
        self.tokens.push(token.clone());

        self.push(Syntax {
            detail: Detail::Token(token_index),
            nodes_range: node_index..node_index,
            tokens_range: token_index..(token_index + 1),
            source_range: self.location.offset..(self.location.offset + token.lexeme.len()),
        });

        Ok(())
    }

    fn reduce(&mut self, rule: ProductionRule) -> Result<(), Self::Error> {
        match Self::ACTIONS[rule.id() as usize] {
            Action::Undefined => unimplemented!("No action defined for: {rule}"),
            Action::Nop => {
                logger::debug!(event = "reduce", action = "nop", %rule);
                Ok(())
            }
            Action::Invoke(action, name) => {
                logger::debug!(event = "reduce", action = name, %rule);
                action(self)
            }
        }
    }

    fn location(&mut self, location: &Location) {
        logger::debug!(event = "location", %location);
        self.location = location.clone();
    }
}

enum Action<'s, H> {
    Undefined,
    Nop,
    Invoke(fn(&mut Processor<'s, H>) -> Result<(), Error>, &'static str),
}

#[derive(Default)]
struct Label {
    symbol: Symbol,
    // TODO: The current implementation does not work with `eval(continue label)`.  The script
    // inside `eval()` will be evaluated at runtime.
    num_continue_statements: usize,
    num_break_statements: usize,
}

struct Refinery<'s, 'p, H> {
    processor: &'p mut Processor<'s, H>,
    location_offset: usize,
}

impl<'s, 'p, H> Refinery<'s, 'p, H> {
    fn new(processor: &'p mut Processor<'s, H>, location_offset: usize) -> Self {
        Self {
            processor,
            location_offset,
        }
    }
}

impl<'s, H> SyntaxHandler<'s> for Refinery<'s, '_, H>
where
    H: NodeHandler<'s>,
{
    type Artifact = ();
    type Error = Error;

    fn accept(&mut self) -> Result<Self::Artifact, Self::Error> {
        Ok(())
    }

    fn shift(&mut self, token: &Token<'s>) -> Result<(), Self::Error> {
        self.processor.shift(token)
    }

    fn reduce(&mut self, rule: ProductionRule) -> Result<(), Self::Error> {
        self.processor.reduce(rule)
    }

    fn location(&mut self, location: &Location) {
        let mut location = location.clone();
        // `Processor` never uses `line` and `column`.
        location.offset += self.location_offset;
        // TODO: calculate `line` and `column` if we support showing the line and column number in
        // the error message.
        self.processor.location(&location);
    }
}
