use std::cmp::Ordering;

use base::macros::debug_assert_ne;
use jsparser::Symbol;

use super::BasicBlock;
use super::Dump;

macro_rules! bb2cstr {
    ($bb:expr, $buf:expr, $len:expr) => {
        $bb.get_name_or_as_operand($buf, $len)
    };
}

#[derive(Default)]
pub struct ControlFlowStack {
    stack: Vec<ControlFlow>,
    break_stack: Vec<BranchTarget>,
    continue_stack: Vec<BranchTarget>,

    // The index of the top-most scope flow on the stack.
    // It's used for building the flow chain from the top-most to the bottom-most.
    scope_index: usize,

    // The index of the top-most switch flow on the stack.
    // It's used for building the flow chain from the top-most to the bottom-most.
    switch_index: usize,

    // The index of the top-most exception flow on the stack.
    // It's used for building the flow chain from the top-most to the bottom-most.
    exception_index: usize,
}

impl ControlFlowStack {
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty() && self.break_stack.is_empty() && self.continue_stack.is_empty()
    }

    pub fn in_finally_block(&self) -> bool {
        if self.exception_index == 0 {
            return false;
        }
        match self.stack.last() {
            Some(ControlFlow::Exception(flow)) => matches!(flow.state, ExceptionState::Finally),
            _ => false,
        }
    }

    pub fn push_function_flow(
        &mut self,
        locals_block: BasicBlock,
        args_block: BasicBlock,
        body_block: BasicBlock,
        return_block: BasicBlock,
    ) {
        debug_assert_ne!(locals_block, BasicBlock::NONE);
        debug_assert_ne!(args_block, BasicBlock::NONE);
        debug_assert_ne!(body_block, BasicBlock::NONE);
        debug_assert_ne!(return_block, BasicBlock::NONE);
        self.stack.push(ControlFlow::Function(FunctionFlow {
            locals_block,
            args_block,
            body_block,
            return_block,
        }));
    }

    pub fn pop_function_flow(&mut self) -> FunctionFlow {
        match self.stack.pop() {
            Some(ControlFlow::Function(flow)) => {
                debug_assert!(self.stack.is_empty());
                debug_assert_eq!(self.scope_index, 0);
                debug_assert_eq!(self.exception_index, 0);
                flow
            }
            _ => unreachable!(),
        }
    }

    pub fn function_flow(&self) -> &FunctionFlow {
        debug_assert!(!self.stack.is_empty());
        match self.stack[0] {
            ControlFlow::Function(ref flow) => flow,
            _ => unreachable!(),
        }
    }

    pub fn push_scope_flow(
        &mut self,
        init_block: BasicBlock,
        hoisted_block: BasicBlock,
        body_block: BasicBlock,
        cleanup_block: BasicBlock,
    ) {
        debug_assert_ne!(init_block, BasicBlock::NONE);
        debug_assert_ne!(hoisted_block, BasicBlock::NONE);
        debug_assert_ne!(body_block, BasicBlock::NONE);
        debug_assert_ne!(cleanup_block, BasicBlock::NONE);
        let outer_index = self.scope_index;
        self.scope_index = self.stack.len();
        self.stack.push(ControlFlow::Scope(ScopeFlow {
            init_block,
            hoisted_block,
            body_block,
            cleanup_block,
            outer_index,
            returned: false,
            thrown: false,
        }));
    }

    pub fn pop_scope_flow(&mut self) -> ScopeFlow {
        match self.stack.pop() {
            Some(ControlFlow::Scope(flow)) => {
                self.scope_index = flow.outer_index;
                if flow.returned {
                    self.propagate_returned();
                }
                if flow.thrown {
                    self.propagate_thrown();
                }
                flow
            }
            _ => unreachable!(),
        }
    }

    pub fn scope_flow(&self) -> &ScopeFlow {
        self.stack
            .get(self.scope_index)
            .and_then(|flow| match flow {
                ControlFlow::Scope(ref flow) => Some(flow),
                _ => None,
            })
            .unwrap()
    }

    fn scope_flow_mut(&mut self) -> &mut ScopeFlow {
        self.stack
            .get_mut(self.scope_index)
            .and_then(|flow| match flow {
                ControlFlow::Scope(ref mut flow) => Some(flow),
                _ => None,
            })
            .unwrap()
    }

    pub fn push_branch_flow(&mut self, before_block: BasicBlock, after_block: BasicBlock) {
        debug_assert_ne!(before_block, BasicBlock::NONE);
        debug_assert_ne!(after_block, BasicBlock::NONE);
        self.stack.push(ControlFlow::Branch(BranchFlow {
            before_block,
            after_block,
        }));
    }

    pub fn pop_branch_flow(&mut self) -> BranchFlow {
        match self.stack.pop() {
            Some(ControlFlow::Branch(flow)) => flow,
            _ => unreachable!(),
        }
    }

    pub fn push_loop_init_flow(&mut self, branch_block: BasicBlock, insert_point: BasicBlock) {
        debug_assert_ne!(branch_block, BasicBlock::NONE);
        debug_assert_ne!(insert_point, BasicBlock::NONE);
        self.stack.push(ControlFlow::LoopInit(LoopInitFlow {
            branch_block,
            insert_point,
        }));
    }

    pub fn pop_loop_init_flow(&mut self) -> LoopInitFlow {
        match self.stack.pop() {
            Some(ControlFlow::LoopInit(flow)) => flow,
            _ => unreachable!(),
        }
    }

    pub fn push_loop_test_flow(
        &mut self,
        then_block: BasicBlock,
        else_block: BasicBlock,
        insert_point: BasicBlock,
    ) {
        debug_assert_ne!(then_block, BasicBlock::NONE);
        debug_assert_ne!(else_block, BasicBlock::NONE);
        debug_assert_ne!(insert_point, BasicBlock::NONE);
        self.stack.push(ControlFlow::LoopTest(LoopTestFlow {
            then_block,
            else_block,
            insert_point,
        }));
    }

    pub fn pop_loop_test_flow(&mut self) -> LoopTestFlow {
        match self.stack.pop() {
            Some(ControlFlow::LoopTest(flow)) => flow,
            _ => unreachable!(),
        }
    }

    pub fn push_loop_next_flow(&mut self, branch_block: BasicBlock, insert_point: BasicBlock) {
        debug_assert_ne!(branch_block, BasicBlock::NONE);
        debug_assert_ne!(insert_point, BasicBlock::NONE);
        self.stack.push(ControlFlow::LoopNext(LoopNextFlow {
            branch_block,
            insert_point,
        }));
    }

    pub fn pop_loop_next_flow(&mut self) -> LoopNextFlow {
        match self.stack.pop() {
            Some(ControlFlow::LoopNext(flow)) => flow,
            _ => unreachable!(),
        }
    }

    pub fn push_loop_body_flow(&mut self, branch_block: BasicBlock, insert_point: BasicBlock) {
        debug_assert_ne!(branch_block, BasicBlock::NONE);
        debug_assert_ne!(insert_point, BasicBlock::NONE);
        self.stack.push(ControlFlow::LoopBody(LoopBodyFlow {
            branch_block,
            insert_point,
        }));
    }

    pub fn pop_loop_body_flow(&mut self) -> LoopBodyFlow {
        match self.stack.pop() {
            Some(ControlFlow::LoopBody(flow)) => flow,
            _ => unreachable!(),
        }
    }

    pub fn push_switch_flow(&mut self, end_block: BasicBlock) {
        debug_assert_ne!(end_block, BasicBlock::NONE);
        let outer_index = self.switch_index;
        self.switch_index = self.stack.len();
        self.stack.push(ControlFlow::Switch(SwitchFlow {
            end_block,
            default_block: BasicBlock::NONE,
            outer_index,
        }));
    }

    pub fn pop_switch_flow(&mut self) -> SwitchFlow {
        match self.stack.pop() {
            Some(ControlFlow::Switch(flow)) => {
                // The `select_index` must be updated just after stack.pop() so that other instance
                // methods such as `select_flow()` work properly.
                self.switch_index = flow.outer_index;
                flow
            }
            _ => unreachable!(),
        }
    }

    pub fn switch_flow(&self) -> &SwitchFlow {
        self.stack
            .get(self.switch_index)
            .and_then(|flow| match flow {
                ControlFlow::Switch(ref flow) => Some(flow),
                _ => None,
            })
            .unwrap()
    }

    fn switch_flow_mut(&mut self) -> &mut SwitchFlow {
        self.stack
            .get_mut(self.switch_index)
            .and_then(|flow| match flow {
                ControlFlow::Switch(ref mut flow) => Some(flow),
                _ => None,
            })
            .unwrap()
    }

    pub fn push_case_banch_flow(&mut self, before_block: BasicBlock, after_block: BasicBlock) {
        debug_assert_ne!(before_block, BasicBlock::NONE);
        debug_assert_ne!(after_block, BasicBlock::NONE);
        self.stack.push(ControlFlow::CaseBranch(CaseBranchFlow {
            before_block,
            after_block,
        }));
    }

    pub fn pop_case_branch_flow(&mut self) -> CaseBranchFlow {
        match self.stack.pop() {
            Some(ControlFlow::CaseBranch(flow)) => flow,
            _ => unreachable!(),
        }
    }

    pub fn push_exception_flow(
        &mut self,
        try_block: BasicBlock,
        catch_block: BasicBlock,
        finally_block: BasicBlock,
        end_block: BasicBlock,
    ) {
        debug_assert_ne!(try_block, BasicBlock::NONE);
        debug_assert_ne!(catch_block, BasicBlock::NONE);
        debug_assert_ne!(finally_block, BasicBlock::NONE);
        debug_assert_ne!(end_block, BasicBlock::NONE);
        let outer_index = self.exception_index;
        self.exception_index = self.stack.len();
        self.stack.push(ControlFlow::Exception(ExceptionFlow {
            try_block,
            catch_block,
            finally_block,
            end_block,
            outer_index,
            state: ExceptionState::Try,
            thrown: false,
        }));
    }

    pub fn pop_exception_flow(&mut self) -> ExceptionFlow {
        match self.stack.pop() {
            Some(ControlFlow::Exception(flow)) => {
                // The `exception_index` must be updated just after stack.pop() so that other
                // instance methods such as `exception_flow()` work properly.
                self.exception_index = flow.outer_index;

                // Any exception flow is enclosed by a scope flow.
                debug_assert!(matches!(self.stack.last(), Some(ControlFlow::Scope(_))));

                if flow.thrown {
                    self.propagate_thrown();
                }

                flow
            }
            _ => unreachable!(),
        }
    }

    pub fn exception_flow(&self) -> &ExceptionFlow {
        self.stack
            .get(self.exception_index)
            .and_then(|flow| match flow {
                ControlFlow::Exception(ref flow) => Some(flow),
                _ => None,
            })
            .unwrap()
    }

    fn exception_flow_mut(&mut self) -> &mut ExceptionFlow {
        self.stack
            .get_mut(self.exception_index)
            .and_then(|flow| match flow {
                ControlFlow::Exception(ref mut flow) => Some(flow),
                _ => None,
            })
            .unwrap()
    }

    pub fn set_returned(&mut self) {
        debug_assert!(self.scope_index > 0);
        self.scope_flow_mut().returned = true;
    }

    pub fn set_thrown(&mut self) {
        debug_assert!(self.scope_index > 0);
        debug_assert!(self.scope_index > self.exception_index);
        self.scope_flow_mut().thrown = true;
    }

    pub fn set_in_catch(&mut self, nominal: bool) {
        debug_assert!(matches!(self.stack.last(), Some(ControlFlow::Exception(_))));
        let flow = self.exception_flow_mut();
        flow.state = ExceptionState::Catch;
        if !nominal {
            flow.thrown = false;
        }
    }

    pub fn set_in_finally(&mut self) {
        debug_assert!(matches!(self.stack.last(), Some(ControlFlow::Exception(_))));
        self.exception_flow_mut().state = ExceptionState::Finally;
    }

    pub fn set_default_case_block(&mut self, block: BasicBlock) {
        debug_assert_ne!(block, BasicBlock::NONE);
        debug_assert!(self.switch_index > 0);
        self.switch_flow_mut().default_block = block;
    }

    pub fn push_break_target(&mut self, block: BasicBlock, symbol: Symbol) {
        debug_assert_ne!(block, BasicBlock::NONE);
        self.break_stack.push(BranchTarget { block, symbol });
    }

    pub fn pop_break_target(&mut self) -> BranchTarget {
        self.break_stack.pop().unwrap()
    }

    pub fn push_continue_target(&mut self, block: BasicBlock, symbol: Symbol) {
        self.continue_stack.push(BranchTarget { block, symbol });
    }

    pub fn pop_continue_target(&mut self) -> BranchTarget {
        self.continue_stack.pop().unwrap()
    }

    pub fn set_continue_target(&mut self, block: BasicBlock) {
        debug_assert_ne!(block, BasicBlock::NONE);
        for target in self.continue_stack.iter_mut().rev() {
            if target.symbol == Symbol::NONE {
                debug_assert_ne!(target.block, BasicBlock::NONE);
                return;
            }
            debug_assert_eq!(target.block, BasicBlock::NONE);
            target.block = block;
        }
    }

    pub fn clear(&mut self) {
        self.stack.clear();
        self.break_stack.clear();
        self.continue_stack.clear();
        self.scope_index = 0;
        self.switch_index = 0;
        self.exception_index = 0;
    }

    fn print_stack(&self, buf: *mut std::ffi::c_char, len: usize) {
        macro_rules! bb {
            ($flow:expr, $bb:ident) => {
                eprintln!(
                    concat!(" ", stringify!($bb), "={:?}"),
                    bb2cstr!($flow.$bb, buf, len)
                );
            };
        }

        for flow in self.stack.iter().rev() {
            match flow {
                ControlFlow::Function(flow) => {
                    eprintln!("function:");
                    bb!(flow, locals_block);
                    bb!(flow, args_block);
                    bb!(flow, body_block);
                    bb!(flow, return_block);
                }
                ControlFlow::Scope(flow) => {
                    eprint!("scope:");
                    if flow.returned {
                        eprint!(" returned");
                    }
                    if flow.thrown {
                        eprint!(" thrown");
                    }
                    eprintln!();
                    bb!(flow, init_block);
                    bb!(flow, hoisted_block);
                    bb!(flow, body_block);
                    bb!(flow, cleanup_block);
                }
                ControlFlow::Branch(flow) => {
                    eprintln!("branch:");
                    bb!(flow, before_block);
                    bb!(flow, after_block);
                }
                ControlFlow::LoopInit(flow) => {
                    eprintln!("loop-init:");
                    bb!(flow, branch_block);
                    bb!(flow, insert_point);
                }
                ControlFlow::LoopTest(flow) => {
                    eprintln!("loop-test:");
                    bb!(flow, then_block);
                    bb!(flow, else_block);
                    bb!(flow, insert_point);
                }
                ControlFlow::LoopNext(flow) => {
                    eprintln!("loop-next:");
                    bb!(flow, branch_block);
                    bb!(flow, insert_point);
                }
                ControlFlow::LoopBody(flow) => {
                    eprintln!("loop-body:");
                    bb!(flow, branch_block);
                    bb!(flow, insert_point);
                }
                ControlFlow::Switch(flow) => {
                    eprintln!("switch:");
                    if flow.default_block != BasicBlock::NONE {
                        bb!(flow, default_block);
                    }
                    bb!(flow, end_block);
                }
                ControlFlow::CaseBranch(flow) => {
                    eprintln!("case-branch:");
                    bb!(flow, before_block);
                    bb!(flow, after_block);
                }
                ControlFlow::Exception(flow) => {
                    eprint!("exception:");
                    if flow.thrown {
                        eprint!(" thrown");
                    }
                    match flow.state {
                        ExceptionState::Try => eprint!(" in-try"),
                        ExceptionState::Catch => eprint!(" in-catch"),
                        ExceptionState::Finally => eprint!(" in-finally"),
                    }
                    eprintln!();
                    bb!(flow, try_block);
                    bb!(flow, catch_block);
                    bb!(flow, finally_block);
                    bb!(flow, end_block);
                }
            }
        }
    }

    fn print_branch_target_stack(stack: &[BranchTarget], buf: *mut std::ffi::c_char, len: usize) {
        for target in stack.iter().rev() {
            eprintln!(
                "block={:?} symbol={}",
                bb2cstr!(target.block, buf, len),
                target.symbol
            );
        }
    }

    pub fn cleanup_block(&self) -> BasicBlock {
        if self.scope_index == 0 {
            self.function_flow().return_block
        } else {
            self.scope_flow().cleanup_block
        }
    }

    pub fn exception_block(&self) -> BasicBlock {
        if self.exception_index > self.scope_index {
            let flow = self.exception_flow();
            match flow.state {
                ExceptionState::Try => flow.catch_block,
                ExceptionState::Catch => flow.finally_block,
                ExceptionState::Finally => unreachable!(),
            }
        } else {
            self.cleanup_block()
        }
    }

    pub fn break_target(&self, symbol: Symbol) -> BasicBlock {
        Self::target_block(&self.break_stack, symbol)
    }

    pub fn continue_target(&self, symbol: Symbol) -> BasicBlock {
        Self::target_block(&self.continue_stack, symbol)
    }

    fn target_block(stack: &[BranchTarget], symbol: Symbol) -> BasicBlock {
        if symbol == Symbol::NONE {
            stack.last().unwrap().block
        } else {
            stack
                .iter()
                .rev()
                .find(|target| target.symbol == symbol)
                .map(|target| target.block)
                .unwrap()
        }
    }

    pub fn propagate_returned(&mut self) {
        if self.scope_index > 0 {
            self.set_returned()
        }
    }

    pub fn propagate_thrown(&mut self) {
        match self.scope_index.cmp(&self.exception_index) {
            // self.scope_index > self.exception_index
            Ordering::Greater => self.scope_flow_mut().thrown = true,
            // self.scope_index < self.exception_index
            Ordering::Less => self.exception_flow_mut().thrown = true,
            // self.scope_index == self.exception_index (== 0)
            _ => debug_assert_eq!(self.scope_index, 0),
        }
    }
}

impl Dump for ControlFlowStack {
    fn dump(&self, buf: *mut std::ffi::c_char, len: usize) {
        eprintln!("### control-flow-stack");
        self.print_stack(buf, len);
        eprintln!();

        eprintln!("### break-stack");
        Self::print_branch_target_stack(&self.break_stack, buf, len);
        eprintln!();

        eprintln!("### continue-stack");
        Self::print_branch_target_stack(&self.continue_stack, buf, len);
        eprintln!();
    }
}

enum ControlFlow {
    Function(FunctionFlow),
    Scope(ScopeFlow),
    Branch(BranchFlow),
    LoopInit(LoopInitFlow),
    LoopTest(LoopTestFlow),
    LoopNext(LoopNextFlow),
    LoopBody(LoopBodyFlow),
    Switch(SwitchFlow),
    CaseBranch(CaseBranchFlow),
    Exception(ExceptionFlow),
}

/// Contains data used for building the root region of a function.
pub struct FunctionFlow {
    #[allow(unused)]
    pub locals_block: BasicBlock,
    #[allow(unused)]
    pub args_block: BasicBlock,
    #[allow(unused)]
    pub body_block: BasicBlock,
    pub return_block: BasicBlock,
}

/// Contains data used for building a region representing a lexical scope.
pub struct ScopeFlow {
    /// The entry block of the scope flow.
    pub init_block: BasicBlock,

    /// A basic block containing instructions for hoisted function and variable declarations.
    pub hoisted_block: BasicBlock,

    /// The first basic block of the container region of the scope flow.
    pub body_block: BasicBlock,

    /// A basic block containing instructions for cleanup.
    /// The container region will be always connected to this basic block.
    pub cleanup_block: BasicBlock,

    /// The index of the enclosing outer scope flow.
    outer_index: usize,

    /// `true` if the scope flow contains return statements.
    pub returned: bool,

    /// `true` if the scope flow has uncaught exceptions.
    pub thrown: bool,
}

pub struct BranchFlow {
    pub before_block: BasicBlock,
    pub after_block: BasicBlock,
}

pub struct LoopInitFlow {
    pub branch_block: BasicBlock,
    pub insert_point: BasicBlock,
}

pub struct LoopTestFlow {
    pub then_block: BasicBlock,
    pub else_block: BasicBlock,
    pub insert_point: BasicBlock,
}

pub struct LoopNextFlow {
    pub branch_block: BasicBlock,
    pub insert_point: BasicBlock,
}

pub struct LoopBodyFlow {
    pub branch_block: BasicBlock,
    pub insert_point: BasicBlock,
}

pub struct SwitchFlow {
    pub end_block: BasicBlock,
    pub default_block: BasicBlock,
    outer_index: usize,
}

pub struct CaseBranchFlow {
    /// The last basic block in the statement lists of a case/default clause before the branch.
    ///
    /// This will be connected to `after_block` if it's not terminated and there is a subsequent
    /// case/default clause in the current `SelectFlow`.  If it's not terminated and there is no
    /// subsequent case/default clause, it will be connected to `SelectFlow::end_block`.
    pub before_block: BasicBlock,

    /// The first basic block in the statement lists of a case/default clause after the branch.
    pub after_block: BasicBlock,
}

pub struct ExceptionFlow {
    #[allow(unused)]
    pub try_block: BasicBlock,
    pub catch_block: BasicBlock,
    pub finally_block: BasicBlock,
    pub end_block: BasicBlock,

    // The index of the enclosing outer exception flow.
    outer_index: usize,

    state: ExceptionState,

    // `true` if the scope flow has uncaught exceptions.
    thrown: bool,
}

enum ExceptionState {
    Try,
    Catch,
    Finally,
}

pub struct BranchTarget {
    pub block: BasicBlock,
    pub symbol: Symbol,
}
