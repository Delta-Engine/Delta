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

impl<'ctx> CodeGenerator<'ctx> {
    pub fn new(context: &'ctx Context, module_name: &str) -> Result<Self, Box<dyn Error>> {
        let module = context.create_module(module_name);
        let builder = context.create_builder();

        Ok(CodeGenerator {
            context,
            module,
            builder,
            variables: HashMap::new(),
        })
    }

    fn get_float_type(&self) -> FloatType<'ctx> {
        self.context.f64_type()
    }

    pub fn compile(&mu self, program: &Program) -> Result<(), Box<dyn Error>> {
        let main_type = self.context.i32_type().fn_type(&[], false);
        let main_fn = self.module.add_function("main", main_type, None);
        let basic_block = self.context.append_basic_block(main_fn, "entry");
        self.builder.position_at_end(basic_block);

        self.add_printf_declaration();

        for statement in &program.statements {
            self.compile_statement(statement)?;
        }

        let return_value = self.context.i32_type().const_int(0, false);
        self.builder.build_return(Some(&return_value))?;

        if let Err(errors) = self.module.verify() {
            return Err(Box::new(CodegenError {
                message: format!("Module verification failed: {}", errors),
            }));
        }

        println!("LLVM IR generated successfully!")
        self.module.print_to_stderr();
        Ok(())
    } 

    fn add_printf_declaration(&mut self) {
        let i8_type = self.context.i8_type();
        let i8_ptr_type = i8_type.ptr_type(inkwell::AddressSpace::default());
        let printf_type = self.context.i32_type().fn_type(&[i8_ptr_type.into()], true);

        self.module.add_function("printf", printf_type, None);
    }

    fn compile_statement(&mut self, statement: &Statement) -> Result<(), Box<dyn Error>> {
        match statement {
            Statement::Let(let_stmt) => {
                let value - self.compile_expression(&let_stmt.value)?;

                let alloc_s = self.builder.build_alloca(self.get_float_type(), &let_stmt.identifier)?;
                self.builder.build_store(alloc_s, value);
                self.variables.insert(let_stmt.identifier.clone(), alloc_s);
            }
            Statement::Show(show_stmt) => {
                let value = self.compile_expression(&show_stmt.value)?;
                self.generate_print_call(value)?;
            }
            Statement::When(when_stmt) => {
                self.compile_when_statement(when_stmt)?;
            }
            Statement::FunctionDef(_) => {
                // TODO: Implement function definitions
                println!("Function definitions not yet implemented in Compiler");
            }
            Statement::Expression(expr) => {
                self.compile_expression(expr)?;
            }
        }

        Ok(())
    }

    // TO ADD ALL FUNCTIONS IN THE ABOVE FUNCTION.
}