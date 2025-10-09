mod compiler;
mod support;

use std::marker::PhantomData;

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

use crate::lambda::LambdaId;
use crate::logger;
use crate::semantics::Function;
use crate::types::Lambda;
use crate::types::LambdaAddr;

use super::CompileError;

use support::EditorSupport;
use support::RuntimeFunctionCache;
use support::RuntimeFunctionIds;

pub use compiler::compile;
pub use compiler::compile_function;

pub fn initialize() {
    // Nothing to do.
}

pub struct CodeRegistry<X> {
    // `JITModule::free_memory()` must be called manually in order to free memory blocks allocated
    // for compiled functions.  Without this, the memory blocks will leak even though the
    // `JITModule` object is disposed.
    //
    // `JITModule::free_memory()` takes the ownership of the object.  The type of `module` must
    // allow transfer of the ownership so that we call `JITModule::free_memory()` in `drop()`.
    // This is the reason why `Option<T>` is used here.
    module: Option<Box<JITModule>>,
    lambda_sig: ir::Signature,
    runtime_func_ids: RuntimeFunctionIds,
    id_map: FxHashMap<LambdaId, FuncId>,
    _phantom: PhantomData<X>,
}

impl<X> CodeRegistry<X> {
    pub fn new() -> Self {
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
        support::register_symbols::<X>(&mut builder);

        let mut module = Box::new(JITModule::new(builder));
        let lambda_sig = support::make_lambda_signature(&mut module);
        let runtime_func_ids = support::declare_functions(&mut module);

        Self {
            module: Some(module),
            lambda_sig,
            runtime_func_ids,
            id_map: Default::default(),
            _phantom: PhantomData,
        }
    }

    pub fn get_lambda(&self, lambda_id: LambdaId) -> Option<Lambda<X>> {
        let func_id = *self.id_map.get(&lambda_id)?;
        let ptr = self
            .module
            .as_ref()
            .unwrap()
            .get_finalized_function(func_id);
        let addr = if ptr.is_null() {
            None
        } else {
            Some(LambdaAddr::from(ptr.addr()))
        };
        addr.map(|addr| addr.into())
    }

    fn target_config(&self) -> isa::TargetFrontendConfig {
        self.module.as_ref().unwrap().target_config()
    }

    fn codegen(&mut self, func: &Function, ctx: &mut codegen::Context) {
        logger::debug!(event = "codegen");
        // It's unnecessary to declare JavaScript functions called in a JavaScript function before
        // the JIT compilation.  Because every JavaScript function will be called indirectly.
        let name = func.id.make_name();
        let module = self.module.as_mut().unwrap();
        let func_id = module
            .declare_function(&name, Linkage::Local, &self.lambda_sig)
            .unwrap();
        self.id_map.insert(func.id, func_id);
        module.define_function(func_id, ctx).unwrap();
        module.clear_context(ctx);
        module.finalize_definitions().unwrap();
    }
}

impl<X> Drop for CodeRegistry<X> {
    fn drop(&mut self) {
        unsafe {
            self.module.take().unwrap().free_memory();
        }
    }
}

impl LambdaId {
    fn make_name(self) -> String {
        format!("fn{}", u32::from(self))
    }
}
