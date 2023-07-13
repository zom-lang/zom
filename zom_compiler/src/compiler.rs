use std::path::Path;

use inkwell::{
    module::Module,
    targets::{
        CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine, TargetTriple,
    },
    OptimizationLevel,
};

pub struct Compiler {
    triple: TargetTriple,
    config: InitializationConfig,
}

impl Compiler {
    pub fn compile(
        triple: TargetTriple,
        config: InitializationConfig,
        module: Module,
        output: &Path,
    ) -> Result<(), inkwell::support::LLVMString> {
        let compiler = Compiler { triple, config };

        Target::initialize_all(&compiler.config);

        let target = Target::from_triple(&compiler.triple)
            .expect("Error, while trying to get the current target from the target triple.");

        let target_machine = target
            .create_target_machine(
                &compiler.triple,
                TargetMachine::get_host_cpu_name()
                    .to_str()
                    .expect("Things went wrong"), // if it doesn't work, try with "generic"
                TargetMachine::get_host_cpu_features()
                    .to_str()
                    .expect("Things went wrong"),
                OptimizationLevel::Default,
                RelocMode::Default,
                CodeModel::Default,
            )
            .unwrap();

        module.set_triple(&compiler.triple);

        target_machine.write_to_file(&module, FileType::Object, output)
    }

    pub fn compile_default(
        module: Module,
        output: &Path,
    ) -> Result<(), inkwell::support::LLVMString> {
        let init_config = InitializationConfig {
            info: true,
            machine_code: true,
            asm_parser: true,
            asm_printer: true,
            base: true,
            disassembler: false,
        };

        Self::compile(
            TargetMachine::get_default_triple(),
            init_config,
            module,
            output,
        )
    }
}
