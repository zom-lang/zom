//! Module related to the transformation of the AST to a LLVM IR.

use std::{collections::HashMap, borrow::Borrow};

use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
    passes::PassManager,
    values::{FunctionValue, PointerValue, FloatValue, IntValue, BasicMetadataValueEnum}, IntPredicate,
};

use crate::fe::parser::{Function, Expression};

/// Defines the `Expression` compiler.
pub struct Compiler<'a, 'ctx> {
    pub context: &'ctx Context,
    pub builder: &'a Builder<'ctx>,
    pub fpm: &'a PassManager<FunctionValue<'ctx>>,
    pub module: &'a Module<'ctx>,
    pub function: &'a Function,

    variables: HashMap<String, PointerValue<'ctx>>,
    fn_value_opt: Option<FunctionValue<'ctx>>,
}

impl<'a, 'ctx> Compiler<'a, 'ctx> {
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
        match *expr {
            Expression::LiteralExpr(nb) => Ok(self.context.i32_type().const_int(nb as u64, true)),

            Expression::VariableExpr(ref name) => match self.variables.get(name.as_str()) {
                Some(var) => Ok(self.builder.build_load(self.context.i32_type(), *var, name.as_str()).into_int_value()),
                None => Err("Could not find a matching variable."),
            },

            Expression::BinaryExpr (
                op,
                ref left,
                ref right,
             ) => {
                if op == '=' {
                    // handle assignement
                    let var_name = match *left.borrow() {
                        Expression::VariableExpr(ref var_name) => var_name,
                        _ => {
                            return Err("Expected variable as left-hand operator of assignement.");
                        },
                    };

                    let var_val = self.compile_expr(right)?;
                    let var = self.variables.get(var_name.as_str()).ok_or("Undefined variable.")?;

                    self.builder.build_store(*var, var_val);

                    Ok(var_val)
                } else {
                    let lhs = self.compile_expr(left)?;
                    let rhs = self.compile_expr(right)?;

                    match op {
                        '+' => Ok(self.builder.build_int_add(lhs, rhs, "tmpadd")),
                        '-' => Ok(self.builder.build_int_sub(lhs, rhs, "tmpsub")),
                        '*' => Ok(self.builder.build_int_mul(lhs, rhs, "tmpmul")),
                        '/' => Ok(self.builder.build_int_signed_div(lhs, rhs, "tmpdiv")),
                        '<' => Ok({
                            let cmp = self
                                .builder
                                .build_int_compare(IntPredicate::ULT, lhs, rhs, "tmpcmp");

                            self.builder
                                .build_int_cast(cmp, self.context.i32_type(), "tmpbool")
                        }),
                        '>' => Ok({
                            let cmp = self
                                .builder
                                .build_int_compare(IntPredicate::ULT, rhs, lhs, "tmpcmp");

                            self.builder
                                .build_int_cast(cmp, self.context.i32_type(), "tmpbool")
                        }),

                        custom => {
                            let mut name = String::from("binary");

                            name.push(custom);

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
                                },

                                None => Err("Undefined binary operator."),
                            }
                        },
                    }
                }
            },

            Expression::CallExpr ( ref fn_name, ref args ) => match self.get_function(fn_name.as_str()) {
                Some(fun) => {
                    let mut compiled_args = Vec::with_capacity(args.len());

                    for arg in args {
                        compiled_args.push(self.compile_expr(arg)?);
                    }

                    let argsv: Vec<BasicMetadataValueEnum> =
                        compiled_args.iter().by_ref().map(|&val| val.into()).collect();

                    match self
                        .builder
                        .build_call(fun, argsv.as_slice(), "tmp")
                        .try_as_basic_value()
                        .left()
                    {
                        Some(value) => Ok(value.into_int_value()),
                        None => Err("Invalid call produced."),
                    }
                },
                None => Err("Unknown function."),
            },
        }
    }
}
