// // use std::collections::HashMap;

// use inkwell::{context::Context as LLVMContext, builder::Builder, types::IntType};

// pub struct Context<'a> {
//     context: Box<LLVMContext>,
//     builder: Builder<'a>,
//     // named_values: HashMap<String, LLVMValueRef>,
//     typ: IntType<'a>
// }

// impl<'a> Context<'a> {
//     pub fn new() -> Context<'a> {

//         let context = Box::new(LLVMContext::create());
//         let builder = context.create_builder();
//         // let named_values = HashMap::new();
//         let typ = context.i32_type();

//         Context { 
//             context,
//             builder,
//             //named_values,
//             typ,
//         }
//     }
// }