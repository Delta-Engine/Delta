use crate::ast::*;
use inkwell::FloatPredicate;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::FloatType;
use inkwell::values::{FloatValue, FunctionValue, PointerValue};
use std::collections::HashMap;
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

enum VariableType<'ctx> {
    Float(PointerValue<'ctx>),
    String(PointerValue<'ctx>),
}

pub struct CodeGenerator<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    variables: HashMap<String, VariableType<'ctx>>,
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

    pub fn compile(&mut self, program: &Program) -> Result<(), Box<dyn Error>> {
        // Create main function
        let main_type = self.context.i32_type().fn_type(&[], false);
        let main_fn = self.module.add_function("main", main_type, None);
        let basic_block = self.context.append_basic_block(main_fn, "entry");
        self.builder.position_at_end(basic_block);

        // Add printf declaration for output
        self.add_printf_declaration();

        // Compile all statements
        for statement in &program.statements {
            self.compile_statement(statement)?;
        }

        // Return 0 from main
        let return_value = self.context.i32_type().const_int(0, false);
        self.builder.build_return(Some(&return_value))?;

        // Verify the module
        if let Err(errors) = self.module.verify() {
            return Err(Box::new(CodegenError {
                message: format!("Module verification failed: {}", errors),
            }));
        }

        println!("LLVM IR generated successfully!");
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
                match &let_stmt.value {
                    Expression::String(s) => {
                        // Handle string variables
                        let global_str = self
                            .builder
                            .build_global_string_ptr(s, &format!("str_{}", let_stmt.identifier))?;
                        let str_ptr = self.builder.build_alloca(
                            self.context
                                .i8_type()
                                .ptr_type(inkwell::AddressSpace::default()),
                            &let_stmt.identifier,
                        )?;
                        self.builder
                            .build_store(str_ptr, global_str.as_pointer_value())?;
                        self.variables
                            .insert(let_stmt.identifier.clone(), VariableType::String(str_ptr));
                    }
                    _ => {
                        // Handle numeric variables
                        let value = self.compile_expression(&let_stmt.value)?;
                        let alloca = self
                            .builder
                            .build_alloca(self.get_float_type(), &let_stmt.identifier)?;
                        self.builder.build_store(alloca, value)?;
                        self.variables
                            .insert(let_stmt.identifier.clone(), VariableType::Float(alloca));
                    }
                }
            }
            Statement::Show(show_stmt) => {
                match &show_stmt.value {
                    Expression::String(_) => {
                        self.compile_expression(&show_stmt.value)?;
                        // The string expression will handle its own printing
                    }
                    Expression::Identifier(name) => {
                        if let Some(var_type) = self.variables.get(name) {
                            match var_type {
                                VariableType::String(ptr) => {
                                    let loaded = self.builder.build_load(
                                        self.context
                                            .i8_type()
                                            .ptr_type(inkwell::AddressSpace::default()),
                                        *ptr,
                                        name,
                                    )?;

                                    // Create format string for printing with newline
                                    let format_str =
                                        self.builder.build_global_string_ptr("%s\n", "fmt_str")?;

                                    let printf_fn = self.module.get_function("printf").unwrap();
                                    self.builder.build_call(
                                        printf_fn,
                                        &[format_str.as_pointer_value().into(), loaded.into()],
                                        "printf_str_var",
                                    )?;
                                }
                                VariableType::Float(ptr) => {
                                    let loaded = self.builder.build_load(
                                        self.get_float_type(),
                                        *ptr,
                                        name,
                                    )?;
                                    self.generate_print_call(loaded.into_float_value())?;
                                }
                            }
                        } else {
                            return Err(Box::new(CodegenError {
                                message: format!("Undefined variable: {}", name),
                            }));
                        }
                    }
                    _ => {
                        let value = self.compile_expression(&show_stmt.value)?;
                        self.generate_print_call(value)?;
                    }
                }
            }
            Statement::When(when_stmt) => {
                self.compile_when_statement(when_stmt)?;
            }
            Statement::FunctionDef(_) => {
                // TODO: Implement function definitions
                println!("Function definitions not yet implemented in compiler");
            }
            Statement::Expression(expr) => {
                self.compile_expression(expr)?;
            }
        }
        Ok(())
    }

    fn compile_when_statement(&mut self, when_stmt: &WhenStatement) -> Result<(), Box<dyn Error>> {
        let condition = self.compile_expression(&when_stmt.condition)?;

        let current_fn = self
            .builder
            .get_insert_block()
            .unwrap()
            .get_parent()
            .unwrap();
        let then_block = self.context.append_basic_block(current_fn, "then");
        let else_block = self.context.append_basic_block(current_fn, "else");
        let merge_block = self.context.append_basic_block(current_fn, "merge");

        // Convert the condition to a boolean (assume it's a float comparison result)
        let zero = self.get_float_type().const_float(0.0);
        let cond_bool = self.builder.build_float_compare(
            FloatPredicate::ONE, // not equal to zero (true)
            condition,
            zero,
            "cond",
        )?;

        self.builder
            .build_conditional_branch(cond_bool, then_block, else_block)?;

        // Compile then block
        self.builder.position_at_end(then_block);
        for stmt in &when_stmt.then_block {
            self.compile_statement(stmt)?;
        }
        self.builder.build_unconditional_branch(merge_block)?;

        // Compile else block
        self.builder.position_at_end(else_block);
        if let Some(otherwise_block) = &when_stmt.otherwise_block {
            for stmt in otherwise_block {
                self.compile_statement(stmt)?;
            }
        }
        self.builder.build_unconditional_branch(merge_block)?;

        // Continue from merge block
        self.builder.position_at_end(merge_block);

        Ok(())
    }

    fn compile_expression(
        &mut self,
        expr: &Expression,
    ) -> Result<FloatValue<'ctx>, Box<dyn Error>> {
        match expr {
            Expression::Number(n) => Ok(self.get_float_type().const_float(*n)),
            Expression::String(s) => {
                // For strings, we need special handling for output
                // For now, we'll just print the string directly
                let printf_fn = self.module.get_function("printf").unwrap();
                let string_ptr = self
                    .builder
                    .build_global_string_ptr(&format!("{}\n", s), "str")?;
                self.builder.build_call(
                    printf_fn,
                    &[string_ptr.as_pointer_value().into()],
                    "printf_string",
                )?;

                // Return 0 as the numeric value
                Ok(self.get_float_type().const_float(0.0))
            }
            Expression::Identifier(name) => {
                if let Some(var_type) = self.variables.get(name) {
                    match var_type {
                        VariableType::Float(ptr) => {
                            let loaded =
                                self.builder.build_load(self.get_float_type(), *ptr, name)?;
                            Ok(loaded.into_float_value())
                        }
                        VariableType::String(_) => {
                            // String identifiers used in expressions evaluate to 0
                            Ok(self.get_float_type().const_float(0.0))
                        }
                    }
                } else {
                    Err(Box::new(CodegenError {
                        message: format!("Undefined variable: {}", name),
                    }))
                }
            }
            Expression::BinaryOp(binop) => {
                let left = self.compile_expression(&binop.left)?;
                let right = self.compile_expression(&binop.right)?;

                match binop.operator {
                    BinaryOperator::Add => Ok(self.builder.build_float_add(left, right, "add")?),
                    BinaryOperator::Subtract => {
                        Ok(self.builder.build_float_sub(left, right, "sub")?)
                    }
                    BinaryOperator::Multiply => {
                        Ok(self.builder.build_float_mul(left, right, "mul")?)
                    }
                    BinaryOperator::Divide => {
                        Ok(self.builder.build_float_div(left, right, "div")?)
                    }
                    BinaryOperator::GreaterThan => {
                        let cmp = self.builder.build_float_compare(
                            FloatPredicate::OGT,
                            left,
                            right,
                            "gt",
                        )?;
                        let result = self.builder.build_unsigned_int_to_float(
                            cmp,
                            self.get_float_type(),
                            "gt_float",
                        )?;
                        Ok(result)
                    }
                    BinaryOperator::LessThan => {
                        let cmp = self.builder.build_float_compare(
                            FloatPredicate::OLT,
                            left,
                            right,
                            "lt",
                        )?;
                        let result = self.builder.build_unsigned_int_to_float(
                            cmp,
                            self.get_float_type(),
                            "lt_float",
                        )?;
                        Ok(result)
                    }
                    BinaryOperator::GreaterThanOrEqual => {
                        let cmp = self.builder.build_float_compare(
                            FloatPredicate::OGE,
                            left,
                            right,
                            "gte",
                        )?;
                        let result = self.builder.build_unsigned_int_to_float(
                            cmp,
                            self.get_float_type(),
                            "gte_float",
                        )?;
                        Ok(result)
                    }
                    BinaryOperator::LessThanOrEqual => {
                        let cmp = self.builder.build_float_compare(
                            FloatPredicate::OLE,
                            left,
                            right,
                            "lte",
                        )?;
                        let result = self.builder.build_unsigned_int_to_float(
                            cmp,
                            self.get_float_type(),
                            "lte_float",
                        )?;
                        Ok(result)
                    }
                    BinaryOperator::Equal => {
                        let cmp = self.builder.build_float_compare(
                            FloatPredicate::OEQ,
                            left,
                            right,
                            "eq",
                        )?;
                        let result = self.builder.build_unsigned_int_to_float(
                            cmp,
                            self.get_float_type(),
                            "eq_float",
                        )?;
                        Ok(result)
                    }
                    BinaryOperator::NotEqual => {
                        let cmp = self.builder.build_float_compare(
                            FloatPredicate::ONE,
                            left,
                            right,
                            "ne",
                        )?;
                        let result = self.builder.build_unsigned_int_to_float(
                            cmp,
                            self.get_float_type(),
                            "ne_float",
                        )?;
                        Ok(result)
                    }
                }
            }
            Expression::FunctionCall(_) => {
                // TODO: Implement function calls
                Ok(self.get_float_type().const_float(0.0))
            }
        }
    }

    fn generate_print_call(&mut self, value: FloatValue<'ctx>) -> Result<(), Box<dyn Error>> {
        let printf_fn = self.module.get_function("printf").unwrap();

        // Create format string for printing floats
        let format_str = self.builder.build_global_string_ptr("%.2f\n", "fmt")?;

        self.builder.build_call(
            printf_fn,
            &[format_str.as_pointer_value().into(), value.into()],
            "printf_call",
        )?;

        Ok(())
    }

    pub fn save_to_file(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        self.module.print_to_file(filename)?;
        Ok(())
    }

    // Keep the interpreter for comparison
    pub fn interpret(&mut self, program: &Program) -> Result<(), String> {
        let mut interpreter_vars: HashMap<String, String> = HashMap::new();

        for statement in &program.statements {
            self.interpret_statement(statement, &mut interpreter_vars)?;
        }
        Ok(())
    }

    fn interpret_statement(
        &self,
        statement: &Statement,
        variables: &mut HashMap<String, String>,
    ) -> Result<(), String> {
        match statement {
            Statement::Show(show) => {
                let value = self.evaluate_expression(&show.value, variables)?;
                println!("{}", value);
            }
            Statement::Let(let_stmt) => {
                let value = self.evaluate_expression(&let_stmt.value, variables)?;
                variables.insert(let_stmt.identifier.clone(), value);
            }
            Statement::When(when_stmt) => {
                let condition_result = self.evaluate_expression(&when_stmt.condition, variables)?;
                if condition_result.contains("true") || condition_result.contains("True") {
                    for stmt in &when_stmt.then_block {
                        self.interpret_statement(stmt, variables)?;
                    }
                } else if let Some(otherwise_block) = &when_stmt.otherwise_block {
                    for stmt in otherwise_block {
                        self.interpret_statement(stmt, variables)?;
                    }
                }
            }
            Statement::FunctionDef(func_def) => {
                println!("Defined function: {}", func_def.name);
            }
            Statement::Expression(expr) => {
                let _value = self.evaluate_expression(expr, variables)?;
            }
        }
        Ok(())
    }

    fn evaluate_expression(
        &self,
        expression: &Expression,
        variables: &HashMap<String, String>,
    ) -> Result<String, String> {
        match expression {
            Expression::Number(n) => Ok(n.to_string()),
            Expression::String(s) => Ok(s.clone()),
            Expression::Identifier(name) => {
                if let Some(value) = variables.get(name) {
                    Ok(value.clone())
                } else {
                    Ok(format!("<undefined: {}>", name))
                }
            }
            Expression::BinaryOp(binop) => {
                let left_val = self.evaluate_expression(&binop.left, variables)?;
                let right_val = self.evaluate_expression(&binop.right, variables)?;

                if let (Ok(left_num), Ok(right_num)) =
                    (left_val.parse::<f64>(), right_val.parse::<f64>())
                {
                    let result = match binop.operator {
                        BinaryOperator::GreaterThan => {
                            return Ok((left_num > right_num).to_string());
                        }
                        BinaryOperator::LessThan => return Ok((left_num < right_num).to_string()),
                        BinaryOperator::GreaterThanOrEqual => {
                            return Ok((left_num >= right_num).to_string());
                        }
                        BinaryOperator::LessThanOrEqual => {
                            return Ok((left_num <= right_num).to_string());
                        }
                        BinaryOperator::Equal => {
                            return Ok(((left_num - right_num).abs() < f64::EPSILON).to_string());
                        }
                        BinaryOperator::NotEqual => {
                            return Ok(((left_num - right_num).abs() >= f64::EPSILON).to_string());
                        }
                        BinaryOperator::Add => left_num + right_num,
                        BinaryOperator::Subtract => left_num - right_num,
                        BinaryOperator::Multiply => left_num * right_num,
                        BinaryOperator::Divide => {
                            if right_num == 0.0 {
                                return Err("Division by zero".to_string());
                            }
                            left_num / right_num
                        }
                    };
                    Ok(result.to_string())
                } else {
                    Ok(format!("({} {:?} {})", left_val, binop.operator, right_val))
                }
            }
            Expression::FunctionCall(call) => Ok(format!("<function call: {}>", call.name)),
        }
    }
}
