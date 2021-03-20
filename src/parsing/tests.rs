use super::*;

use unwrap_to::unwrap_to;

fn pretty_read<'a, T>(input: &'a str, function: &dyn Fn(&'a str) -> ParserResult<T>) -> T {
    let result = function(input);
    match result {
        Ok(tuple) => {
            let (_, result) = tuple;

            result
        }
        Err(e) => {
            match e {
                nom::Err::Error(e) | nom::Err::Failure(e) => {
                    let message = convert_error(input, e);

                    // Makes our error messages more readable when running tests.
                    #[cfg(test)]
                    println!("{}", message);
                    panic!(message);
                }
                nom::Err::Incomplete(_) => {
                    panic!("Unexpected end of file.");
                }
            }
        }
    }
}

fn unwrap_constant<'a>(op: NLOperation<'a>) -> OpConstant<'a> {
    match op {
        NLOperation::Constant(constant) => constant,
        _ => panic!("Expected constant."),
    }
}

fn unwrap_constant_boolean<'a>(op: &NLOperation<'a>) -> bool {
    let constant = unwrap_to!(op => NLOperation::Constant);

    match constant {
        OpConstant::Boolean(constant) => *constant,
        _ => panic!("Expected boolean for constant type, got: {:?}", op),
    }
}

fn unwrap_constant_number(op: &NLOperation) -> u64 {
    let constant = unwrap_to!(op => NLOperation::Constant);
    match constant {
        OpConstant::Integer(value, _) => *value,
        _ => {
            panic!("Expected integer for constant type, got: {:?}");
        }
    }
}

mod root {
    use super::*;

    #[test]
    /// Compile an empty file. We should get no errors or warnings. Name should match.
    fn empty_file() {
        let file = "";
        let file_name = "test_file.nl";

        let file = parse_string(file, file_name).unwrap();

        assert_eq!(file.name, file_name, "File name not copied correctly.");
    }

    #[test]
    /// Compile a file with an empty struct and an empty trait. We should get no errors or warnings.
    fn empty_struct_and_trait() {
        let file_name = "tests/parsing/empty_struct_and_trait.nl";
        parse_file(&mut Path::new(file_name), &|file: &NLFile| {
            assert_eq!(
                file.name, "empty_struct_and_trait.nl",
                "File name not copied correctly."
            );

            assert_eq!(file.traits.len(), 1, "Wrong number of traits.");
            let my_trait = &file.traits[0];
            assert_eq!(my_trait.name, "MyTrait", "Wrong name for trait.");

            assert_eq!(file.structs.len(), 1, "Wrong number of structs.");
            let my_struct = &file.structs[0];
            assert_eq!(my_struct.name, "MyStruct", "Wrong name for struct.");
        })
        .unwrap();
    }

    #[test]
    /// Compile a file with an invalid token in its root.
    fn bad_root() {
        let file_name = "tests/parsing/bad_root.nl";
        let result = parse_file(&mut Path::new(file_name), &|_file: &NLFile| {});
        match result {
            Err(error) => {
                // Everything is fine! ... in a way.
                assert!(error
                    .to_string()
                    .contains("I shouldn't be here in the root."));
            }
            Ok(_) => {
                panic!("No error when one was expected.");
            }
        }
    }

    mod nl_struct {
        use super::*;

        #[test]
        /// Compile a file with a single empty struct. We should get no errors or warnings.
        fn single_empty_struct() {
            let file_name = "tests/parsing/single_struct_empty.nl";
            parse_file(&mut Path::new(file_name), &|file: &NLFile| {
                assert_eq!(
                    file.name, "single_struct_empty.nl",
                    "File name not copied correctly."
                );

                assert_eq!(file.structs.len(), 1, "Wrong number of structs.");
                let my_struct = &file.structs[0];
                assert_eq!(my_struct.name, "MyStruct", "Wrong name for struct.");

                assert_eq!(file.traits.len(), 0, "Wrong number of traits.");
            })
            .unwrap();
        }

        #[test]
        /// Compile a single struct with a single variable.
        fn single_variable_struct() {
            let file_name = "tests/parsing/struct_with_single_variable.nl";
            parse_file(&mut Path::new(file_name), &|file: &NLFile| {
                assert_eq!(file.structs.len(), 1, "Wrong number of structs.");
                let my_struct = &file.structs[0];
                assert_eq!(my_struct.name, "MyStruct", "Wrong name for struct.");
                assert_eq!(my_struct.variables.len(), 1, "Wrong number of variables.");
                let variable = &my_struct.variables[0];
                assert_eq!(variable.name, "variable", "Variable had wrong name.");
                assert_eq!(variable.my_type, NLType::I32, "Variable had wrong type.");
            })
            .unwrap();
        }

        #[test]
        /// Compile a single struct with a single variable. We don't put the trailing comma after this one.
        fn single_variable_struct_no_ending_comma() {
            let file_name = "tests/parsing/struct_with_single_variable_no_comma.nl";
            parse_file(&mut Path::new(file_name), &|file: &NLFile| {
                assert_eq!(file.structs.len(), 1, "Wrong number of structs.");
                let my_struct = &file.structs[0];
                assert_eq!(my_struct.name, "MyStruct", "Wrong name for struct.");
                assert_eq!(my_struct.variables.len(), 1, "Wrong number of variables.");
                let variable = &my_struct.variables[0];
                assert_eq!(variable.name, "variable", "Variable had wrong name.");
                assert_eq!(variable.my_type, NLType::I32, "Variable had wrong type.");
            })
            .unwrap();
        }

        #[test]
        /// Compile a single struct with two variables. We don't put the trailing comma after the last one.
        fn two_variable_struct_no_ending_comma() {
            let file_name = "tests/parsing/struct_with_two_variables_no_ending_comma.nl";
            parse_file(&mut Path::new(file_name), &|file: &NLFile| {
                assert_eq!(file.structs.len(), 1, "Wrong number of structs.");
                let my_struct = &file.structs[0];
                assert_eq!(my_struct.name, "MyStruct", "Wrong name for struct.");
                assert_eq!(my_struct.variables.len(), 2, "Wrong number of variables.");

                let variable = &my_struct.variables[0];
                assert_eq!(variable.name, "variable", "Variable had wrong name.");
                assert_eq!(variable.my_type, NLType::I32, "Variable had wrong type.");

                let variable = &my_struct.variables[1];
                assert_eq!(variable.name, "other_variable", "Variable had wrong name.");
                assert_eq!(variable.my_type, NLType::I32, "Variable had wrong type.");
            })
            .unwrap();
        }

        #[test]
        /// Compile a file with an empty struct and an empty trait. This one is special because it has single line comments in it.
        fn empty_struct_and_trait_single_line_comments() {
            let file_name = "tests/parsing/empty_struct_and_trait_with_single_line_comments.nl";
            parse_file(&mut Path::new(file_name), &|file: &NLFile| {
                assert_eq!(
                    file.name, "empty_struct_and_trait_with_single_line_comments.nl",
                    "File name not copied correctly."
                );

                assert_eq!(file.traits.len(), 1, "Wrong number of traits.");
                let my_trait = &file.traits[0];
                assert_eq!(my_trait.name, "MyTrait", "Wrong name for trait.");

                assert_eq!(file.structs.len(), 1, "Wrong number of structs.");
                let my_struct = &file.structs[0];
                assert_eq!(my_struct.name, "MyStruct", "Wrong name for struct.");
            })
            .unwrap();
        }

        #[test]
        /// Compile a file with an empty struct and an empty trait. This one is special because it has multi line comments in it.
        fn empty_struct_and_trait_multi_line_comments() {
            let file_name = "tests/parsing/empty_struct_and_trait_with_multi_line_comments.nl";
            parse_file(&mut Path::new(file_name), &|file: &NLFile| {
                assert_eq!(
                    file.name, "empty_struct_and_trait_with_multi_line_comments.nl",
                    "File name not copied correctly."
                );

                assert_eq!(file.traits.len(), 1, "Wrong number of traits.");
                let my_trait = &file.traits[0];
                assert_eq!(my_trait.name, "MyTrait", "Wrong name for trait.");

                assert_eq!(file.structs.len(), 1, "Wrong number of structs.");
                let my_struct = &file.structs[0];
                assert_eq!(my_struct.name, "MyStruct", "Wrong name for struct.");
            })
            .unwrap();
        }

        #[test]
        /// Compile a file with an empty struct and an empty trait. This one is special because it has multi line comments in it.
        fn struct_empty_self_implementation() {
            let file_name = "tests/parsing/struct_with_empty_self_implementation.nl";
            parse_file(&mut Path::new(file_name), &|file: &NLFile| {
                assert_eq!(file.structs.len(), 1, "Wrong number of structs.");
                let my_struct = &file.structs[0];

                assert_eq!(
                    my_struct.implementations.len(),
                    1,
                    "Wrong number of implementations."
                );
                let implementation = &my_struct.implementations[0];

                assert_eq!(
                    implementation.name, "Self",
                    "Implementation had wrong name."
                );
            })
            .unwrap();
        }

        #[test]
        /// Compile a file with an empty struct and an empty trait. This one is special because it has multi line comments in it.
        fn struct_self_implementation_with_methods() {
            let file_name = "tests/parsing/struct_self_implementation_with_methods.nl";
            parse_file(&mut Path::new(file_name), &|file: &NLFile| {
                assert_eq!(file.structs.len(), 1, "Wrong number of structs.");
                let my_struct = &file.structs[0];

                assert_eq!(
                    my_struct.implementations.len(),
                    1,
                    "Wrong number of implementations."
                );
                let implementation = &my_struct.implementations[0];

                assert_eq!(
                    implementation.name, "Self",
                    "Implementation had wrong name."
                );
                assert_eq!(
                    implementation.implementors.len(),
                    4,
                    "Wrong number of methods."
                );
            })
            .unwrap();
        }

        #[test]
        /// Compile a file with an empty struct and an empty trait. This one is special because it has multi line comments in it.
        fn struct_self_implementation_with_methods_and_encapsulations() {
            let file_name =
                "tests/parsing/struct_self_implementation_with_methods_and_encapsulations.nl";
            parse_file(&mut Path::new(file_name), &|file: &NLFile| {
                assert_eq!(file.structs.len(), 1, "Wrong number of structs.");
                let my_struct = &file.structs[0];

                assert_eq!(
                    my_struct.implementations.len(),
                    1,
                    "Wrong number of implementations."
                );
                let implementation = &my_struct.implementations[0];

                assert_eq!(
                    implementation.name, "Self",
                    "Implementation had wrong name."
                );
                assert_eq!(
                    implementation.implementors.len(),
                    10,
                    "Wrong number of methods."
                );
            })
            .unwrap();
        }
    }

    mod nl_trait {
        use super::*;

        #[test]
        /// Compile a file with a single empty trait. We should get no errors or warnings.
        fn single_empty_trait() {
            let file_name = "tests/parsing/single_trait_empty.nl";
            parse_file(&mut Path::new(file_name), &|file: &NLFile| {
                assert_eq!(
                    file.name, "single_trait_empty.nl",
                    "File name not copied correctly."
                );

                assert_eq!(file.traits.len(), 1, "Wrong number of traits.");
                let my_trait = &file.traits[0];
                assert_eq!(my_trait.name, "MyTrait", "Wrong name for trait.");

                assert_eq!(file.structs.len(), 0, "Wrong number of structs.");
            })
            .unwrap();
        }

        #[test]
        /// Tests a struct with encapsulations.
        fn trait_with_methods_and_encapsulators() {
            let file_name = "tests/parsing/trait_with_methods_and_encapsulators.nl";
            parse_file(&mut Path::new(file_name), &|file: &NLFile| {
                assert_eq!(file.traits.len(), 1, "Wrong number of traits.");
                let my_trait = &file.traits[0];

                assert_eq!(my_trait.name, "MyTrait", "Implementation had wrong name.");
                assert_eq!(my_trait.implementors.len(), 10, "Wrong number of methods.");
            })
            .unwrap();
        }
    }

    mod argument_list {
        use super::*;

        #[test]
        /// Testing the argument declaration reader.
        fn empty() {
            let code = "()";
            let args = pretty_read(code, &read_argument_deceleration_list);

            assert_eq!(args.len(), 0, "Wrong number of args.");
        }

        #[test]
        /// Testing the argument declaration reader.
        fn single_arg() {
            let code = "(argA : i32)";
            let args = pretty_read(code, &read_argument_deceleration_list);

            assert_eq!(args.len(), 1, "Wrong number of args.");

            let arg = &args[0];
            assert_eq!(arg.name, "argA", "Wrong argument name.");
            assert_eq!(arg.nl_type, NLType::I32, "Wrong argument type.");
        }

        #[test]
        /// Testing the argument declaration reader.
        fn two_args() {
            let code = "(argA : i32, argB : i16)";
            let args = pretty_read(code, &read_argument_deceleration_list);

            assert_eq!(args.len(), 2, "Wrong number of args.");

            let arg = &args[0];
            assert_eq!(arg.name, "argA", "Wrong argument name.");
            assert_eq!(arg.nl_type, NLType::I32, "Wrong argument type.");

            let arg = &args[1];
            assert_eq!(arg.name, "argB", "Wrong argument name.");
            assert_eq!(arg.nl_type, NLType::I16, "Wrong argument type.");
        }

        #[test]
        /// Testing the argument declaration reader.
        fn three_args() {
            let code = "(argA : i32, argB : i16, argC: i8)";
            let args = pretty_read(code, &read_argument_deceleration_list);

            assert_eq!(args.len(), 3, "Wrong number of args.");

            let arg = &args[0];
            assert_eq!(arg.name, "argA", "Wrong argument name.");
            assert_eq!(arg.nl_type, NLType::I32, "Wrong argument type.");

            let arg = &args[1];
            assert_eq!(arg.name, "argB", "Wrong argument name.");
            assert_eq!(arg.nl_type, NLType::I16, "Wrong argument type.");

            let arg = &args[2];
            assert_eq!(arg.name, "argC", "Wrong argument name.");
            assert_eq!(arg.nl_type, NLType::I8, "Wrong argument type.");
        }

        #[test]
        /// Testing the argument declaration reader.
        fn self_reference_arg() {
            let code = "(&self)";
            let args = pretty_read(code, &read_argument_deceleration_list);

            assert_eq!(args.len(), 1, "Wrong number of args.");

            let arg = &args[0];
            assert_eq!(arg.name, "self", "Wrong argument name.");
            assert_eq!(arg.nl_type, NLType::SelfReference, "Wrong argument type.");
        }

        #[test]
        /// Testing the argument declaration reader.
        fn mutable_self_reference_arg() {
            let code = "(&mut self)";
            let args = pretty_read(code, &read_argument_deceleration_list);

            assert_eq!(args.len(), 1, "Wrong number of args.");

            let arg = &args[0];
            assert_eq!(arg.name, "self", "Wrong argument name.");
            assert_eq!(
                arg.nl_type,
                NLType::MutableSelfReference,
                "Wrong argument type."
            );
        }

        #[test]
        /// Testing the argument declaration reader.
        fn mutable_self_reference_arg_odd_spacing() {
            let code = "(&mut\tself)";
            let args = pretty_read(code, &read_argument_deceleration_list);

            assert_eq!(args.len(), 1, "Wrong number of args.");

            let arg = &args[0];
            assert_eq!(arg.name, "self", "Wrong argument name.");
            assert_eq!(
                arg.nl_type,
                NLType::MutableSelfReference,
                "Wrong argument type."
            );
        }

        #[test]
        /// Testing the argument declaration reader.
        fn self_reference_arg_odd_pre_space() {
            let code = "(& self)";
            let args = pretty_read(code, &read_argument_deceleration_list);

            assert_eq!(args.len(), 1, "Wrong number of args.");

            let arg = &args[0];
            assert_eq!(arg.name, "self", "Wrong argument name.");
            assert_eq!(arg.nl_type, NLType::SelfReference, "Wrong argument type.");
        }

        #[test]
        /// Testing the argument declaration reader.
        fn mutable_self_reference_arg_odd_pre_space() {
            let code = "(& mut self)";
            let args = pretty_read(code, &read_argument_deceleration_list);

            assert_eq!(args.len(), 1, "Wrong number of args.");

            let arg = &args[0];
            assert_eq!(arg.name, "self", "Wrong argument name.");
            assert_eq!(
                arg.nl_type,
                NLType::MutableSelfReference,
                "Wrong argument type."
            );
        }

        #[test]
        /// Testing the argument declaration reader.
        fn struct_reference() {
            let code = "(var: &SomeStruct)";
            let args = pretty_read(code, &read_argument_deceleration_list);

            assert_eq!(args.len(), 1, "Wrong number of args.");

            let arg = &args[0];
            assert_eq!(arg.name, "var", "Wrong argument name.");
            assert_eq!(
                arg.nl_type,
                NLType::ReferencedStruct("SomeStruct"),
                "Wrong argument type."
            );
        }

        #[test]
        /// Testing the argument declaration reader.
        fn mutable_struct_reference() {
            let code = "(var: &mut SomeStruct)";
            let args = pretty_read(code, &read_argument_deceleration_list);

            assert_eq!(args.len(), 1, "Wrong number of args.");

            let arg = &args[0];
            assert_eq!(arg.name, "var", "Wrong argument name.");
            assert_eq!(
                arg.nl_type,
                NLType::MutableReferencedStruct("SomeStruct"),
                "Wrong argument type."
            );
        }

        #[test]
        /// Testing the argument declaration reader.
        fn struct_owned() {
            let code = "(var: SomeStruct)";
            let args = pretty_read(code, &read_argument_deceleration_list);

            assert_eq!(args.len(), 1, "Wrong number of args.");

            let arg = &args[0];
            assert_eq!(arg.name, "var", "Wrong argument name.");
            assert_eq!(
                arg.nl_type,
                NLType::OwnedStruct("SomeStruct"),
                "Wrong argument type."
            );
        }

        #[test]
        /// Testing the argument declaration reader.
        fn trait_reference() {
            let code = "(var: &dyn SomeTrait)";
            let args = pretty_read(code, &read_argument_deceleration_list);

            assert_eq!(args.len(), 1, "Wrong number of args.");

            let arg = &args[0];
            assert_eq!(arg.name, "var", "Wrong argument name.");
            assert_eq!(
                arg.nl_type,
                NLType::ReferencedTrait("SomeTrait"),
                "Wrong argument type."
            );
        }

        #[test]
        /// Testing the argument declaration reader.
        fn mutable_trait_reference() {
            let code = "(var: &mut dyn SomeTrait)";
            let args = pretty_read(code, &read_argument_deceleration_list);

            assert_eq!(args.len(), 1, "Wrong number of args.");

            let arg = &args[0];
            assert_eq!(arg.name, "var", "Wrong argument name.");
            assert_eq!(
                arg.nl_type,
                NLType::MutableReferencedTrait("SomeTrait"),
                "Wrong argument type."
            );
        }

        #[test]
        /// Testing the argument declaration reader.
        fn trait_owned() {
            let code = "(var: dyn SomeTrait)";
            let args = pretty_read(code, &read_argument_deceleration_list);

            assert_eq!(args.len(), 1, "Wrong number of args.");

            let arg = &args[0];
            assert_eq!(arg.name, "var", "Wrong argument name.");
            assert_eq!(
                arg.nl_type,
                NLType::OwnedTrait("SomeTrait"),
                "Wrong argument type."
            );
        }
    }

    mod global_functions {
        use super::*;

        #[test]
        fn all_global_function_types() {
            let file_name = "tests/parsing/global_functions.nl";
            parse_file(&mut Path::new(file_name), &|file: &NLFile| {
                assert_eq!(
                    file.name, "global_functions.nl",
                    "File name not copied correctly."
                );

                assert_eq!(file.functions.len(), 4, "Wrong number of functions.");

                // fn my_function();
                let function = &file.functions[0];
                assert_eq!(
                    function.get_name(),
                    "my_function",
                    "Wrong name for function."
                );
                assert_eq!(function.arguments.len(), 0, "Wrong number of arguments.");
                assert_eq!(function.return_type, NLType::None, "Wrong return type.");
                assert_eq!(
                    function.block.is_none(),
                    true,
                    "Function should not have been implemented."
                );

                // fn my_function() {}
                let function = &file.functions[1];
                assert_eq!(
                    function.get_name(),
                    "my_function",
                    "Wrong name for function."
                );
                assert_eq!(function.arguments.len(), 0, "Wrong number of arguments.");
                assert_eq!(function.return_type, NLType::None, "Wrong return type.");
                assert_eq!(
                    function.block.is_some(),
                    true,
                    "Function should not have been implemented."
                );

                // fn my_function() -> i32;
                let function = &file.functions[2];
                assert_eq!(
                    function.get_name(),
                    "my_function",
                    "Wrong name for function."
                );
                assert_eq!(function.arguments.len(), 0, "Wrong number of arguments.");
                assert_eq!(function.return_type, NLType::I32, "Wrong return type.");
                assert_eq!(
                    function.block.is_none(),
                    true,
                    "Function should not have been implemented."
                );

                // fn my_function() -> i32 {}
                let function = &file.functions[3];
                assert_eq!(
                    function.get_name(),
                    "my_function",
                    "Wrong name for function."
                );
                assert_eq!(function.arguments.len(), 0, "Wrong number of arguments.");
                assert_eq!(function.return_type, NLType::I32, "Wrong return type.");
                assert_eq!(
                    function.block.is_some(),
                    true,
                    "Function should not have been implemented."
                );

                assert_eq!(file.traits.len(), 0, "Wrong number of traits.");
                assert_eq!(file.structs.len(), 0, "Wrong number of structs.");
            })
            .unwrap();
        }
    }

    mod nl_methods {
        use super::*;

        fn pretty_read_method(input: &str) -> (&str, NLFunction) {
            let result = read_method(input);
            match result {
                Ok(tuple) => {
                    let (s, method) = tuple;
                    match method {
                        NLImplementor::Method(method) => (s, method),
                        _ => {
                            panic!("Did not get a method.");
                        }
                    }
                }
                Err(e) => {
                    match e {
                        nom::Err::Error(e) | nom::Err::Failure(e) => {
                            let message = convert_error(input, e);

                            // Makes our error messages more readable when running tests.
                            #[cfg(test)]
                            println!("{}", message);
                            panic!(message);
                        }
                        nom::Err::Incomplete(_) => {
                            panic!("Unexpected end of file.");
                        }
                    }
                }
            }
        }

        #[test]
        /// Construct a blank and unimplemented method.
        fn method_no_arg_no_return_no_impl() {
            let code = "met my_method();";

            let (_, method) = pretty_read_method(code);

            assert_eq!(method.name, "my_method", "Method had wrong name.");
            assert_eq!(method.arguments.len(), 0, "Wrong number of arguments.");
            assert_eq!(method.return_type, NLType::None, "Wrong return type.");
            assert_eq!(
                method.block.is_none(),
                true,
                "Method should not have been implemented."
            );
        }

        #[test]
        /// Construct a blank and unimplemented method.
        fn method_no_arg_return_i32_no_impl() {
            let code = "met my_method() -> i32;";

            let (_, method) = pretty_read_method(code);

            assert_eq!(method.name, "my_method", "Method had wrong name.");
            assert_eq!(method.arguments.len(), 0, "Wrong number of arguments.");
            assert_eq!(method.return_type, NLType::I32, "Wrong return type.");
            assert_eq!(
                method.block.is_none(),
                true,
                "Method should not have been implemented."
            );
        }

        #[test]
        /// Construct a blank and unimplemented method.
        fn method_no_arg_no_return_implemented() {
            let code = "met my_method() {}";

            let (_, method) = pretty_read_method(code);

            assert_eq!(method.name, "my_method", "Method had wrong name.");
            assert_eq!(method.arguments.len(), 0, "Wrong number of arguments.");
            assert_eq!(method.return_type, NLType::None, "Wrong return type.");
            assert_eq!(
                method.block.is_none(),
                false,
                "Method should have been implemented."
            );
        }

        #[test]
        /// Construct a blank and unimplemented method.
        fn method_no_arg_return_i32_implemented() {
            let code = "met my_method() -> i32 {}";

            let (_, method) = pretty_read_method(code);

            assert_eq!(method.name, "my_method", "Method had wrong name.");
            assert_eq!(method.arguments.len(), 0, "Wrong number of arguments.");
            assert_eq!(method.return_type, NLType::I32, "Wrong return type.");
            assert_eq!(
                method.block.is_none(),
                false,
                "Method should have been implemented."
            );
        }
    }

    mod nl_getters {
        use super::*;

        fn pretty_read_getter(input: &str) -> (&str, NLGetter) {
            let result = read_getter(input);
            match result {
                Ok(tuple) => {
                    let (s, method) = tuple;
                    match method {
                        NLImplementor::Getter(getter) => (s, getter),
                        _ => {
                            panic!("Did not get a getter.");
                        }
                    }
                }
                Err(e) => {
                    match e {
                        nom::Err::Error(e) | nom::Err::Failure(e) => {
                            let message = convert_error(input, e);

                            // Makes our error messages more readable when running tests.
                            #[cfg(test)]
                            println!("{}", message);
                            panic!(message);
                        }
                        nom::Err::Incomplete(_) => {
                            panic!("Unexpected end of file.");
                        }
                    }
                }
            }
        }

        #[test]
        /// A simple test of the getter syntax.
        fn getter_default_impl() {
            let code = "get my_getter:default;";
            let (_, getter) = pretty_read_getter(code);

            assert_eq!(
                getter.name, "my_getter",
                "Getter did not have expected name."
            );
            assert_eq!(
                getter.block,
                NLEncapsulationBlock::Default,
                "Getter did not state use of default implementation."
            );
            assert_eq!(
                getter.nl_type,
                NLType::None,
                "Getter did not have correct return type."
            );
        }

        #[test]
        /// A simple test of the getter syntax.
        fn getter_impl() {
            let code = "get my_getter(&self) -> i32 {}";
            let (_, getter) = pretty_read_getter(code);

            assert_eq!(
                getter.name, "my_getter",
                "Getter did not have expected name."
            );
            assert_ne!(
                getter.block,
                NLEncapsulationBlock::Default,
                "Getter did not state use of default implementation."
            );
            assert_ne!(
                getter.block,
                NLEncapsulationBlock::None,
                "Getter did not state use of default implementation."
            );
            assert_eq!(
                getter.nl_type,
                NLType::I32,
                "Getter did not have correct return type."
            );
        }

        #[test]
        /// A simple test of the getter syntax.
        fn getter_no_impl() {
            let code = "get my_getter(&self) -> i32;";
            let (_, getter) = pretty_read_getter(code);

            assert_eq!(
                getter.name, "my_getter",
                "Getter did not have expected name."
            );
            assert_eq!(
                getter.block,
                NLEncapsulationBlock::None,
                "Getter did not state use of no implementation."
            );
            assert_eq!(
                getter.nl_type,
                NLType::I32,
                "Getter did not have correct return type."
            );
        }
    }

    mod nl_setters {
        use super::*;

        fn pretty_read_setter(input: &str) -> (&str, NLSetter) {
            let result = read_setter(input);
            match result {
                Ok(tuple) => {
                    let (s, method) = tuple;
                    match method {
                        NLImplementor::Setter(setter) => (s, setter),
                        _ => {
                            panic!("Did not get a setter.");
                        }
                    }
                }
                Err(e) => {
                    match e {
                        nom::Err::Error(e) | nom::Err::Failure(e) => {
                            let message = convert_error(input, e);

                            // Makes our error messages more readable when running tests.
                            #[cfg(test)]
                            println!("{}", message);
                            panic!(message);
                        }
                        nom::Err::Incomplete(_) => {
                            panic!("Unexpected end of file.");
                        }
                    }
                }
            }
        }

        #[test]
        /// A simple test of the getter syntax.
        fn setter_default_impl() {
            let code = "set my_setter:default;";
            let (_, setter) = pretty_read_setter(code);

            assert_eq!(
                setter.name, "my_setter",
                "Setter did not have expected name."
            );
            assert_eq!(
                setter.block,
                NLEncapsulationBlock::Default,
                "Setter did not state use of default implementation."
            );
            assert_eq!(
                setter.args.len(),
                0,
                "Setter did not have correct arguments."
            );
        }

        #[test]
        /// A simple test of the getter syntax.
        fn getter_impl() {
            let code = "set my_setter(value: i32) {}";
            let (_, setter) = pretty_read_setter(code);

            assert_eq!(
                setter.name, "my_setter",
                "Getter did not have expected name."
            );
            assert_ne!(
                setter.block,
                NLEncapsulationBlock::Default,
                "Setter did not state use of default implementation."
            );
            assert_ne!(
                setter.block,
                NLEncapsulationBlock::None,
                "Setter did not state use of default implementation."
            );

            assert_eq!(
                setter.args.len(),
                1,
                "Setter did not have correct number of arguments."
            );
            let arg = &setter.args[0];
            assert_eq!(arg.name, "value", "Variable did not have expected name.");
            assert_eq!(
                arg.nl_type,
                NLType::I32,
                "Variable did not have expected type."
            );
        }

        #[test]
        /// A simple test of the getter syntax.
        fn getter_no_impl() {
            let code = "set my_setter(value: i32);";
            let (_, setter) = pretty_read_setter(code);

            assert_eq!(
                setter.name, "my_setter",
                "Setter did not have expected name."
            );
            assert_eq!(
                setter.block,
                NLEncapsulationBlock::None,
                "Setter did not state use of no implementation."
            );

            assert_eq!(
                setter.args.len(),
                1,
                "Setter did not have correct number of arguments."
            );
            let arg = &setter.args[0];
            assert_eq!(arg.name, "value", "Variable did not have expected name.");
            assert_eq!(
                arg.nl_type,
                NLType::I32,
                "Variable did not have expected type."
            );
        }
    }

    mod variant_enum {
        use super::*;

        #[test]
        fn empty() {
            let code = "enum MyVariant {}";
            let file = parse_string(code, "virtual_file").unwrap();
            let enums = file.get_enums();

            assert_eq!(enums.len(), 1);

            let nl_enum = &enums[0];
            assert_eq!(nl_enum.get_name(), "MyVariant");
            assert_eq!(nl_enum.get_variants().len(), 0);
        }

        #[test]
        fn one_variant() {
            let code = "enum MyVariant { One }";
            let file = parse_string(code, "virtual_file").unwrap();
            let enums = file.get_enums();

            assert_eq!(enums.len(), 1);

            let nl_enum = &enums[0];
            assert_eq!(nl_enum.get_name(), "MyVariant");

            let variants = nl_enum.get_variants();
            assert_eq!(variants.len(), 1);

            let variant = &variants[0];
            assert_eq!(variant.name, "One");
            assert_eq!(variant.get_arguments().len(), 0);
        }

        #[test]
        fn two_variant() {
            let code = "enum MyVariant { One, Two }";
            let file = parse_string(code, "virtual_file").unwrap();
            let enums = file.get_enums();

            assert_eq!(enums.len(), 1);

            let nl_enum = &enums[0];
            assert_eq!(nl_enum.get_name(), "MyVariant");

            let variants = nl_enum.get_variants();
            assert_eq!(variants.len(), 2);

            let variant = &variants[0];
            assert_eq!(variant.name, "One");
            assert_eq!(variant.get_arguments().len(), 0);

            let variant = &variants[1];
            assert_eq!(variant.name, "Two");
            assert_eq!(variant.get_arguments().len(), 0);
        }

        #[test]
        fn one_variant_lazy_comma() {
            let code = "enum MyVariant { One, Two, }";
            let file = parse_string(code, "virtual_file").unwrap();
            let enums = file.get_enums();

            assert_eq!(enums.len(), 1);

            let nl_enum = &enums[0];
            assert_eq!(nl_enum.get_name(), "MyVariant");

            let variants = nl_enum.get_variants();
            assert_eq!(variants.len(), 2);

            let variant = &variants[0];
            assert_eq!(variant.name, "One");
            assert_eq!(variant.get_arguments().len(), 0);

            let variant = &variants[1];
            assert_eq!(variant.name, "Two");
            assert_eq!(variant.get_arguments().len(), 0);
        }

        #[test]
        fn one_variant_single_container() {
            let code = "enum MyVariant { One(a: A), }";
            let file = parse_string(code, "virtual_file").unwrap();
            let enums = file.get_enums();

            assert_eq!(enums.len(), 1);

            let nl_enum = &enums[0];
            assert_eq!(nl_enum.get_name(), "MyVariant");

            let variants = nl_enum.get_variants();
            assert_eq!(variants.len(), 1);

            let variant = &variants[0];
            assert_eq!(variant.name, "One");

            let arguments = variant.get_arguments();
            assert_eq!(arguments.len(), 1);

            let argument = &arguments[0];
            assert_eq!(argument.get_name(), "a");
            assert_eq!(*unwrap_to!(argument.get_type() => NLType::OwnedStruct), "A");
        }

        #[test]
        fn one_variant_double_container() {
            let code = "enum MyVariant { One(a: A, b: B), }";
            let file = parse_string(code, "virtual_file").unwrap();
            let enums = file.get_enums();

            assert_eq!(enums.len(), 1);

            let nl_enum = &enums[0];
            assert_eq!(nl_enum.get_name(), "MyVariant");

            let variants = nl_enum.get_variants();
            assert_eq!(variants.len(), 1);

            let variant = &variants[0];
            assert_eq!(variant.name, "One");

            let arguments = variant.get_arguments();
            assert_eq!(arguments.len(), 2);

            let argument = &arguments[0];
            assert_eq!(argument.get_name(), "a");
            assert_eq!(*unwrap_to!(argument.get_type() => NLType::OwnedStruct), "A");

            let argument = &arguments[1];
            assert_eq!(argument.get_name(), "b");
            assert_eq!(*unwrap_to!(argument.get_type() => NLType::OwnedStruct), "B");
        }

        #[test]
        fn two_variant_double_container() {
            let code = "enum MyVariant { One(a: A, b: B), Two(c: C, d: D), }";
            let file = parse_string(code, "virtual_file").unwrap();
            let enums = file.get_enums();

            assert_eq!(enums.len(), 1);

            let nl_enum = &enums[0];
            assert_eq!(nl_enum.get_name(), "MyVariant");

            let variants = nl_enum.get_variants();
            assert_eq!(variants.len(), 2);

            let variant = &variants[0];
            assert_eq!(variant.name, "One");

            let arguments = variant.get_arguments();
            assert_eq!(arguments.len(), 2);

            let argument = &arguments[0];
            assert_eq!(argument.get_name(), "a");
            assert_eq!(*unwrap_to!(argument.get_type() => NLType::OwnedStruct), "A");

            let argument = &arguments[1];
            assert_eq!(argument.get_name(), "b");
            assert_eq!(*unwrap_to!(argument.get_type() => NLType::OwnedStruct), "B");

            let variant = &variants[1];
            assert_eq!(variant.name, "Two");

            let arguments = variant.get_arguments();
            assert_eq!(arguments.len(), 2);

            let argument = &arguments[0];
            assert_eq!(argument.get_name(), "c");
            assert_eq!(*unwrap_to!(argument.get_type() => NLType::OwnedStruct), "C");

            let argument = &arguments[1];
            assert_eq!(argument.get_name(), "d");
            assert_eq!(*unwrap_to!(argument.get_type() => NLType::OwnedStruct), "D");
        }
    }
}

mod executable_blocks {
    use super::*;

    mod constants {
        use super::*;

        #[test]
        fn number() {
            let code = "5";
            let constant = pretty_read(code, &read_constant);
            let constant = unwrap_constant(constant);

            match constant {
                OpConstant::Integer(constant, cast) => {
                    assert_eq!(constant, 5, "Constant had wrong value.");
                    assert_eq!(cast, NLType::I32, "Wrong type cast recommendation.");
                }
                _ => panic!("Expected integer for constant type."),
            }
        }

        #[test]
        fn negative_number() {
            let code = "-5";
            let constant = pretty_read(code, &read_constant);
            let constant = unwrap_constant(constant);

            match constant {
                OpConstant::Integer(constant, cast) => {
                    assert_eq!(constant as i64, -5, "Constant had wrong value.");
                    assert_eq!(cast, NLType::I32, "Wrong type cast recommendation.");
                }
                _ => panic!("Expected i32 for constant type."),
            }
        }

        #[test]
        fn typed_number() {
            let code = "5i64";
            let constant = pretty_read(code, &read_constant);
            let constant = unwrap_constant(constant);

            match constant {
                OpConstant::Integer(constant, cast) => {
                    assert_eq!(constant, 5, "Constant had wrong value.");
                    assert_eq!(cast, NLType::I64, "Wrong type cast recommendation.");
                }
                _ => panic!("Expected i64 for constant type."),
            }
        }

        #[test]
        fn negative_typed_number() {
            let code = "-5i64";
            let constant = pretty_read(code, &read_constant);
            let constant = unwrap_constant(constant);

            match constant {
                OpConstant::Integer(constant, cast) => {
                    assert_eq!(constant as i64, -5, "Constant had wrong value.");
                    assert_eq!(cast, NLType::I64, "Wrong type cast recommendation.");
                }
                _ => panic!("Expected i64 for constant type."),
            }
        }

        #[test]
        fn float() {
            let code = "5.5";
            let constant = pretty_read(code, &read_constant);
            let constant = unwrap_constant(constant);

            match constant {
                OpConstant::Float32(constant) => {
                    assert_eq!(constant, 5.5, "Constant had wrong value.");
                }
                _ => panic!("Expected float32 for constant type."),
            }
        }

        #[test]
        fn float_with_type() {
            let code = "5.5f64";
            let constant = pretty_read(code, &read_constant);
            let constant = unwrap_constant(constant);

            match constant {
                OpConstant::Float64(constant) => {
                    assert_eq!(constant, 5.5, "Constant had wrong value.");
                }
                _ => panic!("Expected float64 for constant type."),
            }
        }

        #[test]
        fn boolean_true() {
            let code = "true";
            let constant = pretty_read(code, &read_constant);
            let constant = unwrap_constant(constant);

            match constant {
                OpConstant::Boolean(constant) => {
                    assert_eq!(constant, true, "Constant had wrong value.");
                }
                _ => panic!("Expected boolean for constant type."),
            }
        }

        #[test]
        fn boolean_false() {
            let code = "false";
            let constant = pretty_read(code, &read_constant);
            let constant = unwrap_constant(constant);

            match constant {
                OpConstant::Boolean(constant) => {
                    assert_eq!(constant, false, "Constant had wrong value.");
                }
                _ => panic!("Expected boolean for constant type."),
            }
        }

        #[test]
        fn simple_string() {
            let code = "\"A simple string.\"";
            let constant = pretty_read(code, &read_constant);
            let constant = unwrap_constant(constant);

            match constant {
                OpConstant::String(string) => {
                    assert_eq!(string, "A simple string.", "Constant had wrong value.");
                }
                _ => panic!("Expected string for constant type."),
            }
        }
    }

    mod variables {
        use super::*;

        #[test]
        fn variable_access() {
            let code = "bah";
            let operation = pretty_read(code, &read_operation);

            match operation {
                NLOperation::VariableAccess(access) => {
                    assert_eq!(access.name, "bah", "Variable had wrong name.");
                }
                _ => panic!("Expected variable access operation, got {:?}", operation),
            }
        }
    }

    mod tuples {
        use super::*;

        #[test]
        fn tuple_empty() {
            let code = "()";
            let (_, tuple) = read_tuple(code).unwrap();

            match tuple {
                NLOperation::Tuple(tuple) => {
                    assert_eq!(tuple.len(), 0, "Wrong number of items in tuple.");
                }
                _ => panic!("Expected none."),
            }
        }

        #[test]
        fn tuple_one_item() {
            let code = "(1)";
            let (_, tuple) = read_tuple(code).unwrap();

            match tuple {
                NLOperation::Tuple(tuple) => {
                    assert_eq!(tuple.len(), 1, "Wrong number of items in tuple.");
                    assert_eq!(
                        tuple[0],
                        NLOperation::Constant(OpConstant::Integer(1, NLType::I32)),
                        "Wrong value used for first value."
                    );
                }
                _ => panic!("Expected none."),
            }
        }

        #[test]
        fn tuple_two_items() {
            let code = "(1, 2)";
            let (_, tuple) = read_tuple(code).unwrap();

            match tuple {
                NLOperation::Tuple(tuple) => {
                    assert_eq!(tuple.len(), 2, "Wrong number of items in tuple.");
                    assert_eq!(
                        tuple[0],
                        NLOperation::Constant(OpConstant::Integer(1, NLType::I32)),
                        "Wrong value used for first value."
                    );
                    assert_eq!(
                        tuple[1],
                        NLOperation::Constant(OpConstant::Integer(2, NLType::I32)),
                        "Wrong value used for second value."
                    );
                }
                _ => panic!("Expected none."),
            }
        }

        #[test]
        fn tuple_three_items() {
            let code = "(1, 2, 3)";
            let (_, tuple) = read_tuple(code).unwrap();

            match tuple {
                NLOperation::Tuple(tuple) => {
                    assert_eq!(tuple.len(), 3, "Wrong number of items in tuple.");
                    assert_eq!(
                        tuple[0],
                        NLOperation::Constant(OpConstant::Integer(1, NLType::I32)),
                        "Wrong value used for first value."
                    );
                    assert_eq!(
                        tuple[1],
                        NLOperation::Constant(OpConstant::Integer(2, NLType::I32)),
                        "Wrong value used for second value."
                    );
                    assert_eq!(
                        tuple[2],
                        NLOperation::Constant(OpConstant::Integer(3, NLType::I32)),
                        "Wrong value used for third value."
                    );
                }
                _ => panic!("Expected none."),
            }
        }
    }

    mod assignment {
        use super::*;

        #[test]
        fn single_variable_to_constant() {
            let code = "let five = 5;";
            let (_, operation) = read_assignment(code).unwrap();

            match operation {
                NLOperation::Assign(assign) => {
                    assert_eq!(assign.is_new, true, "Assignment should have been  new.");
                    assert_eq!(
                        assign.to_assign.len(),
                        1,
                        "Wrong number of values being assigned."
                    );
                    assert_eq!(
                        assign.type_assignments.len(),
                        0,
                        "Unexpected type specified."
                    );

                    assert_eq!(
                        assign.assignment,
                        Box::new(NLOperation::Constant(OpConstant::Integer(5, NLType::I32))),
                        "Wrong assignment."
                    );

                    let variable = &assign.to_assign[0];

                    assert_eq!(variable.name, "five", "Wrong name given to variable.");
                }
                _ => panic!("Expected assignment operation."),
            };
        }

        #[test]
        fn single_variable_to_constant_scoped() {
            let code = "let numbers.five = 5;";
            let (_, operation) = read_assignment(code).unwrap();

            match operation {
                NLOperation::Assign(assign) => {
                    assert_eq!(assign.is_new, true, "Assignment should have been  new.");
                    assert_eq!(
                        assign.to_assign.len(),
                        1,
                        "Wrong number of values being assigned."
                    );
                    assert_eq!(
                        assign.type_assignments.len(),
                        0,
                        "Unexpected type specified."
                    );

                    assert_eq!(
                        assign.assignment,
                        Box::new(NLOperation::Constant(OpConstant::Integer(5, NLType::I32))),
                        "Wrong assignment."
                    );

                    let variable = &assign.to_assign[0];

                    assert_eq!(
                        variable.name, "numbers.five",
                        "Wrong name given to variable."
                    );
                }
                _ => panic!("Expected assignment operation."),
            };
        }

        #[test]
        fn single_variable_to_constant_with_type_spec() {
            let code = "let five: i32 = 5;";
            let (_, operation) = read_assignment(code).unwrap();

            match operation {
                NLOperation::Assign(assign) => {
                    assert_eq!(assign.is_new, true, "Assignment should have been  new.");
                    assert_eq!(
                        assign.to_assign.len(),
                        1,
                        "Wrong number of values being assigned."
                    );
                    assert_eq!(
                        assign.type_assignments[0],
                        NLType::I32,
                        "Unexpected type specified."
                    );

                    assert_eq!(
                        assign.assignment,
                        Box::new(NLOperation::Constant(OpConstant::Integer(5, NLType::I32))),
                        "Wrong assignment."
                    );

                    let variable = &assign.to_assign[0];

                    assert_eq!(variable.name, "five", "Wrong name given to variable.");
                }
                _ => panic!("Expected assignment operation."),
            };
        }

        #[test]
        fn single_variable_to_constant_with_type_spec_scoped() {
            let code = "let numbers.five: i32 = 5;";
            let (_, operation) = read_assignment(code).unwrap();

            match operation {
                NLOperation::Assign(assign) => {
                    assert_eq!(assign.is_new, true, "Assignment should have been  new.");
                    assert_eq!(
                        assign.to_assign.len(),
                        1,
                        "Wrong number of values being assigned."
                    );
                    assert_eq!(
                        assign.type_assignments[0],
                        NLType::I32,
                        "Unexpected type specified."
                    );

                    assert_eq!(
                        assign.assignment,
                        Box::new(NLOperation::Constant(OpConstant::Integer(5, NLType::I32))),
                        "Wrong assignment."
                    );

                    let variable = &assign.to_assign[0];

                    assert_eq!(
                        variable.name, "numbers.five",
                        "Wrong name given to variable."
                    );
                }
                _ => panic!("Expected assignment operation."),
            };
        }

        #[test]
        fn assign_tuple() {
            let code = "let (fore, five) = (4, 5);";
            let (_, operation) = read_assignment(code).unwrap();

            match operation {
                NLOperation::Assign(assign) => {
                    assert_eq!(assign.is_new, true, "Assignment should have been  new.");
                    assert_eq!(
                        assign.to_assign.len(),
                        2,
                        "Wrong number of values being assigned."
                    );
                    assert_eq!(
                        assign.type_assignments.len(),
                        0,
                        "Unexpected type specified."
                    );

                    assert_eq!(
                        assign.assignment,
                        Box::new(NLOperation::Tuple(vec![
                            NLOperation::Constant(OpConstant::Integer(4, NLType::I32)),
                            NLOperation::Constant(OpConstant::Integer(5, NLType::I32))
                        ])),
                        "Wrong assignment."
                    );

                    let variable = &assign.to_assign[0];
                    assert_eq!(variable.name, "fore", "Wrong name given to variable.");

                    let variable = &assign.to_assign[1];
                    assert_eq!(variable.name, "five", "Wrong name given to variable.");
                }
                _ => panic!("Expected assignment operation."),
            };
        }

        #[test]
        fn assign_tuple_scoped() {
            let code = "let (numbers.fore, numbers.five) = (4, 5);";
            let (_, operation) = read_assignment(code).unwrap();

            match operation {
                NLOperation::Assign(assign) => {
                    assert_eq!(assign.is_new, true, "Assignment should have been  new.");
                    assert_eq!(
                        assign.to_assign.len(),
                        2,
                        "Wrong number of values being assigned."
                    );
                    assert_eq!(
                        assign.type_assignments.len(),
                        0,
                        "Unexpected type specified."
                    );

                    assert_eq!(
                        assign.assignment,
                        Box::new(NLOperation::Tuple(vec![
                            NLOperation::Constant(OpConstant::Integer(4, NLType::I32)),
                            NLOperation::Constant(OpConstant::Integer(5, NLType::I32))
                        ])),
                        "Wrong assignment."
                    );

                    let variable = &assign.to_assign[0];
                    assert_eq!(
                        variable.name, "numbers.fore",
                        "Wrong name given to variable."
                    );

                    let variable = &assign.to_assign[1];
                    assert_eq!(
                        variable.name, "numbers.five",
                        "Wrong name given to variable."
                    );
                }
                _ => panic!("Expected assignment operation."),
            };
        }

        #[test]
        fn assign_no_define() {
            let code = "five = 5;";
            let (_, operation) = read_assignment(code).unwrap();

            match operation {
                NLOperation::Assign(assign) => {
                    assert_eq!(assign.is_new, false, "Assignment should have been  new.");
                    assert_eq!(
                        assign.to_assign.len(),
                        1,
                        "Wrong number of values being assigned."
                    );
                    assert_eq!(
                        assign.type_assignments.len(),
                        0,
                        "Unexpected type specified."
                    );

                    assert_eq!(
                        assign.assignment,
                        Box::new(NLOperation::Constant(OpConstant::Integer(5, NLType::I32))),
                        "Wrong assignment."
                    );

                    let variable = &assign.to_assign[0];

                    assert_eq!(variable.name, "five", "Wrong name given to variable.");
                }
                _ => panic!("Expected assignment operation."),
            };
        }

        #[test]
        fn assign_no_define_scoped() {
            let code = "numbers.five = 5;";
            let (_, operation) = read_assignment(code).unwrap();

            match operation {
                NLOperation::Assign(assign) => {
                    assert_eq!(assign.is_new, false, "Assignment should have been  new.");
                    assert_eq!(
                        assign.to_assign.len(),
                        1,
                        "Wrong number of values being assigned."
                    );
                    assert_eq!(
                        assign.type_assignments.len(),
                        0,
                        "Unexpected type specified."
                    );

                    assert_eq!(
                        assign.assignment,
                        Box::new(NLOperation::Constant(OpConstant::Integer(5, NLType::I32))),
                        "Wrong assignment."
                    );

                    let variable = &assign.to_assign[0];

                    assert_eq!(
                        variable.name, "numbers.five",
                        "Wrong name given to variable."
                    );
                }
                _ => panic!("Expected assignment operation."),
            };
        }
    }

    mod operators {
        use super::*;

        // TODO test chained operators.

        mod comparison {
            use super::*;

            #[test]
            fn equal() {
                let code = "2 == 3";
                let operation = pretty_read(code, &read_operation);
                let operation = unwrap_to!(operation => NLOperation::Operator);
                let (a, b) = unwrap_to!(operation => OpOperator::CompareEqual);

                assert_eq!(
                    unwrap_constant_number(a),
                    2,
                    "Wrong number for left operand."
                );
                assert_eq!(
                    unwrap_constant_number(b),
                    3,
                    "Wrong number for right operand."
                );
            }

            #[test]
            fn not_equal() {
                let code = "2 != 3";
                let operation = pretty_read(code, &read_operation);
                let operation = unwrap_to!(operation => NLOperation::Operator);
                let (a, b) = unwrap_to!(operation => OpOperator::CompareNotEqual);

                assert_eq!(
                    unwrap_constant_number(a),
                    2,
                    "Wrong number for left operand."
                );
                assert_eq!(
                    unwrap_constant_number(b),
                    3,
                    "Wrong number for right operand."
                );
            }

            #[test]
            fn greater() {
                let code = "2 > 3";
                let operation = pretty_read(code, &read_operation);
                let operation = unwrap_to!(operation => NLOperation::Operator);
                let (a, b) = unwrap_to!(operation => OpOperator::CompareGreater);

                assert_eq!(
                    unwrap_constant_number(a),
                    2,
                    "Wrong number for left operand."
                );
                assert_eq!(
                    unwrap_constant_number(b),
                    3,
                    "Wrong number for right operand."
                );
            }

            #[test]
            fn less() {
                let code = "2 < 3";
                let operation = pretty_read(code, &read_operation);
                let operation = unwrap_to!(operation => NLOperation::Operator);
                let (a, b) = unwrap_to!(operation => OpOperator::CompareLess);

                assert_eq!(
                    unwrap_constant_number(a),
                    2,
                    "Wrong number for left operand."
                );
                assert_eq!(
                    unwrap_constant_number(b),
                    3,
                    "Wrong number for right operand."
                );
            }

            #[test]
            fn greater_equal() {
                let code = "2 >= 3";
                let operation = pretty_read(code, &read_operation);
                let operation = unwrap_to!(operation => NLOperation::Operator);
                let (a, b) = unwrap_to!(operation => OpOperator::CompareGreaterEqual);

                assert_eq!(
                    unwrap_constant_number(a),
                    2,
                    "Wrong number for left operand."
                );
                assert_eq!(
                    unwrap_constant_number(b),
                    3,
                    "Wrong number for right operand."
                );
            }

            #[test]
            fn less_equal() {
                let code = "2 <= 3";
                let operation = pretty_read(code, &read_operation);
                let operation = unwrap_to!(operation => NLOperation::Operator);
                let (a, b) = unwrap_to!(operation => OpOperator::CompareLessEqual);

                assert_eq!(
                    unwrap_constant_number(a),
                    2,
                    "Wrong number for left operand."
                );
                assert_eq!(
                    unwrap_constant_number(b),
                    3,
                    "Wrong number for right operand."
                );
            }

            // TODO add tests for proper error messages in =< and => conditions.
        }

        mod logical {
            use super::*;

            #[test]
            fn negate() {
                let code = "!false";
                let operation = pretty_read(code, &read_operation);
                let operation = unwrap_to!(operation => NLOperation::Operator);
                let value = unwrap_to!(operation => OpOperator::LogicalNegate);

                let value = unwrap_constant_boolean(value);
                assert_eq!(value, false, "Wrong value for constant.");
            }

            #[test]
            fn and() {
                let code = "false && true";
                let operation = pretty_read(code, &read_operation);
                let operation = unwrap_to!(operation => NLOperation::Operator);
                let (a, b) = unwrap_to!(operation => OpOperator::LogicalAnd);

                let a = unwrap_constant_boolean(a);
                let b = unwrap_constant_boolean(b);
                assert_eq!(a, false, "Wrong value for constant.");
                assert_eq!(b, true, "Wrong value for constant.");
            }

            #[test]
            fn or() {
                let code = "false || true";
                let operation = pretty_read(code, &read_operation);
                let operation = unwrap_to!(operation => NLOperation::Operator);
                let (a, b) = unwrap_to!(operation => OpOperator::LogicalOr);

                let a = unwrap_constant_boolean(a);
                let b = unwrap_constant_boolean(b);
                assert_eq!(a, false, "Wrong value for constant.");
                assert_eq!(b, true, "Wrong value for constant.");
            }

            #[test]
            fn xor() {
                let code = "false ^^ true";
                let operation = pretty_read(code, &read_operation);
                let operation = unwrap_to!(operation => NLOperation::Operator);
                let (a, b) = unwrap_to!(operation => OpOperator::LogicalXor);

                let a = unwrap_constant_boolean(a);
                let b = unwrap_constant_boolean(b);
                assert_eq!(a, false, "Wrong value for constant.");
                assert_eq!(b, true, "Wrong value for constant.");
            }
        }

        mod bitwise {
            use super::*;

            #[test]
            fn negate() {
                let code = "~0"; // FIXME syntax should be !0.
                let operation = pretty_read(code, &read_operation);
                let operation = unwrap_to!(operation => NLOperation::Operator);
                let value = unwrap_to!(operation => OpOperator::BitNegate);

                let value = unwrap_constant_number(value);
                assert_eq!(value, 0, "Wrong value for constant.");
            }

            #[test]
            fn and() {
                let code = "1 & 2";
                let operation = pretty_read(code, &read_operation);

                let operation = unwrap_to!(operation => NLOperation::Operator);
                let (a, b) = unwrap_to!(operation => OpOperator::BitAnd);

                let a = unwrap_constant_number(a);
                let b = unwrap_constant_number(b);
                assert_eq!(a, 1, "Wrong value for constant.");
                assert_eq!(b, 2, "Wrong value for constant.");
            }

            #[test]
            fn or() {
                let code = "1 | 2";
                let operation = pretty_read(code, &read_operation);
                let operation = unwrap_to!(operation => NLOperation::Operator);
                let (a, b) = unwrap_to!(operation => OpOperator::BitOr);

                let a = unwrap_constant_number(a);
                let b = unwrap_constant_number(b);
                assert_eq!(a, 1, "Wrong value for constant.");
                assert_eq!(b, 2, "Wrong value for constant.");
            }

            #[test]
            fn xor() {
                let code = "1 ^ 2";
                let operation = pretty_read(code, &read_operation);
                let operation = unwrap_to!(operation => NLOperation::Operator);
                let (a, b) = unwrap_to!(operation => OpOperator::BitXor);

                let a = unwrap_constant_number(a);
                let b = unwrap_constant_number(b);
                assert_eq!(a, 1, "Wrong value for constant.");
                assert_eq!(b, 2, "Wrong value for constant.");
            }

            #[test]
            fn left_shift() {
                let code = "1 << 2";
                let operation = pretty_read(code, &read_operation);
                let operation = unwrap_to!(operation => NLOperation::Operator);
                let (a, b) = unwrap_to!(operation => OpOperator::BitLeftShift);

                let a = unwrap_constant_number(a);
                let b = unwrap_constant_number(b);
                assert_eq!(a, 1, "Wrong value for constant.");
                assert_eq!(b, 2, "Wrong value for constant.");
            }

            #[test]
            fn right_shift() {
                let code = "1 >> 2";
                let operation = pretty_read(code, &read_operation);
                let operation = unwrap_to!(operation => NLOperation::Operator);
                let (a, b) = unwrap_to!(operation => OpOperator::BitRightShift);

                let a = unwrap_constant_number(a);
                let b = unwrap_constant_number(b);
                assert_eq!(a, 1, "Wrong value for constant.");
                assert_eq!(b, 2, "Wrong value for constant.");
            }
        }

        mod arithmetic {
            use super::*;

            #[test]
            fn negate() {
                let code = "-(-5)";
                let operation = pretty_read(code, &read_operation);
                let operation = unwrap_to!(operation => NLOperation::Operator);
                let value = unwrap_to!(operation => OpOperator::ArithmeticNegate);
                let tuple = unwrap_to!(**value => NLOperation::Tuple);

                assert_eq!(tuple.len(), 1, "Tuple is wrong size.");
                let value = unwrap_constant_number(&tuple[0]);
                assert_eq!(value as i64, -5, "Wrong value for constant.");
            }

            #[test]
            fn amod() {
                let code = "1 % 2";
                let operation = pretty_read(code, &read_operation);
                let operation = unwrap_to!(operation => NLOperation::Operator);
                let (a, b) = unwrap_to!(operation => OpOperator::ArithmeticMod);

                let a = unwrap_constant_number(a);
                let b = unwrap_constant_number(b);
                assert_eq!(a, 1, "Wrong value for constant.");
                assert_eq!(b, 2, "Wrong value for constant.");
            }

            #[test]
            fn add() {
                let code = "1 + 2";
                let operation = pretty_read(code, &read_operation);
                let operation = unwrap_to!(operation => NLOperation::Operator);
                let (a, b) = unwrap_to!(operation => OpOperator::ArithmeticAdd);

                let a = unwrap_constant_number(a);
                let b = unwrap_constant_number(b);
                assert_eq!(a, 1, "Wrong value for constant.");
                assert_eq!(b, 2, "Wrong value for constant.");
            }

            #[test]
            fn sub() {
                let code = "1 - 2";
                let operation = pretty_read(code, &read_operation);
                let operation = unwrap_to!(operation => NLOperation::Operator);
                let (a, b) = unwrap_to!(operation => OpOperator::ArithmeticSub);

                let a = unwrap_constant_number(a);
                let b = unwrap_constant_number(b);
                assert_eq!(a, 1, "Wrong value for constant.");
                assert_eq!(b, 2, "Wrong value for constant.");
            }

            #[test]
            fn mul() {
                let code = "1 * 2";
                let operation = pretty_read(code, &read_operation);
                let operation = unwrap_to!(operation => NLOperation::Operator);
                let (a, b) = unwrap_to!(operation => OpOperator::ArithmeticMul);

                let a = unwrap_constant_number(a);
                let b = unwrap_constant_number(b);
                assert_eq!(a, 1, "Wrong value for constant.");
                assert_eq!(b, 2, "Wrong value for constant.");
            }

            #[test]
            fn div() {
                let code = "1 / 2";
                let operation = pretty_read(code, &read_operation);
                let operation = unwrap_to!(operation => NLOperation::Operator);
                let (a, b) = unwrap_to!(operation => OpOperator::ArithmeticDiv);

                let a = unwrap_constant_number(a);
                let b = unwrap_constant_number(b);
                assert_eq!(a, 1, "Wrong value for constant.");
                assert_eq!(b, 2, "Wrong value for constant.");
            }

            #[test]
            fn range() {
                let code = "1 .. 2";
                let operation = pretty_read(code, &read_operation);
                let operation = unwrap_to!(operation => NLOperation::Operator);
                let (a, b) = unwrap_to!(operation => OpOperator::Range);

                let a = unwrap_constant_number(a);
                let b = unwrap_constant_number(b);
                assert_eq!(a, 1, "Wrong value for constant.");
                assert_eq!(b, 2, "Wrong value for constant.");
            }
        }
    }

    mod if_statements {
        use super::*;

        #[test]
        fn basic_if() {
            let code = "if true { false }";
            let operation = pretty_read(code, &read_operation);
            let statement = unwrap_to!(operation => NLOperation::If);

            let condition = unwrap_constant_boolean(&statement.condition);
            let true_block = &statement.true_block;
            let false_block = &statement.false_block;

            assert_eq!(condition, true, "Wrong condition value read.");
            assert_eq!(
                true_block.operations.len(),
                1,
                "Wrong number of operations in true block."
            );
            assert_eq!(
                unwrap_constant_boolean(&true_block.operations[0]),
                false,
                "Expected a false boolean in the true block."
            );
            assert_eq!(
                false_block.operations.len(),
                0,
                "Wrong number of operations in false block."
            );
        }

        #[test]
        fn if_else() {
            let code = "if true { false } else { true }";
            let operation = pretty_read(code, &read_operation);
            let statement = unwrap_to!(operation => NLOperation::If);

            let condition = unwrap_constant_boolean(&statement.condition);
            let true_block = &statement.true_block;
            let false_block = &statement.false_block;

            assert_eq!(condition, true, "Wrong condition value read.");
            assert_eq!(
                true_block.operations.len(),
                1,
                "Wrong number of operations in true block."
            );
            assert_eq!(
                unwrap_constant_boolean(&true_block.operations[0]),
                false,
                "Expected a false boolean in the true block."
            );
            assert_eq!(
                false_block.operations.len(),
                1,
                "Wrong number of operations in false block."
            );
            assert_eq!(
                unwrap_constant_boolean(&false_block.operations[0]),
                true,
                "Expected a true boolean in the true block."
            );
        }

        #[test]
        fn and_if() {
            let code = "if true && false {}";
            let operation = pretty_read(code, &read_operation);
            let statement = unwrap_to!(operation => NLOperation::If);

            let condition = &statement.condition;
            let operator = unwrap_to!(**condition => NLOperation::Operator);
            let (op_a, op_b) = unwrap_to!(operator => OpOperator::LogicalAnd);
            let op_a = unwrap_constant_boolean(op_a);
            let op_b = unwrap_constant_boolean(op_b);

            assert_eq!(op_a, true, "Expected true for op_a");
            assert_eq!(op_b, false, "Expected true for ob_b");
        }
    }

    mod loops {
        use super::*;

        #[test]
        fn basic_loop() {
            let code = "loop { true }";
            let operation = pretty_read(code, &read_operation);
            let block = unwrap_to!(operation => NLOperation::Loop);

            assert_eq!(
                block.operations.len(),
                1,
                "Wrong number of operations in block."
            );
            assert_eq!(
                unwrap_constant_boolean(&block.operations[0]),
                true,
                "Expected true for boolean value in block."
            );
        }

        #[test]
        fn while_loop() {
            let code = "while true { false }";
            let operation = pretty_read(code, &read_operation);
            let while_loop = unwrap_to!(operation => NLOperation::WhileLoop);

            assert_eq!(
                unwrap_constant_boolean(&while_loop.condition),
                true,
                "Expected true value for condition."
            );

            assert_eq!(
                while_loop.block.operations.len(),
                1,
                "Wrong number of operations in block."
            );
            assert_eq!(
                unwrap_constant_boolean(&while_loop.block.operations[0]),
                false,
                "Expected false for boolean value in block."
            );
        }

        #[test]
        fn while_loop_with_and() {
            let code = "while true && false { false }";
            let operation = pretty_read(code, &read_operation);
            let while_loop = unwrap_to!(operation => NLOperation::WhileLoop);

            let condition = &while_loop.condition;
            let operator = unwrap_to!(**condition => NLOperation::Operator);
            let (left, right) = unwrap_to!(operator => OpOperator::LogicalAnd);

            assert_eq!(
                unwrap_constant_boolean(&left),
                true,
                "Expected true for left operand of and."
            );
            assert_eq!(
                unwrap_constant_boolean(&right),
                false,
                "Expected false for right operand of and."
            );

            assert_eq!(
                while_loop.block.operations.len(),
                1,
                "Wrong number of operations in block."
            );
            assert_eq!(
                unwrap_constant_boolean(&while_loop.block.operations[0]),
                false,
                "Expected false for boolean value in block."
            );
        }

        #[test]
        fn for_loop() {
            let code = "for bah in false { true }";
            let operation = pretty_read(code, &read_operation);
            let for_loop = unwrap_to!(operation => NLOperation::ForLoop);

            assert_eq!(
                for_loop.variable.name, "bah",
                "Wrong name given to variable."
            );
            assert_eq!(
                unwrap_constant_boolean(&for_loop.iterator),
                false,
                "Expected false for range."
            );
            assert_eq!(
                for_loop.block.operations.len(),
                1,
                "Wrong number of operations in block."
            );
            assert_eq!(
                unwrap_constant_boolean(&for_loop.block.operations[0]),
                true,
                "Expected true for boolean value in block."
            );
        }

        #[test]
        fn break_keyword() {
            let code = "break";
            let operation = pretty_read(code, &read_operation);

            match operation {
                NLOperation::Break => {
                    // We pass. That's it.
                }
                _ => panic!("Expected break operation, got {:?}", operation),
            }
        }
    }

    mod match_statements {
        use super::*;

        #[test]
        fn basic_match() {
            let code = "match variable {}";
            let operation = pretty_read(code, &read_operation);
            let nl_match = unwrap_to!(operation => NLOperation::Match);

            assert_eq!(
                unwrap_to!(*nl_match.input => NLOperation::VariableAccess).get_name(),
                "variable"
            );
            assert_eq!(nl_match.branches.len(), 0);
        }

        #[test]
        fn one_branch() {
            let code = "match variable { Enum::One => 0, }";
            let operation = pretty_read(code, &read_operation);
            let nl_match = unwrap_to!(operation => NLOperation::Match);

            assert_eq!(
                unwrap_to!(*nl_match.input => NLOperation::VariableAccess).get_name(),
                "variable"
            );

            let branches = &nl_match.branches;
            assert_eq!(branches.len(), 1);

            let (branch, operation) = &branches[0];
            let branch = unwrap_to!(branch => MatchBranch::Enum);
            assert_eq!(branch.nl_enum, "Enum");
            assert_eq!(branch.variant, "One");

            assert_eq!(unwrap_constant_number(operation), 0);

            assert_eq!(branch.variables.len(), 0);
        }

        #[test]
        fn one_branch_no_comma() {
            let code = "match variable { Enum::One => 0 }";
            let operation = pretty_read(code, &read_operation);
            let nl_match = unwrap_to!(operation => NLOperation::Match);

            assert_eq!(
                unwrap_to!(*nl_match.input => NLOperation::VariableAccess).get_name(),
                "variable"
            );

            let branches = &nl_match.branches;
            assert_eq!(branches.len(), 1);

            let (branch, operation) = &branches[0];
            let branch = unwrap_to!(branch => MatchBranch::Enum);
            assert_eq!(branch.nl_enum, "Enum");
            assert_eq!(branch.variant, "One");

            assert_eq!(unwrap_constant_number(operation), 0);

            assert_eq!(branch.variables.len(), 0);
        }

        #[test]
        fn one_branch_one_variable() {
            let code = "match variable { Enum::One(a) => 0, }";
            let operation = pretty_read(code, &read_operation);
            let nl_match = unwrap_to!(operation => NLOperation::Match);

            assert_eq!(
                unwrap_to!(*nl_match.input => NLOperation::VariableAccess).get_name(),
                "variable"
            );

            let branches = &nl_match.branches;
            assert_eq!(branches.len(), 1);

            let (branch, operation) = &branches[0];
            let branch = unwrap_to!(branch => MatchBranch::Enum);
            assert_eq!(branch.nl_enum, "Enum");
            assert_eq!(branch.variant, "One");

            assert_eq!(unwrap_constant_number(operation), 0);

            let variables = &branch.variables;
            assert_eq!(variables.len(), 1);
            assert_eq!(variables[0], "a");
        }

        #[test]
        fn one_branch_two_variable() {
            let code = "match variable { Enum::One(a, b) => 0, }";
            let operation = pretty_read(code, &read_operation);
            let nl_match = unwrap_to!(operation => NLOperation::Match);

            assert_eq!(
                unwrap_to!(*nl_match.input => NLOperation::VariableAccess).get_name(),
                "variable"
            );

            let branches = &nl_match.branches;
            assert_eq!(branches.len(), 1);

            let (branch, operation) = &branches[0];
            let branch = unwrap_to!(branch => MatchBranch::Enum);
            assert_eq!(branch.nl_enum, "Enum");
            assert_eq!(branch.variant, "One");

            assert_eq!(unwrap_constant_number(operation), 0);

            let variables = &branch.variables;
            assert_eq!(variables.len(), 2);
            assert_eq!(variables[0], "a");
            assert_eq!(variables[1], "b");
        }

        #[test]
        fn two_branch() {
            let code = "match variable { Enum::One => 0, Enum::Two => 1, }";
            let operation = pretty_read(code, &read_operation);
            let nl_match = unwrap_to!(operation => NLOperation::Match);

            assert_eq!(
                unwrap_to!(*nl_match.input => NLOperation::VariableAccess).get_name(),
                "variable"
            );

            let branches = &nl_match.branches;
            assert_eq!(branches.len(), 2);

            let (branch, operation) = &branches[0];
            let branch = unwrap_to!(branch => MatchBranch::Enum);
            assert_eq!(branch.nl_enum, "Enum");
            assert_eq!(branch.variant, "One");

            assert_eq!(unwrap_constant_number(operation), 0);

            assert_eq!(branch.variables.len(), 0);

            let (branch, operation) = &branches[1];
            let branch = unwrap_to!(branch => MatchBranch::Enum);
            assert_eq!(branch.nl_enum, "Enum");
            assert_eq!(branch.variant, "Two");

            assert_eq!(unwrap_constant_number(operation), 1);

            assert_eq!(branch.variables.len(), 0);
        }

        #[test]
        fn two_branch_no_comma() {
            let code = "match variable { Enum::One => 0, Enum::Two => 1 }";
            let operation = pretty_read(code, &read_operation);
            let nl_match = unwrap_to!(operation => NLOperation::Match);

            assert_eq!(
                unwrap_to!(*nl_match.input => NLOperation::VariableAccess).get_name(),
                "variable"
            );

            let branches = &nl_match.branches;
            assert_eq!(branches.len(), 2);

            let (branch, operation) = &branches[0];
            let branch = unwrap_to!(branch => MatchBranch::Enum);
            assert_eq!(branch.nl_enum, "Enum");
            assert_eq!(branch.variant, "One");

            assert_eq!(unwrap_constant_number(operation), 0);

            assert_eq!(branch.variables.len(), 0);

            let (branch, operation) = &branches[1];
            let branch = unwrap_to!(branch => MatchBranch::Enum);
            assert_eq!(branch.nl_enum, "Enum");
            assert_eq!(branch.variant, "Two");

            assert_eq!(unwrap_constant_number(operation), 1);

            assert_eq!(branch.variables.len(), 0);
        }

        #[test]
        fn one_branch_constant() {
            let code = "match variable { 42 => 0, }";
            let operation = pretty_read(code, &read_operation);
            let nl_match = unwrap_to!(operation => NLOperation::Match);

            assert_eq!(
                unwrap_to!(*nl_match.input => NLOperation::VariableAccess).get_name(),
                "variable"
            );

            let branches = &nl_match.branches;
            assert_eq!(branches.len(), 1);

            let (branch, operation) = &branches[0];
            let branch = unwrap_to!(branch => MatchBranch::Constant);
            match branch {
                OpConstant::Integer(value, _) => {
                    assert_eq!(*value, 42);
                }
                _ => {
                    panic!("Expected integer for constant type, got: {:?}");
                }
            }

            assert_eq!(unwrap_constant_number(operation), 0);
        }

        #[test]
        fn one_branch_range() {
            let code = "match variable { 25..42 => 0, }";
            let operation = pretty_read(code, &read_operation);
            println!("{:?}", operation);
            let nl_match = unwrap_to!(operation => NLOperation::Match);

            assert_eq!(
                unwrap_to!(*nl_match.input => NLOperation::VariableAccess).get_name(),
                "variable"
            );

            let branches = &nl_match.branches;
            assert_eq!(branches.len(), 1);

            let (branch, operation) = &branches[0];
            let (low, high) = unwrap_to!(branch => MatchBranch::Range);

            assert_eq!(*low, 25);
            assert_eq!(*high, 42);

            assert_eq!(unwrap_constant_number(operation), 0);
        }
    }

    mod function_calls {
        use super::*;

        #[test]
        fn call() {
            let code = "function()";
            let operation = pretty_read(code, &read_operation);
            let function = unwrap_to!(operation => NLOperation::FunctionCall);

            assert_eq!(function.path, "function");
            assert_eq!(function.arguments.len(), 0);
        }

        #[test]
        fn call_from_namespace() {
            let code = "namespace.function()";
            let operation = pretty_read(code, &read_operation);
            let function = unwrap_to!(operation => NLOperation::FunctionCall);

            assert_eq!(function.path, "namespace.function");
            assert_eq!(function.arguments.len(), 0);
        }

        #[test]
        fn call_one_arg() {
            let code = "function(one)";
            let operation = pretty_read(code, &read_operation);
            let function = unwrap_to!(operation => NLOperation::FunctionCall);

            assert_eq!(function.path, "function");

            let arguments = &function.arguments;
            assert_eq!(arguments.len(), 1);
            assert_eq!(arguments[0], "one");
        }

        #[test]
        fn call_two_arg() {
            let code = "function(one, two)";
            let operation = pretty_read(code, &read_operation);
            let function = unwrap_to!(operation => NLOperation::FunctionCall);

            assert_eq!(function.path, "function");

            let arguments = &function.arguments;

            assert_eq!(arguments.len(), 2);
            assert_eq!(arguments[0], "one");
            assert_eq!(arguments[1], "two");
        }
    }
}
