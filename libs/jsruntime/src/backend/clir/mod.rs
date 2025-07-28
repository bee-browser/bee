mod compiler;
mod support;

use std::ffi::c_void;

use cranelift::codegen;
use cranelift::codegen::ir;
use cranelift::codegen::settings::Configurable as _;
use cranelift::prelude::isa;
use cranelift_jit::JITBuilder;
use cranelift_jit::JITModule;
use cranelift_module::FuncId;
use cranelift_module::Linkage;
use cranelift_module::Module;
use rustc_hash::FxHashMap;

use jsparser::Symbol;

use crate::Runtime;
use crate::lambda::LambdaId;
use crate::lambda::LambdaInfo;
use crate::logger;
use crate::semantics::Function;
use crate::types::Lambda;

use super::CompileError;
use super::RuntimeFunctions;

use support::EditorSupport;
use support::RuntimeFunctionCache;
use support::RuntimeFunctionIds;

pub use compiler::compile;
pub use compiler::compile_function;

pub fn initialize() {
    // Nothing to do.
}

pub trait CompilerSupport {
    // RuntimePref
    fn is_scope_cleanup_checker_enabled(&self) -> bool;

    // SymbolRegistry
    fn get_symbol_name(&self, symbol: Symbol) -> &[u16];
    fn make_symbol_from_name(&mut self, name: Vec<u16>) -> Symbol;

    // LambdaRegistry
    fn get_lambda_info(&self, lambda_id: LambdaId) -> &LambdaInfo;
    fn get_lambda_info_mut(&mut self, lambda_id: LambdaId) -> &mut LambdaInfo;

    // Program
    fn get_function(&self, lambda_id: LambdaId) -> &Function;

    // Executor
    fn target_config(&self) -> isa::TargetFrontendConfig;

    // GlobalObject
    fn global_object(&mut self) -> *mut c_void;

    // Intrinsics
    fn object_prototype(&self) -> *mut c_void;
    fn function_prototype(&self) -> *mut c_void;
}

impl<X> CompilerSupport for Runtime<X> {
    fn is_scope_cleanup_checker_enabled(&self) -> bool {
        self.pref.enable_scope_cleanup_checker
    }

    fn get_symbol_name(&self, symbol: Symbol) -> &[u16] {
        self.symbol_registry.resolve(symbol).unwrap()
    }

    fn make_symbol_from_name(&mut self, name: Vec<u16>) -> Symbol {
        self.symbol_registry.intern_utf16(name)
    }

    fn get_lambda_info(&self, lambda_id: LambdaId) -> &LambdaInfo {
        self.lambda_registry.get(lambda_id)
    }

    fn get_lambda_info_mut(&mut self, lambda_id: LambdaId) -> &mut LambdaInfo {
        self.lambda_registry.get_mut(lambda_id)
    }

    fn get_function(&self, lambda_id: LambdaId) -> &Function {
        let info = self.lambda_registry.get(lambda_id);
        &self.programs[info.program_id.index()].functions[info.function_index as usize]
    }

    fn target_config(&self) -> isa::TargetFrontendConfig {
        self.executor.target_config()
    }

    fn global_object(&mut self) -> *mut c_void {
        self.global_object.as_ptr()
    }

    fn object_prototype(&self) -> *mut c_void {
        debug_assert!(!self.object_prototype.is_null());
        self.object_prototype
    }

    fn function_prototype(&self) -> *mut c_void {
        debug_assert!(!self.function_prototype.is_null());
        self.function_prototype
    }
}

pub struct Executor {
    module: Box<JITModule>,
    lambda_sig: ir::Signature,
    runtime_func_ids: RuntimeFunctionIds,
    id_map: FxHashMap<LambdaId, FuncId>,
}

impl Executor {
    pub fn new(runtime_functions: &RuntimeFunctions) -> Self {
        let mut flag_builder = codegen::settings::builder();
        flag_builder.set("use_colocated_libcalls", "false").unwrap();
        flag_builder.set("is_pic", "false").unwrap();

        let isa_builder = cranelift_native::builder().unwrap_or_else(|msg| {
            panic!("host machine is not supported: {msg}");
        });

        let isa = isa_builder
            .finish(codegen::settings::Flags::new(flag_builder))
            .unwrap();

        let mut builder = JITBuilder::with_isa(isa, cranelift_module::default_libcall_names());
        support::register_symbols(&mut builder, runtime_functions);

        let mut module = Box::new(JITModule::new(builder));
        let lambda_sig = support::make_lambda_signature(&mut module);
        let runtime_func_ids = support::declare_functions(&mut module);

        Self {
            module,
            lambda_sig,
            runtime_func_ids,
            id_map: Default::default(),
        }
    }

    pub fn target_config(&self) -> isa::TargetFrontendConfig {
        self.module.target_config()
    }

    pub fn get_lambda(&self, lambda_id: LambdaId) -> Option<Lambda> {
        let func_id = *self.id_map.get(&lambda_id)?;
        let addr = self.module.get_finalized_function(func_id);
        (!addr.is_null()).then(|| unsafe { std::mem::transmute::<_, Lambda>(addr) })
    }

    pub fn codegen(&mut self, func: &Function, ctx: &mut codegen::Context) {
        logger::debug!(event = "codegen");
        // It's unnecessary to declare JavaScript functions called in a JavaScript function before
        // the JIT compilation.  Because every JavaScript function will be called indirectly.
        let name = func.id.make_name();
        let func_id = self
            .module
            .declare_function(&name, Linkage::Local, &self.lambda_sig)
            .unwrap();
        self.id_map.insert(func.id, func_id);
        self.module.define_function(func_id, ctx).unwrap();
        self.module.clear_context(ctx);
        self.module.finalize_definitions().unwrap();
    }
}

impl LambdaId {
    fn make_name(self) -> String {
        format!("fn{}", u32::from(self))
    }
}
