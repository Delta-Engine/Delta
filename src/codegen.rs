use crate::ast::*;

pub struct CodeGenerator {
    // TODO: Add Compiler.
    // For now, this will be interpreted.
}

impl CodeGenerator {
    pub fn new() -> Self {
        CodeGenerator {}
    }

    pub fn generate(&mut self, _program: &Program) -> Result<(), String> {
        // TODO: Add LLVM IR Generation Code.
        // For now, we wil just interpret the AST.

        println!("Code Generator not Implemented yet. Using Interpreter.");
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
            Statement::Let(_) => {
                // TODO: Add Let Statement Match
                // For now, we wil acknowledge the let statement.
                println!("Let statement (not yet implemented in interpreter)");
            }
            Statement::When(_) => {
                // TODO: Add When Statement Match
                // For now, we wil acknowledge the when statement.
                println!("When statement (not yet implemented in interpreter)");
            }
            Statement::FunctionDef(_) => {
                // TODO: Add Function Statement Match
                // For now, we wil acknowledge the function definition statement.
                println!("Function definition (not yet implemented in interpreter)");
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
                // TODO: Add Identifier Match.
                // For now, les just return the identifier name.

                Ok(format!("<identifier: {}>", name))
            }
            Expression::BinaryOp(binop) => {
                let left = self.evaluate_expression(&binop.left)?;
                let right = self.evaluate_expression(&binop.right)?;
                Ok(format!("({} {:?} {})", left, binop.operator, right))
            }
            Expression::FunctionCall(call) => {
                Ok(format!("<function call: {}>", call.name))
            }
        }
    }

}