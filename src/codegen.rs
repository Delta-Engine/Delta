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
        Ok(());
    }

    fn interpret(&mut self, program: &Program) -> Result<(), String> {
        for statement in &program.statements {
            self.interpret_statement(statement)?;
        }
    }

}