use rustc_hash::FxHashMap;

use base::macros::debug_assert_ne;
use jsparser::Symbol;

use super::BasicBlock;
use super::Dump;
use super::ScopeRef;
use super::SwitchIr;

macro_rules! bb2cstr {
    ($bb:expr, $buf:expr, $len:expr) => {
        $bb.get_name_or_as_operand($buf, $len)
    };
}

#[derive(Clone, Copy, Debug)]
pub struct ExitId(u8);

impl ExitId {
    pub fn depth(&self) -> u32 {
        // See the definition of the flow selector in compiler.hh
        (self.0 as u32) << 8
    }
}

#[derive(Default)]
pub struct ControlFlowStack {
    stack: Vec<ControlFlow>,

    // TODO: Currently, we use a separate stack for non-local exits.  However, we can probably
    // reuse the control flow stack for this purpose with a small modification.
    exit_stack: Vec<ExitTarget>,
    exit_label_map: FxHashMap<Symbol, usize>,

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
        self.stack.is_empty() && self.exit_stack.is_empty()
    }

    pub fn has_scope_flow(&self) -> bool {
        self.scope_index != 0
    }

    pub fn push_function_flow(
        &mut self,
        locals_block: BasicBlock,
        init_block: BasicBlock,
        args_block: BasicBlock,
        body_block: BasicBlock,
        return_block: BasicBlock,
    ) {
        debug_assert_ne!(locals_block, BasicBlock::NONE);
        debug_assert_ne!(init_block, BasicBlock::NONE);
        debug_assert_ne!(args_block, BasicBlock::NONE);
        debug_assert_ne!(body_block, BasicBlock::NONE);
        debug_assert_ne!(return_block, BasicBlock::NONE);
        self.stack.push(ControlFlow::Function(FunctionFlow {
            locals_block,
            init_block,
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

    pub fn push_coroutine_flow(
        &mut self,
        switch_inst: SwitchIr,
        dormant_block: BasicBlock,
        num_states: u32,
    ) {
        self.stack.push(ControlFlow::Coroutine(CoroutineFlow {
            switch_inst,
            dormant_block,
            num_states,
            next_state: 1,
        }));
    }

    pub fn pop_coroutine_flow(&mut self) -> CoroutineFlow {
        match self.stack.pop() {
            Some(ControlFlow::Coroutine(flow)) => {
                debug_assert_eq!(flow.next_state, flow.num_states - 1);
                flow
            }
            _ => unreachable!(),
        }
    }

    pub fn coroutine_switch_inst(&self) -> SwitchIr {
        debug_assert!(self.stack.len() >= 2);
        match self.stack[1] {
            ControlFlow::Coroutine(ref flow) => flow.switch_inst,
            _ => unreachable!(),
        }
    }

    pub fn coroutine_next_state(&mut self) -> u32 {
        debug_assert!(self.stack.len() >= 2);
        let flow = match self.stack[1] {
            ControlFlow::Coroutine(ref mut flow) => flow,
            _ => unreachable!(),
        };
        let next_state = flow.next_state;
        flow.next_state += 1;
        next_state
    }

    pub fn push_scope_flow(
        &mut self,
        scope_ref: ScopeRef,
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
            scope_ref,
            init_block,
            hoisted_block,
            body_block,
            cleanup_block,
            outer_index,
        }));
    }

    pub fn pop_scope_flow(&mut self) -> ScopeFlow {
        match self.stack.pop() {
            Some(ControlFlow::Scope(flow)) => {
                self.scope_index = flow.outer_index;
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

    pub fn push_if_then_else_flow(&mut self, then_block: BasicBlock, else_block: BasicBlock) {
        debug_assert_ne!(then_block, BasicBlock::NONE);
        debug_assert_ne!(else_block, BasicBlock::NONE);
        self.stack.push(ControlFlow::IfThenElse(IfThenElseFlow {
            then_block,
            else_block,
        }));
    }

    pub fn pop_if_then_else_flow(&mut self) -> IfThenElseFlow {
        match self.stack.pop() {
            Some(ControlFlow::IfThenElse(flow)) => flow,
            _ => unreachable!(),
        }
    }

    pub fn update_then_block(&mut self, then_block: BasicBlock) -> BasicBlock {
        debug_assert_ne!(then_block, BasicBlock::NONE);
        match self.stack.last_mut() {
            Some(ControlFlow::IfThenElse(flow)) => {
                flow.then_block = then_block;
                flow.else_block
            }
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

    pub fn push_case_flow(&mut self, next_case_block: BasicBlock, clause_start_block: BasicBlock) {
        debug_assert_ne!(next_case_block, BasicBlock::NONE);
        debug_assert_ne!(clause_start_block, BasicBlock::NONE);
        self.stack.push(ControlFlow::Case(CaseFlow {
            next_case_block,
            clause_start_block,
            clause_end_block: BasicBlock::NONE,
            clause_has_statement: false,
        }));
    }

    pub fn pop_case_flow(&mut self) -> CaseFlow {
        match self.stack.pop() {
            Some(ControlFlow::Case(flow)) => flow,
            _ => unreachable!(),
        }
    }

    pub fn update_case_flow(
        &mut self,
        clause_end_block: BasicBlock,
        clause_has_statement: bool,
    ) -> BasicBlock {
        debug_assert_ne!(clause_end_block, BasicBlock::NONE);
        match self.stack.last_mut() {
            Some(ControlFlow::Case(flow)) => {
                flow.clause_end_block = clause_end_block;
                flow.clause_has_statement = clause_has_statement;
                flow.next_case_block
            }
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

    pub fn set_in_catch(&mut self, _nominal: bool) {
        debug_assert!(matches!(self.stack.last(), Some(ControlFlow::Exception(_))));
        let flow = self.exception_flow_mut();
        flow.state = ExceptionState::Catch;
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

    pub fn push_exit_target(&mut self, block: BasicBlock, breakable: bool) {
        debug_assert_ne!(block, BasicBlock::NONE);
        // TODO: should treat as a compilation error.
        assert!(self.exit_stack.len() <= u8::MAX as usize);
        self.exit_stack.push(ExitTarget { block, breakable });
    }

    pub fn set_exit_label(&mut self, label: Symbol) {
        debug_assert_ne!(label, Symbol::NONE);
        debug_assert!(!self.exit_stack.is_empty());
        let index = self.exit_stack.len() - 1;
        self.exit_label_map.insert(label, index);
    }

    pub fn pop_exit_target(&mut self) -> BasicBlock {
        debug_assert!(!self.exit_stack.is_empty());
        let index = self.exit_stack.len() - 1;
        self.exit_label_map.retain(|_, v| *v != index); // TODO: inefficient...
        self.exit_stack.pop().unwrap().block
    }

    pub fn exit_id(&self) -> ExitId {
        debug_assert!(!self.exit_stack.is_empty());
        ExitId((self.exit_stack.len() - 1) as u8)
    }

    pub fn exit_id_for_label(&self, label: Symbol) -> ExitId {
        debug_assert!(!self.exit_stack.is_empty());
        if label == Symbol::NONE {
            self.exit_stack
                .iter()
                .enumerate()
                .rev()
                .find_map(|(index, target)| target.breakable.then_some(ExitId(index as u8)))
                .unwrap()
        } else {
            ExitId(*self.exit_label_map.get(&label).unwrap() as u8)
        }
    }

    pub fn exit_block(&self) -> BasicBlock {
        debug_assert!(!self.exit_stack.is_empty());
        self.exit_stack.last().unwrap().block
    }

    pub fn clear(&mut self) {
        self.stack.clear();
        self.exit_stack.clear();
        self.exit_label_map.clear();
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
                ControlFlow::Coroutine(flow) => {
                    eprintln!("coroutine: state={}/{}", flow.next_state, flow.num_states);
                }
                ControlFlow::Scope(flow) => {
                    eprint!("scope:");
                    eprintln!();
                    bb!(flow, init_block);
                    bb!(flow, hoisted_block);
                    bb!(flow, body_block);
                    bb!(flow, cleanup_block);
                }
                ControlFlow::IfThenElse(flow) => {
                    eprintln!("then-else:");
                    bb!(flow, then_block);
                    bb!(flow, else_block);
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
                ControlFlow::Case(flow) => {
                    if flow.clause_has_statement {
                        eprintln!("case: has-statement");
                    } else {
                        eprintln!("case:");
                    }
                    bb!(flow, next_case_block);
                    bb!(flow, clause_start_block);
                    if flow.clause_end_block != BasicBlock::NONE {
                        bb!(flow, clause_end_block);
                    }
                }
                ControlFlow::Exception(flow) => {
                    eprint!("exception:");
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

    fn print_exit_stack(&self, buf: *mut std::ffi::c_char, len: usize) {
        for target in self.exit_stack.iter().rev() {
            let block = bb2cstr!(target.block, buf, len);
            eprintln!("block={block:?} breakable={}", target.breakable);
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
}

impl Dump for ControlFlowStack {
    fn dump(&self, buf: *mut std::ffi::c_char, len: usize) {
        eprintln!("### control-flow-stack");
        self.print_stack(buf, len);
        eprintln!();

        eprintln!("### exit-stack");
        self.print_exit_stack(buf, len);
        eprintln!();
    }
}

enum ControlFlow {
    Function(FunctionFlow),
    Coroutine(CoroutineFlow),
    Scope(ScopeFlow),
    IfThenElse(IfThenElseFlow),
    LoopInit(LoopInitFlow),
    LoopTest(LoopTestFlow),
    LoopNext(LoopNextFlow),
    LoopBody(LoopBodyFlow),
    Switch(SwitchFlow),
    Case(CaseFlow),
    Exception(ExceptionFlow),
}

/// Contains data used for building the root region of a function.
pub struct FunctionFlow {
    #[allow(unused)]
    pub locals_block: BasicBlock,
    #[allow(unused)]
    pub init_block: BasicBlock,
    #[allow(unused)]
    pub args_block: BasicBlock,
    #[allow(unused)]
    pub body_block: BasicBlock,
    pub return_block: BasicBlock,
}

pub struct CoroutineFlow {
    switch_inst: SwitchIr,
    pub dormant_block: BasicBlock,
    num_states: u32,
    next_state: u32,
}

/// Contains data used for building a region representing a lexical scope.
pub struct ScopeFlow {
    /// The reference to the scope in the scope tree.
    pub scope_ref: ScopeRef,

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
}

pub struct IfThenElseFlow {
    pub then_block: BasicBlock,
    pub else_block: BasicBlock,
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

pub struct CaseFlow {
    pub next_case_block: BasicBlock,
    pub clause_start_block: BasicBlock,
    pub clause_end_block: BasicBlock,
    pub clause_has_statement: bool,
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
}

enum ExceptionState {
    Try,
    Catch,
    Finally,
}

struct ExitTarget {
    block: BasicBlock,
    breakable: bool,
}
