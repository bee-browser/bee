mod compiler;

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
use crate::semantics::Function;
use crate::types::Lambda;

use super::CompileError;
use super::Program;
use super::RuntimeFunctions;

pub use compiler::compile;
pub use compiler::runtime::RuntimeFunctionIds;

pub fn initialize() {
    // TODO
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
        compiler::runtime::register_symbols(&mut builder, runtime_functions);

        let mut module = Box::new(JITModule::new(builder));
        let lambda_sig = compiler::runtime::make_lambda_signature(&mut module);
        let runtime_func_ids = compiler::runtime::declare_functions(&mut module);

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

    pub fn id_map(&self) -> &FxHashMap<LambdaId, FuncId> {
        &self.id_map
    }

    pub fn runtime_func_ids(&self) -> &RuntimeFunctionIds {
        &self.runtime_func_ids
    }

    pub fn link(&mut self) {
        self.module.finalize_definitions().unwrap();
    }

    pub fn get_lambda(&self, lambda_id: LambdaId) -> Option<Lambda> {
        let func_id = *self.id_map.get(&lambda_id).unwrap();
        let addr = self.module.get_finalized_function(func_id);
        (!addr.is_null()).then(|| unsafe { std::mem::transmute::<_, Lambda>(addr) })
    }

    pub fn declare_functions(&mut self, program: &Program) {
        for func in program.functions.iter() {
            let name = func.id.make_name();
            let func_id = self
                .module
                .declare_function(&name, Linkage::Local, &self.lambda_sig)
                .unwrap();
            self.id_map.insert(func.id, func_id);
        }
    }

    pub fn define_function(&mut self, func: &Function, ctx: &mut codegen::Context) {
        let func_id = *self.id_map.get(&func.id).unwrap();
        self.module.define_function(func_id, ctx).unwrap();
        self.module.clear_context(ctx);
    }
}

impl LambdaId {
    fn make_name(self) -> String {
        format!("fn{}", u32::from(self))
    }
}
