use crate::parsing::*;

use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{DataContext, Linkage, Module};
use std::collections::HashMap;

// All tests are kept in their own module.
#[cfg(test)]
mod tests;

// The representation of functions is done using a stack.
// Arguments are proveded by leaving them on the stack.
// Values are returned by leaving them on the stack.

enum CompileError<'a> {
    VariableUndefined(&'a str), // String is the name of the variable.
    TypeUnspecified,            // We do not yet support type derive. The type must be specified.
}

type Result<'a, T> = std::result::Result<T, CompileError<'a>>;

pub struct Compiler {
    builder_context: FunctionBuilderContext,
    module: JITModule,

    // TODO make these on a per-thread basis.
    ctx: codegen::Context,
    data_ctx: DataContext,
}

struct VariableTracker<'a> {
    variable: Variable,
    var_type: NLType<'a>,
}

struct StackScope<'a> {
    parent: Option<&'a StackScope<'a>>,
    next_variable: usize,
    variables: HashMap<&'a str, VariableTracker<'a>>,
}

impl<'a> StackScope<'a> {
    fn new(parent: Option<&'a StackScope<'a>>) -> StackScope<'a> {
        let next_variable = if let Some(parent) = parent {
            parent.next_variable
        } else {
            0
        };

        StackScope {
            parent,
            next_variable,
            variables: HashMap::new(),
        }
    }
    fn declare_variable(&mut self, name: &'a str, var_type: NLType<'a>) -> &VariableTracker<'a> {
        // use std::collections::hash_map::Entry;

        // match self.variables.entry(name) {
        //     Entry::Occupied(mut variable) => {
        //         // The variable exists, so we just have to update it.
        //         let var = variable.into_mut();
        //         var.var_type = var_type;
        //         var
        //     }
        //     Entry::Vacant(vacancy) => {
        //         // If the variable doesn't exist, we have to create it.
        //         let variable = VariableTracker {
        //             var_type,
        //             variable: Variable::new(self.next_variable),
        //         };
        //         self.next_variable += 1;

        //         vacancy.insert(variable)
        //     }
        // }
        unimplemented!()
    }

    fn get_variable(&self, name: &'a str) -> Option<&VariableTracker<'a>> {
        self.variables.get(name)
    }
}

impl Compiler {
    fn compile_function(&mut self, function: NLFunction) -> Result<()> {
        let mut builder = FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_context);

        // Adding the arguments.
        // for _p in &params {
        //     self.ctx.func.signature.params.push(AbiParam::new(int));
        // }

        // Adding the return values.
        // function.return_type
        // self.ctx.func.signature.returns.push(AbiParam::new());

        if let Some(block) = function.get_block() {
            let entry_block = builder.create_block();
            builder.append_block_params_for_function_params(entry_block);
            builder.seal_block(entry_block);

            Self::compile_block(None, &mut builder, &block);

            Ok(())
        } else {
            // TODO return some kind of linkable function signature.
            unimplemented!()
        }
    }

    fn compile_block<'a>(
        parent_scope: Option<&'a StackScope<'a>>,
        builder: &mut FunctionBuilder,
        block: &NLBlock,
    ) {
        let operations = block.get_operations();

        // Start by getting all of the local variables.
        let local_variables = StackScope::new(parent_scope);

        for operation in operations {
            match operation {
                NLOperation::Block(block) => {
                    Self::compile_block(Some(&local_variables), builder, block);
                }
                NLOperation::Constant(constant) => match constant {
                    OpConstant::Boolean(value) => {
                        builder.ins().bconst(types::B1, *value);
                    }
                    OpConstant::Integer(value, nl_type) => {
                        unimplemented!()
                    }
                    OpConstant::Float(value, nl_type) => {
                        unimplemented!()
                    }
                    OpConstant::String(value) => {
                        unimplemented!()
                    }
                },
                NLOperation::Assign(_assignment) => {
                    // if assignment.is_new() {
                    //     // New variable. We need to allocate it a space on the stack (or reuse the space of a variable that's being redefined)
                    //     for (name, var_type) in assignment
                    //         .get_variable_to_assign()
                    //         .iter()
                    //         .zip(assignment.get_types())
                    //     {
                    //         local_variables.declare_variable(name.get_name(), *var_type);
                    //     }
                    // }
                }
                NLOperation::VariableAccess(_variable) => {
                    unimplemented!()
                }
                NLOperation::Tuple(_operations) => {
                    unimplemented!()
                }
                NLOperation::Operator(_operator) => {
                    unimplemented!()
                }
                NLOperation::If(_if_statement) => {
                    unimplemented!()
                }
                NLOperation::Loop(_loop_block) => {
                    unimplemented!()
                }
                NLOperation::WhileLoop(_while_loop) => {
                    unimplemented!()
                }
                NLOperation::ForLoop(_for_loop) => {
                    unimplemented!()
                }
                NLOperation::Break => {
                    unimplemented!()
                }
                NLOperation::Match(_match_statement) => {
                    unimplemented!()
                }
                NLOperation::FunctionCall(_function_call) => {
                    unimplemented!()
                }
            }
        }

        unimplemented!()
    }
}
