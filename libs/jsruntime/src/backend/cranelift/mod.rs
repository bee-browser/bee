mod compiler;

use cranelift::prelude::*;
use cranelift_jit::JITModule;
use cranelift_module::Linkage;
use cranelift_module::Module as _;

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
    main: Option<*const u8>,
}

impl Executor {
    pub fn new(_functions: &RuntimeFunctions) -> Self {
        Self {
            module: None,
            main: None,
        }
    }

    pub fn register_module(&mut self, mut module: Module) {
        let id = module
            .inner
            .declare_function("main", Linkage::Export, &module.context.func.signature)
            .unwrap();
        module
            .inner
            .define_function(id, &mut module.context)
            .unwrap();
        module.inner.clear_context(&mut module.context);
        module.inner.finalize_definitions().unwrap();
        let addr = module.inner.get_finalized_function(id);
        self.module = Some(module);
        self.main = Some(addr);
    }

    pub fn get_lambda(&self, _lambda_id: LambdaId) -> Option<Lambda> {
        self.main
            .map(|addr| unsafe { std::mem::transmute::<_, Lambda>(addr) })
    }
}
