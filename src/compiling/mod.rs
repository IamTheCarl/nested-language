
use crate::parsing::*;
use std::collections::HashMap;

// All tests are kept in their own module.
#[cfg(test)]
mod tests;

// The representation of functions is done using a stack.
// Arguments are proveded by leaving them on the stack.
// Values are returned by leaving them on the stack.

enum CompileError<'a> {
    VariableUndefined(&'a str), // String is the name of the variable.
    TypeUnspecified, // We do not yet support type derive. The type must be specified.
}

struct ObjectFile {
    functions: HashMap<String, Function>,
    structs: HashMap<String, Struct>,
    enums: HashMap<String, Enum>,
}

enum ArithmeticMode {
    I8, I16, I32, I64,
    U8, U16, U32, U64,
}

enum OperandSource {
    Constant,
    Struct,
}

enum Instruction {
    StackPush,
    StackPop,
}

struct Struct {

}

struct Enum {

}

struct Function {
    local_variable_name_map: HashMap<String, usize>,
    instructions: Vec<Instruction>,
}

// fn compile_function(function: NLFunction) {
//     if let Some(block) = function.get_block() {
//         compile_block(&block);
//     } else {
//         // TODO return some kind of linkable function signature.
//     }
// }

fn compile_block(block: &NLBlock, instructions: &mut Vec<Instruction>) {
    let operations = block.get_operations();
    
    let mut type_stack: Vec<NLType> = Vec::new();
    let mut variable_map: HashMap<String, usize> = HashMap::new();

    for operation in operations {
        match operation {
            NLOperation::Block(block) => {
                compile_block(block, instructions);
            },
            NLOperation::Constant(constant) => {

            },
            NLOperation::Assign(assignment) => {
                if assignment.is_new() {
                    // So this means we have something like "let variable = 10".

                    let types = assignment.get_types();
                    let variables = assignment.get_variable_to_assign();

                    for i in 0..variables.len() {
                        let variable = &variables[i];
                        let type_assignment = &types.get(i);

                        if let Some(type_assignment) = type_assignment {
                            let pre_existing = variable_map.get(variable.get_name());
                            if let Some(pre_existing) = pre_existing {
                                // Okay so the variable already exists. Is it the same type?
                                let stack_type = &type_stack[*pre_existing];
    
                                // if stack_type == type_assignment {
                                //     // We have the same type. We can reuse the stack index.
                                //     // To do so, we don't have to do anything.
                                // } else {
                                //     // Its a new type.
                                // }
    
                            } else {
    
                            }
                        } else {

                        }
                    }
                } else {
                    
                }
            },
            NLOperation::VariableAccess(variable) => {

            },
            NLOperation::Tuple(operations) => {

            },
            NLOperation::Operator(operator) => {

            },
            NLOperation::If(if_statement) => {

            },
            NLOperation::Loop(loop_block) => {

            },
            NLOperation::WhileLoop(while_loop) => {

            },
            NLOperation::ForLoop(for_loop) => {

            },
            NLOperation::Break => {

            },
            NLOperation::Match(match_statement) => {

            },
            NLOperation::FunctionCall(function_call) => {

            },
        }
    }

    unimplemented!()
}