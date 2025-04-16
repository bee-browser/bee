use std::ffi::CStr;

use cranelift::codegen::ir;
use cranelift::codegen::ir::InstBuilder as _;
use cranelift::codegen::isa;
use cranelift::frontend::FunctionBuilder;
use cranelift::frontend::Switch;
use cranelift_module::FuncId;
use rustc_hash::FxHashMap;

use base::static_assert_eq;

use crate::logger;
use crate::types;

use super::AnyIr;
use super::ArgvIr;
use super::BooleanIr;
use super::CaptureIr;
use super::ClosureIr;
use super::CoroutineIr;
use super::FlowSelector;
use super::LambdaId;
use super::LambdaIr;
use super::NumberIr;
use super::ObjectIr;
use super::PromiseIr;
use super::RuntimeFunctionCache;
use super::RuntimeFunctionIds;
use super::ScopeRef;
use super::Status;
use super::StatusIr;
use super::StringIr;
use super::Symbol;

pub struct Editor<'a> {
    builder: FunctionBuilder<'a>,

    /// A map from a LambdaId to a corresponding FuncId.
    id_map: &'a FxHashMap<LambdaId, FuncId>,

    runtime_func_cache: RuntimeFunctionCache<'a>,
    lambda_cache: FxHashMap<FuncId, LambdaIr>,

    addr_type: ir::Type,
    lambda_sig: ir::SigRef,
    entry_block: ir::Block,
    captures: ir::Value,
    fcs: ir::StackSlot,

    // FunctionBuilder::is_filled() is a private method.
    block_terminated: bool,

    /// Enabled if the function is a coroutine.
    coroutine_mode: bool,
}

impl<'a> Editor<'a> {
    pub fn new(
        mut builder: FunctionBuilder<'a>,
        target_config: isa::TargetFrontendConfig,
        id_map: &'a FxHashMap<LambdaId, FuncId>,
        runtime_func_ids: &'a RuntimeFunctionIds,
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

        let captures = builder.block_params(entry_block)[1];

        let fcs = builder.create_sized_stack_slot(ir::StackSlotData {
            kind: ir::StackSlotKind::ExplicitSlot,
            size: 10, // [status, flow_selector, scope_id]
            align_shift: 2,
        });

        Self {
            builder,
            id_map,
            runtime_func_cache: RuntimeFunctionCache::new(runtime_func_ids, target_config),
            lambda_cache: Default::default(),
            addr_type: target_config.pointer_type(),
            lambda_sig,
            entry_block,
            captures,
            fcs,
            block_terminated: false,
            coroutine_mode: false,
        }
    }

    pub fn put_declare_lambda(&mut self, lambda_id: LambdaId) -> LambdaIr {
        logger::debug!(event = "put_declare_lambda", ?lambda_id);
        let func_id = *self.id_map.get(&lambda_id).unwrap();
        *self
            .lambda_cache
            .entry(func_id)
            .or_insert_with_key(|&func_id| {
                // The following implementation is based on
                // cranelift_module::Module::declare_func_in_func().
                let name = ir::UserExternalName::new(0, func_id.as_u32());
                let name_ref = self.builder.func.declare_imported_user_function(name);
                let func_ref = self.builder.func.import_function(ir::ExtFuncData {
                    name: ir::ExternalName::user(name_ref),
                    signature: self.lambda_sig,
                    colocated: true, // Linkage::Local
                });
                LambdaIr(self.builder.ins().func_addr(self.addr_type, func_ref))
            })
    }

    pub fn end(mut self) {
        self.builder.seal_all_blocks();
        self.builder.finalize();
    }

    // function parameters

    fn runtime(&self) -> ir::Value {
        self.lambda_params(0)
    }

    fn coroutine(&self) -> CoroutineIr {
        CoroutineIr(self.lambda_params(1))
    }

    fn argc(&self) -> ir::Value {
        self.lambda_params(2)
    }

    fn argv(&self) -> ArgvIr {
        ArgvIr(self.lambda_params(3))
    }

    pub fn retv(&self) -> AnyIr {
        AnyIr(self.lambda_params(4))
    }

    pub fn exception(&self) -> AnyIr {
        AnyIr(self.lambda_params(4))
    }

    fn lambda_params(&self, index: usize) -> ir::Value {
        self.builder.block_params(self.entry_block)[index]
    }

    // arguments

    pub fn put_get_argument(&mut self, index: u16) -> AnyIr {
        logger::debug!(event = "put_get_argument", ?index);
        // TODO: bounds checking
        let _argc = self.argc();
        let argv = self.argv();
        let offset = types::Value::SIZE * index as usize;
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
        let status = self.builder.ins().iconst(ir::types::I32, status.0 as i64);
        self.builder.ins().stack_store(status, self.fcs, 0);
    }

    pub fn put_is_exception_status(&mut self, status: StatusIr) -> BooleanIr {
        logger::debug!(event = "put_is_exception_status", ?status);
        use ir::condcodes::IntCC::Equal;
        BooleanIr(
            self.builder
                .ins()
                .icmp_imm(Equal, status.0, Status::EXCEPTION.0 as i64),
        )
    }

    // function control set | flow_selector

    pub fn put_store_flow_selector(&mut self, fs: FlowSelector) {
        logger::debug!(event = "put_store_flow_selector", ?fs);
        let value = self.builder.ins().iconst(ir::types::I32, fs.0 as i64);
        self.builder.ins().stack_store(value, self.fcs, 4);
    }

    pub fn put_is_flow_selector_normal(&mut self) -> BooleanIr {
        logger::debug!(event = "pub_is_flow_selector_normal");
        use ir::condcodes::IntCC::Equal;
        let fs = self.builder.ins().stack_load(ir::types::I32, self.fcs, 4);
        BooleanIr(
            self.builder
                .ins()
                .icmp_imm(Equal, fs, FlowSelector::NORMAL.0 as i64),
        )
    }

    pub fn put_is_flow_selector_normal_or_continue(&mut self, depth: u32) -> BooleanIr {
        logger::debug!(event = "pub_is_flow_selector_normal_or_continue");
        let fs = self.builder.ins().stack_load(ir::types::I32, self.fcs, 4);
        use ir::condcodes::IntCC::UnsignedGreaterThan;
        BooleanIr(self.builder.ins().icmp_imm(
            UnsignedGreaterThan,
            fs,
            FlowSelector::break_at(depth).0 as i64,
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
        then_args: &[ir::Value],
        else_block: ir::Block,
        else_args: &[ir::Value],
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
    pub fn put_jump(&mut self, block: ir::Block, args: &[ir::Value]) {
        logger::debug!(event = "put_jump", ?block, ?args);
        debug_assert!(!self.block_terminated);
        self.builder.ins().jump(block, args);
        self.block_terminated = true;
    }

    pub fn put_jump_if_not_terminated(&mut self, block: ir::Block, args: &[ir::Value]) {
        if self.block_terminated {
            // We should not append any instructions after a terminator instruction.
        } else {
            self.put_jump(block, args);
        }
    }

    pub fn put_switch(&mut self, switch: Switch, state: ir::Value, block: ir::Block) {
        logger::debug!(event = "put_switch", ?switch, ?state, ?block);
        switch.emit(&mut self.builder, state, block);
    }

    pub fn put_return(&mut self) {
        logger::debug!(event = "put_return");
        debug_assert!(!self.block_terminated);
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
            .iconst(ir::types::I32, Status::SUSPEND.0 as i64);
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

    pub fn put_create_string(&mut self, value: &[u16]) -> StringIr {
        logger::debug!(event = "put_create_string", ?value);

        let slot = self.builder.create_sized_stack_slot(ir::StackSlotData {
            kind: ir::StackSlotKind::ExplicitSlot,
            size: types::Char16Seq::SIZE as u32,
            align_shift: types::Char16Seq::ALIGNMENT.ilog2() as u8,
        });

        let next = self.builder.ins().iconst(self.addr_type, 0);
        self.put_store_to_slot(next, slot, types::Char16Seq::NEXT_OFFSET);

        let ptr = self
            .builder
            .ins()
            .iconst(self.addr_type, value.as_ptr() as i64);
        self.put_store_to_slot(ptr, slot, types::Char16Seq::PTR_OFFSET);

        debug_assert!(value.len() <= u32::MAX as usize);
        let len = self
            .builder
            .ins()
            .iconst(ir::types::I32, value.len() as i64);
        self.put_store_to_slot(len, slot, types::Char16Seq::LEN_OFFSET);

        let kind = self
            .builder
            .ins()
            .iconst(ir::types::I8, types::Char16SeqKind::Stack as i64);
        self.put_store_to_slot(kind, slot, types::Char16Seq::KIND_OFFSET);

        StringIr(self.builder.ins().stack_addr(self.addr_type, slot, 0))
    }

    pub fn put_string_on_stack(&mut self, string: StringIr) -> BooleanIr {
        logger::debug!(event = "put_string_on_stack", ?string);
        use ir::condcodes::IntCC::Equal;
        let kind = self.put_load_kind_from_string(string);
        BooleanIr(self.builder.ins().icmp_imm(
            Equal,
            kind,
            crate::types::Char16SeqKind::Stack as i64,
        ))
    }

    fn put_load_kind_from_string(&mut self, string: StringIr) -> ir::Value {
        self.put_load_i8(string.0, types::Char16Seq::KIND_OFFSET)
    }

    // any

    pub fn put_alloc_any(&mut self) -> AnyIr {
        logger::debug!(event = "put_alloc_any");
        let slot = self.builder.create_sized_stack_slot(ir::StackSlotData {
            kind: ir::StackSlotKind::ExplicitSlot,
            size: types::Value::SIZE as u32,
            align_shift: types::Value::ALIGNMENT.ilog2() as u8,
        });
        AnyIr(self.builder.ins().stack_addr(self.addr_type, slot, 0))
    }

    pub fn put_load_kind(&mut self, any: AnyIr) -> ir::Value {
        self.put_load_i8(any.0, types::Value::KIND_OFFSET)
    }

    pub fn put_has_value(&mut self, any: AnyIr) -> BooleanIr {
        logger::debug!(event = "put_has_value", ?any);
        use ir::condcodes::IntCC::NotEqual;
        let kind = self.put_load_i8(any.0, types::Value::KIND_OFFSET);
        // TODO(refactor): Value::KIND_NONE
        BooleanIr(self.builder.ins().icmp_imm(NotEqual, kind, 0))
    }

    pub fn put_load_boolean(&mut self, any: AnyIr) -> BooleanIr {
        logger::debug!(event = "put_load_boolean", ?any);
        BooleanIr(self.put_load_i8(any.0, types::Value::HOLDER_OFFSET))
    }

    pub fn put_load_number(&mut self, any: AnyIr) -> NumberIr {
        logger::debug!(event = "put_load_number", ?any);
        NumberIr(self.put_load_f64(any.0, types::Value::HOLDER_OFFSET))
    }

    pub fn put_load_closure(&mut self, any: AnyIr) -> ClosureIr {
        logger::debug!(event = "put_load_closure", ?any);
        ClosureIr(self.put_load_addr(any.0, types::Value::HOLDER_OFFSET))
    }

    pub fn put_load_promise(&mut self, any: AnyIr) -> PromiseIr {
        logger::debug!(event = "put_load_promise", ?any);
        PromiseIr(self.put_load_i32(any.0, types::Value::HOLDER_OFFSET))
    }

    pub fn put_load_object(&mut self, any: AnyIr) -> ObjectIr {
        logger::debug!(event = "put_load_closure", ?any);
        ObjectIr(self.put_load_addr(any.0, types::Value::HOLDER_OFFSET))
    }

    // capture

    pub fn put_load_capture(&mut self, index: u16) -> CaptureIr {
        logger::debug!(event = "put_load_capture", index);
        let offset = (self.addr_type.bytes() as usize) * (index as usize);
        CaptureIr(self.put_load_addr(self.captures, offset))
    }

    pub fn put_load_captured_value(&mut self, index: u16) -> AnyIr {
        logger::debug!(event = "put_load_captured_value", index);
        let offset = (self.addr_type.bytes() as usize) * (index as usize);
        let capture = self.put_load_addr(self.captures, offset);
        AnyIr(self.put_load_addr(capture, types::Capture::TARGET_OFFSET))
    }

    pub fn put_escape_value(&mut self, capture: CaptureIr, value: AnyIr) {
        logger::debug!(event = "put_escape_value", ?capture, ?value);
        let escaped = self
            .builder
            .ins()
            .iadd_imm(capture.0, types::Capture::ESCAPED_OFFSET as i64);
        self.put_store(escaped, capture.0, types::Capture::TARGET_OFFSET);
        self.put_copy_i128(value.0, escaped);
    }

    // closure

    pub fn put_load_lambda_from_closure(&mut self, closure: ClosureIr) -> LambdaIr {
        logger::debug!(event = "put_load_lambda_from_closure", ?closure);
        LambdaIr(self.put_load_addr(closure.0, types::Closure::LAMBDA_OFFSET))
    }

    pub fn put_get_captures_from_closure(&mut self, closure: ClosureIr) -> ir::Value {
        logger::debug!(event = "put_get_captures_from_closure", ?closure);
        self.builder
            .ins()
            .iadd_imm(closure.0, types::Closure::CAPTURES_OFFSET as i64)
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
            types::Closure::CAPTURES_OFFSET + (self.addr_type.bytes() as usize) * (index as usize);
        self.put_store(capture.0, closure.0, offset);
    }

    pub fn put_call(
        &mut self,
        closure: ClosureIr,
        argc: u16,
        argv: ArgvIr,
        retv: AnyIr,
    ) -> StatusIr {
        logger::debug!(event = "put_call", ?closure, argc, ?argv, ?retv);
        let lambda = self.put_load_lambda_from_closure(closure);
        let context = self.put_get_captures_from_closure(closure);
        let args = &[
            self.runtime(),
            context,
            self.builder.ins().iconst(ir::types::I16, argc as i64),
            argv.0,
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
        self.captures = self.put_load_captures_from_coroutine();
    }

    pub fn put_load_num_locals_from_coroutine(&mut self) -> ir::Value {
        logger::debug!(event = "put_load_num_locals_from_coroutine");
        let coroutine = self.coroutine();
        self.put_load_i16(coroutine.0, types::Coroutine::NUM_LOCALS_OFFSET)
    }

    pub fn put_load_state_from_coroutine(&mut self) -> ir::Value {
        logger::debug!(event = "put_load_state_from_coroutine");
        let coroutine = self.coroutine();
        self.put_load_i32(coroutine.0, types::Coroutine::STATE_OFFSET)
    }

    pub fn put_load_captures_from_coroutine(&mut self) -> ir::Value {
        logger::debug!(event = "put_load_captures_from_coroutine");
        let closure = self.put_load_closure_from_coroutine();
        self.put_get_captures_from_closure(closure)
    }

    pub fn put_get_local_from_coroutine(&mut self, index: u16) -> AnyIr {
        logger::debug!(event = "put_get_local_from_coroutine", index);
        // TODO: emit assert(index < coroutine.num_locals)
        let coroutine = self.coroutine();
        let offset = types::Coroutine::LOCALS_OFFSET + types::Value::SIZE * (index as usize);
        AnyIr(self.builder.ins().iadd_imm(coroutine.0, offset as i64))
    }

    fn put_load_closure_from_coroutine(&mut self) -> ClosureIr {
        let coroutine = self.coroutine();
        ClosureIr(self.put_load_addr(coroutine.0, types::Coroutine::CLOSURE_OFFSET))
    }

    pub fn put_store_state_to_coroutine(&mut self, state: u32) {
        logger::debug!(event = "put_store_state_to_coroutine", state);
        const FLAGS: ir::MemFlags = ir::MemFlags::new().with_aligned().with_notrap();
        let coroutine = self.coroutine();
        let state = self.builder.ins().iconst(ir::types::I32, state as i64);
        self.builder.ins().store(
            FLAGS,
            state,
            coroutine.0,
            types::Coroutine::STATE_OFFSET as i32,
        );
    }

    // argv

    pub fn put_alloc_argv(&mut self, argc: u16) -> ArgvIr {
        logger::debug!(event = "put_alloc_argv", argc);

        if argc == 0 {
            return ArgvIr(self.put_nullptr());
        }

        let slot = self.builder.create_sized_stack_slot(ir::StackSlotData {
            kind: ir::StackSlotKind::ExplicitSlot,
            size: (types::Value::SIZE as u32) * (argc as u32),
            align_shift: types::Value::ALIGNMENT.ilog2() as u8,
        });

        ArgvIr(self.builder.ins().stack_addr(self.addr_type, slot, 0))
    }

    pub fn put_get_arg(&mut self, argv: ArgvIr, index: u16) -> AnyIr {
        logger::debug!(event = "put_get_arg", ?argv, index);
        let offset = (types::Value::SIZE as i64) * (index as i64);
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
        // TODO: Value::KIND_NONE
        self.put_store_kind_to_any(0, any);
    }

    pub fn put_store_undefined_to_any(&mut self, any: AnyIr) {
        logger::debug!(event = "put_store_undefined_to_any", ?any);
        // TODO: Value::KIND_UNDEFINED
        self.put_store_kind_to_any(1, any);
    }

    pub fn put_store_null_to_any(&mut self, any: AnyIr) {
        logger::debug!(event = "put_store_null_to_any", ?any);
        // TODO: Value::KIND_NULL
        self.put_store_kind_to_any(2, any);
    }

    pub fn put_store_boolean_to_any(&mut self, boolean: BooleanIr, any: AnyIr) {
        logger::debug!(event = "put_store_boolean_to_any", ?boolean, ?any);
        // TODO: Value::KIND_BOOLEAN
        self.put_store_kind_and_value_to_any(3, boolean.0, any);
    }

    pub fn put_store_number_to_any(&mut self, number: NumberIr, any: AnyIr) {
        logger::debug!(event = "put_store_number_to_any", ?number, ?any);
        // TODO: Value::KIND_NUMBER
        self.put_store_kind_and_value_to_any(4, number.0, any);
    }

    pub fn put_store_string_to_any(&mut self, string: StringIr, any: AnyIr) {
        logger::debug!(event = "put_store_string_to_any", ?string, ?any);
        // TODO: Value::KIND_STRING
        self.put_store_kind_and_value_to_any(5, string.0, any);
    }

    pub fn put_store_closure_to_any(&mut self, closure: ClosureIr, any: AnyIr) {
        logger::debug!(event = "put_store_closure_to_any", ?closure, ?any);
        // TODO: Value::KIND_CLOSURE
        self.put_store_kind_and_value_to_any(6, closure.0, any);
    }

    pub fn put_store_promise_to_any(&mut self, promise: PromiseIr, any: AnyIr) {
        logger::debug!(event = "put_store_promise_to_any", ?promise, ?any);
        // TODO: Value::KIND_PROMISE
        self.put_store_kind_and_value_to_any(7, promise.0, any);
    }

    pub fn put_store_object_to_any(&mut self, object: ObjectIr, any: AnyIr) {
        logger::debug!(event = "put_store_object_to_any", ?object, ?any);
        // TODO: Value::KIND_OBJECT
        self.put_store_kind_and_value_to_any(8, object.0, any);
    }

    pub fn put_store_any_to_any(&mut self, src: AnyIr, dst: AnyIr) {
        logger::debug!(event = "put_store_any_to_any", ?src, ?dst);
        // TODO(perf): should use memcpy?
        static_assert_eq!(types::Value::SIZE * 8, 128);
        self.put_copy_i128(src.0, dst.0);
    }

    fn put_store_kind_and_value_to_any(&mut self, kind: u8, value: ir::Value, any: AnyIr) {
        self.put_store_kind_to_any(kind, any);
        self.put_store(value, any.0, types::Value::HOLDER_OFFSET);
    }

    fn put_store_kind_to_any(&mut self, kind: u8, any: AnyIr) {
        let kind = self.builder.ins().iconst(ir::types::I8, kind as i64);
        self.put_store(kind, any.0, types::Value::KIND_OFFSET);
    }

    // copy operations

    fn put_copy_i128(&mut self, src: ir::Value, dst: ir::Value) {
        let opaque = self.put_load_i128(src, 0);
        self.put_store(opaque, dst, 0);
    }

    // unary operators

    pub fn put_negate(&mut self, value: NumberIr) -> NumberIr {
        logger::debug!(event = "put_negate", ?value);
        NumberIr(self.builder.ins().fneg(value.0))
    }

    // 6.1.6.1.2 Number::bitwiseNOT ( x )
    pub fn put_bitwise_not(&mut self, value: NumberIr) -> NumberIr {
        logger::debug!(event = "put_bitwise_not", ?value);
        let int32 = self.put_runtime_to_int32(value);
        let bnot = self.builder.ins().bnot(int32);
        self.put_i32_to_f64(bnot)
    }

    pub fn put_logical_not(&mut self, value: BooleanIr) -> BooleanIr {
        logger::debug!(event = "put_logical_not", ?value);
        BooleanIr(self.builder.ins().bxor_imm(value.0, 1))
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

    pub fn put_rem(&mut self, lhs: NumberIr, rhs: NumberIr) -> NumberIr {
        logger::debug!(event = "put_rem", ?lhs, ?rhs);
        let func = self.runtime_func_cache.get_fmod(self.builder.func);
        let call = self.builder.ins().call(func, &[lhs.0, rhs.0]);
        NumberIr(self.builder.inst_results(call)[0])
    }

    pub fn put_exp(&mut self, lhs: NumberIr, rhs: NumberIr) -> NumberIr {
        logger::debug!(event = "put_exp", ?lhs, ?rhs);
        let func = self.runtime_func_cache.get_pow(self.builder.func);
        let call = self.builder.ins().call(func, &[lhs.0, rhs.0]);
        NumberIr(self.builder.inst_results(call)[0])
    }

    // shift operators

    // 6.1.6.1.9 Number::leftShift ( x, y )
    pub fn put_left_shift(&mut self, x: NumberIr, y: NumberIr) -> NumberIr {
        logger::debug!(event = "put_left_shift", ?x, ?y);
        let lnum = self.put_runtime_to_int32(x);
        let rnum = self.put_runtime_to_uint32(y);
        let shift_count = self.builder.ins().urem_imm(rnum, 32);
        let shifted = self.builder.ins().ishl(lnum, shift_count);
        self.put_i32_to_f64(shifted)
    }

    // 6.1.6.1.10 Number::signedRightShift ( x, y )
    pub fn put_signed_right_shift(&mut self, x: NumberIr, y: NumberIr) -> NumberIr {
        logger::debug!(event = "put_signed_right_shift", ?x, ?y);
        let lnum = self.put_runtime_to_int32(x);
        let rnum = self.put_runtime_to_uint32(y);
        let shift_count = self.builder.ins().urem_imm(rnum, 32);
        let shifted = self.builder.ins().sshr(lnum, shift_count);
        self.put_i32_to_f64(shifted)
    }

    // 6.1.6.1.11 Number::unsignedRightShift ( x, y )
    pub fn put_unsigned_right_shift(&mut self, x: NumberIr, y: NumberIr) -> NumberIr {
        logger::debug!(event = "put_unsigned_right_shift", ?x, ?y);
        let lnum = self.put_runtime_to_uint32(x);
        let rnum = self.put_runtime_to_uint32(y);
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

    pub fn put_is_undefined(&mut self, any: AnyIr) -> BooleanIr {
        logger::debug!(event = "put_is_undefined", ?any);
        // TODO(refactor): Value::KIND_UNDEFINED
        self.put_is_kind_of(1, any)
    }

    pub fn put_is_null(&mut self, any: AnyIr) -> BooleanIr {
        logger::debug!(event = "put_is_null", ?any);
        // TODO(refactor): Value::KIND_NULL
        self.put_is_kind_of(2, any)
    }

    pub fn put_is_boolean(&mut self, any: AnyIr) -> BooleanIr {
        logger::debug!(event = "put_is_boolean", ?any);
        // TODO(refactor): Value::KIND_BOOLEAN
        self.put_is_kind_of(3, any)
    }

    pub fn put_is_number(&mut self, any: AnyIr) -> BooleanIr {
        logger::debug!(event = "put_is_number", ?any);
        // TODO(refactor): Value::KIND_NUMBER
        self.put_is_kind_of(4, any)
    }

    pub fn put_is_closure(&mut self, any: AnyIr) -> BooleanIr {
        logger::debug!(event = "put_is_closure", ?any);
        // TODO(refactor): Value::KIND_CLOSURE
        self.put_is_kind_of(6, any)
    }

    pub fn put_is_promise(&mut self, any: AnyIr) -> BooleanIr {
        logger::debug!(event = "put_is_promise", ?any);
        // TODO(refactor): Value::KIND_PROMISE
        self.put_is_kind_of(7, any)
    }

    pub fn put_is_object(&mut self, any: AnyIr) -> BooleanIr {
        logger::debug!(event = "put_is_object", ?any);
        // TODO(refactor): Value::KIND_OBJECT
        self.put_is_kind_of(8, any)
    }

    pub fn put_is_non_nullish(&mut self, any: AnyIr) -> BooleanIr {
        logger::debug!(event = "put_is_non_nullish", ?any);
        use ir::condcodes::IntCC::UnsignedGreaterThan;
        let kind = self.put_load_kind(any);
        // TODO(refactor): Value::KIND_NULL
        BooleanIr(self.builder.ins().icmp_imm(UnsignedGreaterThan, kind, 2))
    }

    pub fn put_is_nullptr(&mut self, any: AnyIr) -> BooleanIr {
        logger::debug!(event = "put_is_nullptr", ?any);
        use ir::condcodes::IntCC::Equal;
        BooleanIr(self.builder.ins().icmp_imm(Equal, any.0, 0))
    }

    pub fn put_is_same_boolean(&mut self, lhs: BooleanIr, rhs: BooleanIr) -> BooleanIr {
        logger::debug!(event = "put_is_same_boolean", ?lhs, ?rhs);
        self.put_is_same_int_value(lhs.0, rhs.0)
    }

    pub fn put_is_same_number(&mut self, lhs: NumberIr, rhs: NumberIr) -> BooleanIr {
        logger::debug!(event = "put_is_same_number", ?lhs, ?rhs);
        self.put_is_same_float_value(lhs.0, rhs.0)
    }

    pub fn put_is_same_closure(&mut self, lhs: ClosureIr, rhs: ClosureIr) -> BooleanIr {
        logger::debug!(event = "put_is_same_closure", ?lhs, ?rhs);
        self.put_is_same_int_value(lhs.0, rhs.0)
    }

    pub fn put_is_same_promise(&mut self, lhs: PromiseIr, rhs: PromiseIr) -> BooleanIr {
        logger::debug!(event = "put_is_same_promise", ?lhs, ?rhs);
        self.put_is_same_int_value(lhs.0, rhs.0)
    }

    pub fn put_is_same_object(&mut self, lhs: ObjectIr, rhs: ObjectIr) -> BooleanIr {
        logger::debug!(event = "put_is_same_object", ?lhs, ?rhs);
        self.put_is_same_int_value(lhs.0, rhs.0)
    }

    pub fn put_is_kind_of<T>(&mut self, kind_imm: T, any: AnyIr) -> BooleanIr
    where
        T: Into<ir::immediates::Imm64>,
    {
        use ir::condcodes::IntCC::Equal;
        let kind = self.put_load_kind(any);
        BooleanIr(self.builder.ins().icmp_imm(Equal, kind, kind_imm))
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
    pub fn put_bitwise_and(&mut self, x: NumberIr, y: NumberIr) -> NumberIr {
        logger::debug!(event = "put_bitwise_and", ?x, ?y);
        let lnum = self.put_runtime_to_int32(x);
        let rnum = self.put_runtime_to_int32(y);
        let result = self.builder.ins().band(lnum, rnum);
        self.put_i32_to_f64(result)
    }

    // 6.1.6.1.18 Number::bitwiseXOR ( x, y )
    pub fn put_bitwise_xor(&mut self, x: NumberIr, y: NumberIr) -> NumberIr {
        logger::debug!(event = "put_bitwise_xor", ?x, ?y);
        let lnum = self.put_runtime_to_int32(x);
        let rnum = self.put_runtime_to_int32(y);
        let result = self.builder.ins().bxor(lnum, rnum);
        self.put_i32_to_f64(result)
    }

    // 6.1.6.1.19 Number::bitwiseOR ( x, y )
    pub fn put_bitwise_or(&mut self, x: NumberIr, y: NumberIr) -> NumberIr {
        logger::debug!(event = "put_bitwise_or", ?x, ?y);
        let lnum = self.put_runtime_to_int32(x);
        let rnum = self.put_runtime_to_int32(y);
        let result = self.builder.ins().bor(lnum, rnum);
        self.put_i32_to_f64(result)
    }

    // operations on a scratch buffer

    pub fn put_get_scratch_buffer_from_coroutine(&mut self) -> ir::Value {
        logger::debug!(event = "put_get_scratch_buffer_from_coroutine");
        let coroutine = self.coroutine();
        // TODO(perf): compile-time evaluation
        let num_locals = self.put_load_num_locals_from_coroutine();
        let num_locals = self.builder.ins().uextend(self.addr_type, num_locals);
        let offset = self
            .builder
            .ins()
            .imul_imm(num_locals, types::Value::SIZE as i64);
        let offset = self
            .builder
            .ins()
            .iadd_imm(offset, types::Coroutine::LOCALS_OFFSET as i64);
        self.builder.ins().iadd(coroutine.0, offset)
    }

    pub fn put_write_boolean_to_scratch_buffer(
        &mut self,
        value: BooleanIr,
        scratch_buffer: ir::Value,
        offset: usize,
    ) {
        logger::debug!(
            event = "put_write_boolean_to_scratch_buffer",
            ?value,
            ?scratch_buffer,
            offset,
        );
        self.put_store(value.0, scratch_buffer, offset);
    }

    pub fn put_write_number_to_scratch_buffer(
        &mut self,
        value: NumberIr,
        scratch_buffer: ir::Value,
        offset: usize,
    ) {
        logger::debug!(
            event = "put_write_number_to_scratch_buffer",
            ?value,
            ?scratch_buffer,
            offset,
        );
        self.put_store(value.0, scratch_buffer, offset);
    }

    pub fn put_write_string_to_scratch_buffer(
        &mut self,
        value: StringIr,
        scratch_buffer: ir::Value,
        offset: usize,
    ) {
        logger::debug!(
            event = "put_write_string_to_scratch_buffer",
            ?value,
            ?scratch_buffer,
            offset,
        );
        self.put_store(value.0, scratch_buffer, offset);
    }

    pub fn put_write_closure_to_scratch_buffer(
        &mut self,
        value: ClosureIr,
        scratch_buffer: ir::Value,
        offset: usize,
    ) {
        logger::debug!(
            event = "put_write_closure_to_scratch_buffer",
            ?value,
            ?scratch_buffer,
            offset,
        );
        self.put_store(value.0, scratch_buffer, offset);
    }

    pub fn put_write_object_to_scratch_buffer(
        &mut self,
        value: ObjectIr,
        scratch_buffer: ir::Value,
        offset: usize,
    ) {
        logger::debug!(
            event = "put_write_object_to_scratch_buffer",
            ?value,
            ?scratch_buffer,
            offset,
        );
        self.put_store(value.0, scratch_buffer, offset);
    }

    pub fn put_write_promise_to_scratch_buffer(
        &mut self,
        value: PromiseIr,
        scratch_buffer: ir::Value,
        offset: usize,
    ) {
        logger::debug!(
            event = "put_write_promise_to_scratch_buffer",
            ?value,
            ?scratch_buffer,
            offset,
        );
        self.put_store(value.0, scratch_buffer, offset);
    }

    pub fn put_write_any_to_scratch_buffer(
        &mut self,
        value: AnyIr,
        scratch_buffer: ir::Value,
        offset: usize,
    ) {
        logger::debug!(
            event = "put_write_any_to_scratch_buffer",
            ?value,
            ?scratch_buffer,
            offset,
        );
        let opaque = self.put_load_i128(value.0, 0);
        self.put_store(opaque, scratch_buffer, offset);
    }

    pub fn put_read_boolean_from_scratch_buffer(
        &mut self,
        scratch_buffer: ir::Value,
        offset: usize,
    ) -> BooleanIr {
        logger::debug!(
            event = "put_read_boolean_from_scratch_buffer",
            ?scratch_buffer,
            offset
        );
        BooleanIr(self.put_load_i8(scratch_buffer, offset))
    }

    pub fn put_read_number_from_scratch_buffer(
        &mut self,
        scratch_buffer: ir::Value,
        offset: usize,
    ) -> NumberIr {
        logger::debug!(
            event = "put_read_number_from_scratch_buffer",
            ?scratch_buffer,
            offset
        );
        NumberIr(self.put_load_f64(scratch_buffer, offset))
    }

    pub fn put_read_string_from_scratch_buffer(
        &mut self,
        scratch_buffer: ir::Value,
        offset: usize,
    ) -> StringIr {
        logger::debug!(
            event = "put_read_string_from_scratch_buffer",
            ?scratch_buffer,
            offset
        );
        StringIr(self.put_load_addr(scratch_buffer, offset))
    }

    pub fn put_read_closure_from_scratch_buffer(
        &mut self,
        scratch_buffer: ir::Value,
        offset: usize,
    ) -> ClosureIr {
        logger::debug!(
            event = "put_read_closure_from_scratch_buffer",
            ?scratch_buffer,
            offset
        );
        ClosureIr(self.put_load_addr(scratch_buffer, offset))
    }

    pub fn put_read_object_from_scratch_buffer(
        &mut self,
        scratch_buffer: ir::Value,
        offset: usize,
    ) -> ObjectIr {
        logger::debug!(
            event = "put_read_object_from_scratch_buffer",
            ?scratch_buffer,
            offset
        );
        ObjectIr(self.put_load_addr(scratch_buffer, offset))
    }

    pub fn put_read_promise_from_scratch_buffer(
        &mut self,
        scratch_buffer: ir::Value,
        offset: usize,
    ) -> PromiseIr {
        logger::debug!(
            event = "put_read_promise_from_scratch_buffer",
            ?scratch_buffer,
            offset
        );
        PromiseIr(self.put_load_i32(scratch_buffer, offset))
    }

    pub fn put_read_any_from_scratch_buffer(
        &mut self,
        scratch_buffer: ir::Value,
        offset: usize,
    ) -> AnyIr {
        logger::debug!(
            event = "put_read_boolean_from_scratch_buffer",
            ?scratch_buffer,
            offset
        );
        // Just return the address on the scratch buffer where the value has been stored.
        AnyIr(self.builder.ins().iadd_imm(scratch_buffer, offset as i64))
    }

    // runtime function calls

    pub fn put_runtime_to_boolean(&mut self, value: AnyIr) -> BooleanIr {
        logger::debug!(event = "put_runtime_to_boolean", ?value);
        let func = self.runtime_func_cache.get_to_boolean(self.builder.func);
        let args = [self.runtime(), value.0];
        let call = self.builder.ins().call(func, &args);
        BooleanIr(self.builder.inst_results(call)[0])
    }

    pub fn put_runtime_to_numeric(&mut self, value: AnyIr) -> NumberIr {
        logger::debug!(event = "put_runtime_to_numeric", ?value);
        let func = self.runtime_func_cache.get_to_numeric(self.builder.func);
        let args = [self.runtime(), value.0];
        let call = self.builder.ins().call(func, &args);
        NumberIr(self.builder.inst_results(call)[0])
    }

    pub fn put_runtime_to_object(&mut self, any: AnyIr) -> ObjectIr {
        logger::debug!(event = "put_runtime_to_object", ?any);
        let func = self.runtime_func_cache.get_to_object(self.builder.func);
        let args = [self.runtime(), any.0];
        let call = self.builder.ins().call(func, &args);
        ObjectIr(self.builder.inst_results(call)[0])
    }

    // 7.1.6 ToInt32 ( argument )
    pub fn put_runtime_to_int32(&mut self, value: NumberIr) -> ir::Value {
        logger::debug!(event = "put_runtime_to_int32", ?value);
        let func = self.runtime_func_cache.get_to_int32(self.builder.func);
        let args = [self.runtime(), value.0];
        let call = self.builder.ins().call(func, &args);
        self.builder.inst_results(call)[0]
    }

    pub fn put_runtime_to_uint32(&mut self, value: NumberIr) -> ir::Value {
        logger::debug!(event = "put_runtime_to_uint32", ?value);
        let func = self.runtime_func_cache.get_to_uint32(self.builder.func);
        let args = [self.runtime(), value.0];
        let call = self.builder.ins().call(func, &args);
        self.builder.inst_results(call)[0]
    }

    pub fn put_runtime_is_loosely_equal(&mut self, lhs: AnyIr, rhs: AnyIr) -> BooleanIr {
        logger::debug!(event = "put_runtime_is_loosely_equal", ?lhs, ?rhs);
        let func = self
            .runtime_func_cache
            .get_is_loosely_equal(self.builder.func);
        let args = [self.runtime(), lhs.0, rhs.0];
        let call = self.builder.ins().call(func, &args);
        BooleanIr(self.builder.inst_results(call)[0])
    }

    pub fn put_runtime_is_strictly_equal(&mut self, lhs: AnyIr, rhs: AnyIr) -> BooleanIr {
        logger::debug!(event = "put_runtime_is_strictly_equal", ?lhs, ?rhs);
        let func = self
            .runtime_func_cache
            .get_is_strictly_equal(self.builder.func);
        let args = [self.runtime(), lhs.0, rhs.0];
        let call = self.builder.ins().call(func, &args);
        BooleanIr(self.builder.inst_results(call)[0])
    }

    pub fn put_runtime_typeof(&mut self, value: AnyIr) -> StringIr {
        logger::debug!(event = "put_runtime_typeof", ?value);
        let func = self.runtime_func_cache.get_get_typeof(self.builder.func);
        let args = [self.runtime(), value.0];
        let call = self.builder.ins().call(func, &args);
        StringIr(self.builder.inst_results(call)[0])
    }

    pub fn put_runtime_migrate_string_to_heap(&mut self, string: StringIr) -> StringIr {
        logger::debug!(event = "putruntime_migrate_string_to_heap", ?string);
        let func = self
            .runtime_func_cache
            .get_migrate_string_to_heap(self.builder.func);
        let args = [self.runtime(), string.0];
        let call = self.builder.ins().call(func, &args);
        StringIr(self.builder.inst_results(call)[0])
    }

    pub fn put_runtime_create_capture(&mut self, target: AnyIr) -> CaptureIr {
        logger::debug!(event = "put_runtime_create_capture", ?target);
        let func = self
            .runtime_func_cache
            .get_create_capture(self.builder.func);
        let args = [self.runtime(), target.0];
        let call = self.builder.ins().call(func, &args);
        CaptureIr(self.builder.inst_results(call)[0])
    }

    pub fn put_runtime_create_closure(&mut self, lambda: LambdaIr, num_captures: u16) -> ClosureIr {
        logger::debug!(event = "put_runtime_create_closure", ?lambda, num_captures);
        let func = self
            .runtime_func_cache
            .get_create_closure(self.builder.func);
        let num_captures = self
            .builder
            .ins()
            .iconst(ir::types::I16, num_captures as i64);
        let args = [self.runtime(), lambda.0, num_captures];
        let call = self.builder.ins().call(func, &args);
        ClosureIr(self.builder.inst_results(call)[0])
    }

    pub fn put_runtime_create_coroutine(
        &mut self,
        closure: ClosureIr,
        num_locals: u16,
        scratch_buffer_len: u16,
    ) -> CoroutineIr {
        logger::debug!(
            event = "put_runtime_create_coroutine",
            ?closure,
            num_locals,
            scratch_buffer_len
        );
        let func = self
            .runtime_func_cache
            .get_create_coroutine(self.builder.func);
        let num_locals = self.builder.ins().iconst(ir::types::I16, num_locals as i64);
        let scratch_buffer_len = self
            .builder
            .ins()
            .iconst(ir::types::I16, scratch_buffer_len as i64);
        let args = [self.runtime(), closure.0, num_locals, scratch_buffer_len];
        let call = self.builder.ins().call(func, &args);
        CoroutineIr(self.builder.inst_results(call)[0])
    }

    pub fn put_runtime_register_promise(&mut self, coroutine: CoroutineIr) -> PromiseIr {
        logger::debug!(event = "put_runtime_register_promise", ?coroutine);
        let func = self
            .runtime_func_cache
            .get_register_promise(self.builder.func);
        let args = [self.runtime(), coroutine.0];
        let call = self.builder.ins().call(func, &args);
        PromiseIr(self.builder.inst_results(call)[0])
    }

    pub fn put_runtime_await_promise(&mut self, promise: PromiseIr, awaiting: PromiseIr) {
        logger::debug!(event = "put_runtime_await_promise", ?promise, ?awaiting);
        let func = self.runtime_func_cache.get_await_promise(self.builder.func);
        let args = [self.runtime(), promise.0, awaiting.0];
        self.builder.ins().call(func, &args);
    }

    pub fn put_runtime_resume(&mut self, promise: PromiseIr) {
        logger::debug!(event = "put_runtime_resume", ?promise);
        let func = self.runtime_func_cache.get_resume(self.builder.func);
        let args = [self.runtime(), promise.0];
        self.builder.ins().call(func, &args);
    }

    pub fn put_runtime_emit_promise_resolved(&mut self, promise: PromiseIr, result: AnyIr) {
        logger::debug!(
            event = "put_runtime_emit_promise_resolved",
            ?promise,
            ?result
        );
        let func = self
            .runtime_func_cache
            .get_emit_promise_resolved(self.builder.func);
        let args = [self.runtime(), promise.0, result.0];
        self.builder.ins().call(func, &args);
    }

    pub fn put_runtime_create_object(&mut self) -> ObjectIr {
        logger::debug!(event = "put_runtime_create_object");
        let func = self.runtime_func_cache.get_create_object(self.builder.func);
        let args = [self.runtime()];
        let call = self.builder.ins().call(func, &args);
        ObjectIr(self.builder.inst_results(call)[0])
    }

    pub fn put_runtime_get_value_by_symbol(
        &mut self,
        object: ObjectIr,
        key: Symbol,
        strict: bool,
    ) -> AnyIr {
        logger::debug!(
            event = "put_runtime_get_value_by_symbol",
            ?object,
            ?key,
            strict
        );
        let func = self
            .runtime_func_cache
            .get_get_value_by_symbol(self.builder.func);
        let key = self.builder.ins().iconst(ir::types::I32, key.id() as i64);
        let strict = self.put_boolean(strict);
        let args = [self.runtime(), object.0, key, strict.0];
        let call = self.builder.ins().call(func, &args);
        AnyIr(self.builder.inst_results(call)[0])
    }

    pub fn put_runtime_get_value_by_number(
        &mut self,
        object: ObjectIr,
        key: f64,
        strict: bool,
    ) -> AnyIr {
        logger::debug!(
            event = "put_runtime_get_value_by_number",
            ?object,
            key,
            strict
        );
        let func = self
            .runtime_func_cache
            .get_get_value_by_number(self.builder.func);
        let key = self.put_number(key);
        let strict = self.put_boolean(strict);
        let args = [self.runtime(), object.0, key.0, strict.0];
        let call = self.builder.ins().call(func, &args);
        AnyIr(self.builder.inst_results(call)[0])
    }

    pub fn put_runtime_get_value_by_any(
        &mut self,
        object: ObjectIr,
        key: AnyIr,
        strict: bool,
    ) -> AnyIr {
        logger::debug!(
            event = "put_runtime_get_value_by_any",
            ?object,
            ?key,
            strict
        );
        let func = self
            .runtime_func_cache
            .get_get_value_by_value(self.builder.func);
        let strict = self.put_boolean(strict);
        let args = [self.runtime(), object.0, key.0, strict.0];
        let call = self.builder.ins().call(func, &args);
        AnyIr(self.builder.inst_results(call)[0])
    }

    pub fn put_runtime_set_value_by_symbol(&mut self, object: ObjectIr, key: Symbol, value: AnyIr) {
        logger::debug!(
            event = "put_runtime_set_value_by_symbol",
            ?object,
            ?key,
            ?value
        );
        let func = self
            .runtime_func_cache
            .get_set_value_by_symbol(self.builder.func);
        let key = self.builder.ins().iconst(ir::types::I32, key.id() as i64);
        let args = [self.runtime(), object.0, key, value.0];
        self.builder.ins().call(func, &args);
    }

    pub fn put_runtime_set_value_by_number(&mut self, object: ObjectIr, key: f64, value: AnyIr) {
        logger::debug!(
            event = "put_runtime_set_value_by_number",
            ?object,
            key,
            ?value
        );
        let func = self
            .runtime_func_cache
            .get_set_value_by_number(self.builder.func);
        let key = self.builder.ins().f64const(key);
        let args = [self.runtime(), object.0, key, value.0];
        self.builder.ins().call(func, &args);
    }

    pub fn put_runtime_set_value_by_any(&mut self, object: ObjectIr, key: AnyIr, value: AnyIr) {
        logger::debug!(
            event = "put_runtime_set_value_by_any",
            ?object,
            ?key,
            ?value
        );
        let func = self
            .runtime_func_cache
            .get_set_value_by_value(self.builder.func);
        let args = [self.runtime(), object.0, key.0, value.0];
        self.builder.ins().call(func, &args);
    }

    pub fn put_runtime_create_data_property_by_symbol(
        &mut self,
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
        let func = self
            .runtime_func_cache
            .get_create_data_property_by_symbol(self.builder.func);
        let key = self.builder.ins().iconst(ir::types::I32, key.id() as i64);
        let args = [self.runtime(), object.0, key, value.0, retv.0];
        let call = self.builder.ins().call(func, &args);
        StatusIr(self.builder.inst_results(call)[0])
    }

    pub fn put_runtime_create_data_property_by_number(
        &mut self,
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
        let func = self
            .runtime_func_cache
            .get_create_data_property_by_number(self.builder.func);
        let key = self.put_number(key);
        let args = [self.runtime(), object.0, key.0, value.0, retv.0];
        let call = self.builder.ins().call(func, &args);
        StatusIr(self.builder.inst_results(call)[0])
    }

    pub fn put_runtime_create_data_property_by_any(
        &mut self,
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
        let func = self
            .runtime_func_cache
            .get_create_data_property_by_value(self.builder.func);
        let args = [self.runtime(), object.0, key.0, value.0, retv.0];
        let call = self.builder.ins().call(func, &args);
        StatusIr(self.builder.inst_results(call)[0])
    }

    pub fn put_runtime_copy_data_properties(
        &mut self,
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
        let func = self
            .runtime_func_cache
            .get_copy_data_properties(self.builder.func);
        let args = [self.runtime(), target.0, source.0, retv.0];
        let call = self.builder.ins().call(func, &args);
        StatusIr(self.builder.inst_results(call)[0])
    }

    pub fn put_runtime_push_array_element(
        &mut self,
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
        let func = self.runtime_func_cache.get_push_value(self.builder.func);
        let args = [self.runtime(), target.0, value.0, retv.0];
        let call = self.builder.ins().call(func, &args);
        StatusIr(self.builder.inst_results(call)[0])
    }

    pub fn put_runtime_assert(&mut self, assertion: BooleanIr, msg: &'static CStr) {
        logger::debug!(event = "put_runtime_assert", ?assertion, ?msg);
        let func = self.runtime_func_cache.get_assert(self.builder.func);
        let msg = self
            .builder
            .ins()
            .iconst(self.addr_type, msg.as_ptr() as i64);
        let args = [self.runtime(), assertion.0, msg];
        self.builder.ins().call(func, &args);
    }

    #[allow(unused)]
    pub fn put_runtime_print_boolean(&mut self, value: BooleanIr, msg: &'static CStr) {
        logger::debug!(event = "put_runtime_print_boolean", ?value);
        let func = self.runtime_func_cache.get_print_bool(self.builder.func);
        let msg = self
            .builder
            .ins()
            .iconst(self.addr_type, msg.as_ptr() as i64);
        let args = [self.runtime(), value.0, msg];
        self.builder.ins().call(func, &args);
    }

    #[allow(unused)]
    pub fn put_runtime_print_number(&mut self, value: NumberIr, msg: &'static CStr) {
        logger::debug!(event = "put_runtime_print_number", ?value);
        let func = self.runtime_func_cache.get_print_f64(self.builder.func);
        let msg = self
            .builder
            .ins()
            .iconst(self.addr_type, msg.as_ptr() as i64);
        let args = [self.runtime(), value.0, msg];
        self.builder.ins().call(func, &args);
    }

    #[allow(unused)]
    pub fn put_runtime_print_any(&mut self, value: AnyIr, msg: &'static CStr) {
        logger::debug!(event = "put_runtime_print_any", ?value, ?msg);
        let func = self.runtime_func_cache.get_print_value(self.builder.func);
        let msg = self
            .builder
            .ins()
            .iconst(self.addr_type, msg.as_ptr() as i64);
        let args = [self.runtime(), value.0, msg];
        self.builder.ins().call(func, &args);
    }

    #[allow(unused)]
    pub fn put_runtime_print_capture(&mut self, capture: CaptureIr, msg: &'static CStr) {
        logger::debug!(event = "put_runtime_print_capture", ?capture, ?msg);
        let func = self.runtime_func_cache.get_print_capture(self.builder.func);
        let msg = self
            .builder
            .ins()
            .iconst(self.addr_type, msg.as_ptr() as i64);
        let args = [self.runtime(), capture.0, msg];
        self.builder.ins().call(func, &args);
    }

    #[allow(unused)]
    pub fn put_runtime_print_message(&mut self, msg: &'static CStr) {
        logger::debug!(event = "put_runtime_print_message", ?msg);
        let func = self.runtime_func_cache.get_print_message(self.builder.func);
        let msg = self
            .builder
            .ins()
            .iconst(self.addr_type, msg.as_ptr() as i64);
        let args = [self.runtime(), msg];
        self.builder.ins().call(func, &args);
    }

    pub fn put_runtime_launch_debugger(&mut self) {
        logger::debug!(event = "put_runtime_launch_debugger");
        let func = self
            .runtime_func_cache
            .get_launch_debugger(self.builder.func);
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
            self.put_store(scope_id, coroutine.0, types::Coroutine::SCOPE_ID_OFFSET);
        } else {
            self.builder.ins().stack_store(scope_id, self.fcs, 8);
        }
    }

    pub fn put_assert_scope_id(&mut self, expected: ScopeRef) {
        logger::debug!(event = "put_assert_scope_id", ?expected);
        use ir::condcodes::IntCC::Equal;
        let scope_id = if self.coroutine_mode {
            let coroutine = self.coroutine();
            self.put_load_i16(coroutine.0, types::Coroutine::SCOPE_ID_OFFSET)
        } else {
            self.builder.ins().stack_load(ir::types::I16, self.fcs, 8)
        };
        let assertion = self
            .builder
            .ins()
            .icmp_imm(Equal, scope_id, expected.id() as i64);
        self.put_runtime_assert(BooleanIr(assertion), c"invalid scope");
    }
}
