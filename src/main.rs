mod ast;
mod codegen;

use inkwell::context::Context;
use inkwell::OptimizationLevel;
use inkwell::execution_engine::JitFunction;
use ress::Scanner;
use std::error::Error;
use ast::{parse_expression, Expr};
use codegen::generate_code;

fn main() -> Result<(), Box<dyn Error>> {
    // Exemplo de entrada JavaScript
    let input = "42 + 3;";

    // Parsear a entrada
    let mut scanner = Scanner::new(input);
    let mut tokens = Vec::new();
    while let Some(item) = scanner.next() {
        match item {
            Ok(token) => tokens.push(token.token),
            Err(e) => eprintln!("Error: {:?}", e),
        }
    }

    // Construir AST
    let expr = parse_expression(tokens)?;

    // Extrair lhs e rhs da AST para uso posterior
    let (lhs, rhs) = match expr {
        Expr::Add(ref lhs, ref rhs) => match (&**lhs, &**rhs) {
            (Expr::Number(lhs), Expr::Number(rhs)) => (*lhs as i32, *rhs as i32),
            _ => return Err("Error parsing input".into()),
        },
        _ => return Err("Error parsing input".into()),
    };

    // LLVM and JIT part
    let context = Context::create();
    let module = context.create_module("js_module");
    let execution_engine = module.create_jit_execution_engine(OptimizationLevel::None)?;

    generate_code(&context, &module, &expr);

    // Ensure the module is verified before running the function
    module.print_to_stderr();

    unsafe {
        let sum: JitFunction<codegen::SumFunc> = execution_engine.get_function("sum")?;
        let result = sum.call(lhs, rhs);
        println!("{} + {} = {}", lhs, rhs, result);
    }

    Ok(())
}
