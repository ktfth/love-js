use inkwell::context::Context;
use inkwell::values::{BasicValueEnum, FunctionValue};
use crate::ast::Expr;

pub type SumFunc = unsafe extern "C" fn(i32, i32) -> i32;

pub fn generate_code<'ctx>(context: &'ctx Context, module: &inkwell::module::Module<'ctx>, expr: &Expr) -> FunctionValue<'ctx> {
    let int32_type = context.i32_type();
    let fn_type = int32_type.fn_type(&[int32_type.into(), int32_type.into()], false);
    let function = module.add_function("sum", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    let builder = context.create_builder();
    builder.position_at_end(basic_block);

    match expr {
        Expr::Number(value) => {
            let number = int32_type.const_int(*value as u64, false);
            let _ = builder.build_return(Some(&number));
        }
        Expr::Add(left, right) => {
            if let (Expr::Number(left_val), Expr::Number(right_val)) = (&**left, &**right) {
                let left = int32_type.const_int(*left_val as u64, false);
                let right = int32_type.const_int(*right_val as u64, false);
                let sum = builder.build_int_add(left, right, "sum").expect("Error building int add");
                let sum_value: BasicValueEnum = sum.into();
                let _ = builder.build_return(Some(&sum_value));
            }
        }
    }

    function
}
