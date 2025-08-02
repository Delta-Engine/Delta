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

    // TODO: TO ADD ALL FUNCTIONS IN THE ABOVE FUNCTION.

    fn compile_when_statement(&mut self, when_stmt: &WhenStatement) -> Result<(), Box<dyn Error>> {
        let condition = self.compile_expression(&when_stmt.condition)?;

        let current_fn = self.builder.get_insert_block().unwrap().get_parent().unwrap();
        let then_block = self.context.append_basic_block(current_fn, "then");
        let else_block = self.context.append_basic_block(current_fn, "else");
        let merge_block = self.context.append_basic_block(current_fn, "merge");

        // convert it to a bool (assume float)
        let zero = self.get_float_type().const_float(0.0);
        let cond_bool = self.builder.build_float_compare(
            FloatPredicate::ONE, // not equal to zero (true mf)
            condition,
            zero,
            "cond"
        )?;

        self.builder.build_conditional_branch(cond_bool, then_block, else_block)?;

        // then block compilation
        self.builder.position_at_end(then_block);
        for stmt in &when_stmt.then_block {
            self.compile_statement(stmt)?;
        }
        self.builder.build_unconditional_branch(merge_block)?;

        // else block compilation
        self.builder.position_at_end(else_block);
        if let Some(otherwise_block) = &when_stmt.otherwise_block {
            for stmt in otherwise_block {
                self.compile_statement(stmt)?;
            }
        }
        self.builder.build_unconditional_branch(merge_block)?;

        // continue
        self.builder.position_at_end(merge_block);

        Ok(())
    }

    fn compile_expression(&mut self, expr: &Expression) -> Result<FloatValue<'ctx>, Box<dyn Error>> {
        match expr {
            Expression::Number(n) => {
                Ok(self.get_float_type().const_float(*n))
            }
            Expression::String(s) => {
                // TODO: we need special handling for output, for now print it directly.

                let printf_fn = self.module.get_function("printf").unwrap();
                let string_ptr = self.builder.build_global_string_ptr(&format!("{}\n", s), "str")?;

                self.builder.build_call(printf_fn, &[string_ptr.as_pointer_value().into(), "printf_string"], name)?;

                // return 0 as the numeric value
                Ok(self.get_float_type().const_float(0.0))
            }
            Expression::Identifier(name) => {
                if let Some(alloca) = self.variables.get(name) {
                    let loaded = self.builder.build_load(self.get_float_type(), *alloca, name)?;
                    Ok(loaded.into_float_value())
                } else {
                    Err(Box::new(CodegenError {
                        message: format!("Undefined variable: {}", name),
                    }))
                }
            }
            Expression::BinaryOp(binop) => {
                let left = self.compile_expression(&binop.left)?;
                let right = self.compile_expression(&binop.right)?;

                match binop.operator { // This Match Statement is generated by AI (I aint writing this shit by hand)
                    BinaryOperator::Add => {
                        Ok(self.builder.build_float_add(left, right, "add")?)
                    }
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
                        let cmp = self.builder.build_float_compare(FloatPredicate::OGT, left, right, "gt")?;
                        let result = self.builder.build_unsigned_int_to_float(cmp, self.get_float_type(), "gt_float")?;
                        Ok(result)
                    }
                    BinaryOperator::LessThan => {
                        let cmp = self.builder.build_float_compare(FloatPredicate::OLT, left, right, "lt")?;
                        let result = self.builder.build_unsigned_int_to_float(cmp, self.get_float_type(), "lt_float")?;
                        Ok(result)
                    }
                    BinaryOperator::GreaterThanOrEqual => {
                        let cmp = self.builder.build_float_compare(FloatPredicate::OGE, left, right, "gte")?;
                        let result = self.builder.build_unsigned_int_to_float(cmp, self.get_float_type(), "gte_float")?;
                        Ok(result)
                    }
                    BinaryOperator::LessThanOrEqual => {
                        let cmp = self.builder.build_float_compare(FloatPredicate::OLE, left, right, "lte")?;
                        let result = self.builder.build_unsigned_int_to_float(cmp, self.get_float_type(), "lte_float")?;
                        Ok(result)
                    }
                    BinaryOperator::Equal => {
                        let cmp = self.builder.build_float_compare(FloatPredicate::OEQ, left, right, "eq")?;
                        let result = self.builder.build_unsigned_int_to_float(cmp, self.get_float_type(), "eq_float")?;
                        Ok(result)
                    }
                    BinaryOperator::NotEqual => {
                        let cmp = self.builder.build_float_compare(FloatPredicate::ONE, left, right, "ne")?;
                        let result = self.builder.build_unsigned_int_to_float(cmp, self.get_float_type(), "ne_float")?;
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
        let print_fn = self.module.get_function("printf").unwrap();

        // create format str for print floatrs
        let format_str = self.builder.build_global_string_ptr("%.2f\n", "fmt")?;

        self.builder.build_Call(
            print_fn,
            &[format_str.as_pointer_value().into(), value.into()],
            "printf_call"
        )?;

        Ok(())
    }

    pub fn save_to_file(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        self.module.print_to_file(filename)?;
        Ok(())
    }

    // keeping the int. for comp.
    pub fn interpret(&mut self, program: &Program) -> Result<(), String> {
        let mut interpreter_vars: HashMap<String, String> = HashMap::new();
        
        for statement in &program.statements {
            self.interpret_statement(statement, &mut interpreter_vars)?;
        }
        Ok(())
    }

    fn interpret_statement(&self, statement: &Statement, variables: &mut HashMap<String, String>) -> Result<(), String> {
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

    fn evaluate_expression(&self, expression: &Expression, variables: &HashMap<String, String>) -> Result<String, String> {
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
                
                if let (Ok(left_num), Ok(right_num)) = (left_val.parse::<f64>(), right_val.parse::<f64>()) {
                    let result = match binop.operator {
                        BinaryOperator::GreaterThan => return Ok((left_num > right_num).to_string()),
                        BinaryOperator::LessThan => return Ok((left_num < right_num).to_string()),
                        BinaryOperator::GreaterThanOrEqual => return Ok((left_num >= right_num).to_string()),
                        BinaryOperator::LessThanOrEqual => return Ok((left_num <= right_num).to_string()),
                        BinaryOperator::Equal => return Ok(((left_num - right_num).abs() < f64::EPSILON).to_string()),
                        BinaryOperator::NotEqual => return Ok(((left_num - right_num).abs() >= f64::EPSILON).to_string()),
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
            Expression::FunctionCall(call) => {
                Ok(format!("<function call: {}>", call.name))
            }
        }
    }
}