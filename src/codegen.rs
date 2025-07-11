use crate::ast::*;
use std::collections::HashMap;

pub struct CodeGenerator {
    variables: HashMap<String, String>,
}

impl CodeGenerator {
    pub fn new() -> Self {
        CodeGenerator {
            variables: HashMap::new(),
        }
    }
    
    pub fn generate(&mut self, _program: &Program) -> Result<(), String> {
        // TODO: Add LLVM IR Generation Code.
        // For now, we wil just interpret the AST.
        println!("Code generation not yet implemented - use interpreter");
        Ok(())
    }
    
    pub fn interpret(&mut self, program: &Program) -> Result<(), String> {
        for statement in &program.statements {
            self.interpret_statement(statement)?;
        }
        Ok(())
    }
    
    fn interpret_statement(&mut self, statement: &Statement) -> Result<(), String> {
        match statement {
            Statement::Show(show) => {
                let value = self.evaluate_expression(&show.value)?;
                println!("{}", value);
            }
            Statement::Let(let_stmt) => {
                let value = self.evaluate_expression(&let_stmt.value)?;
                self.variables.insert(let_stmt.identifier.clone(), value);
            }
            Statement::When(when_stmt) => {
                let condition_result = self.evaluate_expression(&when_stmt.condition)?;
                // For now, simple string-based evaluation
                if condition_result.contains("true") || condition_result.contains("True") {
                    for stmt in &when_stmt.then_block {
                        self.interpret_statement(stmt)?;
                    }
                } else if let Some(otherwise_block) = &when_stmt.otherwise_block {
                    for stmt in otherwise_block {
                        self.interpret_statement(stmt)?;
                    }
                }
            }
            Statement::FunctionDef(func_def) => {
                println!("Defined function: {}", func_def.name);
            }
            Statement::Expression(expr) => {
                let _value = self.evaluate_expression(expr)?;
                // Expression statements don't print by default
            }
        }
        Ok(())
    }
    
    fn evaluate_expression(&mut self, expression: &Expression) -> Result<String, String> {
        match expression {
            Expression::Number(n) => Ok(n.to_string()),
            Expression::String(s) => Ok(s.clone()),
            Expression::Identifier(name) => {
                if let Some(value) = self.variables.get(name) {
                    Ok(value.clone())
                } else {
                    Ok(format!("<undefined: {}>", name))
                }
            }
            Expression::BinaryOp(binop) => {
                let left_val = self.evaluate_expression(&binop.left)?;
                let right_val = self.evaluate_expression(&binop.right)?;
                
                // Try to parse as numbers for comparison
                if let (Ok(left_num), Ok(right_num)) = (left_val.parse::<f64>(), right_val.parse::<f64>()) {
                    let result = match binop.operator {
                        BinaryOperator::GreaterThan => left_num > right_num,
                        BinaryOperator::LessThan => left_num < right_num,
                        BinaryOperator::GreaterThanOrEqual => left_num >= right_num,
                        BinaryOperator::LessThanOrEqual => left_num <= right_num,
                        BinaryOperator::Equal => (left_num - right_num).abs() < f64::EPSILON,
                        BinaryOperator::NotEqual => (left_num - right_num).abs() >= f64::EPSILON,
                        _ => return Ok(format!("({} {:?} {})", left_val, binop.operator, right_val)),
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