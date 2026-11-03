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

/// A registry that manages JIT compilation state, function mappings, and metadata.
pub struct CodeRegistry<X> {
    /// The underlying Cranelift JIT module.
    ///
    /// `JITModule::free_memory()` must be called manually in order to free memory blocks allocated
    /// for compiled functions.  Without this, the memory blocks will leak even though the
    /// `JITModule` object is disposed.
    ///
    /// `JITModule::free_memory()` takes the ownership of the object.  The type of `module` must
    /// allow transfer of the ownership so that we call `JITModule::free_memory()` in `drop()`.
    /// This is the reason why `Option<T>` is used here.
    module: Option<Box<JITModule>>,

    /// The signature used for all generated lambdas.
    lambda_sig: ir::Signature,

    runtime_func_ids: RuntimeFunctionIds,

    /// Maps each LambdaId to its corresponding JIT metadata.
    code_info_map: FxHashMap<LambdaId, JitCodeInfo>,

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
            code_info_map: Default::default(),
            _phantom: PhantomData,
        }
    }

    /// Retrieves the finalized lambda representation for execution.
    pub fn get_lambda(&self, lambda_id: LambdaId) -> Option<Lambda<X>> {
        self.get_code_info(lambda_id).map(|(addr, _)| addr.into())
    }

    /// Retrieves both the execution address and the compiled code size simultaneously.
    ///
    /// Useful for application-layer integrations such as Perf Map output.
    pub fn get_code_info(&self, lambda_id: LambdaId) -> Option<(LambdaAddr, u32)> {
        let info = self.code_info_map.get(&lambda_id)?;
        let ptr = self
            .module
            .as_ref()
            .unwrap()
            .get_finalized_function(info.func_id);
        if ptr.is_null() {
            None
        } else {
            Some((LambdaAddr::from(ptr.addr()), info.code_size))
        }
    }

    fn target_config(&self) -> isa::TargetFrontendConfig {
        self.module.as_ref().unwrap().target_config()
    }

    /// Compiles a function, extracts its code size, finalizes definitions, and registers the
    /// compiled metadata.
    fn codegen(&mut self, func: &Function, ctx: &mut codegen::Context) {
        logger::debug!(event = "codegen");
        // It's unnecessary to declare JavaScript functions called in a JavaScript function before
        // the JIT compilation.  Because every JavaScript function will be called indirectly.
        let name = func.id.to_string();
        let module = self.module.as_mut().unwrap();

        let func_id = module
            .declare_function(&name, Linkage::Local, &self.lambda_sig)
            .unwrap_or_else(|err| panic!("Failed to declare function '{}': {:?}", name, err));

        #[cfg(debug_assertions)]
        {
            let flags = codegen::settings::Flags::new(codegen::settings::builder());
            if let Err(err) = codegen::verify_function(&ctx.func, &flags) {
                let err = codegen::print_errors::pretty_verifier_error(&ctx.func, None, err);
                panic!("{err}");
            }
        }

        // TODO(feat): define_function() may fail due to problems in Cranelift backend or
        // insufficient memory.  However, we don't handle those cases at this point for simplicity.
        // Simply crash the program.
        module.define_function(func_id, ctx).unwrap_or_else(|err| {
            panic!(
                "Failed to define function '{}' in Cranelift module: {:?}",
                name, err
            )
        });

        // Save the function code size before clear_context().
        //
        // NOTE: Ideally, the Cranelift JIT module manages the code buffers internally and should be
        // able to provide the size of a finalized function. However, since the current Cranelift
        // API does not provide a direct method to retrieve it after compilation, we explicitly
        // extract and store it here.  By encapsulating this inside `CodeRegistry`, any future
        // updates if Cranelift adds such a method will be completely contained within this module,
        // preventing changes from leaking outward to the application layer.
        let code_size = ctx
            .compiled_code()
            .unwrap_or_else(|| {
                panic!(
                    "Compiled code missing immediately after define_function for '{}'",
                    name
                )
            })
            .code_info()
            .total_size;

        // Clear the context for reuse.
        module.clear_context(ctx);

        module.finalize_definitions().unwrap_or_else(|err| {
            panic!(
                "Failed to finalize module definitions while compiling '{}': {:?}",
                name, err
            )
        });

        self.code_info_map
            .insert(func.id, JitCodeInfo { func_id, code_size });
    }
}

impl<X> Drop for CodeRegistry<X> {
    fn drop(&mut self) {
        unsafe {
            self.module.take().unwrap().free_memory();
        }
    }
}

/// Metadata for JIT-managed compiled code.
struct JitCodeInfo {
    /// The Cranelift function ID.
    func_id: FuncId,

    /// The total compiled machine code size in bytes.
    code_size: u32,
}
