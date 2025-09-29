use std::time::Duration;
use std::time::Instant;

use jsruntime::Runtime;
use jsruntime::Value;

use crate::driver::Error;
use crate::driver::TestCase;

pub fn run(test_case: &TestCase) -> Result<Duration, Error> {
    //eprintln!("{}...", test_case.name);
    let mut runner = Runner::new();
    runner.setup_runtime();
    runner.load_harness(test_case)?;
    runner.perform_test(test_case)
}

// in-process runner

struct Runner {
    runtime: Runtime<Context>,
    start: Instant,
}

impl Runner {
    fn new() -> Self {
        Self {
            runtime: Runtime::with_extension(Default::default()),
            start: Instant::now(),
        }
    }

    fn setup_runtime(&mut self) {
        self.runtime.enable_scope_cleanup_checker();
        self.runtime.enable_runtime_assert();
        self.runtime.register_host_function("print", Self::print); // TODO
    }

    fn load_harness(&mut self, test_case: &TestCase) -> Result<(), Error> {
        for harness in test_case.includes.iter() {
            if self.run(Source::Script(harness.content())).is_err() {
                return Err(Error::Harness {
                    duration: Instant::elapsed(&self.start),
                    harness: harness.clone(),
                });
            }
        }
        Ok(())
    }

    fn perform_test(&mut self, test_case: &TestCase) -> Result<Duration, Error> {
        let content = match std::fs::read_to_string(&test_case.path) {
            Ok(content) => content,
            Err(err) => panic!("{}: {err:?}", test_case.path.display()),
        };
        let content = if test_case.strict {
            // TODO: the source location is shifted...
            format!("use strict;\n{content}")
        } else {
            content
        };
        let source = if test_case.metadata.is_module() {
            Source::Module(&content)
        } else {
            Source::Script(&content)
        };
        self.run(source)
    }

    fn run(&mut self, source: Source<'_>) -> Result<Duration, Error> {
        let result = match source {
            Source::Module(source) => self.runtime.parse_module(source),
            Source::Script(source) => self.runtime.parse_script(source),
        };
        let program_id = match result {
            Ok(program_id) => program_id,
            Err(error) => {
                return Err(Error::Parse {
                    duration: Instant::elapsed(&self.start),
                    error,
                });
            }
        };
        match self.runtime.run(program_id, true) {
            Ok(_value) => Ok(Instant::elapsed(&self.start)),
            Err(_value) => Err(Error::Runtime {
                duration: Instant::elapsed(&self.start),
            }),
        }
    }

    fn print(_runtime: &mut Runtime<Context>, _args: &[Value]) {}
}

struct Context;

impl Default for Context {
    fn default() -> Self {
        Self
    }
}

enum Source<'a> {
    Script(&'a str),
    Module(&'a str),
}
