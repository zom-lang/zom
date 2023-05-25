// use std::collections::HashMap;

use inkwell::context::Context;
use inkwell::builder::Builder;

pub struct CodeGen<'ctx> {
    context: &'ctx Context,
    builder: Builder<'ctx>,
    // named_values: HashMap<String, LLVMValueRef>
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(context: &'ctx Context, builder: Builder<'ctx>) -> CodeGen<'ctx> {
        CodeGen {
            context,
            builder,
        }
    }
}