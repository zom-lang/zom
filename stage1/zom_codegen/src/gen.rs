//! Module related to the transformation of the AST to a LLVM IR.

use std::{borrow::Borrow, collections::HashMap};

use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
    passes::PassManager,
    types::BasicMetadataTypeEnum,
    values::{BasicMetadataValueEnum, FunctionValue, IntValue, PointerValue},
    IntPredicate,
};

use zom_fe::parser::{
    expr::Expression,
    function::{Function, Prototype},
    ASTNode,
};

use zom_common::token::*;

/// Defines the `Expression` compiler.
pub struct CodeGen<'a, 'ctx> {
    pub context: &'ctx Context,
    pub builder: &'a Builder<'ctx>,
    pub fpm: &'a PassManager<FunctionValue<'ctx>>,
    pub module: &'a Module<'ctx>,
    pub function: &'a Function,

    variables: HashMap<String, PointerValue<'ctx>>,
    fn_value_opt: Option<FunctionValue<'ctx>>,
}

pub type GeneratedCode<'ctx> = Result<Vec<FunctionValue<'ctx>>, &'static str>;

impl<'a, 'ctx> CodeGen<'a, 'ctx> {
    /// Gets a defined function given its name.
    #[inline]
    fn get_function(&self, name: &str) -> Option<FunctionValue<'ctx>> {
        self.module.get_function(name)
    }

    /// Returns the `FunctionValue` representing the function being compiled.
    ///
    /// Panic:
    ///   Can panic because of the use of `unwrap`
    #[inline]
    fn fn_value(&self) -> FunctionValue<'ctx> {
        self.fn_value_opt.unwrap()
    }

    /// Creates a new stack allocation instruction in the entry block of the function.
    fn create_entry_block_alloca(&self, name: &str) -> PointerValue<'ctx> {
        let builder = self.context.create_builder();

        let entry = self.fn_value().get_first_basic_block().unwrap();

        match entry.get_first_instruction() {
            Some(first_instr) => builder.position_before(&first_instr),
            None => builder.position_at_end(entry),
        }

        builder.build_alloca(self.context.f64_type(), name)
    }

    /// Compiles the specified `Expr` into an LLVM `FloatValue`.
    fn compile_expr(&mut self, expr: &Expression) -> Result<IntValue<'ctx>, &'static str> {
        match expr {
            Expression::LiteralExpr(nb) => Ok(self.context.i32_type().const_int(*nb as u64, true)),

            Expression::VariableExpr(ref name) => match self.variables.get(name.as_str()) {
                Some(var) => Ok(self
                    .builder
                    .build_load(self.context.i32_type(), *var, name.as_str())
                    .into_int_value()),
                None => Err("Could not find a matching variable."),
            },

            Expression::BinaryExpr {
                op,
                lhs: ref left,
                rhs: ref right,
            } => {
                if op == &"=".to_owned() {
                    // handle assignement
                    let var_name = match *left.borrow() {
                        Expression::VariableExpr(ref var_name) => var_name,
                        _ => {
                            return Err("Expected variable as left-hand operator of assignement.");
                        }
                    };

                    let var_val = self.compile_expr(right)?;
                    let var = self
                        .variables
                        .get(var_name.as_str())
                        .ok_or("Undefined variable.")?;

                    self.builder.build_store(*var, var_val);

                    Ok(var_val)
                } else {
                    let lhs = self.compile_expr(left)?;
                    let rhs = self.compile_expr(right)?;

                    match op.as_str() {
                        OP_PLUS => Ok(self.builder.build_int_add(lhs, rhs, "tmpadd")),
                        OP_MINUS => Ok(self.builder.build_int_sub(lhs, rhs, "tmpsub")),
                        OP_MUL => Ok(self.builder.build_int_mul(lhs, rhs, "tmpmul")),
                        OP_DIV => Ok(self.builder.build_int_signed_div(lhs, rhs, "tmpdiv")),
                        OP_COMP_LT => Ok({
                            let cmp = self.builder.build_int_compare(
                                IntPredicate::ULT,
                                lhs,
                                rhs,
                                "tmpcmp",
                            );

                            self.builder
                                .build_int_cast(cmp, self.context.i32_type(), "tmpbool")
                        }),
                        OP_COMP_GT => Ok({
                            let cmp = self.builder.build_int_compare(
                                IntPredicate::ULT,
                                rhs,
                                lhs,
                                "tmpcmp",
                            );

                            self.builder
                                .build_int_cast(cmp, self.context.i32_type(), "tmpbool")
                        }),

                        custom => {
                            let mut name = String::from("binary");

                            name.push_str(custom);

                            match self.get_function(name.as_str()) {
                                Some(fun) => {
                                    match self
                                        .builder
                                        .build_call(fun, &[lhs.into(), rhs.into()], "tmpbin")
                                        .try_as_basic_value()
                                        .left()
                                    {
                                        Some(value) => Ok(value.into_int_value()),
                                        None => Err("Invalid call produced."),
                                    }
                                }

                                None => Err("Undefined binary operator."),
                            }
                        }
                    }
                }
            }

            Expression::CallExpr(ref fn_name, ref args) => {
                match self.get_function(fn_name.as_str()) {
                    Some(fun) => {
                        let mut compiled_args = Vec::with_capacity(args.len());

                        for arg in args {
                            compiled_args.push(self.compile_expr(arg)?);
                        }

                        let argsv: Vec<BasicMetadataValueEnum> = compiled_args
                            .iter()
                            .by_ref()
                            .map(|&val| val.into())
                            .collect();

                        match self
                            .builder
                            .build_call(fun, argsv.as_slice(), "tmp")
                            .try_as_basic_value()
                            .left()
                        {
                            Some(value) => Ok(value.into_int_value()),
                            None => Err("Invalid call produced."),
                        }
                    }
                    None => Err("Unknown function."),
                }
            }
            _ => Err("expression not covered by the codegen."),
        }
    }

    /// Compiles the specified `Prototype` into an extern LLVM `FunctionValue`.
    fn compile_prototype(&self, proto: &Prototype) -> Result<FunctionValue<'ctx>, &'static str> {
        let ret_type = self.context.i32_type();
        let args_types = std::iter::repeat(ret_type)
            .take(proto.args.len())
            .map(|f| f.into())
            .collect::<Vec<BasicMetadataTypeEnum>>();
        let args_types = args_types.as_slice();

        let fn_type = self.context.i32_type().fn_type(args_types, false);
        let fn_val = self.module.add_function(proto.name.as_str(), fn_type, None);

        // set arguments names
        for (i, arg) in fn_val.get_param_iter().enumerate() {
            arg.set_name(&proto.args[i].name);
        }

        // finally return built prototype
        Ok(fn_val)
    }

    /// Compiles the specified `Function` into an LLVM `FunctionValue`.
    fn compile_fn(&mut self) -> Result<FunctionValue<'ctx>, &'static str> {
        let proto = &self.function.prototype;
        let function = self.compile_prototype(proto)?;

        // got external function, returning only compiled prototype
        if self.function.body.is_none() {
            return Ok(function);
        }

        let entry = self.context.append_basic_block(function, "entry");

        self.builder.position_at_end(entry);

        // update fn field
        self.fn_value_opt = Some(function);

        // build variables map
        self.variables.reserve(proto.args.len());

        for (i, arg) in function.get_param_iter().enumerate() {
            let arg_name = proto.args[i].name.as_str();
            let alloca = self.create_entry_block_alloca(arg_name);

            self.builder.build_store(alloca, arg);

            self.variables.insert(proto.args[i].name.clone(), alloca);
        }

        // compile body
        let body = self.compile_expr(self.function.body.as_ref().unwrap())?;

        self.builder.build_return(Some(&body));

        // return the whole thing after verification and optimization
        if function.verify(true) {
            self.fpm.run_on(&function);

            Ok(function)
        } else {
            unsafe {
                function.delete();
            }

            Err("Invalid generated function.")
        }
    }

    /// Compiles the specified `Function` in the given `Context` and using the specified `Builder`, `PassManager`, and `Module`.
    pub fn compile(
        context: &'ctx Context,
        builder: &'a Builder<'ctx>,
        pass_manager: &'a PassManager<FunctionValue<'ctx>>,
        module: &'a Module<'ctx>,
        function: &Function,
    ) -> Result<FunctionValue<'ctx>, &'static str> {
        let mut compiler = CodeGen {
            context,
            builder,
            fpm: pass_manager,
            module,
            function,
            fn_value_opt: None,
            variables: HashMap::new(),
        };

        compiler.compile_fn()
    }

    /// Compiles the specified `AST` in the given `Context` and using the specified `Builder`, `PassManager`, and `Module`.
    ///
    /// This call either `compile_fn(...)` if it's a FunctionNode or,
    /// calls `compile_ext(...)` if it's a ExternNode
    pub fn compile_ast(
        context: &'ctx Context,
        builder: &'a Builder<'ctx>,
        pass_manager: &'a PassManager<FunctionValue<'ctx>>,
        module: &'a Module<'ctx>,
        ast: &[ASTNode],
    ) -> GeneratedCode<'ctx> {
        let mut result = vec![];

        for node in ast {
            match node {
                ASTNode::FunctionNode(fun) => {
                    let mut compiler = CodeGen {
                        context,
                        builder,
                        fpm: pass_manager,
                        module,
                        function: fun,
                        fn_value_opt: None,
                        variables: HashMap::new(),
                    };

                    result.push(compiler.compile_fn()?);
                }
            }
        }

        Ok(result)
    }
}
