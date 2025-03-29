mod compiler;

use cranelift::prelude::*;
use cranelift_jit::JITModule;
use cranelift_module::FuncId;
use rustc_hash::FxHashMap;

use crate::lambda::LambdaId;
use crate::types::Lambda;

use super::CompileError;
use super::Program;
use super::RuntimeFunctions;

pub use compiler::compile;

pub fn initialize() {
    // TODO
}

pub struct Module {
    inner: JITModule,
    context: codegen::Context,
    id_map: FxHashMap<LambdaId, FuncId>,
}

impl Module {
    pub fn print(&self, stderr: bool) {
        if stderr {
            eprintln!("{}", self.context.func);
        } else {
            println!("{}", self.context.func);
        }
    }
}

pub struct Executor {
    module: Option<Module>,
}

impl Executor {
    pub fn new(_functions: &RuntimeFunctions) -> Self {
        Self { module: None }
    }

    pub fn register_module(&mut self, mut module: Module) {
        module.inner.finalize_definitions().unwrap();
        self.module = Some(module);
    }

    pub fn get_lambda(&self, lambda_id: LambdaId) -> Option<Lambda> {
        let module = self.module.as_ref().unwrap();
        let func_id = *module.id_map.get(&lambda_id).unwrap();
        let addr = module.inner.get_finalized_function(func_id);
        (!addr.is_null()).then(|| unsafe { std::mem::transmute::<_, Lambda>(addr) })
    }
}

impl LambdaId {
    fn make_name(self) -> String {
        format!("fn{}", u32::from(self))
    }
}
