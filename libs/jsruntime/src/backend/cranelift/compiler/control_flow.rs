// TODO: remove
#![allow(unused)]

use cranelift::codegen::ir::Block;
use rustc_hash::FxHashMap;

use base::macros::debug_assert_ne;
use jsparser::Symbol;

use super::FunctionControlSet;
use super::ScopeRef;
type SwitchIr = ();

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

    // The index of the top-most if-then-else flow on the stack.
    // It's used for building the flow chain from the top-most to the bottom-most.
    if_then_else_index: usize,

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
        entry_block: Block,
        body_block: Block,
        exit_block: Block,
        fcs: FunctionControlSet,
    ) {
        self.stack.push(ControlFlow::Function(FunctionFlow {
            entry_block,
            body_block,
            exit_block,
            fcs,
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
        dormant_block: Block,
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
        body_block: Block,
        cleanup_block: Block,
    ) {
        let outer_index = self.scope_index;
        self.scope_index = self.stack.len();
        self.stack.push(ControlFlow::Scope(ScopeFlow {
            scope_ref,
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
                ControlFlow::Scope(flow) => Some(flow),
                _ => None,
            })
            .unwrap()
    }

    pub fn push_if_then_else_flow(
        &mut self,
        then_block: Block,
        else_block: Block,
        merge_block: Block,
    ) {
        let outer_index = self.if_then_else_index;
        self.if_then_else_index = self.stack.len();
        self.stack.push(ControlFlow::IfThenElse(IfThenElseFlow {
            then_block,
            else_block,
            merge_block,
            outer_index,
        }));
    }

    pub fn pop_if_then_else_flow(&mut self) -> IfThenElseFlow {
        match self.stack.pop() {
            Some(ControlFlow::IfThenElse(flow)) => {
                self.if_then_else_index = flow.outer_index;
                flow
            }
            _ => unreachable!(),
        }
    }

    pub fn merge_block(&self) -> Block {
        self.stack
            .get(self.if_then_else_index)
            .map(|flow| match flow {
                ControlFlow::IfThenElse(flow) => flow.merge_block,
                _ => panic!(),
            })
            .unwrap()
    }

    pub fn update_then_block(&mut self, then_block: Block) -> Block {
        match self.stack.last_mut() {
            Some(ControlFlow::IfThenElse(flow)) => {
                flow.then_block = then_block;
                flow.else_block
            }
            _ => unreachable!(),
        }
    }

    pub fn push_loop_init_flow(&mut self, branch_block: Block, insert_point: Block) {
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
        then_block: Block,
        else_block: Block,
        insert_point: Block,
    ) {
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

    pub fn push_loop_next_flow(&mut self, branch_block: Block, insert_point: Block) {
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

    pub fn push_loop_body_flow(&mut self, branch_block: Block, insert_point: Block) {
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

    pub fn push_switch_flow(&mut self, end_block: Block) {
        let outer_index = self.switch_index;
        self.switch_index = self.stack.len();
        self.stack.push(ControlFlow::Switch(SwitchFlow {
            end_block,
            default_block: None,
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
                ControlFlow::Switch(flow) => Some(flow),
                _ => None,
            })
            .unwrap()
    }

    fn switch_flow_mut(&mut self) -> &mut SwitchFlow {
        self.stack
            .get_mut(self.switch_index)
            .and_then(|flow| match flow {
                ControlFlow::Switch(flow) => Some(flow),
                _ => None,
            })
            .unwrap()
    }

    pub fn push_case_flow(&mut self, next_case_block: Block, clause_start_block: Block) {
        self.stack.push(ControlFlow::Case(CaseFlow {
            next_case_block,
            clause_start_block,
            clause_end_block: None,
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
        clause_end_block: Block,
        clause_has_statement: bool,
    ) -> Block {
        match self.stack.last_mut() {
            Some(ControlFlow::Case(flow)) => {
                flow.clause_end_block = Some(clause_end_block);
                flow.clause_has_statement = clause_has_statement;
                flow.next_case_block
            }
            _ => unreachable!(),
        }
    }

    pub fn push_exception_flow(
        &mut self,
        try_block: Block,
        catch_block: Block,
        finally_block: Block,
        end_block: Block,
    ) {
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
                ControlFlow::Exception(flow) => Some(flow),
                _ => None,
            })
            .unwrap()
    }

    fn exception_flow_mut(&mut self) -> &mut ExceptionFlow {
        self.stack
            .get_mut(self.exception_index)
            .and_then(|flow| match flow {
                ControlFlow::Exception(flow) => Some(flow),
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

    pub fn set_default_case_block(&mut self, block: Block) {
        debug_assert!(self.switch_index > 0);
        self.switch_flow_mut().default_block = Some(block);
    }

    pub fn push_exit_target(&mut self, block: Block, breakable: bool) {
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

    pub fn pop_exit_target(&mut self) -> Block {
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

    pub fn exit_block(&self) -> Block {
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

    pub fn cleanup_block(&self) -> Block {
        if self.scope_index == 0 {
            self.function_flow().exit_block
        } else {
            self.scope_flow().cleanup_block
        }
    }

    pub fn exception_block(&self) -> Block {
        if self.exception_index > self.scope_index {
            let flow = self.exception_flow();
            match flow.state {
                ExceptionState::Try => flow.catch_block,
                ExceptionState::Catch => flow.finally_block,
                ExceptionState::Finally => unreachable!(),
            }
        } else {
            //self.cleanup_block()
            todo!()
        }
    }

    pub fn dump(&self) {
        eprintln!("### control-flow-stack");
        self.print_stack();
        eprintln!();

        eprintln!("### exit-stack");
        self.print_exit_stack();
        eprintln!();
    }

    fn print_stack(&self) {
        macro_rules! eprintln_block {
            ($block:ident, $flow:ident) => {
                eprintln!(concat!(" ", stringify!($block), "={:?}"), $flow.$block)
            };
            ($block:ident) => {
                eprintln!(concat!(" ", stringify!($block), "={:?}"), $block)
            };
        }

        for flow in self.stack.iter().rev() {
            match flow {
                ControlFlow::Function(flow) => {
                    eprintln!("function:");
                    eprintln_block!(entry_block, flow);
                    eprintln_block!(body_block, flow);
                    eprintln_block!(exit_block, flow);
                }
                ControlFlow::Coroutine(flow) => {
                    eprintln!("coroutine: state={}/{}", flow.next_state, flow.num_states);
                }
                ControlFlow::Scope(flow) => {
                    eprintln!("scope:");
                    eprintln_block!(body_block, flow);
                    eprintln_block!(cleanup_block, flow);
                }
                ControlFlow::IfThenElse(flow) => {
                    eprintln!("then-else:");
                    eprintln_block!(then_block, flow);
                    eprintln_block!(else_block, flow);
                    eprintln_block!(merge_block, flow);
                }
                ControlFlow::LoopInit(flow) => {
                    eprintln!("loop-init:");
                    eprintln_block!(branch_block, flow);
                    eprintln_block!(insert_point, flow);
                }
                ControlFlow::LoopTest(flow) => {
                    eprintln!("loop-test:");
                    eprintln_block!(then_block, flow);
                    eprintln_block!(else_block, flow);
                    eprintln_block!(insert_point, flow);
                }
                ControlFlow::LoopNext(flow) => {
                    eprintln!("loop-next:");
                    eprintln_block!(branch_block, flow);
                    eprintln_block!(insert_point, flow);
                }
                ControlFlow::LoopBody(flow) => {
                    eprintln!("loop-body:");
                    eprintln_block!(branch_block, flow);
                    eprintln_block!(insert_point, flow);
                }
                ControlFlow::Switch(flow) => {
                    eprintln!("switch:");
                    if let Some(default_block) = flow.default_block {
                        eprintln_block!(default_block);
                    }
                    eprintln_block!(end_block, flow);
                }
                ControlFlow::Case(flow) => {
                    if flow.clause_has_statement {
                        eprintln!("case: has-statement");
                    } else {
                        eprintln!("case:");
                    }
                    eprintln_block!(next_case_block, flow);
                    eprintln_block!(clause_start_block, flow);
                    if let Some(clause_end_block) = flow.clause_end_block {
                        eprintln_block!(clause_end_block);
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
                    eprintln_block!(try_block, flow);
                    eprintln_block!(catch_block, flow);
                    eprintln_block!(finally_block, flow);
                    eprintln_block!(end_block, flow);
                }
            }
        }
    }

    fn print_exit_stack(&self) {
        for target in self.exit_stack.iter().rev() {
            eprintln!("block={:?} breakable={}", target.block, target.breakable);
        }
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
    pub entry_block: Block,
    pub body_block: Block,
    pub exit_block: Block,
    pub fcs: FunctionControlSet,
}

pub struct CoroutineFlow {
    switch_inst: SwitchIr,
    pub dormant_block: Block,
    num_states: u32,
    next_state: u32,
}

/// Contains data used for building a region representing a lexical scope.
pub struct ScopeFlow {
    /// The reference to the scope in the scope tree.
    pub scope_ref: ScopeRef,

    /// The first basic block of the container region of the scope flow.
    pub body_block: Block,

    pub cleanup_block: Block,

    /// The index of the enclosing outer scope flow.
    outer_index: usize,
}

pub struct IfThenElseFlow {
    pub then_block: Block,
    pub else_block: Block,
    pub merge_block: Block,

    /// The index of the enclosing outer if-then-else flow.
    outer_index: usize,
}

pub struct LoopInitFlow {
    pub branch_block: Block,
    pub insert_point: Block,
}

pub struct LoopTestFlow {
    pub then_block: Block,
    pub else_block: Block,
    pub insert_point: Block,
}

pub struct LoopNextFlow {
    pub branch_block: Block,
    pub insert_point: Block,
}

pub struct LoopBodyFlow {
    pub branch_block: Block,
    pub insert_point: Block,
}

pub struct SwitchFlow {
    pub end_block: Block,
    pub default_block: Option<Block>,
    outer_index: usize,
}

pub struct CaseFlow {
    pub next_case_block: Block,
    pub clause_start_block: Block,
    pub clause_end_block: Option<Block>,
    pub clause_has_statement: bool,
}

pub struct ExceptionFlow {
    #[allow(unused)]
    pub try_block: Block,
    pub catch_block: Block,
    pub finally_block: Block,
    pub end_block: Block,

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
    block: Block,
    breakable: bool,
}
