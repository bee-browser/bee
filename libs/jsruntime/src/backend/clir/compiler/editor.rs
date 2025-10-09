use std::ffi::CStr;

use cranelift::codegen::ir;
use cranelift::codegen::ir::InstBuilder as _;
use cranelift::codegen::isa;
use cranelift::frontend::FunctionBuilder;
use cranelift::frontend::Switch;

use base::static_assert_eq;

use crate::StringHandle;
use crate::lambda::LambdaKind;
use crate::logger;
use crate::objects::Object;
use crate::objects::ObjectFlags;
use crate::types::CallContext;
use crate::types::CallContextFlags;
use crate::types::Capture;
use crate::types::Closure;
use crate::types::Coroutine;
use crate::types::StringFragment;
use crate::types::StringFragmentKind;
use crate::types::Value;

use super::AnyIr;
use super::ArgvIr;
use super::BooleanIr;
use super::CaptureIr;
use super::ClosureIr;
use super::CoroutineIr;
use super::EditorSupport;
use super::FlowSelector;
use super::LambdaId;
use super::LambdaIr;
use super::NumberIr;
use super::ObjectIr;
use super::PromiseIr;
use super::RuntimeFunctionCache;
use super::ScopeRef;
use super::Status;
use super::StatusIr;
use super::StringIr;
use super::Symbol;

pub struct Editor<'a> {
    builder: FunctionBuilder<'a>,

    runtime_func_cache: RuntimeFunctionCache,

    addr_type: ir::Type,
    lambda_sig: ir::SigRef,
    entry_block: ir::Block,
    closure: ir::Value,
    call_context: ir::StackSlot,
    fcs: ir::StackSlot,

    // FunctionBuilder::is_filled() is a private method.
    block_terminated: bool,

    /// Enabled if the function is a coroutine.
    coroutine_mode: bool,

    /// Enable the runtime assertions.
    runtime_assert_enabled: bool,
}

impl<'a> Editor<'a> {
    pub fn new(
        runtime_assert_enabled: bool,
        mut builder: FunctionBuilder<'a>,
        target_config: isa::TargetFrontendConfig,
    ) -> Self {
        let lambda_sig = builder.import_signature(builder.func.signature.clone());

        let entry_block = builder.create_block();
        // As described in the following document, the incoming function arguments must be passed
        // to the entry block as block parameters:
        // //cranelift/docs/ir.md#static-single-assignment-form in bytecodealliance/wasmtime
        builder.append_block_params_for_function_params(entry_block);
        // Immediately call `seal_block()` because this block is the first block and there is no
        // predecessor of the entry block.
        builder.seal_block(entry_block);

        let call_context = builder.create_sized_stack_slot(ir::StackSlotData {
            kind: ir::StackSlotKind::ExplicitSlot,
            size: CallContext::SIZE as u32,
            align_shift: CallContext::ALIGNMENT.ilog2() as u8,
        });

        let fcs = builder.create_sized_stack_slot(ir::StackSlotData {
            kind: ir::StackSlotKind::ExplicitSlot,
            size: 10, // [status, flow_selector, scope_id]
            align_shift: 2,
        });

        Self {
            builder,
            runtime_func_cache: Default::default(),
            addr_type: target_config.pointer_type(),
            lambda_sig,
            entry_block,
            call_context,
            closure: ir::Value::from_u32(0), // dummy
            fcs,
            block_terminated: false,
            coroutine_mode: false,
            runtime_assert_enabled,
        }
    }

    pub fn put_declare_lazy_compile(
        &mut self,
        support: &mut impl EditorSupport,
        lambda_kind: LambdaKind,
    ) -> LambdaIr {
        logger::debug!(event = "put_declare_lazy_compile", ?lambda_kind);
        let func_ref = match lambda_kind {
            LambdaKind::Normal => self
                .runtime_func_cache
                .import_runtime_lazy_compile_normal(support, self.builder.func),
            LambdaKind::Ramp => self
                .runtime_func_cache
                .import_runtime_lazy_compile_ramp(support, self.builder.func),
            LambdaKind::Coroutine => self
                .runtime_func_cache
                .import_runtime_lazy_compile_coroutine(support, self.builder.func),
        };
        LambdaIr(self.builder.ins().func_addr(self.addr_type, func_ref))
    }

    pub fn end(mut self) {
        self.builder.seal_all_blocks();
        self.builder.finalize();
    }

    // assertions

    fn put_assert_null(
        &mut self,
        support: &mut impl EditorSupport,
        ptr: ir::Value,
        msg: &'static CStr,
    ) {
        use ir::condcodes::IntCC::Equal;
        let is_null = BooleanIr(self.builder.ins().icmp_imm(Equal, ptr, 0));
        self.put_assert(support, is_null, msg);
    }

    pub fn put_assert_non_null(
        &mut self,
        support: &mut impl EditorSupport,
        ptr: ir::Value,
        msg: &'static CStr,
    ) {
        use ir::condcodes::IntCC::NotEqual;
        let is_non_null = BooleanIr(self.builder.ins().icmp_imm(NotEqual, ptr, 0));
        self.put_assert(support, is_non_null, msg);
    }

    pub fn put_assert(
        &mut self,
        support: &mut impl EditorSupport,
        assertion: BooleanIr,
        msg: &'static CStr,
    ) {
        logger::debug!(event = "put_assert", ?assertion, ?msg);
        let panic_block = self.create_block();
        let merge_block = self.create_block();
        self.put_branch(assertion, merge_block, &[], panic_block, &[]);
        self.switch_to_block(panic_block);
        self.put_runtime_panic(support, msg);
        self.put_jump(merge_block, &[]);
        self.switch_to_block(merge_block);
    }

    // function parameters

    fn runtime(&self) -> ir::Value {
        self.lambda_params(0)
    }

    fn context(&self) -> ir::Value {
        self.lambda_params(1)
    }

    /// Returns the `this` argument of the lambda function.
    ///
    /// Don't be confused.  The value is **NOT** equal to the return value of
    /// `ResolveThisBinding()` defined in the ECMA-262 specification.
    pub fn this_argument(&mut self) -> AnyIr {
        const OFFSET: i64 = CallContext::THIS_OFFSET as i64;
        let context = self.context();
        AnyIr(self.builder.ins().iadd_imm(context, OFFSET))
    }

    fn envp(&mut self) -> ir::Value {
        let context = self.context();
        self.put_load_addr(context, CallContext::ENVP_OFFSET)
    }

    fn coroutine(&mut self) -> CoroutineIr {
        CoroutineIr(self.envp())
    }

    fn argc(&mut self) -> ir::Value {
        let context = self.context();
        self.put_load_i16(context, CallContext::ARGC_OFFSET)
    }

    pub fn argc_as_jump_table_index(&mut self) -> ir::Value {
        let argc = self.argc();
        self.builder.ins().uextend(ir::types::I32, argc)
    }

    fn argv(&mut self) -> ArgvIr {
        let context = self.context();
        ArgvIr(self.put_load_addr(context, CallContext::ARGV_OFFSET))
    }

    pub fn retv(&self) -> AnyIr {
        AnyIr(self.lambda_params(2))
    }

    pub fn exception(&self) -> AnyIr {
        AnyIr(self.lambda_params(2))
    }

    fn lambda_params(&self, index: usize) -> ir::Value {
        self.builder.block_params(self.entry_block)[index]
    }

    pub fn put_assert_lambda_params(&mut self, support: &mut impl EditorSupport, is_entry: bool) {
        if self.runtime_assert_enabled {
            self.put_assert_runtime_is_non_null(support);
            self.put_assert_context_is_non_null(support);
            self.put_assert_this_is_non_null(support);
            if is_entry {
                self.put_assert_envp_is_null(support);
            } else {
                self.put_assert_envp_is_non_null(support);
            }
            self.put_assert_argv_is_non_null(support);
            self.put_assert_retv_is_non_null(support);
        }
    }

    fn put_assert_runtime_is_non_null(&mut self, support: &mut impl EditorSupport) {
        let runtime = self.runtime();
        self.put_assert_non_null(support, runtime, c"runtime must be non-null");
    }

    fn put_assert_context_is_non_null(&mut self, support: &mut impl EditorSupport) {
        let context = self.context();
        self.put_assert_non_null(support, context, c"context must be non-null");
    }

    fn put_assert_this_is_non_null(&mut self, support: &mut impl EditorSupport) {
        let this = self.this_argument();
        self.put_assert_non_null(support, this.0, c"this must be non-null");
    }

    fn put_assert_envp_is_null(&mut self, support: &mut impl EditorSupport) {
        let envp = self.envp();
        self.put_assert_null(support, envp, c"context.envp must be null");
    }

    fn put_assert_envp_is_non_null(&mut self, support: &mut impl EditorSupport) {
        let envp = self.envp();
        self.put_assert_non_null(support, envp, c"context.envp must be non-null");
    }

    fn put_assert_argv_is_non_null(&mut self, support: &mut impl EditorSupport) {
        let argv = self.argv();
        self.put_assert_non_null(support, argv.0, c"argv must be non-null");
    }

    fn put_assert_retv_is_non_null(&mut self, support: &mut impl EditorSupport) {
        let retv = self.retv();
        self.put_assert_non_null(support, retv.0, c"retv must be non-null");
    }

    // arguments

    pub fn put_get_argument(&mut self, support: &mut impl EditorSupport, index: u16) -> AnyIr {
        logger::debug!(event = "put_get_argument", ?index);
        if self.runtime_assert_enabled {
            use ir::condcodes::IntCC::UnsignedLessThan;
            let index = self.builder.ins().iconst(ir::types::I16, index as i64);
            let argc = self.argc();
            let cond = BooleanIr(self.builder.ins().icmp(UnsignedLessThan, index, argc));
            self.put_assert(support, cond, c"put_get_argument: index out of bounds");
        }
        let argv = self.argv();
        let offset = Value::SIZE * index as usize;
        AnyIr(self.builder.ins().iadd_imm(argv.0, offset as i64))
    }

    // basic block

    pub fn entry_block(&self) -> ir::Block {
        self.entry_block
    }

    pub fn current_block(&self) -> ir::Block {
        self.builder.current_block().unwrap()
    }

    pub fn create_block(&mut self) -> ir::Block {
        logger::debug!(event = "create_block");
        self.builder.create_block()
    }

    pub fn create_block_with_i8(&mut self) -> ir::Block {
        logger::debug!(event = "create_block_with_i8");
        let block = self.builder.create_block();
        self.builder.append_block_param(block, ir::types::I8);
        block
    }

    pub fn create_block_with_addr(&mut self) -> ir::Block {
        logger::debug!(event = "create_block_with_addr");
        let block = self.builder.create_block();
        self.builder.append_block_param(block, self.addr_type);
        block
    }

    pub fn create_block_with_argv(&mut self, argc: u16) -> ir::Block {
        logger::debug!(event = "create_block_with_argv");
        let block = self.builder.create_block();
        for _ in 0..argc {
            self.builder.append_block_param(block, self.addr_type);
        }
        block
    }

    pub fn create_block_for_deadcode(&mut self) -> ir::Block {
        logger::debug!(event = "create_block_for_deadcode");
        let block = self.builder.create_block();
        self.builder.set_cold_block(block);
        block
    }

    pub fn switch_to_block(&mut self, block: ir::Block) {
        logger::debug!(event = "switch_to_block", ?block);
        self.builder.switch_to_block(block);
        self.block_terminated = false;
    }

    pub fn get_block_param(&self, block: ir::Block, index: usize) -> ir::Value {
        self.builder.block_params(block)[index]
    }

    // function control set | status

    fn put_load_status(&mut self) -> StatusIr {
        StatusIr(self.builder.ins().stack_load(ir::types::I32, self.fcs, 0))
    }

    pub fn put_store_status(&mut self, status: Status) {
        logger::debug!(event = "put_store_status", ?status);
        let status = self
            .builder
            .ins()
            .iconst(ir::types::I32, status.imm() as i64);
        self.builder.ins().stack_store(status, self.fcs, 0);
    }

    pub fn put_is_exception_status(&mut self, status: StatusIr) -> BooleanIr {
        logger::debug!(event = "put_is_exception_status", ?status);
        use ir::condcodes::IntCC::Equal;
        BooleanIr(
            self.builder
                .ins()
                .icmp_imm(Equal, status.0, Status::EXCEPTION.imm() as i64),
        )
    }

    // function control set | flow_selector

    pub fn put_store_flow_selector(&mut self, fs: FlowSelector) {
        logger::debug!(event = "put_store_flow_selector", ?fs);
        let value = self.builder.ins().iconst(ir::types::I32, fs.imm() as i64);
        self.builder.ins().stack_store(value, self.fcs, 4);
    }

    pub fn put_is_flow_selector_normal(&mut self) -> BooleanIr {
        logger::debug!(event = "put_is_flow_selector_normal");
        use ir::condcodes::IntCC::Equal;
        let fs = self.builder.ins().stack_load(ir::types::I32, self.fcs, 4);
        BooleanIr(
            self.builder
                .ins()
                .icmp_imm(Equal, fs, FlowSelector::NORMAL.imm() as i64),
        )
    }

    pub fn put_is_flow_selector_throw(&mut self) -> BooleanIr {
        logger::debug!(event = "put_is_flow_selector_normal");
        use ir::condcodes::IntCC::Equal;
        let fs = self.builder.ins().stack_load(ir::types::I32, self.fcs, 4);
        BooleanIr(
            self.builder
                .ins()
                .icmp_imm(Equal, fs, FlowSelector::THROW.imm() as i64),
        )
    }

    pub fn put_is_flow_selector_normal_or_continue(&mut self, depth: u32) -> BooleanIr {
        logger::debug!(event = "put_is_flow_selector_normal_or_continue");
        let fs = self.builder.ins().stack_load(ir::types::I32, self.fcs, 4);
        use ir::condcodes::IntCC::UnsignedGreaterThan;
        BooleanIr(self.builder.ins().icmp_imm(
            UnsignedGreaterThan,
            fs,
            FlowSelector::break_at(depth).imm() as i64,
        ))
    }

    pub fn put_is_flow_selector_break_or_continue(&mut self, depth: u32) -> BooleanIr {
        logger::debug!(event = "put_is_flow_selector_break_or_continue");
        use ir::condcodes::IntCC::Equal;
        let fs = self.builder.ins().stack_load(ir::types::I32, self.fcs, 4);
        let fs_depth = self.builder.ins().band_imm(fs, 0x0000FF00);
        BooleanIr(self.builder.ins().icmp_imm(Equal, fs_depth, depth as i64))
    }

    pub fn put_is_flow_selector_break(&mut self, depth: u32) -> BooleanIr {
        logger::debug!(event = "put_is_flow_selector_break");
        use ir::condcodes::IntCC::Equal;
        let fs = self.builder.ins().stack_load(ir::types::I32, self.fcs, 4);
        let fs_depth = self.builder.ins().band_imm(fs, 0x0000FF00);
        BooleanIr(self.builder.ins().icmp_imm(Equal, fs_depth, depth as i64))
    }

    // control flow

    /// Puts a conditional branch instruction.
    pub fn put_branch(
        &mut self,
        cond: BooleanIr,
        then_block: ir::Block,
        then_args: &[ir::BlockArg],
        else_block: ir::Block,
        else_args: &[ir::BlockArg],
    ) {
        logger::debug!(
            event = "put_branch",
            ?cond,
            ?then_block,
            ?then_args,
            ?else_block,
            ?else_args
        );
        self.builder
            .ins()
            .brif(cond.0, then_block, then_args, else_block, else_args);
    }

    /// Puts an unconditional branch instruction.
    pub fn put_jump(&mut self, block: ir::Block, args: &[ir::BlockArg]) {
        logger::debug!(event = "put_jump", ?block, ?args);
        debug_assert!(!self.block_terminated);
        self.builder.ins().jump(block, args);
        self.block_terminated = true;
    }

    pub fn put_jump_if_not_terminated(&mut self, block: ir::Block, args: &[ir::BlockArg]) {
        if self.block_terminated {
            // We should not append any instructions after a terminator instruction.
        } else {
            self.put_jump(block, args);
        }
    }

    pub fn put_switch_blocks(&mut self, value: ir::Value, num_blocks: u32) -> Vec<ir::Block> {
        logger::debug!(event = "put_switch_blocks", ?value, num_blocks);
        debug_assert!(num_blocks >= 2);
        // TODO(perf): use JumpTable
        let mut switch = Switch::new();
        let mut blocks = vec![];
        for i in 0..num_blocks - 1 {
            let block = self.create_block();
            blocks.push(block);
            switch.set_entry(i as u128, block);
        }
        let done_block = self.create_block();
        blocks.push(done_block);
        debug_assert_eq!(blocks.len(), num_blocks as usize);
        switch.emit(&mut self.builder, value, done_block);
        blocks
    }

    pub fn put_jump_table(&mut self, index: ir::Value, blocks: &[ir::Block], default: ir::Block) {
        logger::debug!(event = "put_jump_table", ?index, ?blocks, ?default);
        debug_assert!(!self.block_terminated);
        let mut table = Vec::with_capacity(blocks.len());
        for block in blocks {
            table.push(self.builder.func.dfg.block_call(*block, &[]));
        }
        let default = self.builder.func.dfg.block_call(default, &[]);
        let jump_table = self
            .builder
            .create_jump_table(ir::JumpTableData::new(default, &table));
        self.builder.ins().br_table(index, jump_table);
    }

    pub fn put_return(&mut self, support: &mut impl EditorSupport) {
        logger::debug!(event = "put_return");
        debug_assert!(!self.block_terminated);
        if self.runtime_assert_enabled {
            let retv = self.retv();
            let is_return_safe = self.put_is_return_safe(retv);
            self.put_assert(support, is_return_safe, c"retv must be return-safe");
        }
        let status = self.put_load_status();
        let masked = self.builder.ins().band_imm(status.0, Status::MASK as i64);
        self.builder.ins().return_(&[masked]);
        self.block_terminated = true;
    }

    pub fn put_suspend(&mut self) {
        logger::debug!(event = "put_suspend");
        let status = self
            .builder
            .ins()
            .iconst(ir::types::I32, Status::SUSPEND.imm() as i64);
        self.builder.ins().return_(&[status]);
        self.block_terminated = true;
    }

    pub fn put_unreachable(&mut self) {
        logger::debug!(event = "put_unreachable");
        self.builder.ins().trap(ir::TrapCode::unwrap_user(128));
        self.block_terminated = true;
    }

    // elementary data types

    pub fn put_boolean(&mut self, value: bool) -> BooleanIr {
        logger::debug!(event = "put_boolean", value);
        BooleanIr(self.builder.ins().iconst(ir::types::I8, value as i64))
    }

    pub fn put_number(&mut self, value: f64) -> NumberIr {
        logger::debug!(event = "put_number", value);
        NumberIr(self.builder.ins().f64const(value))
    }

    pub fn put_nullptr(&mut self) -> ir::Value {
        self.builder.ins().iconst(self.addr_type, 0)
    }

    // type conversions for elementary data types

    pub fn put_boolean_to_number(&mut self, value: BooleanIr) -> NumberIr {
        logger::debug!(event = "put_boolean_to_number", ?value);
        NumberIr(self.builder.ins().fcvt_from_uint(ir::types::F64, value.0))
    }

    pub fn put_number_to_boolean(&mut self, value: NumberIr) -> BooleanIr {
        logger::debug!(event = "put_number_to_boolean", ?value);
        use ir::condcodes::FloatCC::NotEqual;
        let zero = self.builder.ins().f64const(0.0);
        BooleanIr(self.builder.ins().fcmp(NotEqual, value.0, zero))
    }

    // string

    pub fn put_alloc_string(&mut self) -> StringIr {
        logger::debug!(event = "put_alloc_string");

        let slot = self.builder.create_sized_stack_slot(ir::StackSlotData {
            kind: ir::StackSlotKind::ExplicitSlot,
            size: StringFragment::SIZE as u32,
            align_shift: StringFragment::ALIGNMENT.ilog2() as u8,
        });

        let next = self.builder.ins().iconst(self.addr_type, 0);
        self.put_store_to_slot(next, slot, StringFragment::NEXT_OFFSET);

        let kind = self
            .builder
            .ins()
            .iconst(ir::types::I8, StringFragmentKind::Stack as i64);
        self.put_store_to_slot(kind, slot, StringFragment::KIND_OFFSET);

        StringIr(self.builder.ins().stack_addr(self.addr_type, slot, 0))
    }

    pub fn put_set_string(&mut self, value: &[u16], target: StringIr) {
        logger::debug!(event = "put_set_string", ?value, ?target);
        const FLAGS: ir::MemFlags = ir::MemFlags::new().with_aligned().with_notrap();

        let ptr = self
            .builder
            .ins()
            .iconst(self.addr_type, value.as_ptr() as i64);
        self.builder
            .ins()
            .store(FLAGS, ptr, target.0, StringFragment::PTR_OFFSET as i32);

        debug_assert!(value.len() <= u32::MAX as usize);
        let len = self
            .builder
            .ins()
            .iconst(ir::types::I32, value.len() as i64);
        self.builder
            .ins()
            .store(FLAGS, len, target.0, StringFragment::LEN_OFFSET as i32);
    }

    pub fn put_create_string(&mut self, value: &[u16]) -> StringIr {
        logger::debug!(event = "put_create_string", ?value);

        let slot = self.builder.create_sized_stack_slot(ir::StackSlotData {
            kind: ir::StackSlotKind::ExplicitSlot,
            size: StringFragment::SIZE as u32,
            align_shift: StringFragment::ALIGNMENT.ilog2() as u8,
        });

        let next = self.builder.ins().iconst(self.addr_type, 0);
        self.put_store_to_slot(next, slot, StringFragment::NEXT_OFFSET);

        let ptr = self.builder.ins().iconst(
            self.addr_type,
            if value.is_empty() {
                0
            } else {
                value.as_ptr() as i64
            },
        );
        self.put_store_to_slot(ptr, slot, StringFragment::PTR_OFFSET);

        debug_assert!(value.len() <= u32::MAX as usize);
        let len = self
            .builder
            .ins()
            .iconst(ir::types::I32, value.len() as i64);
        self.put_store_to_slot(len, slot, StringFragment::LEN_OFFSET);

        let kind = self
            .builder
            .ins()
            .iconst(ir::types::I8, StringFragmentKind::Stack as i64);
        self.put_store_to_slot(kind, slot, StringFragment::KIND_OFFSET);

        StringIr(self.builder.ins().stack_addr(self.addr_type, slot, 0))
    }

    pub fn put_string_on_stack(&mut self, string: StringIr) -> BooleanIr {
        logger::debug!(event = "put_string_on_stack", ?string);
        use ir::condcodes::IntCC::Equal;
        let kind = self.put_load_kind_from_string(string);
        BooleanIr(
            self.builder
                .ins()
                .icmp_imm(Equal, kind, StringFragmentKind::Stack as i64),
        )
    }

    fn put_load_kind_from_string(&mut self, string: StringIr) -> ir::Value {
        self.put_load_i8(string.0, StringFragment::KIND_OFFSET)
    }

    // any

    pub fn put_alloc_any(&mut self) -> AnyIr {
        logger::debug!(event = "put_alloc_any");
        let slot = self.builder.create_sized_stack_slot(ir::StackSlotData {
            kind: ir::StackSlotKind::ExplicitSlot,
            size: Value::SIZE as u32,
            align_shift: Value::ALIGNMENT.ilog2() as u8,
        });
        AnyIr(self.builder.ins().stack_addr(self.addr_type, slot, 0))
    }

    pub fn put_load_kind(&mut self, any: AnyIr) -> ir::Value {
        self.put_load_i8(any.0, Value::KIND_OFFSET)
    }

    pub fn put_has_value(&mut self, any: AnyIr) -> BooleanIr {
        logger::debug!(event = "put_has_value", ?any);
        use ir::condcodes::IntCC::NotEqual;
        let kind = self.put_load_i8(any.0, Value::KIND_OFFSET);
        BooleanIr(
            self.builder
                .ins()
                .icmp_imm(NotEqual, kind, Value::KIND_NONE as i64),
        )
    }

    fn put_is_return_safe(&mut self, any: AnyIr) -> BooleanIr {
        logger::debug!(event = "put_is_return_safe", ?any);
        let then_block = self.create_block();
        let merge_block = self.create_block_with_i8();

        // if any.is_string()
        let is_string = self.put_is_string(any);
        self.put_branch(
            is_string,
            then_block,
            &[],
            merge_block,
            &[is_string.0.into()],
        );
        // {
        self.switch_to_block(then_block);
        let string = self.put_load_string(any);
        let on_stack = self.put_string_on_stack(string);
        self.put_jump(merge_block, &[on_stack.0.into()]);
        // }

        self.switch_to_block(merge_block);
        let not_return_safe = BooleanIr(self.get_block_param(merge_block, 0));
        self.put_logical_not(not_return_safe)
    }

    pub fn put_load_boolean(&mut self, any: AnyIr) -> BooleanIr {
        logger::debug!(event = "put_load_boolean", ?any);
        BooleanIr(self.put_load_i8(any.0, Value::HOLDER_OFFSET))
    }

    pub fn put_load_number(&mut self, any: AnyIr) -> NumberIr {
        logger::debug!(event = "put_load_number", ?any);
        NumberIr(self.put_load_f64(any.0, Value::HOLDER_OFFSET))
    }

    pub fn put_load_string(&mut self, any: AnyIr) -> StringIr {
        logger::debug!(event = "put_load_string", ?any);
        StringIr(self.put_load_addr(any.0, Value::HOLDER_OFFSET))
    }

    pub fn put_load_promise(&mut self, any: AnyIr) -> PromiseIr {
        logger::debug!(event = "put_load_promise", ?any);
        PromiseIr(self.put_load_i32(any.0, Value::HOLDER_OFFSET))
    }

    pub fn put_load_object(&mut self, any: AnyIr) -> ObjectIr {
        logger::debug!(event = "put_load_object", ?any);
        ObjectIr(self.put_load_addr(any.0, Value::HOLDER_OFFSET))
    }

    // capture

    pub fn put_load_capture(&mut self, index: u16) -> CaptureIr {
        logger::debug!(event = "put_load_capture", index);
        let offset = (self.addr_type.bytes() as usize) * (index as usize);
        let captures = self.put_get_captures_from_closure();
        CaptureIr(self.put_load_addr(captures, offset))
    }

    pub fn put_load_captured_value(&mut self, index: u16) -> AnyIr {
        logger::debug!(event = "put_load_captured_value", index);
        let offset = (self.addr_type.bytes() as usize) * (index as usize);
        let captures = self.put_get_captures_from_closure();
        let capture = self.put_load_addr(captures, offset);
        AnyIr(self.put_load_addr(capture, Capture::TARGET_OFFSET))
    }

    pub fn put_escape_value(&mut self, capture: CaptureIr, value: AnyIr) {
        logger::debug!(event = "put_escape_value", ?capture, ?value);
        let escaped = self
            .builder
            .ins()
            .iadd_imm(capture.0, Capture::ESCAPED_OFFSET as i64);
        self.put_store(escaped, capture.0, Capture::TARGET_OFFSET);
        self.put_safe_copy_i128(value.0, escaped, 0);
    }

    // closure

    fn put_get_captures_from_closure(&mut self) -> ir::Value {
        self.builder
            .ins()
            .iadd_imm(self.closure, Closure::CAPTURES_OFFSET as i64)
    }

    pub fn put_load_lambda_from_closure(&mut self, closure: ClosureIr) -> LambdaIr {
        logger::debug!(event = "put_load_lambda_from_closure", ?closure);
        LambdaIr(self.put_load_addr(closure.0, Closure::LAMBDA_OFFSET))
    }

    pub fn put_store_capture_to_closure(
        &mut self,
        capture: CaptureIr,
        closure: ClosureIr,
        index: u16,
    ) {
        logger::debug!(
            event = "put_store_capture_to_closure",
            ?capture,
            ?closure,
            index
        );
        let offset =
            Closure::CAPTURES_OFFSET + (self.addr_type.bytes() as usize) * (index as usize);
        self.put_store(capture.0, closure.0, offset);
    }

    pub fn put_call(
        &mut self,
        closure: ClosureIr,
        flags: CallContextFlags,
        retv: AnyIr,
    ) -> StatusIr {
        logger::debug!(event = "put_call", ?closure, ?flags, ?retv);
        self.put_store_closure_to_call_context(closure);
        self.put_store_flags_to_call_context(flags);
        let lambda = self.put_load_lambda_from_closure(closure);
        let args = &[
            self.runtime(),
            self.builder
                .ins()
                .stack_addr(self.addr_type, self.call_context, 0),
            retv.0,
        ];
        let call = self
            .builder
            .ins()
            .call_indirect(self.lambda_sig, lambda.0, args);
        StatusIr(self.builder.inst_results(call)[0])
    }

    // coroutine

    pub fn put_set_coroutine_mode(&mut self) {
        logger::debug!(event = "put_set_coroutine_mode");
        debug_assert!(!self.coroutine_mode);
        self.coroutine_mode = true;
        let coroutine = self.coroutine();
        self.closure = self.put_load_addr(coroutine.0, Coroutine::CLOSURE_OFFSET);
    }

    pub fn put_load_state_from_coroutine(&mut self) -> ir::Value {
        logger::debug!(event = "put_load_state_from_coroutine");
        let coroutine = self.coroutine();
        self.put_load_i32(coroutine.0, Coroutine::STATE_OFFSET)
    }

    pub fn put_load_num_locals_from_coroutine(&mut self) -> ir::Value {
        logger::debug!(event = "put_load_num_locals_from_coroutine");
        let coroutine = self.coroutine();
        self.put_load_i16(coroutine.0, Coroutine::NUM_LOCALS_OFFSET)
    }

    pub fn put_load_scratch_buffer_len_from_coroutine(&mut self) -> ir::Value {
        logger::debug!(event = "put_load_scratch_buffer_len_from_coroutine");
        let coroutine = self.coroutine();
        self.put_load_i16(coroutine.0, Coroutine::SCRATCH_BUFFER_LEN_OFFSET)
    }

    pub fn put_get_local_from_coroutine(&mut self, index: u16) -> AnyIr {
        logger::debug!(event = "put_get_local_from_coroutine", index);
        // TODO: emit assert(index < coroutine.num_locals)
        let coroutine = self.coroutine();
        let offset = Coroutine::LOCALS_OFFSET + Value::SIZE * (index as usize);
        AnyIr(self.builder.ins().iadd_imm(coroutine.0, offset as i64))
    }

    pub fn put_store_state_to_coroutine(&mut self, state: u32) {
        logger::debug!(event = "put_store_state_to_coroutine", state);
        const FLAGS: ir::MemFlags = ir::MemFlags::new().with_aligned().with_notrap();
        let coroutine = self.coroutine();
        let state = self.builder.ins().iconst(ir::types::I32, state as i64);
        self.builder
            .ins()
            .store(FLAGS, state, coroutine.0, Coroutine::STATE_OFFSET as i32);
    }

    // object

    pub fn put_is_constructor(&mut self, object: ObjectIr) -> BooleanIr {
        logger::debug!(event = "put_is_constructor", ?object);
        use ir::condcodes::IntCC::NotEqual;
        let flags = self.put_load_flags_from_object(object);
        let masked = self
            .builder
            .ins()
            .band_imm(flags, ObjectFlags::CONSTRUCTOR.bits() as i64);
        BooleanIr(self.builder.ins().icmp_imm(NotEqual, masked, 0))
    }

    pub fn put_is_callable(&mut self, object: ObjectIr) -> BooleanIr {
        logger::debug!(event = "put_is_callable", ?object);
        use ir::condcodes::IntCC::NotEqual;
        let flags = self.put_load_flags_from_object(object);
        let masked = self
            .builder
            .ins()
            .band_imm(flags, ObjectFlags::CALLABLE.bits() as i64);
        BooleanIr(self.builder.ins().icmp_imm(NotEqual, masked, 0))
    }

    fn put_load_flags_from_object(&mut self, object: ObjectIr) -> ir::Value {
        self.put_load_i8(object.0, Object::FLAGS_OFFSET)
    }

    pub fn put_object(&mut self, addr: usize) -> ObjectIr {
        logger::debug!(event = "put_object", ?addr);
        ObjectIr(self.builder.ins().iconst(self.addr_type, addr as i64))
    }

    pub fn put_load_closure_from_object(&mut self, object: ObjectIr) -> ClosureIr {
        logger::debug!(event = "put_load_closure_from_object", ?object);
        ClosureIr(self.put_load_addr(object.0, Object::NUCLEUS_OFFSET))
    }

    pub fn put_store_closure_to_object(&mut self, closure: ClosureIr, object: ObjectIr) {
        logger::debug!(event = "put_store_closure_to_object", ?object);
        const FLAGS: ir::MemFlags = ir::MemFlags::new().with_aligned().with_notrap();
        const OFFSET: i32 = Object::NUCLEUS_OFFSET as i32;
        // TODO: The CONSTRUCTOR flag should be set in MakeConstructor()
        const OBJECT_FLAGS: ObjectFlags = ObjectFlags::CONSTRUCTOR.union(ObjectFlags::CALLABLE);
        self.builder.ins().store(FLAGS, closure.0, object.0, OFFSET);
        // Set callable flag
        let flags = self.put_load_flags_from_object(object);
        let flags = self
            .builder
            .ins()
            .bor_imm(flags, OBJECT_FLAGS.bits() as i64);
        self.put_store_flags_to_object(flags, object);
    }

    fn put_store_flags_to_object(&mut self, flags: ir::Value, object: ObjectIr) {
        logger::debug!(event = "put_store_flags_to_object", ?flags, ?object);
        const FLAGS: ir::MemFlags = ir::MemFlags::new().with_aligned().with_notrap();
        const OFFSET: i32 = Object::FLAGS_OFFSET as i32;
        self.builder.ins().store(FLAGS, flags, object.0, OFFSET);
    }

    // call context

    pub fn put_store_caller_to_call_context(&mut self) {
        const OFFSET: i32 = CallContext::CALLER_OFFSET as i32;
        let caller = self.context();
        self.builder
            .ins()
            .stack_store(caller, self.call_context, OFFSET);
    }

    pub fn put_store_flags_to_call_context(&mut self, flags: CallContextFlags) {
        const OFFSET: i32 = CallContext::FLAGS_OFFSET as i32;
        let flags = self
            .builder
            .ins()
            .iconst(ir::types::I16, flags.bits() as i64);
        self.builder
            .ins()
            .stack_store(flags, self.call_context, OFFSET);
    }

    pub fn put_store_depth_to_call_context(&mut self) {
        const OFFSET: i32 = CallContext::DEPTH_OFFSET as i32;
        let caller = self.context();
        let depth = self.put_load_i16(caller, CallContext::DEPTH_OFFSET);
        let depth = self.builder.ins().iadd_imm(depth, 1);
        self.builder
            .ins()
            .stack_store(depth, self.call_context, OFFSET);
    }

    pub fn put_store_argc_to_call_context(&mut self, argc: u16) {
        const OFFSET: i32 = CallContext::ARGC_OFFSET as i32;
        let argc = self.builder.ins().iconst(ir::types::I16, argc as i64);
        self.builder
            .ins()
            .stack_store(argc, self.call_context, OFFSET);
    }

    pub fn put_store_argc_max_to_call_context(&mut self, argc_max: u16) {
        const OFFSET: i32 = CallContext::ARGC_MAX_OFFSET as i32;
        let argc_max = self.builder.ins().iconst(ir::types::I16, argc_max as i64);
        self.builder
            .ins()
            .stack_store(argc_max, self.call_context, OFFSET);
    }

    pub fn put_store_argv_to_call_context(&mut self, argv: ArgvIr) {
        const OFFSET: i32 = CallContext::ARGV_OFFSET as i32;
        self.builder
            .ins()
            .stack_store(argv.0, self.call_context, OFFSET);
    }

    pub fn put_get_this_from_call_context(&mut self) -> AnyIr {
        const OFFSET: i32 = CallContext::THIS_OFFSET as i32;
        AnyIr(
            self.builder
                .ins()
                .stack_addr(self.addr_type, self.call_context, OFFSET),
        )
    }

    pub fn put_get_argv_from_call_context(&mut self) -> ArgvIr {
        const OFFSET: i32 = CallContext::ARGV_OFFSET as i32;
        ArgvIr(
            self.builder
                .ins()
                .stack_load(self.addr_type, self.call_context, OFFSET),
        )
    }

    pub fn put_set_closure_mode(&mut self) {
        self.closure = self.envp();
    }

    pub fn put_store_closure_to_call_context(&mut self, closure: ClosureIr) {
        const OFFSET: i32 = CallContext::ENVP_OFFSET as i32;
        self.builder
            .ins()
            .stack_store(closure.0, self.call_context, OFFSET);
    }

    pub fn put_call_stack_too_deep(&mut self, max: u16) -> BooleanIr {
        use ir::condcodes::IntCC::UnsignedGreaterThan;
        const OFFSET: i32 = CallContext::DEPTH_OFFSET as i32;
        let depth = self
            .builder
            .ins()
            .stack_load(ir::types::I16, self.call_context, OFFSET);
        BooleanIr(
            self.builder
                .ins()
                .icmp_imm(UnsignedGreaterThan, depth, max as i64),
        )
    }

    // argv

    pub fn put_alloc_argv(&mut self, argc: u16) -> ArgvIr {
        logger::debug!(event = "put_alloc_argv", argc);

        // The `argv` argument of a lambda function must be non-null even when `argc` is 0.

        let slot = self.builder.create_sized_stack_slot(ir::StackSlotData {
            kind: ir::StackSlotKind::ExplicitSlot,
            size: (Value::SIZE as u32) * (argc as u32),
            align_shift: Value::ALIGNMENT.ilog2() as u8,
        });

        ArgvIr(self.builder.ins().stack_addr(self.addr_type, slot, 0))
    }

    pub fn put_get_arg(&mut self, argv: ArgvIr, index: u16) -> AnyIr {
        logger::debug!(event = "put_get_arg", ?argv, index);
        let offset = (Value::SIZE as i64) * (index as i64);
        let addr = self.builder.ins().iadd_imm(argv.0, offset);
        AnyIr(addr)
    }

    // load

    fn put_load_i8(&mut self, addr: ir::Value, offset: usize) -> ir::Value {
        const FLAGS: ir::MemFlags = ir::MemFlags::new().with_aligned().with_notrap();
        debug_assert!(offset <= i32::MAX as usize);
        self.builder
            .ins()
            .load(ir::types::I8, FLAGS, addr, offset as i32)
    }

    fn put_load_i16(&mut self, addr: ir::Value, offset: usize) -> ir::Value {
        const FLAGS: ir::MemFlags = ir::MemFlags::new().with_aligned().with_notrap();
        debug_assert!(offset <= i32::MAX as usize);
        self.builder
            .ins()
            .load(ir::types::I16, FLAGS, addr, offset as i32)
    }

    fn put_load_i32(&mut self, addr: ir::Value, offset: usize) -> ir::Value {
        const FLAGS: ir::MemFlags = ir::MemFlags::new().with_aligned().with_notrap();
        debug_assert!(offset <= i32::MAX as usize);
        self.builder
            .ins()
            .load(ir::types::I32, FLAGS, addr, offset as i32)
    }

    fn put_load_i64(&mut self, addr: ir::Value, offset: usize) -> ir::Value {
        const FLAGS: ir::MemFlags = ir::MemFlags::new().with_aligned().with_notrap();
        debug_assert!(offset <= i32::MAX as usize);
        self.builder
            .ins()
            .load(ir::types::I64, FLAGS, addr, offset as i32)
    }

    // TODO(perf): using this method is faster than put_load_i64() * 2 for loading a `Value`.
    #[allow(unused)]
    fn put_load_i128(&mut self, addr: ir::Value, offset: usize) -> ir::Value {
        const FLAGS: ir::MemFlags = ir::MemFlags::new().with_aligned().with_notrap();
        debug_assert!(offset <= i32::MAX as usize);
        self.builder
            .ins()
            .load(ir::types::I128, FLAGS, addr, offset as i32)
    }

    fn put_load_f64(&mut self, addr: ir::Value, offset: usize) -> ir::Value {
        const FLAGS: ir::MemFlags = ir::MemFlags::new().with_aligned().with_notrap();
        debug_assert!(offset <= i32::MAX as usize);
        self.builder
            .ins()
            .load(ir::types::F64, FLAGS, addr, offset as i32)
    }

    fn put_load_addr(&mut self, addr: ir::Value, offset: usize) -> ir::Value {
        const FLAGS: ir::MemFlags = ir::MemFlags::new().with_aligned().with_notrap();
        debug_assert!(offset <= i32::MAX as usize);
        self.builder
            .ins()
            .load(self.addr_type, FLAGS, addr, offset as i32)
    }

    // store

    fn put_store(&mut self, value: ir::Value, addr: ir::Value, offset: usize) {
        const FLAGS: ir::MemFlags = ir::MemFlags::new().with_aligned().with_notrap();
        debug_assert!(offset <= i32::MAX as usize);
        self.builder.ins().store(FLAGS, value, addr, offset as i32);
    }

    fn put_store_to_slot(&mut self, value: ir::Value, slot: ir::StackSlot, offset: usize) {
        debug_assert!(offset <= i32::MAX as usize);
        self.builder.ins().stack_store(value, slot, offset as i32);
    }

    pub fn put_store_none_to_any(&mut self, any: AnyIr) {
        logger::debug!(event = "put_store_none_to_any", ?any);
        self.put_store_kind_to_any(Value::KIND_NONE, any);
    }

    pub fn put_store_undefined_to_any(&mut self, any: AnyIr) {
        logger::debug!(event = "put_store_undefined_to_any", ?any);
        self.put_store_kind_to_any(Value::KIND_UNDEFINED, any);
    }

    pub fn put_store_null_to_any(&mut self, any: AnyIr) {
        logger::debug!(event = "put_store_null_to_any", ?any);
        self.put_store_kind_to_any(Value::KIND_NULL, any);
    }

    pub fn put_store_boolean_to_any(&mut self, boolean: BooleanIr, any: AnyIr) {
        logger::debug!(event = "put_store_boolean_to_any", ?boolean, ?any);
        self.put_store_kind_and_value_to_any(Value::KIND_BOOLEAN, boolean.0, any);
    }

    pub fn put_store_number_to_any(&mut self, number: NumberIr, any: AnyIr) {
        logger::debug!(event = "put_store_number_to_any", ?number, ?any);
        self.put_store_kind_and_value_to_any(Value::KIND_NUMBER, number.0, any);
    }

    pub fn put_store_string_to_any(&mut self, string: StringIr, any: AnyIr) {
        logger::debug!(event = "put_store_string_to_any", ?string, ?any);
        self.put_store_kind_and_value_to_any(Value::KIND_STRING, string.0, any);
    }

    pub fn put_store_promise_to_any(&mut self, promise: PromiseIr, any: AnyIr) {
        logger::debug!(event = "put_store_promise_to_any", ?promise, ?any);
        self.put_store_kind_and_value_to_any(Value::KIND_PROMISE, promise.0, any);
    }

    pub fn put_store_object_to_any(&mut self, object: ObjectIr, any: AnyIr) {
        logger::debug!(event = "put_store_object_to_any", ?object, ?any);
        self.put_store_kind_and_value_to_any(Value::KIND_OBJECT, object.0, any);
    }

    pub fn put_store_any_to_any(&mut self, src: AnyIr, dst: AnyIr) {
        logger::debug!(event = "put_store_any_to_any", ?src, ?dst);
        // TODO(perf): should use memcpy?
        static_assert_eq!(Value::SIZE * 8, 128);
        self.put_safe_copy_i128(src.0, dst.0, 0);
    }

    fn put_store_kind_and_value_to_any(&mut self, kind: u8, value: ir::Value, any: AnyIr) {
        self.put_store_kind_to_any(kind, any);
        self.put_store(value, any.0, Value::HOLDER_OFFSET);
    }

    fn put_store_kind_to_any(&mut self, kind: u8, any: AnyIr) {
        let kind = self.builder.ins().iconst(ir::types::I8, kind as i64);
        self.put_store(kind, any.0, Value::KIND_OFFSET);
    }

    // copy operations

    fn put_safe_copy_i128(&mut self, src: ir::Value, dst: ir::Value, offset: usize) {
        // The address may not be 128-bit aligned, but 64-bit aligned.
        let opaque64 = self.put_load_i64(src, 0);
        self.put_store(opaque64, dst, offset);
        let opaque64 = self.put_load_i64(src, 8);
        self.put_store(opaque64, dst, offset + 8);
    }

    // unary operators

    pub fn put_negate(&mut self, value: NumberIr) -> NumberIr {
        logger::debug!(event = "put_negate", ?value);
        NumberIr(self.builder.ins().fneg(value.0))
    }

    // 6.1.6.1.2 Number::bitwiseNOT ( x )
    pub fn put_bitwise_not(
        &mut self,
        support: &mut impl EditorSupport,
        value: NumberIr,
    ) -> NumberIr {
        logger::debug!(event = "put_bitwise_not", ?value);
        let int32 = self.put_runtime_to_int32(support, value);
        let bnot = self.builder.ins().bnot(int32);
        self.put_i32_to_f64(bnot)
    }

    pub fn put_logical_not(&mut self, value: BooleanIr) -> BooleanIr {
        logger::debug!(event = "put_logical_not", ?value);
        BooleanIr(self.builder.ins().bxor_imm(value.0, 1))
    }

    pub fn put_logical_or(&mut self, lhs: BooleanIr, rhs: BooleanIr) -> BooleanIr {
        logger::debug!(event = "put_logical_or", ?lhs, ?rhs);
        BooleanIr(self.builder.ins().bor(lhs.0, rhs.0))
    }

    // arithmetic binary operators

    pub fn put_add(&mut self, lhs: NumberIr, rhs: NumberIr) -> NumberIr {
        logger::debug!(event = "put_add", ?lhs, ?rhs);
        NumberIr(self.builder.ins().fadd(lhs.0, rhs.0))
    }

    pub fn put_sub(&mut self, lhs: NumberIr, rhs: NumberIr) -> NumberIr {
        logger::debug!(event = "put_sub", ?lhs, ?rhs);
        NumberIr(self.builder.ins().fsub(lhs.0, rhs.0))
    }

    pub fn put_mul(&mut self, lhs: NumberIr, rhs: NumberIr) -> NumberIr {
        logger::debug!(event = "put_mul", ?lhs, ?rhs);
        NumberIr(self.builder.ins().fmul(lhs.0, rhs.0))
    }

    pub fn put_div(&mut self, lhs: NumberIr, rhs: NumberIr) -> NumberIr {
        logger::debug!(event = "put_div", ?lhs, ?rhs);
        NumberIr(self.builder.ins().fdiv(lhs.0, rhs.0))
    }

    pub fn put_rem(
        &mut self,
        support: &mut impl EditorSupport,
        lhs: NumberIr,
        rhs: NumberIr,
    ) -> NumberIr {
        logger::debug!(event = "put_rem", ?lhs, ?rhs);
        let func = self
            .runtime_func_cache
            .import_fmod(support, self.builder.func);
        let call = self.builder.ins().call(func, &[lhs.0, rhs.0]);
        NumberIr(self.builder.inst_results(call)[0])
    }

    pub fn put_exp(
        &mut self,
        support: &mut impl EditorSupport,
        lhs: NumberIr,
        rhs: NumberIr,
    ) -> NumberIr {
        logger::debug!(event = "put_exp", ?lhs, ?rhs);
        let func = self
            .runtime_func_cache
            .import_pow(support, self.builder.func);
        let call = self.builder.ins().call(func, &[lhs.0, rhs.0]);
        NumberIr(self.builder.inst_results(call)[0])
    }

    // shift operators

    // 6.1.6.1.9 Number::leftShift ( x, y )
    pub fn put_left_shift(
        &mut self,
        support: &mut impl EditorSupport,
        x: NumberIr,
        y: NumberIr,
    ) -> NumberIr {
        logger::debug!(event = "put_left_shift", ?x, ?y);
        let lnum = self.put_runtime_to_int32(support, x);
        let rnum = self.put_runtime_to_uint32(support, y);
        let shift_count = self.builder.ins().urem_imm(rnum, 32);
        let shifted = self.builder.ins().ishl(lnum, shift_count);
        self.put_i32_to_f64(shifted)
    }

    // 6.1.6.1.10 Number::signedRightShift ( x, y )
    pub fn put_signed_right_shift(
        &mut self,
        support: &mut impl EditorSupport,
        x: NumberIr,
        y: NumberIr,
    ) -> NumberIr {
        logger::debug!(event = "put_signed_right_shift", ?x, ?y);
        let lnum = self.put_runtime_to_int32(support, x);
        let rnum = self.put_runtime_to_uint32(support, y);
        let shift_count = self.builder.ins().urem_imm(rnum, 32);
        let shifted = self.builder.ins().sshr(lnum, shift_count);
        self.put_i32_to_f64(shifted)
    }

    // 6.1.6.1.11 Number::unsignedRightShift ( x, y )
    pub fn put_unsigned_right_shift(
        &mut self,
        support: &mut impl EditorSupport,
        x: NumberIr,
        y: NumberIr,
    ) -> NumberIr {
        logger::debug!(event = "put_unsigned_right_shift", ?x, ?y);
        let lnum = self.put_runtime_to_uint32(support, x);
        let rnum = self.put_runtime_to_uint32(support, y);
        let shift_count = self.builder.ins().urem_imm(rnum, 32);
        let shifted = self.builder.ins().ushr(lnum, shift_count);
        self.put_i32_to_f64(shifted)
    }

    fn put_i32_to_f64(&mut self, value: ir::Value) -> NumberIr {
        NumberIr(self.builder.ins().fcvt_from_sint(ir::types::F64, value))
    }

    // relational operators

    pub fn put_less_than(&mut self, lhs: NumberIr, rhs: NumberIr) -> BooleanIr {
        logger::debug!(event = "put_less_than", ?lhs, ?rhs);
        use ir::condcodes::FloatCC::LessThan;
        BooleanIr(self.builder.ins().fcmp(LessThan, lhs.0, rhs.0))
    }

    pub fn put_greater_than(&mut self, lhs: NumberIr, rhs: NumberIr) -> BooleanIr {
        logger::debug!(event = "put_greater_than", ?lhs, ?rhs);
        use ir::condcodes::FloatCC::GreaterThan;
        BooleanIr(self.builder.ins().fcmp(GreaterThan, lhs.0, rhs.0))
    }

    pub fn put_less_than_or_equal(&mut self, lhs: NumberIr, rhs: NumberIr) -> BooleanIr {
        logger::debug!(event = "put_less_than_or_equal", ?lhs, ?rhs);
        use ir::condcodes::FloatCC::LessThanOrEqual;
        BooleanIr(self.builder.ins().fcmp(LessThanOrEqual, lhs.0, rhs.0))
    }

    pub fn put_greater_than_or_equal(&mut self, lhs: NumberIr, rhs: NumberIr) -> BooleanIr {
        logger::debug!(event = "put_greater_than_or_equal", ?lhs, ?rhs);
        use ir::condcodes::FloatCC::GreaterThanOrEqual;
        BooleanIr(self.builder.ins().fcmp(GreaterThanOrEqual, lhs.0, rhs.0))
    }

    // equality operators

    pub fn put_is_none(&mut self, any: AnyIr) -> BooleanIr {
        logger::debug!(event = "put_is_none", ?any);
        self.put_is_kind_of(Value::KIND_NONE, any)
    }

    pub fn put_is_undefined(&mut self, any: AnyIr) -> BooleanIr {
        logger::debug!(event = "put_is_undefined", ?any);
        self.put_is_kind_of(Value::KIND_UNDEFINED, any)
    }

    pub fn put_is_null(&mut self, any: AnyIr) -> BooleanIr {
        logger::debug!(event = "put_is_null", ?any);
        self.put_is_kind_of(Value::KIND_NULL, any)
    }

    pub fn put_is_boolean(&mut self, any: AnyIr) -> BooleanIr {
        logger::debug!(event = "put_is_boolean", ?any);
        self.put_is_kind_of(Value::KIND_BOOLEAN, any)
    }

    pub fn put_is_number(&mut self, any: AnyIr) -> BooleanIr {
        logger::debug!(event = "put_is_number", ?any);
        self.put_is_kind_of(Value::KIND_NUMBER, any)
    }

    pub fn put_is_string(&mut self, any: AnyIr) -> BooleanIr {
        logger::debug!(event = "put_is_string", ?any);
        self.put_is_kind_of(Value::KIND_STRING, any)
    }

    pub fn put_is_promise(&mut self, any: AnyIr) -> BooleanIr {
        logger::debug!(event = "put_is_promise", ?any);
        self.put_is_kind_of(Value::KIND_PROMISE, any)
    }

    pub fn put_is_object(&mut self, any: AnyIr) -> BooleanIr {
        logger::debug!(event = "put_is_object", ?any);
        self.put_is_kind_of(Value::KIND_OBJECT, any)
    }

    pub fn put_is_non_nullish(&mut self, any: AnyIr) -> BooleanIr {
        logger::debug!(event = "put_is_non_nullish", ?any);
        use ir::condcodes::IntCC::UnsignedGreaterThan;
        let kind = self.put_load_kind(any);
        BooleanIr(
            self.builder
                .ins()
                .icmp_imm(UnsignedGreaterThan, kind, Value::KIND_NULL as i64),
        )
    }

    pub fn put_is_nullptr(&mut self, value: ir::Value) -> BooleanIr {
        logger::debug!(event = "put_is_nullptr", ?value);
        use ir::condcodes::IntCC::Equal;
        BooleanIr(self.builder.ins().icmp_imm(Equal, value, 0))
    }

    pub fn put_is_same_boolean(&mut self, lhs: BooleanIr, rhs: BooleanIr) -> BooleanIr {
        logger::debug!(event = "put_is_same_boolean", ?lhs, ?rhs);
        self.put_is_same_int_value(lhs.0, rhs.0)
    }

    pub fn put_is_same_number(&mut self, lhs: NumberIr, rhs: NumberIr) -> BooleanIr {
        logger::debug!(event = "put_is_same_number", ?lhs, ?rhs);
        self.put_is_same_float_value(lhs.0, rhs.0)
    }

    pub fn put_is_same_string(
        &mut self,
        support: &mut impl EditorSupport,
        lhs: StringIr,
        rhs: StringIr,
    ) -> BooleanIr {
        logger::debug!(event = "put_is_same_string", ?lhs, ?rhs);
        self.put_runtime_is_same_string(support, lhs, rhs)
    }

    pub fn put_is_same_promise(&mut self, lhs: PromiseIr, rhs: PromiseIr) -> BooleanIr {
        logger::debug!(event = "put_is_same_promise", ?lhs, ?rhs);
        self.put_is_same_int_value(lhs.0, rhs.0)
    }

    pub fn put_is_same_object(&mut self, lhs: ObjectIr, rhs: ObjectIr) -> BooleanIr {
        logger::debug!(event = "put_is_same_object", ?lhs, ?rhs);
        self.put_is_same_int_value(lhs.0, rhs.0)
    }

    pub fn put_is_kind_of(&mut self, kind_imm: u8, any: AnyIr) -> BooleanIr {
        use ir::condcodes::IntCC::Equal;
        let kind = self.put_load_kind(any);
        BooleanIr(self.builder.ins().icmp_imm(Equal, kind, kind_imm as i64))
    }

    fn put_is_same_int_value(&mut self, lhs: ir::Value, rhs: ir::Value) -> BooleanIr {
        use ir::condcodes::IntCC::Equal;
        BooleanIr(self.builder.ins().icmp(Equal, lhs, rhs))
    }

    fn put_is_same_float_value(&mut self, lhs: ir::Value, rhs: ir::Value) -> BooleanIr {
        use ir::condcodes::FloatCC::Equal;
        BooleanIr(self.builder.ins().fcmp(Equal, lhs, rhs))
    }

    // bitwise operators

    // 6.1.6.1.17 Number::bitwiseAND ( x, y )
    pub fn put_bitwise_and(
        &mut self,
        support: &mut impl EditorSupport,
        x: NumberIr,
        y: NumberIr,
    ) -> NumberIr {
        logger::debug!(event = "put_bitwise_and", ?x, ?y);
        let lnum = self.put_runtime_to_int32(support, x);
        let rnum = self.put_runtime_to_int32(support, y);
        let result = self.builder.ins().band(lnum, rnum);
        self.put_i32_to_f64(result)
    }

    // 6.1.6.1.18 Number::bitwiseXOR ( x, y )
    pub fn put_bitwise_xor(
        &mut self,
        support: &mut impl EditorSupport,
        x: NumberIr,
        y: NumberIr,
    ) -> NumberIr {
        logger::debug!(event = "put_bitwise_xor", ?x, ?y);
        let lnum = self.put_runtime_to_int32(support, x);
        let rnum = self.put_runtime_to_int32(support, y);
        let result = self.builder.ins().bxor(lnum, rnum);
        self.put_i32_to_f64(result)
    }

    // 6.1.6.1.19 Number::bitwiseOR ( x, y )
    pub fn put_bitwise_or(
        &mut self,
        support: &mut impl EditorSupport,
        x: NumberIr,
        y: NumberIr,
    ) -> NumberIr {
        logger::debug!(event = "put_bitwise_or", ?x, ?y);
        let lnum = self.put_runtime_to_int32(support, x);
        let rnum = self.put_runtime_to_int32(support, y);
        let result = self.builder.ins().bor(lnum, rnum);
        self.put_i32_to_f64(result)
    }

    // operations on the scratch buffer of the coroutine

    pub fn put_get_scratch_buffer_from_coroutine(&mut self) -> ScratchBuffer {
        logger::debug!(event = "put_get_scratch_buffer_from_coroutine");
        let coroutine = self.coroutine();
        // TODO(perf): compile-time evaluation
        let num_locals = self.put_load_num_locals_from_coroutine();
        let num_locals = self.builder.ins().uextend(self.addr_type, num_locals);
        let offset = self.builder.ins().imul_imm(num_locals, Value::SIZE as i64);
        let offset = self
            .builder
            .ins()
            .iadd_imm(offset, Coroutine::LOCALS_OFFSET as i64);
        ScratchBuffer {
            addr: self.builder.ins().iadd(coroutine.0, offset),
            offset: 0,
        }
    }

    pub fn put_write_boolean_to_scratch_buffer(
        &mut self,
        value: BooleanIr,
        scratch_buffer: &mut ScratchBuffer,
    ) {
        logger::debug!(
            event = "put_write_boolean_to_scratch_buffer",
            ?value,
            ?scratch_buffer,
        );
        let ScratchBuffer { addr, offset } = *scratch_buffer;
        scratch_buffer.offset += Value::HOLDER_SIZE;
        self.put_store(value.0, addr, offset);
    }

    pub fn put_write_number_to_scratch_buffer(
        &mut self,
        value: NumberIr,
        scratch_buffer: &mut ScratchBuffer,
    ) {
        logger::debug!(
            event = "put_write_number_to_scratch_buffer",
            ?value,
            ?scratch_buffer,
        );
        let ScratchBuffer { addr, offset } = *scratch_buffer;
        scratch_buffer.offset += Value::HOLDER_SIZE;
        self.put_store(value.0, addr, offset);
    }

    pub fn put_write_string_to_scratch_buffer(
        &mut self,
        value: StringIr,
        scratch_buffer: &mut ScratchBuffer,
    ) {
        logger::debug!(
            event = "put_write_string_to_scratch_buffer",
            ?value,
            ?scratch_buffer,
        );
        let ScratchBuffer { addr, offset } = *scratch_buffer;
        scratch_buffer.offset += Value::HOLDER_SIZE;
        self.put_store(value.0, addr, offset);
    }

    pub fn put_write_closure_to_scratch_buffer(
        &mut self,
        value: ClosureIr,
        scratch_buffer: &mut ScratchBuffer,
    ) {
        logger::debug!(
            event = "put_write_closure_to_scratch_buffer",
            ?value,
            ?scratch_buffer,
        );
        let ScratchBuffer { addr, offset } = *scratch_buffer;
        scratch_buffer.offset += Value::HOLDER_SIZE;
        self.put_store(value.0, addr, offset);
    }

    pub fn put_write_object_to_scratch_buffer(
        &mut self,
        value: ObjectIr,
        scratch_buffer: &mut ScratchBuffer,
    ) {
        logger::debug!(
            event = "put_write_object_to_scratch_buffer",
            ?value,
            ?scratch_buffer,
        );
        let ScratchBuffer { addr, offset } = *scratch_buffer;
        scratch_buffer.offset += Value::HOLDER_SIZE;
        self.put_store(value.0, addr, offset);
    }

    pub fn put_write_promise_to_scratch_buffer(
        &mut self,
        value: PromiseIr,
        scratch_buffer: &mut ScratchBuffer,
    ) {
        logger::debug!(
            event = "put_write_promise_to_scratch_buffer",
            ?value,
            ?scratch_buffer,
        );
        let ScratchBuffer { addr, offset } = *scratch_buffer;
        scratch_buffer.offset += Value::HOLDER_SIZE;
        self.put_store(value.0, addr, offset);
    }

    pub fn put_write_any_to_scratch_buffer(
        &mut self,
        value: AnyIr,
        scratch_buffer: &mut ScratchBuffer,
    ) {
        logger::debug!(
            event = "put_write_any_to_scratch_buffer",
            ?value,
            ?scratch_buffer,
        );
        let ScratchBuffer { addr, offset } = *scratch_buffer;
        static_assert_eq!(Value::SIZE, Value::HOLDER_SIZE * 2);
        scratch_buffer.offset += Value::SIZE;
        self.put_safe_copy_i128(value.0, addr, offset);
    }

    pub fn put_read_boolean_from_scratch_buffer(
        &mut self,
        scratch_buffer: &mut ScratchBuffer,
    ) -> BooleanIr {
        logger::debug!(
            event = "put_read_boolean_from_scratch_buffer",
            ?scratch_buffer,
        );
        let ScratchBuffer { addr, offset } = *scratch_buffer;
        scratch_buffer.offset += Value::HOLDER_SIZE;
        BooleanIr(self.put_load_i8(addr, offset))
    }

    pub fn put_read_number_from_scratch_buffer(
        &mut self,
        scratch_buffer: &mut ScratchBuffer,
    ) -> NumberIr {
        logger::debug!(
            event = "put_read_number_from_scratch_buffer",
            ?scratch_buffer,
        );
        let ScratchBuffer { addr, offset } = *scratch_buffer;
        scratch_buffer.offset += Value::HOLDER_SIZE;
        NumberIr(self.put_load_f64(addr, offset))
    }

    pub fn put_read_string_from_scratch_buffer(
        &mut self,
        scratch_buffer: &mut ScratchBuffer,
    ) -> StringIr {
        logger::debug!(
            event = "put_read_string_from_scratch_buffer",
            ?scratch_buffer,
        );
        let ScratchBuffer { addr, offset } = *scratch_buffer;
        scratch_buffer.offset += Value::HOLDER_SIZE;
        StringIr(self.put_load_addr(addr, offset))
    }

    pub fn put_read_closure_from_scratch_buffer(
        &mut self,
        scratch_buffer: &mut ScratchBuffer,
    ) -> ClosureIr {
        logger::debug!(
            event = "put_read_closure_from_scratch_buffer",
            ?scratch_buffer,
        );
        let ScratchBuffer { addr, offset } = *scratch_buffer;
        scratch_buffer.offset += Value::HOLDER_SIZE;
        ClosureIr(self.put_load_addr(addr, offset))
    }

    pub fn put_read_object_from_scratch_buffer(
        &mut self,
        scratch_buffer: &mut ScratchBuffer,
    ) -> ObjectIr {
        logger::debug!(
            event = "put_read_object_from_scratch_buffer",
            ?scratch_buffer,
        );
        let ScratchBuffer { addr, offset } = *scratch_buffer;
        scratch_buffer.offset += Value::HOLDER_SIZE;
        ObjectIr(self.put_load_addr(addr, offset))
    }

    pub fn put_read_promise_from_scratch_buffer(
        &mut self,
        scratch_buffer: &mut ScratchBuffer,
    ) -> PromiseIr {
        logger::debug!(
            event = "put_read_promise_from_scratch_buffer",
            ?scratch_buffer,
        );
        let ScratchBuffer { addr, offset } = *scratch_buffer;
        scratch_buffer.offset += Value::HOLDER_SIZE;
        PromiseIr(self.put_load_i32(addr, offset))
    }

    pub fn put_read_any_from_scratch_buffer(
        &mut self,
        scratch_buffer: &mut ScratchBuffer,
    ) -> AnyIr {
        logger::debug!(
            event = "put_read_boolean_from_scratch_buffer",
            ?scratch_buffer,
        );
        let ScratchBuffer { addr, offset } = *scratch_buffer;
        static_assert_eq!(Value::SIZE, Value::HOLDER_SIZE * 2);
        scratch_buffer.offset += Value::SIZE;
        // Just return the address on the scratch buffer where the value has been stored.
        static_assert_eq!(Value::ALIGNMENT, Value::HOLDER_SIZE);
        AnyIr(self.builder.ins().iadd_imm(addr, offset as i64))
    }

    // operations on the capture buffer of the coroutine

    pub fn put_get_capture_buffer_from_coroutine(&mut self) -> ir::Value {
        let coroutine = self.coroutine();
        // TODO(perf): compile-time evaluation
        let num_locals = self.put_load_num_locals_from_coroutine();
        let num_locals = self.builder.ins().uextend(self.addr_type, num_locals);
        let offset = self.builder.ins().imul_imm(num_locals, Value::SIZE as i64);
        let offset = self
            .builder
            .ins()
            .iadd_imm(offset, Coroutine::LOCALS_OFFSET as i64);
        let scratch_buffer_len = self.put_load_scratch_buffer_len_from_coroutine();
        let scratch_buffer_len = self
            .builder
            .ins()
            .uextend(self.addr_type, scratch_buffer_len);
        let offset = self.builder.ins().iadd(offset, scratch_buffer_len);
        self.builder.ins().iadd(coroutine.0, offset)
    }

    pub fn put_load_capture_from_capture_buffer(&mut self, offset: usize) -> CaptureIr {
        logger::debug!(event = "put_load_capture_from_capture_buffer");
        let addr = self.put_get_capture_buffer_from_coroutine(); // TODO(perf): inefficient
        CaptureIr(self.put_load_addr(addr, offset))
    }

    pub fn put_store_capture_to_capture_buffer(&mut self, capture: CaptureIr, offset: usize) {
        logger::debug!(event = "put_store_capture_from_capture_buffer");
        let addr = self.put_get_capture_buffer_from_coroutine(); // TODO(perf): inefficient
        self.put_store(capture.0, addr, offset);
    }

    // runtime function calls

    pub fn put_runtime_to_boolean(
        &mut self,
        support: &mut impl EditorSupport,
        value: AnyIr,
    ) -> BooleanIr {
        logger::debug!(event = "put_runtime_to_boolean", ?value);
        if self.runtime_assert_enabled {
            self.put_assert_non_null(
                support,
                value.0,
                c"value passed to runtime_to_boolean() must be non-null",
            );
        }
        let func = self
            .runtime_func_cache
            .import_runtime_to_boolean(support, self.builder.func);
        let args = [self.runtime(), value.0];
        let call = self.builder.ins().call(func, &args);
        BooleanIr(self.builder.inst_results(call)[0])
    }

    pub fn put_runtime_to_numeric(
        &mut self,
        support: &mut impl EditorSupport,
        value: AnyIr,
    ) -> NumberIr {
        logger::debug!(event = "put_runtime_to_numeric", ?value);
        if self.runtime_assert_enabled {
            self.put_assert_non_null(
                support,
                value.0,
                c"value passed to runtime_to_numeric() must be non-null",
            );
        }
        let func = self
            .runtime_func_cache
            .import_runtime_to_numeric(support, self.builder.func);
        let args = [self.runtime(), value.0];
        let call = self.builder.ins().call(func, &args);
        NumberIr(self.builder.inst_results(call)[0])
    }

    pub fn put_runtime_to_string(
        &mut self,
        support: &mut impl EditorSupport,
        value: AnyIr,
    ) -> StringIr {
        logger::debug!(event = "put_runtime_to_string", ?value);
        if self.runtime_assert_enabled {
            self.put_assert_non_null(
                support,
                value.0,
                c"value passed to runtime_to_string() must be non-null",
            );
        }
        let func = self
            .runtime_func_cache
            .import_runtime_to_string(support, self.builder.func);
        let args = [self.runtime(), value.0];
        let call = self.builder.ins().call(func, &args);
        StringIr(self.builder.inst_results(call)[0])
    }

    pub fn put_runtime_number_to_string(
        &mut self,
        support: &mut impl EditorSupport,
        value: NumberIr,
    ) -> StringIr {
        logger::debug!(event = "put_runtime_number_to_string", ?value);
        let func = self
            .runtime_func_cache
            .import_runtime_number_to_string(support, self.builder.func);
        let args = [self.runtime(), value.0];
        let call = self.builder.ins().call(func, &args);
        StringIr(self.builder.inst_results(call)[0])
    }

    pub fn put_runtime_to_object(
        &mut self,
        support: &mut impl EditorSupport,
        any: AnyIr,
        retv: AnyIr,
    ) -> StatusIr {
        logger::debug!(event = "put_runtime_to_object", ?any);
        if self.runtime_assert_enabled {
            self.put_assert_non_null(
                support,
                any.0,
                c"value passed to runtime_to_object() must be non-null",
            );
        }
        let func = self
            .runtime_func_cache
            .import_runtime_to_object(support, self.builder.func);
        let args = [self.runtime(), any.0, retv.0];
        let call = self.builder.ins().call(func, &args);
        StatusIr(self.builder.inst_results(call)[0])
    }

    // 7.1.6 ToInt32 ( argument )
    pub fn put_runtime_to_int32(
        &mut self,
        support: &mut impl EditorSupport,
        value: NumberIr,
    ) -> ir::Value {
        logger::debug!(event = "put_runtime_to_int32", ?value);
        let func = self
            .runtime_func_cache
            .import_runtime_to_int32(support, self.builder.func);
        let args = [self.runtime(), value.0];
        let call = self.builder.ins().call(func, &args);
        self.builder.inst_results(call)[0]
    }

    pub fn put_runtime_to_uint32(
        &mut self,
        support: &mut impl EditorSupport,
        value: NumberIr,
    ) -> ir::Value {
        logger::debug!(event = "put_runtime_to_uint32", ?value);
        let func = self
            .runtime_func_cache
            .import_runtime_to_uint32(support, self.builder.func);
        let args = [self.runtime(), value.0];
        let call = self.builder.ins().call(func, &args);
        self.builder.inst_results(call)[0]
    }

    pub fn put_runtime_is_same_string(
        &mut self,
        support: &mut impl EditorSupport,
        lhs: StringIr,
        rhs: StringIr,
    ) -> BooleanIr {
        logger::debug!(event = "put_runtime_is_same_string", ?lhs, ?rhs);
        if self.runtime_assert_enabled {
            self.put_assert_non_null(
                support,
                lhs.0,
                c"lhs passed to runtime_is_same_string() must be non-null",
            );
            self.put_assert_non_null(
                support,
                rhs.0,
                c"rhs passed to runtime_is_same_string() must be non-null",
            );
        }
        let func = self
            .runtime_func_cache
            .import_runtime_is_same_string(support, self.builder.func);
        let args = [self.runtime(), lhs.0, rhs.0];
        let call = self.builder.ins().call(func, &args);
        BooleanIr(self.builder.inst_results(call)[0])
    }

    pub fn put_runtime_is_loosely_equal(
        &mut self,
        support: &mut impl EditorSupport,
        lhs: AnyIr,
        rhs: AnyIr,
    ) -> BooleanIr {
        logger::debug!(event = "put_runtime_is_loosely_equal", ?lhs, ?rhs);
        if self.runtime_assert_enabled {
            self.put_assert_non_null(
                support,
                lhs.0,
                c"lhs passed to runtime_is_loosely_equal() must be non-null",
            );
            self.put_assert_non_null(
                support,
                rhs.0,
                c"rhs passed to runtime_is_loosely_equal() must be non-null",
            );
        }
        let func = self
            .runtime_func_cache
            .import_runtime_is_loosely_equal(support, self.builder.func);
        let args = [self.runtime(), lhs.0, rhs.0];
        let call = self.builder.ins().call(func, &args);
        BooleanIr(self.builder.inst_results(call)[0])
    }

    pub fn put_runtime_is_strictly_equal(
        &mut self,
        support: &mut impl EditorSupport,
        lhs: AnyIr,
        rhs: AnyIr,
    ) -> BooleanIr {
        logger::debug!(event = "put_runtime_is_strictly_equal", ?lhs, ?rhs);
        if self.runtime_assert_enabled {
            self.put_assert_non_null(
                support,
                lhs.0,
                c"lhs passed to runtime_is_strictly_equal() must be non-null",
            );
            self.put_assert_non_null(
                support,
                rhs.0,
                c"rhs passed to runtime_is_strictly_equal() must be non-null",
            );
        }
        let func = self
            .runtime_func_cache
            .import_runtime_is_strictly_equal(support, self.builder.func);
        let args = [self.runtime(), lhs.0, rhs.0];
        let call = self.builder.ins().call(func, &args);
        BooleanIr(self.builder.inst_results(call)[0])
    }

    pub fn put_runtime_typeof(
        &mut self,
        support: &mut impl EditorSupport,
        value: AnyIr,
    ) -> StringIr {
        logger::debug!(event = "put_runtime_typeof", ?value);
        if self.runtime_assert_enabled {
            self.put_assert_non_null(
                support,
                value.0,
                c"value passed to runtime_typeof() must be non-null",
            );
        }
        let func = self
            .runtime_func_cache
            .import_runtime_get_typeof(support, self.builder.func);
        let args = [self.runtime(), value.0];
        let call = self.builder.ins().call(func, &args);
        StringIr(self.builder.inst_results(call)[0])
    }

    pub fn put_runtime_migrate_string_to_heap(
        &mut self,
        support: &mut impl EditorSupport,
        string: StringIr,
    ) -> StringIr {
        logger::debug!(event = "put_runtime_migrate_string_to_heap", ?string);
        if self.runtime_assert_enabled {
            self.put_assert_non_null(
                support,
                string.0,
                c"string passed to runtime_migrate_string_to_heap() must be non-null",
            );
        }
        let func = self
            .runtime_func_cache
            .import_runtime_migrate_string_to_heap(support, self.builder.func);
        let args = [self.runtime(), string.0];
        let call = self.builder.ins().call(func, &args);
        StringIr(self.builder.inst_results(call)[0])
    }

    pub fn put_runtime_create_capture(
        &mut self,
        support: &mut impl EditorSupport,
        target: AnyIr,
    ) -> CaptureIr {
        logger::debug!(event = "put_runtime_create_capture", ?target);
        let func = self
            .runtime_func_cache
            .import_runtime_create_capture(support, self.builder.func);
        let args = [self.runtime(), target.0];
        let call = self.builder.ins().call(func, &args);
        CaptureIr(self.builder.inst_results(call)[0])
    }

    pub fn put_runtime_create_closure(
        &mut self,
        support: &mut impl EditorSupport,
        lambda: LambdaIr,
        lambda_id: LambdaId,
        num_captures: u16,
    ) -> ClosureIr {
        logger::debug!(
            event = "put_runtime_create_closure",
            ?lambda,
            ?lambda_id,
            num_captures
        );
        let func = self
            .runtime_func_cache
            .import_runtime_create_closure(support, self.builder.func);
        let lambda_id: u32 = lambda_id.into();
        let lambda_id = self.builder.ins().iconst(ir::types::I32, lambda_id as i64);
        let num_captures = self
            .builder
            .ins()
            .iconst(ir::types::I16, num_captures as i64);
        let args = [self.runtime(), lambda.0, lambda_id, num_captures];
        let call = self.builder.ins().call(func, &args);
        ClosureIr(self.builder.inst_results(call)[0])
    }

    pub fn put_runtime_create_coroutine(
        &mut self,
        support: &mut impl EditorSupport,
        closure: ClosureIr,
        num_locals: u16,
        scratch_buffer_len: u16,
        capture_buffer_len: u16,
    ) -> CoroutineIr {
        logger::debug!(
            event = "put_runtime_create_coroutine",
            ?closure,
            num_locals,
            scratch_buffer_len,
            capture_buffer_len,
        );
        let func = self
            .runtime_func_cache
            .import_runtime_create_coroutine(support, self.builder.func);
        let num_locals = self.builder.ins().iconst(ir::types::I16, num_locals as i64);
        let scratch_buffer_len = self
            .builder
            .ins()
            .iconst(ir::types::I16, scratch_buffer_len as i64);
        let capture_buffer_len = self
            .builder
            .ins()
            .iconst(ir::types::I16, capture_buffer_len as i64);
        let args = [
            self.runtime(),
            closure.0,
            num_locals,
            scratch_buffer_len,
            capture_buffer_len,
        ];
        let call = self.builder.ins().call(func, &args);
        CoroutineIr(self.builder.inst_results(call)[0])
    }

    pub fn put_runtime_register_promise(
        &mut self,
        support: &mut impl EditorSupport,
        coroutine: CoroutineIr,
    ) -> PromiseIr {
        logger::debug!(event = "put_runtime_register_promise", ?coroutine);
        let func = self
            .runtime_func_cache
            .import_runtime_register_promise(support, self.builder.func);
        let args = [self.runtime(), coroutine.0];
        let call = self.builder.ins().call(func, &args);
        PromiseIr(self.builder.inst_results(call)[0])
    }

    pub fn put_runtime_await_promise(
        &mut self,
        support: &mut impl EditorSupport,
        promise: PromiseIr,
        awaiting: PromiseIr,
    ) {
        logger::debug!(event = "put_runtime_await_promise", ?promise, ?awaiting);
        let func = self
            .runtime_func_cache
            .import_runtime_await_promise(support, self.builder.func);
        let args = [self.runtime(), promise.0, awaiting.0];
        self.builder.ins().call(func, &args);
    }

    pub fn put_runtime_resume(&mut self, support: &mut impl EditorSupport, promise: PromiseIr) {
        logger::debug!(event = "put_runtime_resume", ?promise);
        let func = self
            .runtime_func_cache
            .import_runtime_resume(support, self.builder.func);
        let args = [self.runtime(), promise.0];
        self.builder.ins().call(func, &args);
    }

    pub fn put_runtime_emit_promise_resolved(
        &mut self,
        support: &mut impl EditorSupport,
        promise: PromiseIr,
        result: AnyIr,
    ) {
        logger::debug!(
            event = "put_runtime_emit_promise_resolved",
            ?promise,
            ?result
        );
        if self.runtime_assert_enabled {
            self.put_assert_non_null(
                support,
                result.0,
                c"result passed to runtime_emit_promise_resolved() must be non-null",
            );
        }
        let func = self
            .runtime_func_cache
            .import_runtime_emit_promise_resolved(support, self.builder.func);
        let args = [self.runtime(), promise.0, result.0];
        self.builder.ins().call(func, &args);
    }

    pub fn put_runtime_create_object(
        &mut self,
        support: &mut impl EditorSupport,
        prototype: ObjectIr,
    ) -> ObjectIr {
        logger::debug!(event = "put_runtime_create_object", ?prototype);
        let func = self
            .runtime_func_cache
            .import_runtime_create_object(support, self.builder.func);
        let args = [self.runtime(), prototype.0];
        let call = self.builder.ins().call(func, &args);
        ObjectIr(self.builder.inst_results(call)[0])
    }

    pub fn put_runtime_create_reference_error(
        &mut self,
        support: &mut impl EditorSupport,
    ) -> ObjectIr {
        logger::debug!(event = "put_runtime_create_reference_error");
        let func = self
            .runtime_func_cache
            .import_runtime_create_reference_error(support, self.builder.func);
        let args = [self.runtime()];
        let call = self.builder.ins().call(func, &args);
        ObjectIr(self.builder.inst_results(call)[0])
    }

    pub fn put_runtime_create_type_error(&mut self, support: &mut impl EditorSupport) -> ObjectIr {
        logger::debug!(event = "put_runtime_create_type_error");
        let func = self
            .runtime_func_cache
            .import_runtime_create_type_error(support, self.builder.func);
        let args = [self.runtime()];
        let call = self.builder.ins().call(func, &args);
        ObjectIr(self.builder.inst_results(call)[0])
    }

    pub fn put_runtime_create_internal_error(
        &mut self,
        support: &mut impl EditorSupport,
        message: StringHandle,
    ) -> ObjectIr {
        logger::debug!(event = "put_runtime_create_internal_error", ?message);
        debug_assert!(message.is_const());
        let func = self
            .runtime_func_cache
            .import_runtime_create_internal_error(support, self.builder.func);
        let message = self
            .builder
            .ins()
            .iconst(self.addr_type, message.as_addr() as i64);
        let args = [self.runtime(), message];
        let call = self.builder.ins().call(func, &args);
        ObjectIr(self.builder.inst_results(call)[0])
    }

    pub fn put_runtime_get_value_by_symbol(
        &mut self,
        support: &mut impl EditorSupport,
        object: ObjectIr,
        key: Symbol,
        strict: bool,
        retv: AnyIr,
    ) -> StatusIr {
        logger::debug!(
            event = "put_runtime_get_value_by_symbol",
            ?object,
            ?key,
            strict,
            ?retv,
        );
        if self.runtime_assert_enabled {
            self.put_assert_non_null(
                support,
                object.0,
                c"object passed to runtime_get_value_by_symbol() must be non-null",
            );
        }
        let func = self
            .runtime_func_cache
            .import_runtime_get_value_by_symbol(support, self.builder.func);
        let key = self.builder.ins().iconst(ir::types::I32, key.id() as i64);
        let strict = self.put_boolean(strict);
        let args = [self.runtime(), object.0, key, strict.0, retv.0];
        let call = self.builder.ins().call(func, &args);
        StatusIr(self.builder.inst_results(call)[0])
    }

    pub fn put_runtime_get_value_by_number(
        &mut self,
        support: &mut impl EditorSupport,
        object: ObjectIr,
        key: f64,
        strict: bool,
        retv: AnyIr,
    ) -> StatusIr {
        logger::debug!(
            event = "put_runtime_get_value_by_number",
            ?object,
            key,
            strict,
            ?retv,
        );
        if self.runtime_assert_enabled {
            self.put_assert_non_null(
                support,
                object.0,
                c"object passed to runtime_get_value_by_number() must be non-null",
            );
        }
        let func = self
            .runtime_func_cache
            .import_runtime_get_value_by_number(support, self.builder.func);
        let key = self.put_number(key);
        let strict = self.put_boolean(strict);
        let args = [self.runtime(), object.0, key.0, strict.0, retv.0];
        let call = self.builder.ins().call(func, &args);
        StatusIr(self.builder.inst_results(call)[0])
    }

    pub fn put_runtime_get_value_by_any(
        &mut self,
        support: &mut impl EditorSupport,
        object: ObjectIr,
        key: AnyIr,
        strict: bool,
        retv: AnyIr,
    ) -> StatusIr {
        logger::debug!(
            event = "put_runtime_get_value_by_any",
            ?object,
            ?key,
            strict,
            ?retv,
        );
        if self.runtime_assert_enabled {
            self.put_assert_non_null(
                support,
                object.0,
                c"object passed to runtime_get_value_by_any() must be non-null",
            );
            self.put_assert_non_null(
                support,
                key.0,
                c"key passed to runtime_get_value_by_any() must be non-null",
            );
        }
        let func = self
            .runtime_func_cache
            .import_runtime_get_value_by_value(support, self.builder.func);
        let strict = self.put_boolean(strict);
        let args = [self.runtime(), object.0, key.0, strict.0, retv.0];
        let call = self.builder.ins().call(func, &args);
        StatusIr(self.builder.inst_results(call)[0])
    }

    pub fn put_runtime_set_value_by_symbol(
        &mut self,
        support: &mut impl EditorSupport,
        object: ObjectIr,
        key: Symbol,
        value: AnyIr,
        retv: AnyIr,
    ) -> StatusIr {
        logger::debug!(
            event = "put_runtime_set_value_by_symbol",
            ?object,
            ?key,
            ?value,
            ?retv,
        );
        if self.runtime_assert_enabled {
            self.put_assert_non_null(
                support,
                object.0,
                c"object passed to runtime_set_value_by_symbol() must be non-null",
            );
            self.put_assert_non_null(
                support,
                value.0,
                c"value passed to runtime_set_value_by_symbol() must be non-null",
            );
        }
        let func = self
            .runtime_func_cache
            .import_runtime_set_value_by_symbol(support, self.builder.func);
        let key = self.builder.ins().iconst(ir::types::I32, key.id() as i64);
        let args = [self.runtime(), object.0, key, value.0, retv.0];
        let call = self.builder.ins().call(func, &args);
        StatusIr(self.builder.inst_results(call)[0])
    }

    pub fn put_runtime_set_value_by_number(
        &mut self,
        support: &mut impl EditorSupport,
        object: ObjectIr,
        key: f64,
        value: AnyIr,
        retv: AnyIr,
    ) -> StatusIr {
        logger::debug!(
            event = "put_runtime_set_value_by_number",
            ?object,
            key,
            ?value,
            ?retv,
        );
        if self.runtime_assert_enabled {
            self.put_assert_non_null(
                support,
                object.0,
                c"object passed to runtime_set_value_by_number() must be non-null",
            );
            self.put_assert_non_null(
                support,
                value.0,
                c"value passed to runtime_set_value_by_number() must be non-null",
            );
        }
        let func = self
            .runtime_func_cache
            .import_runtime_set_value_by_number(support, self.builder.func);
        let key = self.builder.ins().f64const(key);
        let args = [self.runtime(), object.0, key, value.0, retv.0];
        let call = self.builder.ins().call(func, &args);
        StatusIr(self.builder.inst_results(call)[0])
    }

    pub fn put_runtime_set_value_by_any(
        &mut self,
        support: &mut impl EditorSupport,
        object: ObjectIr,
        key: AnyIr,
        value: AnyIr,
        retv: AnyIr,
    ) -> StatusIr {
        logger::debug!(
            event = "put_runtime_set_value_by_any",
            ?object,
            ?key,
            ?value,
            ?retv,
        );
        if self.runtime_assert_enabled {
            self.put_assert_non_null(
                support,
                object.0,
                c"object passed to runtime_set_value_by_any() must be non-null",
            );
            self.put_assert_non_null(
                support,
                key.0,
                c"key passed to runtime_set_value_by_any() must be non-null",
            );
            self.put_assert_non_null(
                support,
                value.0,
                c"value passed to runtime_set_value_by_any() must be non-null",
            );
        }
        let func = self
            .runtime_func_cache
            .import_runtime_set_value_by_value(support, self.builder.func);
        let args = [self.runtime(), object.0, key.0, value.0, retv.0];
        let call = self.builder.ins().call(func, &args);
        StatusIr(self.builder.inst_results(call)[0])
    }

    pub fn put_runtime_concat_strings(
        &mut self,
        support: &mut impl EditorSupport,
        head: StringIr,
        tail: StringIr,
    ) -> StringIr {
        logger::debug!(event = "put_runtime_concat_strings", ?head, ?tail);
        if self.runtime_assert_enabled {
            self.put_assert_non_null(
                support,
                head.0,
                c"head passed to runtime_concat_strings() must be non-null",
            );
            self.put_assert_non_null(
                support,
                tail.0,
                c"tail passed to runtime_concat_strings() must be non-null",
            );
        }
        let func = self
            .runtime_func_cache
            .import_runtime_concat_strings(support, self.builder.func);
        let args = [self.runtime(), head.0, tail.0];
        let call = self.builder.ins().call(func, &args);
        StringIr(self.builder.inst_results(call)[0])
    }

    pub fn put_runtime_create_data_property_by_symbol(
        &mut self,
        support: &mut impl EditorSupport,
        object: ObjectIr,
        key: Symbol,
        value: AnyIr,
        retv: AnyIr,
    ) -> StatusIr {
        logger::debug!(
            event = "put_runtime_create_data_property_by_symbol",
            ?object,
            ?key,
            ?value,
            ?retv
        );
        if self.runtime_assert_enabled {
            self.put_assert_non_null(
                support,
                object.0,
                c"object passed to runtime_create_data_property_by_symbol() must be non-null",
            );
            self.put_assert_non_null(
                support,
                value.0,
                c"value passed to runtime_create_data_property_by_symbol() must be non-null",
            );
            self.put_assert_non_null(
                support,
                retv.0,
                c"retv passed to runtime_create_data_property_by_symbol() must be non-null",
            );
        }
        let func = self
            .runtime_func_cache
            .import_runtime_create_data_property_by_symbol(support, self.builder.func);
        let key = self.builder.ins().iconst(ir::types::I32, key.id() as i64);
        let args = [self.runtime(), object.0, key, value.0, retv.0];
        let call = self.builder.ins().call(func, &args);
        StatusIr(self.builder.inst_results(call)[0])
    }

    pub fn put_runtime_create_data_property_by_number(
        &mut self,
        support: &mut impl EditorSupport,
        object: ObjectIr,
        key: f64,
        value: AnyIr,
        retv: AnyIr,
    ) -> StatusIr {
        logger::debug!(
            event = "put_runtime_create_data_property_by_number",
            ?object,
            key,
            ?value,
            ?retv
        );
        if self.runtime_assert_enabled {
            self.put_assert_non_null(
                support,
                object.0,
                c"object passed to runtime_create_data_property_by_number() must be non-null",
            );
            self.put_assert_non_null(
                support,
                value.0,
                c"value passed to runtime_create_data_property_by_number() must be non-null",
            );
            self.put_assert_non_null(
                support,
                retv.0,
                c"retv passed to runtime_create_data_property_by_number() must be non-null",
            );
        }
        let func = self
            .runtime_func_cache
            .import_runtime_create_data_property_by_number(support, self.builder.func);
        let key = self.put_number(key);
        let args = [self.runtime(), object.0, key.0, value.0, retv.0];
        let call = self.builder.ins().call(func, &args);
        StatusIr(self.builder.inst_results(call)[0])
    }

    pub fn put_runtime_create_data_property_by_any(
        &mut self,
        support: &mut impl EditorSupport,
        object: ObjectIr,
        key: AnyIr,
        value: AnyIr,
        retv: AnyIr,
    ) -> StatusIr {
        logger::debug!(
            event = "put_runtime_create_data_property_by_any",
            ?object,
            ?key,
            ?value,
            ?retv
        );
        if self.runtime_assert_enabled {
            self.put_assert_non_null(
                support,
                object.0,
                c"object passed to runtime_create_data_property_by_any() must be non-null",
            );
            self.put_assert_non_null(
                support,
                key.0,
                c"key passed to runtime_create_data_property_by_any() must be non-null",
            );
            self.put_assert_non_null(
                support,
                value.0,
                c"value passed to runtime_create_data_property_by_any() must be non-null",
            );
            self.put_assert_non_null(
                support,
                retv.0,
                c"retv passed to runtime_create_data_property_by_any() must be non-null",
            );
        }
        let func = self
            .runtime_func_cache
            .import_runtime_create_data_property_by_value(support, self.builder.func);
        let args = [self.runtime(), object.0, key.0, value.0, retv.0];
        let call = self.builder.ins().call(func, &args);
        StatusIr(self.builder.inst_results(call)[0])
    }

    pub fn put_runtime_copy_data_properties(
        &mut self,
        support: &mut impl EditorSupport,
        target: ObjectIr,
        source: AnyIr,
        retv: AnyIr,
    ) -> StatusIr {
        logger::debug!(
            event = "put_runtime_copy_data_properties",
            ?target,
            ?source,
            ?retv
        );
        if self.runtime_assert_enabled {
            self.put_assert_non_null(
                support,
                target.0,
                c"target passed to runtime_copy_data_properties() must be non-null",
            );
            self.put_assert_non_null(
                support,
                source.0,
                c"source passed to runtime_copy_data_properties() must be non-null",
            );
            self.put_assert_non_null(
                support,
                retv.0,
                c"retv passed to runtime_copy_data_properties() must be non-null",
            );
        }
        let func = self
            .runtime_func_cache
            .import_runtime_copy_data_properties(support, self.builder.func);
        let args = [self.runtime(), target.0, source.0, retv.0];
        let call = self.builder.ins().call(func, &args);
        StatusIr(self.builder.inst_results(call)[0])
    }

    pub fn put_runtime_push_array_element(
        &mut self,
        support: &mut impl EditorSupport,
        target: ObjectIr,
        value: AnyIr,
        retv: AnyIr,
    ) -> StatusIr {
        logger::debug!(
            event = "put_runtime_push_array_element",
            ?target,
            ?value,
            ?retv
        );
        if self.runtime_assert_enabled {
            self.put_assert_non_null(
                support,
                target.0,
                c"target passed to runtime_push_array_element() must be non-null",
            );
            self.put_assert_non_null(
                support,
                value.0,
                c"source passed to runtime_push_array_element() must be non-null",
            );
            self.put_assert_non_null(
                support,
                retv.0,
                c"retv passed to runtime_push_array_element() must be non-null",
            );
        }
        let func = self
            .runtime_func_cache
            .import_runtime_push_value(support, self.builder.func);
        let args = [self.runtime(), target.0, value.0, retv.0];
        let call = self.builder.ins().call(func, &args);
        StatusIr(self.builder.inst_results(call)[0])
    }

    pub fn put_runtime_panic(&mut self, support: &mut impl EditorSupport, msg: &'static CStr) {
        logger::debug!(event = "put_runtime_panic", ?msg);
        let func = self
            .runtime_func_cache
            .import_runtime_panic(support, self.builder.func);
        let msg = self
            .builder
            .ins()
            .iconst(self.addr_type, msg.as_ptr() as i64);
        let args = [self.runtime(), msg];
        self.builder.ins().call(func, &args);
    }

    #[allow(unused)]
    pub fn put_runtime_print_boolean(
        &mut self,
        support: &mut impl EditorSupport,
        value: BooleanIr,
        msg: &'static CStr,
    ) {
        logger::debug!(event = "put_runtime_print_boolean", ?value);
        let func = self
            .runtime_func_cache
            .import_runtime_print_bool(support, self.builder.func);
        let msg = self
            .builder
            .ins()
            .iconst(self.addr_type, msg.as_ptr() as i64);
        let args = [self.runtime(), value.0, msg];
        self.builder.ins().call(func, &args);
    }

    #[allow(unused)]
    pub fn put_runtime_print_number(
        &mut self,
        support: &mut impl EditorSupport,
        value: NumberIr,
        msg: &'static CStr,
    ) {
        logger::debug!(event = "put_runtime_print_number", ?value);
        let func = self
            .runtime_func_cache
            .import_runtime_print_f64(support, self.builder.func);
        let msg = self
            .builder
            .ins()
            .iconst(self.addr_type, msg.as_ptr() as i64);
        let args = [self.runtime(), value.0, msg];
        self.builder.ins().call(func, &args);
    }

    #[allow(unused)]
    pub fn put_runtime_print_any(
        &mut self,
        support: &mut impl EditorSupport,
        value: AnyIr,
        msg: &'static CStr,
    ) {
        logger::debug!(event = "put_runtime_print_any", ?value, ?msg);
        if self.runtime_assert_enabled {
            self.put_assert_non_null(
                support,
                value.0,
                c"value passed to runtime_print_any() must be non-null",
            );
        }
        let func = self
            .runtime_func_cache
            .import_runtime_print_value(support, self.builder.func);
        let msg = self
            .builder
            .ins()
            .iconst(self.addr_type, msg.as_ptr() as i64);
        let args = [self.runtime(), value.0, msg];
        self.builder.ins().call(func, &args);
    }

    #[allow(unused)]
    pub fn put_runtime_print_capture(
        &mut self,
        support: &mut impl EditorSupport,
        capture: CaptureIr,
        msg: &'static CStr,
    ) {
        logger::debug!(event = "put_runtime_print_capture", ?capture, ?msg);
        if self.runtime_assert_enabled {
            self.put_assert_non_null(
                support,
                capture.0,
                c"capture passed to runtime_print_capture() must be non-null",
            );
        }
        let func = self
            .runtime_func_cache
            .import_runtime_print_capture(support, self.builder.func);
        let msg = self
            .builder
            .ins()
            .iconst(self.addr_type, msg.as_ptr() as i64);
        let args = [self.runtime(), capture.0, msg];
        self.builder.ins().call(func, &args);
    }

    #[allow(unused)]
    pub fn put_runtime_print_message(
        &mut self,
        support: &mut impl EditorSupport,
        msg: &'static CStr,
    ) {
        logger::debug!(event = "put_runtime_print_message", ?msg);
        let func = self
            .runtime_func_cache
            .import_runtime_print_message(support, self.builder.func);
        let msg = self
            .builder
            .ins()
            .iconst(self.addr_type, msg.as_ptr() as i64);
        let args = [self.runtime(), msg];
        self.builder.ins().call(func, &args);
    }

    pub fn put_runtime_launch_debugger(&mut self, support: &mut impl EditorSupport) {
        logger::debug!(event = "put_runtime_launch_debugger");
        let func = self
            .runtime_func_cache
            .import_runtime_launch_debugger(support, self.builder.func);
        let args = [self.runtime()];
        self.builder.ins().call(func, &args);
    }

    // scope cleanup checker

    pub fn put_init_scope_cleanup_checker(&mut self) {
        logger::debug!(event = "put_init_scope_cleanup_cheker");
        if !self.coroutine_mode {
            let zero = self.builder.ins().iconst(ir::types::I16, 0);
            self.builder.ins().stack_store(zero, self.fcs, 8);
        }
    }

    pub fn put_store_scope_id_for_checker(&mut self, scope_ref: ScopeRef) {
        logger::debug!(event = "put_store_scope_id_for_checker", ?scope_ref);
        let scope_id = self
            .builder
            .ins()
            .iconst(ir::types::I16, scope_ref.id() as i64);
        if self.coroutine_mode {
            let coroutine = self.coroutine();
            self.put_store(scope_id, coroutine.0, Coroutine::SCOPE_ID_OFFSET);
        } else {
            self.builder.ins().stack_store(scope_id, self.fcs, 8);
        }
    }

    pub fn put_assert_scope_id(&mut self, support: &mut impl EditorSupport, expected: ScopeRef) {
        logger::debug!(event = "put_assert_scope_id", ?expected);
        use ir::condcodes::IntCC::Equal;
        let scope_id = if self.coroutine_mode {
            let coroutine = self.coroutine();
            self.put_load_i16(coroutine.0, Coroutine::SCOPE_ID_OFFSET)
        } else {
            self.builder.ins().stack_load(ir::types::I16, self.fcs, 8)
        };
        let assertion = self
            .builder
            .ins()
            .icmp_imm(Equal, scope_id, expected.id() as i64);
        self.put_assert(support, BooleanIr(assertion), c"invalid scope");
    }
}

#[derive(Debug)]
pub struct ScratchBuffer {
    addr: ir::Value,
    pub offset: usize,
}
