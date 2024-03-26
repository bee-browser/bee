use jsparser::BinaryOperator;
use jsparser::Error;
use jsparser::Node;
use jsparser::NodeHandler;
use jsparser::Parser;
use jsparser::Processor;
use jsparser::SymbolTable;

use super::bridge;
use super::logger;
use super::Module;
use super::Runtime;

impl Runtime {
    pub fn compile_script(&mut self, source: &str) -> Option<Module> {
        Parser::for_script(source, Processor::new(Compiler::new(self), false))
            .parse()
            .ok()
    }
}

struct Compiler<'r> {
    runtime: &'r mut Runtime,
    peer: *mut bridge::Compiler,
    scope_stack: Vec<ScopeState>,
}

#[derive(Default)]
struct ScopeState {
    returned: bool,
}

impl<'r> Compiler<'r> {
    pub fn new(runtime: &'r mut Runtime) -> Self {
        let peer = unsafe { bridge::compiler_peer_new() };
        Self {
            runtime,
            peer,
            scope_stack: vec![],
        }
    }
}

impl<'r> Drop for Compiler<'r> {
    fn drop(&mut self) {
        unsafe {
            bridge::compiler_peer_delete(self.peer);
        }
    }
}

impl<'r, 's> NodeHandler<'s> for Compiler<'r> {
    type Artifact = Module;

    fn start(&mut self) {
        logger::debug!(event = "start");
        unsafe {
            bridge::compiler_peer_start(self.peer);
        }
    }

    fn accept(&mut self) -> Result<Self::Artifact, Error> {
        logger::debug!(event = "accept");
        let peer = unsafe { bridge::compiler_peer_end(self.peer) };
        Ok(Module { peer })
    }

    fn handle_nodes(&mut self, nodes: impl Iterator<Item = Node<'s>>) -> Result<(), Error> {
        for node in nodes {
            logger::debug!(event = "handle_nodes", ?node);
            match node {
                Node::Null => {
                    // TODO
                }
                Node::Boolean(_value) => {
                    // TODO
                }
                Node::Number(value, ..) => unsafe {
                    bridge::compiler_peer_number(self.peer, value);
                },
                Node::String(_value, ..) => {
                    // TODO
                }
                Node::IdentifierReference(symbol) => unsafe {
                    bridge::compiler_peer_symbol(self.peer, symbol.id());
                },
                Node::BindingIdentifier(symbol) => unsafe {
                    bridge::compiler_peer_symbol(self.peer, symbol.id());
                },
                Node::LabelIdentifier(_symbol) => {
                    // TODO
                }
                Node::ArgumentListHead(empty, _spread) => unsafe {
                    bridge::compiler_peer_push_args(self.peer);
                    if !empty {
                        bridge::compiler_peer_push_arg(self.peer);
                    }
                },
                Node::ArgumentListItem(_spread) => unsafe {
                    bridge::compiler_peer_push_arg(self.peer);
                },
                Node::Arguments => {
                    // TODO
                }
                Node::CallExpression => unsafe {
                    bridge::compiler_peer_call(self.peer);
                },
                Node::UpdateExpression(_op) => {
                    // TODO
                }
                Node::UnaryExpression(_op) => {
                    // TODO
                }
                Node::BinaryExpression(BinaryOperator::Equality) => unsafe {
                    bridge::compiler_peer_eq(self.peer);
                },
                Node::BinaryExpression(BinaryOperator::Inequality) => unsafe {
                    bridge::compiler_peer_ne(self.peer);
                },
                Node::BinaryExpression(BinaryOperator::StrictEquality) => {
                    // TODO: check type
                    unsafe {
                        bridge::compiler_peer_eq(self.peer);
                    }
                }
                Node::BinaryExpression(BinaryOperator::StrictInequality) => {
                    // TODO: check type
                    unsafe {
                        bridge::compiler_peer_ne(self.peer);
                    }
                }
                Node::BinaryExpression(BinaryOperator::LessThan) => unsafe {
                    bridge::compiler_peer_lt(self.peer);
                },
                Node::BinaryExpression(BinaryOperator::LessThanOrEqual) => unsafe {
                    bridge::compiler_peer_lte(self.peer);
                },
                Node::BinaryExpression(BinaryOperator::GreaterThan) => unsafe {
                    bridge::compiler_peer_gt(self.peer);
                },
                Node::BinaryExpression(BinaryOperator::GreaterThanOrEqual) => unsafe {
                    bridge::compiler_peer_gte(self.peer);
                },
                Node::BinaryExpression(BinaryOperator::LeftShift) => {
                    // TODO
                }
                Node::BinaryExpression(BinaryOperator::RightShift) => {
                    // TODO
                }
                Node::BinaryExpression(BinaryOperator::UnsignedRightShift) => {
                    // TODO
                }
                Node::BinaryExpression(BinaryOperator::Addition) => unsafe {
                    bridge::compiler_peer_add(self.peer);
                },
                Node::BinaryExpression(BinaryOperator::Subtraction) => unsafe {
                    bridge::compiler_peer_sub(self.peer);
                },
                Node::BinaryExpression(BinaryOperator::Multiplication) => unsafe {
                    bridge::compiler_peer_mul(self.peer);
                },
                Node::BinaryExpression(BinaryOperator::Division) => unsafe {
                    bridge::compiler_peer_div(self.peer);
                },
                Node::BinaryExpression(BinaryOperator::Remainder) => unsafe {
                    bridge::compiler_peer_rem(self.peer);
                },
                Node::BinaryExpression(BinaryOperator::BitwiseOr) => {
                    // TODO
                }
                Node::BinaryExpression(BinaryOperator::BitwiseXor) => {
                    // TODO
                }
                Node::BinaryExpression(BinaryOperator::BitwiseAnd) => {
                    // TODO
                }
                Node::BinaryExpression(BinaryOperator::In) => {
                    // TODO
                }
                Node::BinaryExpression(BinaryOperator::Instanceof) => {
                    // TODO
                }
                Node::BinaryExpression(BinaryOperator::Exponentiation) => {
                    // TODO
                }
                Node::LogicalExpression(_op) => {
                    // TODO
                }
                Node::ConditionalExpression => unsafe {
                    bridge::compiler_peer_conditional_expression(self.peer);
                },
                Node::AssignmentExpression(_op) => unsafe {
                    bridge::compiler_peer_set(self.peer);
                },
                Node::BlockStatement => {
                    // TODO
                }
                Node::LexicalBinding => unsafe {
                    bridge::compiler_peer_declare_undefined(self.peer);
                },
                Node::LexicalBindingWithInitializer => unsafe {
                    bridge::compiler_peer_declare_variable(self.peer);
                },
                Node::LexicalBindingForConst => unsafe {
                    bridge::compiler_peer_declare_const(self.peer);
                },
                Node::LetDeclaration(_n) => {
                    // TODO
                }
                Node::ConstDeclaration(_n) => {
                    // TODO
                }
                Node::BindingElement(_has_initializer) => {
                    // TODO
                }
                Node::EmptyStatement => {
                    // TODO
                }
                Node::ExpressionStatement => unsafe {
                    bridge::compiler_peer_void(self.peer);
                },
                Node::IfElseStatement => unsafe {
                    bridge::compiler_peer_if_else_statement(self.peer);
                },
                Node::IfStatement => unsafe {
                    bridge::compiler_peer_if_statement(self.peer);
                },
                Node::ReturnStatement(n) => {
                    self.scope_stack.last_mut().unwrap().returned = true;
                    unsafe {
                        bridge::compiler_peer_return(self.peer, n as usize);
                    }
                }
                Node::FormalParameter => {
                    // TODO
                }
                Node::FormalParameters(_n) => {
                    // TODO
                }
                Node::FunctionDeclaration => {
                    self.scope_stack.pop();
                    unsafe {
                        bridge::compiler_peer_end_function(self.peer);
                    }
                }
                Node::ThenBlock => unsafe {
                    bridge::compiler_peer_to_boolean(self.peer);
                    bridge::compiler_peer_block(self.peer);
                },
                Node::ElseBlock => unsafe {
                    bridge::compiler_peer_block(self.peer);
                },
                Node::StartScope => {
                    self.scope_stack.push(Default::default());
                    unsafe {
                        bridge::compiler_peer_start_scope(self.peer);
                    }
                }
                Node::EndScope => {
                    if self.scope_stack.last().unwrap().returned {
                        // The scope will be removed from the stack in `llvmir::call()`.
                    } else {
                        unsafe {
                            bridge::compiler_peer_end_scope(self.peer);
                        }
                    }
                }
                Node::FunctionSignature(symbol, formal_parameters) => {
                    let (func_id, func_name) =
                        self.runtime.create_native_function(formal_parameters);
                    let name = func_name.as_ptr();
                    unsafe {
                        bridge::compiler_peer_declare_function(self.peer, symbol.id(), func_id.0);
                        bridge::compiler_peer_start_function(self.peer, name);
                        // TODO: arguments
                    }
                    self.scope_stack.push(Default::default());
                }
            }
            // unsafe {
            //     bridge::compiler_peer_dump_stack(self.peer);
            // }
        }
        Ok(())
    }

    fn symbol_table_mut(&mut self) -> &mut SymbolTable {
        &mut self.runtime.symbol_table
    }
}
