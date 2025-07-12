use crate::ast::*;
use std::collections::HashMap;
use inkwell::context::Context;
use inkwell::builder::Builder;
use inkwell::module::Module;
use inkwell::values::{FloatValue, FunctionValue, PointerValue};
use inkwell::types::FloatType;
use inkwell::FloatPredicate;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct CodegenError {
    message: String,
}

impl fmt::Display for CodegenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for CodegenError {}

pub struct CodeGenerator<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    variables: Hashmap<String, PointerValue<'ctx>>,
}