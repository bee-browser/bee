mod actions;
mod logger;

use std::ops::Range;

use smallvec::smallvec;
use smallvec::SmallVec;

use super::Error;
use super::Location;
use super::ProductionRule;
use super::Symbol;
use super::SymbolRegistry;
use super::SyntaxHandler;
use super::Token;
use super::TokenKind;

pub trait NodeHandler<'s> {
    type Artifact;

    fn start(&mut self);
    fn accept(&mut self) -> Result<Self::Artifact, Error>;
    fn handle_nodes(&mut self, nodes: impl Iterator<Item = Node<'s>>) -> Result<(), Error>;
    fn symbol_registry_mut(&mut self) -> &mut SymbolRegistry;
}

pub struct Processor<'s, H> {
    handler: H,
    location: Location,
    stack: Vec<Syntax>,
    nodes: Vec<Node<'s>>,
    tokens: Vec<Token<'s>>,
    strict_mode: bool,
    module: bool,
}

#[derive(Debug)]
struct Syntax {
    detail: Detail,
    nodes_range: Range<usize>,
    tokens_range: Range<usize>,
}

#[derive(Debug)]
enum Detail {
    Token(usize),
    Literal,
    Identifier(Symbol),
    IdentifierReference(#[allow(unused)] Symbol), // TODO: SS
    BindingIdentifier(Symbol),
    LabelIdentifier(#[allow(unused)] Symbol), // TODO: SS
    CpeaaplExpression,
    CpeaaplFormalParameters,
    CpeaaplEmpty,
    CpeaaplRestParameter,
    CpeaaplRestPattern,
    CpeaaplFormalParametersWithRestParameter,
    CpeaaplFormalParametersWithRestPattern,
    Arguments,
    ArgumentList,
    Expression,
    Initializer,
    Block,
    LexicalBinding(LexicalDeclarationSemantics),
    BindingList(LexicalDeclarationSemantics),
    LetDeclaration(#[allow(unused)] SmallVec<[Symbol; 4]>), // TODO: SS
    ConstDeclaration(#[allow(unused)] SmallVec<[Symbol; 4]>), // TODO: SS
    SingleNameBinding(Symbol, bool),
    BindingElement(BindingElement),
    Statement,
    Declaration,
    FormalParameters(SmallVec<[Symbol; 4]>),
    StatementList,
    CoverCallExpressionAndAsyncArrowHead,
}

#[derive(Debug)]
struct LexicalDeclarationSemantics {
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
    IdentifierReference(Symbol),
    BindingIdentifier(Symbol),
    LabelIdentifier(Symbol),
    ArgumentListHead(bool, bool),
    ArgumentListItem(bool),
    Arguments,
    CallExpression,
    UpdateExpression(UpdateOperator),
    UnaryExpression(UnaryOperator),
    BinaryExpression(BinaryOperator),
    LogicalExpression(LogicalOperator),
    ConditionalExpression,
    AssignmentExpression(AssignmentOperator),
    BlockStatement,
    LexicalBinding(bool),
    LetDeclaration(u32),
    ConstDeclaration(u32),
    BindingElement(bool),
    EmptyStatement,
    ExpressionStatement,
    IfElseStatement,
    IfStatement,
    ReturnStatement(u32),
    FormalParameter,
    FormalParameters(u32),
    FunctionContext,
    FunctionSignature(Symbol),
    FunctionDeclaration,
    FunctionExpression(bool),
    ThenBlock,
    ElseBlock,
    FalsyShortCircuit,
    TruthyShortCircuit,
    NullishShortCircuit,
    FalsyShortCircuitAssignment,
    TruthyShortCircuitAssignment,
    NullishShortCircuitAssignment,
    StartBlockScope,
    EndBlockScope,
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

impl<'s, H> Processor<'s, H>
where
    H: NodeHandler<'s>,
{
    const INITIAL_STACK_CAPACITY: usize = 64;
    const INITIAL_QUEUE_CAPACITY: usize = 128;
    const INITIAL_TOKENS_CAPACITY: usize = 1024;

    pub fn new(handler: H, module: bool) -> Self {
        Self {
            handler,
            location: Default::default(),
            stack: Vec::with_capacity(Self::INITIAL_STACK_CAPACITY),
            nodes: Vec::with_capacity(Self::INITIAL_QUEUE_CAPACITY),
            tokens: Vec::with_capacity(Self::INITIAL_TOKENS_CAPACITY),
            strict_mode: false,
            module,
        }
    }

    #[inline(always)]
    fn top(&mut self) -> &Syntax {
        let len = self.stack.len();
        debug_assert!(len >= 1);
        &self.stack[len - 1]
    }

    #[inline(always)]
    fn top_mut(&mut self) -> &mut Syntax {
        let len = self.stack.len();
        debug_assert!(len >= 1);
        &mut self.stack[len - 1]
    }

    #[inline(always)]
    fn push(&mut self, syntax: Syntax) {
        self.stack.push(syntax);
    }

    #[inline(always)]
    fn pop(&mut self) -> Syntax {
        self.stack.pop().unwrap()
    }

    fn replace(&mut self, n: usize, detail: Detail) {
        debug_assert!(n > 0);
        let nodes_end = self.nodes.len();
        let tokens_end = self.tokens.len();
        self.stack.truncate(self.stack.len() - (n - 1));
        let syntax = self.stack.last_mut().unwrap();
        syntax.detail = detail;
        syntax.nodes_range.end = nodes_end;
        syntax.tokens_range.end = tokens_end;
    }

    #[inline(always)]
    fn enqueue(&mut self, event: Node<'s>) -> usize {
        let index = self.nodes.len();
        self.nodes.push(event);
        index
    }

    #[inline(always)]
    fn make_symbol(&mut self, token_index: usize) -> Symbol {
        let lexeme = self.tokens[token_index].lexeme;
        self.handler.symbol_registry_mut().intern_str(lexeme)
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

    // _THEN_BLOCK_
    fn process_then_block(&mut self) -> Result<(), Error> {
        self.enqueue(Node::ThenBlock);
        Ok(())
    }

    // _ELSE_BLOCK_
    fn process_else_block(&mut self) -> Result<(), Error> {
        self.enqueue(Node::ElseBlock);
        Ok(())
    }

    // _BLOCK_SCOPE_
    fn process_block_scope(&mut self) -> Result<(), Error> {
        self.enqueue(Node::StartBlockScope);
        Ok(())
    }

    // _FUNCTION_CONTEXT_
    fn process_function_context(&mut self) -> Result<(), Error> {
        self.enqueue(Node::FunctionContext);
        Ok(())
    }

    // _FUNCTION_SIGNATURE_
    fn process_function_signature(&mut self) -> Result<(), Error> {
        let func_name = match self.stack[self.stack.len() - 4].detail {
            Detail::BindingIdentifier(symbol) => symbol,
            Detail::Token(index) => {
                debug_assert!(matches!(self.tokens[index].kind, TokenKind::Function));
                Symbol::NONE // anonymous function
            }
            _ => unreachable!(),
        };
        self.enqueue(Node::FunctionSignature(func_name));
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
        if self.strict_mode {
            // 13.1.1 Static Semantics: Early Errors
            Err(Error::SyntaxError)
        } else {
            self.process_identifier_reference()
        }
    }

    // IdentifierReference : await
    fn process_identifier_reference_only_in_script(&mut self) -> Result<(), Error> {
        if self.module {
            // 13.1.1 Static Semantics: Early Errors
            Err(Error::SyntaxError)
        } else {
            self.process_identifier_reference()
        }
    }

    // IdentifierReference_Await : Identifier
    fn process_identifier_reference_except_for_await(&mut self) -> Result<(), Error> {
        let symbol = match self.top().detail {
            Detail::Identifier(symbol) => symbol,
            _ => unreachable!(),
        };
        match symbol {
            // 13.1.1 Static Semantics: Early Errors
            SymbolRegistry::AWAIT => Err(Error::SyntaxError),
            _ => self.process_identifier_reference(),
        }
    }

    // IdentifierReference_Yield : Identifier
    fn process_identifier_reference_except_for_yield(&mut self) -> Result<(), Error> {
        let symbol = match self.top().detail {
            Detail::Identifier(symbol) => symbol,
            _ => unreachable!(),
        };
        match symbol {
            // 13.1.1 Static Semantics: Early Errors
            SymbolRegistry::YIELD => Err(Error::SyntaxError),
            _ => self.process_identifier_reference(),
        }
    }

    // IdentifierReference_Yield_Await : Identifier
    fn process_identifier_reference_except_for_yield_await(&mut self) -> Result<(), Error> {
        let symbol = match self.top().detail {
            Detail::Identifier(symbol) => symbol,
            _ => unreachable!(),
        };
        match symbol {
            // 13.1.1 Static Semantics: Early Errors
            SymbolRegistry::YIELD | SymbolRegistry::AWAIT => Err(Error::SyntaxError),
            _ => self.process_identifier_reference(),
        }
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
        match symbol {
            // 13.1.1 Static Semantics: Early Errors
            SymbolRegistry::ARGUMENTS | SymbolRegistry::EVAL if self.strict_mode => {
                Err(Error::SyntaxError)
            }
            _ => self.process_binding_identifier(),
        }
    }

    // BindingIdentifier : yield
    fn process_binding_identifier_only_in_non_strict(&mut self) -> Result<(), Error> {
        if self.strict_mode {
            // 13.1.1 Static Semantics: Early Errors
            Err(Error::SyntaxError)
        } else {
            self.process_binding_identifier()
        }
    }

    // BindingIdentifier : await
    fn process_binding_identifier_only_in_script(&mut self) -> Result<(), Error> {
        if self.module {
            // 13.1.1 Static Semantics: Early Errors
            Err(Error::SyntaxError)
        } else {
            self.process_binding_identifier()
        }
    }

    // BindingIdentifier_Await : Identifier
    fn process_binding_identifier_except_for_await(&mut self) -> Result<(), Error> {
        let symbol = match self.top().detail {
            Detail::Identifier(symbol) => symbol,
            _ => unreachable!(),
        };
        match symbol {
            // 13.1.1 Static Semantics: Early Errors
            SymbolRegistry::AWAIT => Err(Error::SyntaxError),
            _ => self.process_binding_identifier(),
        }
    }

    // BindingIdentifier_Yield : Identifier
    fn process_binding_identifier_except_for_yield(&mut self) -> Result<(), Error> {
        let symbol = match self.top().detail {
            Detail::Identifier(symbol) => symbol,
            _ => unreachable!(),
        };
        match symbol {
            // 13.1.1 Static Semantics: Early Errors
            SymbolRegistry::YIELD => Err(Error::SyntaxError),
            _ => self.process_binding_identifier(),
        }
    }

    // BindingIdentifier_Yield_Await : Identifier
    fn process_binding_identifier_except_for_yield_await(&mut self) -> Result<(), Error> {
        let symbol = match self.top().detail {
            Detail::Identifier(symbol) => symbol,
            _ => unreachable!(),
        };
        match symbol {
            // 13.1.1 Static Semantics: Early Errors
            SymbolRegistry::YIELD | SymbolRegistry::AWAIT => Err(Error::SyntaxError),
            _ => self.process_binding_identifier(),
        }
    }

    // LabelIdentifier : Identifier
    // LabelIdentifier_Yield : await
    // LabelIdentifier_Await : yield
    fn process_label_identifier(&mut self) -> Result<(), Error> {
        let symbol = match self.top().detail {
            Detail::Identifier(symbol) => symbol,
            _ => unreachable!(),
        };
        self.enqueue(Node::LabelIdentifier(symbol));
        self.replace(1, Detail::LabelIdentifier(symbol));
        Ok(())
    }

    // LabelIdentifier : yield
    fn process_label_identifier_only_in_non_strict(&mut self) -> Result<(), Error> {
        if self.strict_mode {
            // 13.1.1 Static Semantics: Early Errors
            Err(Error::SyntaxError)
        } else {
            self.process_label_identifier()
        }
    }

    // LabelIdentifier : await
    fn process_label_identifier_only_in_script(&mut self) -> Result<(), Error> {
        if self.module {
            // 13.1.1 Static Semantics: Early Errors
            Err(Error::SyntaxError)
        } else {
            self.process_label_identifier()
        }
    }

    // LabelIdentifier_Await : Identifier
    fn process_label_identifier_except_for_await(&mut self) -> Result<(), Error> {
        let symbol = match self.top().detail {
            Detail::Identifier(symbol) => symbol,
            _ => unreachable!(),
        };
        match symbol {
            // 13.1.1 Static Semantics: Early Errors
            SymbolRegistry::AWAIT => Err(Error::SyntaxError),
            _ => self.process_label_identifier(),
        }
    }

    // LabelIdentifier_Yield : Identifier
    fn process_label_identifier_except_for_yield(&mut self) -> Result<(), Error> {
        let symbol = match self.top().detail {
            Detail::Identifier(symbol) => symbol,
            _ => unreachable!(),
        };
        match symbol {
            // 13.1.1 Static Semantics: Early Errors
            SymbolRegistry::YIELD => Err(Error::SyntaxError),
            _ => self.process_label_identifier(),
        }
    }

    // LabelIdentifier_Yield_Await : Identifier
    fn process_label_identifier_except_for_yield_await(&mut self) -> Result<(), Error> {
        let symbol = match self.top().detail {
            Detail::Identifier(symbol) => symbol,
            _ => unreachable!(),
        };
        match symbol {
            // 13.1.1 Static Semantics: Early Errors
            SymbolRegistry::YIELD | SymbolRegistry::AWAIT => Err(Error::SyntaxError),
            _ => self.process_label_identifier(),
        }
    }

    // Identifier : IdentifierName but not ReservedWord
    fn process_identifier(&mut self) -> Result<(), Error> {
        self.pop(); // Token
        let token_index = self.tokens.len() - 1;
        let symbol = self.make_symbol(token_index);
        match symbol {
            // 13.1.1 Static Semantics: Early Errors
            SymbolRegistry::IMPLEMENTS
            | SymbolRegistry::LET
            | SymbolRegistry::PACKAGE
            | SymbolRegistry::PRIVATE
            | SymbolRegistry::PROTECTED
            | SymbolRegistry::PUBLIC
            | SymbolRegistry::STATIC
            | SymbolRegistry::YIELD
                if self.strict_mode =>
            {
                Err(Error::SyntaxError)
            }
            SymbolRegistry::AWAIT if self.module => Err(Error::SyntaxError),
            _ => {
                let node_index = self.nodes.len();
                self.push(Syntax {
                    detail: Detail::Identifier(symbol),
                    nodes_range: node_index..node_index,
                    tokens_range: token_index..(token_index + 1),
                });
                Ok(())
            }
        }
    }

    // 13.2 Primary Expression

    // PrimaryExpression[Yield, Await] : IdentifierReference[?Yield, ?Await]
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

    // CoverParenthesizedExpressionAndArrowParameterList[Yield, Await] : ( )
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
        self.pop(); // Token
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
                let value = token.lexeme.encode_utf16().collect();
                self.enqueue(Node::String(value, token.lexeme))
            }
            _ => unreachable!(),
        };
        self.push(Syntax {
            detail: Detail::Literal,
            nodes_range: node_index..(node_index + 1),
            tokens_range: token_index..(token_index + 1),
        });
        Ok(())
    }

    // 13.2.5 Object Initializer

    // Initializer[In, Yield, Await] : = AssignmentExpression[?In, ?Yield, ?Await]
    fn process_initializer(&mut self) -> Result<(), Error> {
        self.replace(2, Detail::Initializer);
        Ok(())
    }

    // 13.3 Left-Hand-Side Expressions

    // CallExpression[Yield, Await] : CoverCallExpressionAndAsyncArrowHead[?Yield, ?Await]
    fn process_call_expression(&mut self) -> Result<(), Error> {
        self.enqueue(Node::CallExpression);
        self.replace(1, Detail::Expression);
        Ok(())
    }

    // Arguments[Yield, Await] : ( )
    fn process_arguments_empty(&mut self) -> Result<(), Error> {
        self.enqueue(Node::ArgumentListHead(true, false));
        self.enqueue(Node::Arguments);
        self.replace(2, Detail::Arguments);
        Ok(())
    }

    // Arguments[Yield, Await] : ( ArgumentList[?Yield, ?Await] )
    fn process_arguments(&mut self) -> Result<(), Error> {
        self.enqueue(Node::Arguments);
        self.replace(3, Detail::Arguments);
        Ok(())
    }

    // Arguments[Yield, Await] : ( ArgumentList[?Yield, ?Await] , )
    fn process_arguments_with_comma(&mut self) -> Result<(), Error> {
        self.enqueue(Node::Arguments);
        self.replace(4, Detail::Arguments);
        Ok(())
    }

    // ArgumentList[Yield, Await] : AssignmentExpression[+In, ?Yield, ?Await]
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

    // 14 ECMAScript Language: Statements and Declarations

    fn process_statement(&mut self) -> Result<(), Error> {
        // TODO
        Ok(())
    }

    fn process_declaration(&mut self) -> Result<(), Error> {
        self.replace(1, Detail::Declaration);
        Ok(())
    }

    fn process_hoistable_declaration(&mut self) -> Result<(), Error> {
        // TODO
        Ok(())
    }

    // 14.2 Block

    // BlockStatement[Yield, Await, Return] : Block[?Yield, ?Await, ?Return]
    fn process_block_statement(&mut self) -> Result<(), Error> {
        self.enqueue(Node::BlockStatement);
        self.replace(1, Detail::Statement);
        Ok(())
    }

    // Block[Yield, Await, Return] : { }
    fn process_empty_block(&mut self) -> Result<(), Error> {
        self.replace(2, Detail::Block);
        Ok(())
    }

    // Block[Yield, Await, Return] : { StatementList[?Yield, ?Await, ?Return] }
    fn process_block(&mut self) -> Result<(), Error> {
        self.enqueue(Node::EndBlockScope);
        self.replace(3, Detail::Block);
        Ok(())
    }

    // StatementList[Yield, Await, Return] : StatementListItem[?Yield, ?Await, ?Return]
    fn process_statement_list_head(&mut self) -> Result<(), Error> {
        self.top_mut().detail = Detail::StatementList;
        Ok(())
    }

    // StatementList[Yield, Await, Return] :
    //   StatementList[?Yield, ?Await, ?Return] StatementListItem[?Yield, ?Await, ?Return]
    fn process_statement_list_item(&mut self) -> Result<(), Error> {
        self.pop();
        self.top_mut().nodes_range.end = self.nodes.len();
        self.top_mut().tokens_range.end = self.tokens.len();
        Ok(())
    }

    // 14.3.1 Let and Const Declarations

    // LexicalDeclaration[In, Yield, Await] : let BindingList[?In, ?Yield, ?Await] ;
    fn process_let_declaration(&mut self) -> Result<(), Error> {
        let index = self.stack.len() - 2;
        let bound_names = match self.stack[index].detail {
            Detail::BindingList(ref mut list) => std::mem::take(&mut list.bound_names),
            _ => unreachable!(),
        };
        self.enqueue(Node::LetDeclaration(bound_names.len() as u32));
        self.replace(3, Detail::LetDeclaration(bound_names));
        Ok(())
    }

    // LexicalDeclaration[In, Yield, Await] : const BindingList[?In, ?Yield, ?Await] ;
    fn process_const_declaration(&mut self) -> Result<(), Error> {
        let index = self.stack.len() - 2;
        let (bound_names, has_initializer) = match self.stack[index].detail {
            Detail::BindingList(ref mut list) => {
                let bound_names = std::mem::take(&mut list.bound_names);
                (bound_names, list.has_initializer)
            }
            _ => unreachable!(),
        };
        // 14.3.1.1 Static Semantics: Early Errors
        if !has_initializer {
            return Err(Error::SyntaxError);
        }
        self.enqueue(Node::ConstDeclaration(bound_names.len() as u32));
        self.replace(3, Detail::ConstDeclaration(bound_names));
        Ok(())
    }

    // BindingList[In, Yield, Await] : LexicalBinding[?In, ?Yield, ?Await]
    fn process_binding_list_head(&mut self) -> Result<(), Error> {
        let mut syntax = self.pop();
        syntax.detail = Detail::BindingList(match syntax.detail {
            Detail::LexicalBinding(decl) => decl,
            _ => unreachable!(),
        });
        self.push(syntax);
        Ok(())
    }

    // BindingList[In, Yield, Await] :
    //   BindingList[?In, ?Yield, ?Await] COMMA LexicalBinding[?In, ?Yield, ?Await]
    fn process_binding_list_item(&mut self) -> Result<(), Error> {
        let decl = match self.pop().detail {
            Detail::LexicalBinding(decl) => decl,
            _ => unreachable!(),
        };
        self.pop(); // Token(,)
        match self.top_mut().detail {
            Detail::BindingList(ref mut list) => {
                for name in decl.bound_names.into_iter() {
                    // 14.3.1.1 Static Semantics: Early Errors
                    if list.bound_names.contains(&name) {
                        return Err(Error::SyntaxError);
                    }
                    list.bound_names.push(name);
                }
                if !decl.has_initializer {
                    list.has_initializer = false;
                }
                Ok(())
            }
            _ => unreachable!(),
        }
    }

    // LexicalBinding[In, Yield, Await] : BindingIdentifier[?Yield, ?Await]
    fn process_lexical_binding_identifier(&mut self) -> Result<(), Error> {
        let symbol = match self.top().detail {
            Detail::BindingIdentifier(symbol) => symbol,
            _ => unreachable!(),
        };

        // 14.3.1.1 Static Semantics: Early Errors
        if symbol == SymbolRegistry::LET {
            return Err(Error::SyntaxError);
        }

        const HAS_INITIALIZER: bool = false;
        self.enqueue(Node::LexicalBinding(HAS_INITIALIZER));
        self.replace(
            1,
            Detail::LexicalBinding(LexicalDeclarationSemantics {
                bound_names: smallvec![symbol],
                has_initializer: HAS_INITIALIZER,
            }),
        );

        Ok(())
    }

    // LexicalBinding[In, Yield, Await] :
    //   BindingIdentifier[?Yield, ?Await] Initializer[?In, ?Yield, ?Await]
    fn process_lexical_binding_identifier_with_initializer(&mut self) -> Result<(), Error> {
        let symbol = match self.stack[self.stack.len() - 2].detail {
            Detail::BindingIdentifier(symbol) => symbol,
            _ => unreachable!(),
        };

        // 14.3.1.1 Static Semantics: Early Errors
        if symbol == SymbolRegistry::LET {
            return Err(Error::SyntaxError);
        }

        const HAS_INITIALIZER: bool = true;
        self.enqueue(Node::LexicalBinding(HAS_INITIALIZER));
        self.replace(
            2,
            Detail::LexicalBinding(LexicalDeclarationSemantics {
                bound_names: smallvec![symbol],
                has_initializer: HAS_INITIALIZER,
            }),
        );

        Ok(())
    }

    // 14.3.3 Destructuring Binding Patterns

    // BindingElement[Yield, Await] : SingleNameBinding[?Yield, ?Await]
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

    // SingleNameBinding[Yield, Await] : BindingIdentifier[?Yield, ?Await]
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

    // EmptyStatement : ;
    fn process_empty_statement(&mut self) -> Result<(), Error> {
        //self.check_token(TokenKind::Semicolon);
        let node_index = self.enqueue(Node::EmptyStatement);
        let syntax = self.top_mut();
        syntax.detail = Detail::Statement;
        syntax.nodes_range = node_index..(node_index + 1);
        Ok(())
    }

    // 14.5 Expression Statement

    // ExpressionStatement[Yield, Await] :
    //   [lookahead ∉ { {, function, async [no LineTerminator here] function, class, let [ }]
    //   Expression[+In, ?Yield, ?Await] ;
    fn process_expression_statement(&mut self) -> Result<(), Error> {
        self.enqueue(Node::ExpressionStatement);
        self.replace(2, Detail::Statement);
        Ok(())
    }

    // 14.6 The if Statement

    // IfStatement[Yield, Await, Return] :
    //   if ( Expression[+In, ?Yield, ?Await] ) Statement[?Yield, ?Await, ?Return]
    //   else Statement[?Yield, ?Await, ?Return]
    fn process_if_else_statement(&mut self) -> Result<(), Error> {
        // TODO: 14.6.1 Static Semantics: Early Errors
        self.enqueue(Node::IfElseStatement);
        self.replace(7, Detail::Statement);
        Ok(())
    }

    // IfStatement[Yield, Await, Return] :
    //   if ( Expression[+In, ?Yield, ?Await] ) Statement[?Yield, ?Await, ?Return]
    //   [lookahead ≠ else]
    fn process_if_statement(&mut self) -> Result<(), Error> {
        // TODO: 14.6.1 Static Semantics: Early Errors
        self.enqueue(Node::IfStatement);
        self.replace(5, Detail::Statement);
        Ok(())
    }

    // 14.10 The return Statement

    // ReturnStatement[Yield, Await] : return ;
    fn process_return_statement(&mut self) -> Result<(), Error> {
        self.enqueue(Node::ReturnStatement(0));
        self.replace(2, Detail::Statement);
        Ok(())
    }

    // ReturnStatement[Yield, Await] : return [no LineTerminator here] Expression[+In, ?Yield, ?Await] ;
    fn process_return_value_statement(&mut self) -> Result<(), Error> {
        self.enqueue(Node::ReturnStatement(1));
        self.replace(3, Detail::Statement);
        Ok(())
    }

    // 15.9 Async Arrow Function Definitions

    // CoverCallExpressionAndAsyncArrowHead[Yield, Await] :
    //   MemberExpression[?Yield, ?Await] Arguments[?Yield, ?Await]
    fn process_cover_call_expression_and_async_arrow_head(&mut self) -> Result<(), Error> {
        self.replace(2, Detail::CoverCallExpressionAndAsyncArrowHead);
        Ok(())
    }

    // 15.1 Parameter Lists

    // FormalParameters[Yield, Await] : [empty]
    fn process_formal_parameters_empty(&mut self) -> Result<(), Error> {
        let node_index = self.enqueue(Node::FormalParameters(0));
        let token_index = self.tokens.len();
        self.push(Syntax {
            detail: Detail::FormalParameters(smallvec![]),
            nodes_range: node_index..(node_index + 1),
            tokens_range: token_index..token_index,
        });
        Ok(())
    }

    // FormalParameters[Yield, Await] : FormalParameterList[?Yield, ?Await]
    fn process_formal_parameters_list(&mut self) -> Result<(), Error> {
        let n = match self.top().detail {
            Detail::FormalParameters(ref bound_names) => bound_names.len(),
            _ => unreachable!(),
        };
        self.enqueue(Node::FormalParameters(n as u32));
        Ok(())
    }

    // FormalParameters[Yield, Await] : FormalParameterList[?Yield, ?Await] ,
    fn process_formal_parameters_list_with_comma(&mut self) -> Result<(), Error> {
        self.pop();
        let n = match self.top().detail {
            Detail::FormalParameters(ref bound_names) => bound_names.len(),
            _ => unreachable!(),
        };
        self.enqueue(Node::FormalParameters(n as u32));
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
                    if dest.contains(&name) {
                        return Err(Error::SyntaxError);
                    }
                    dest.push(name);
                }
            }
            _ => unreachable!(),
        }
        Ok(())
    }

    // FormalParameter[Yield, Await] : BindingElement[?Yield, ?Await]
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

    // FunctionStatementList[Yield, Await] : [empty]
    fn process_function_statement_list_empty(&mut self) -> Result<(), Error> {
        let node_index = self.nodes.len();
        let token_index = self.tokens.len();
        // 15.2.1 Static Semantics: Early Errors
        self.push(Syntax {
            detail: Detail::StatementList,
            nodes_range: node_index..node_index,
            tokens_range: token_index..token_index,
        });
        Ok(())
    }

    // 16.1 Scripts

    fn process_empty_script(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn process_script(&mut self) -> Result<(), Error> {
        self.pop();
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
