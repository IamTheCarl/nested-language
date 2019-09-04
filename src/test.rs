
use super::*;

mod structure_and_root {
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
        let file_name = "tests/empty_struct_and_trait.nl";
        parse_file(&mut Path::new(file_name), &|file: &NLFile| {
            assert_eq!(file.name, "empty_struct_and_trait.nl", "File name not copied correctly.");

            assert_eq!(file.traits.len(), 1, "Wrong number of traits.");
            let my_trait = &file.traits[0];
            assert_eq!(my_trait.name, "MyTrait", "Wrong name for trait.");

            assert_eq!(file.structs.len(), 1, "Wrong number of structs.");
            let my_struct = &file.structs[0];
            assert_eq!(my_struct.name, "MyStruct", "Wrong name for struct.");
        }).unwrap();
    }

    #[test]
    /// Compile a file with an invalid token in its root.
    fn bad_root() {
        let file_name = "tests/bad_root.nl";
        let result = parse_file(&mut Path::new(file_name), &|_file: &NLFile| {});
        match result {
            Err(error) => {
                // Everything is fine! ... in a way.
                assert!(error.description().contains("I shouldn't be here in the root."));
            },
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
            let file_name = "tests/single_struct_empty.nl";
            parse_file(&mut Path::new(file_name), &|file: &NLFile| {
                assert_eq!(file.name, "single_struct_empty.nl", "File name not copied correctly.");

                assert_eq!(file.structs.len(), 1, "Wrong number of structs.");
                let my_struct = &file.structs[0];
                assert_eq!(my_struct.name, "MyStruct", "Wrong name for struct.");

                assert_eq!(file.traits.len(), 0, "Wrong number of traits.");
            }).unwrap();
        }

        #[test]
        /// Compile a single struct with a single variable.
        fn single_variable_struct() {
            let file_name = "tests/struct_with_single_variable.nl";
            parse_file(&mut Path::new(file_name), &|file: &NLFile| {
                assert_eq!(file.structs.len(), 1, "Wrong number of structs.");
                let my_struct = &file.structs[0];
                assert_eq!(my_struct.name, "MyStruct", "Wrong name for struct.");
                assert_eq!(my_struct.variables.len(), 1, "Wrong number of variables.");
                let variable = &my_struct.variables[0];
                assert_eq!(variable.name, "variable", "Variable had wrong name.");
                assert_eq!(variable.my_type, NLType::I32, "Variable had wrong type.");
            }).unwrap();
        }

        #[test]
        /// Compile a single struct with a single variable. We don't put the trailing comma after this one.
        fn single_variable_struct_no_ending_comma() {
            let file_name = "tests/struct_with_single_variable_no_comma.nl";
            parse_file(&mut Path::new(file_name), &|file: &NLFile| {
                assert_eq!(file.structs.len(), 1, "Wrong number of structs.");
                let my_struct = &file.structs[0];
                assert_eq!(my_struct.name, "MyStruct", "Wrong name for struct.");
                assert_eq!(my_struct.variables.len(), 1, "Wrong number of variables.");
                let variable = &my_struct.variables[0];
                assert_eq!(variable.name, "variable", "Variable had wrong name.");
                assert_eq!(variable.my_type, NLType::I32, "Variable had wrong type.");
            }).unwrap();
        }

        #[test]
        /// Compile a single struct with two variables. We don't put the trailing comma after the last one.
        fn two_variable_struct_no_ending_comma() {
            let file_name = "tests/struct_with_two_variables_no_ending_comma.nl";
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
            }).unwrap();
        }

        #[test]
        /// Compile a file with an empty struct and an empty trait. This one is special because it has single line comments in it.
        fn empty_struct_and_trait_single_line_comments() {
            let file_name = "tests/empty_struct_and_trait_with_single_line_comments.nl";
            parse_file(&mut Path::new(file_name), &|file: &NLFile| {
                assert_eq!(file.name, "empty_struct_and_trait_with_single_line_comments.nl", "File name not copied correctly.");

                assert_eq!(file.traits.len(), 1, "Wrong number of traits.");
                let my_trait = &file.traits[0];
                assert_eq!(my_trait.name, "MyTrait", "Wrong name for trait.");

                assert_eq!(file.structs.len(), 1, "Wrong number of structs.");
                let my_struct = &file.structs[0];
                assert_eq!(my_struct.name, "MyStruct", "Wrong name for struct.");
            }).unwrap();
        }

        #[test]
        /// Compile a file with an empty struct and an empty trait. This one is special because it has multi line comments in it.
        fn empty_struct_and_trait_multi_line_comments() {
            let file_name = "tests/empty_struct_and_trait_with_multi_line_comments.nl";
            parse_file(&mut Path::new(file_name), &|file: &NLFile| {
                assert_eq!(file.name, "empty_struct_and_trait_with_multi_line_comments.nl", "File name not copied correctly.");

                assert_eq!(file.traits.len(), 1, "Wrong number of traits.");
                let my_trait = &file.traits[0];
                assert_eq!(my_trait.name, "MyTrait", "Wrong name for trait.");

                assert_eq!(file.structs.len(), 1, "Wrong number of structs.");
                let my_struct = &file.structs[0];
                assert_eq!(my_struct.name, "MyStruct", "Wrong name for struct.");
            }).unwrap();
        }

        #[test]
        /// Compile a file with an empty struct and an empty trait. This one is special because it has multi line comments in it.
        fn struct_empty_self_implementation() {
            let file_name = "tests/struct_with_empty_self_implementation.nl";
            parse_file(&mut Path::new(file_name), &|file: &NLFile| {
                assert_eq!(file.structs.len(), 1, "Wrong number of structs.");
                let my_struct = &file.structs[0];

                assert_eq!(my_struct.implementations.len(), 1, "Wrong number of implementations.");
                let implementation = &my_struct.implementations[0];

                assert_eq!(implementation.name, "Self", "Implementation had wrong name.");
            }).unwrap();
        }

        #[test]
        /// Compile a file with an empty struct and an empty trait. This one is special because it has multi line comments in it.
        fn struct_self_implementation_with_methods() {
            let file_name = "tests/struct_self_implementation_with_methods.nl";
            parse_file(&mut Path::new(file_name), &|file: &NLFile| {
                assert_eq!(file.structs.len(), 1, "Wrong number of structs.");
                let my_struct = &file.structs[0];

                assert_eq!(my_struct.implementations.len(), 1, "Wrong number of implementations.");
                let implementation = &my_struct.implementations[0];

                assert_eq!(implementation.name, "Self", "Implementation had wrong name.");
                assert_eq!(implementation.implementors.len(), 4, "Wrong number of methods.");
            }).unwrap();
        }

        #[test]
        /// Compile a file with an empty struct and an empty trait. This one is special because it has multi line comments in it.
        fn struct_self_implementation_with_methods_and_encapsulations() {
            let file_name = "tests/struct_self_implementation_with_methods_and_encapsulations.nl";
            parse_file(&mut Path::new(file_name), &|file: &NLFile| {
                assert_eq!(file.structs.len(), 1, "Wrong number of structs.");
                let my_struct = &file.structs[0];

                assert_eq!(my_struct.implementations.len(), 1, "Wrong number of implementations.");
                let implementation = &my_struct.implementations[0];

                assert_eq!(implementation.name, "Self", "Implementation had wrong name.");
                assert_eq!(implementation.implementors.len(), 10, "Wrong number of methods.");
            }).unwrap();
        }
    }

    mod nl_trait {
        use super::*;

        #[test]
        /// Compile a file with a single empty trait. We should get no errors or warnings.
        fn single_empty_trait() {
            let file_name = "tests/single_trait_empty.nl";
            parse_file(&mut Path::new(file_name), &|file: &NLFile| {
                assert_eq!(file.name, "single_trait_empty.nl", "File name not copied correctly.");

                assert_eq!(file.traits.len(), 1, "Wrong number of traits.");
                let my_trait = &file.traits[0];
                assert_eq!(my_trait.name, "MyTrait", "Wrong name for trait.");

                assert_eq!(file.structs.len(), 0, "Wrong number of structs.");
            }).unwrap();
        }

        #[test]
        /// Tests a struct with encapsulations.
        fn trait_with_methods_and_encapsulators() {
            let file_name = "tests/trait_with_methods_and_encapsulators.nl";
            parse_file(&mut Path::new(file_name), &|file: &NLFile| {
                assert_eq!(file.traits.len(), 1, "Wrong number of traits.");
                let my_trait = &file.traits[0];

                assert_eq!(my_trait.name, "MyTrait", "Implementation had wrong name.");
                assert_eq!(my_trait.implementors.len(), 10, "Wrong number of methods.");
            }).unwrap();
        }
    }

    mod argument_list {
        use super::*;

        fn pretty_read(input: &str) -> Vec<NLArgument> {
            let result = read_argument_deceleration_list(input);
            match result {
                Ok(tuple) => {
                    let (_, args) = tuple;

                    args
                },
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
        /// Testing the argument declaration reader.
        fn empty() {
            let code = "()";
            let args = pretty_read(code);

            assert_eq!(args.len(), 0, "Wrong number of args.");
        }

        #[test]
        /// Testing the argument declaration reader.
        fn single_arg() {
            let code = "(argA : i32)";
            let args = pretty_read(code);

            assert_eq!(args.len(), 1, "Wrong number of args.");

            let arg = &args[0];
            assert_eq!(arg.name, "argA", "Wrong argument name.");
            assert_eq!(arg.nl_type, NLType::I32, "Wrong argument type.");
        }

        #[test]
        /// Testing the argument declaration reader.
        fn two_args() {
            let code = "(argA : i32, argB : i16)";
            let args = pretty_read(code);

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
            let args = pretty_read(code);

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
            let args = pretty_read(code);

            assert_eq!(args.len(), 1, "Wrong number of args.");

            let arg = &args[0];
            assert_eq!(arg.name, "self", "Wrong argument name.");
            assert_eq!(arg.nl_type, NLType::SelfReference, "Wrong argument type.");
        }

        #[test]
        /// Testing the argument declaration reader.
        fn mutable_self_reference_arg() {
            let code = "(&mut self)";
            let args = pretty_read(code);

            assert_eq!(args.len(), 1, "Wrong number of args.");

            let arg = &args[0];
            assert_eq!(arg.name, "self", "Wrong argument name.");
            assert_eq!(arg.nl_type, NLType::MutableSelfReference, "Wrong argument type.");
        }

        #[test]
        /// Testing the argument declaration reader.
        fn mutable_self_reference_arg_odd_spacing() {
            let code = "(&mut\tself)";
            let args = pretty_read(code);

            assert_eq!(args.len(), 1, "Wrong number of args.");

            let arg = &args[0];
            assert_eq!(arg.name, "self", "Wrong argument name.");
            assert_eq!(arg.nl_type, NLType::MutableSelfReference, "Wrong argument type.");
        }

        #[test]
        /// Testing the argument declaration reader.
        fn self_reference_arg_odd_pre_space() {
            let code = "(& self)";
            let args = pretty_read(code);

            assert_eq!(args.len(), 1, "Wrong number of args.");

            let arg = &args[0];
            assert_eq!(arg.name, "self", "Wrong argument name.");
            assert_eq!(arg.nl_type, NLType::SelfReference, "Wrong argument type.");
        }

        #[test]
        /// Testing the argument declaration reader.
        fn mutable_self_reference_arg_odd_pre_space() {
            let code = "(& mut self)";
            let args = pretty_read(code);

            assert_eq!(args.len(), 1, "Wrong number of args.");

            let arg = &args[0];
            assert_eq!(arg.name, "self", "Wrong argument name.");
            assert_eq!(arg.nl_type, NLType::MutableSelfReference, "Wrong argument type.");
        }

        #[test]
        /// Testing the argument declaration reader.
        fn struct_reference() {
            let code = "(var: &SomeStruct)";
            let args = pretty_read(code);

            assert_eq!(args.len(), 1, "Wrong number of args.");

            let arg = &args[0];
            assert_eq!(arg.name, "var", "Wrong argument name.");
            assert_eq!(arg.nl_type, NLType::ReferencedStruct("SomeStruct"), "Wrong argument type.");
        }

        #[test]
        /// Testing the argument declaration reader.
        fn mutable_struct_reference() {
            let code = "(var: &mut SomeStruct)";
            let args = pretty_read(code);

            assert_eq!(args.len(), 1, "Wrong number of args.");

            let arg = &args[0];
            assert_eq!(arg.name, "var", "Wrong argument name.");
            assert_eq!(arg.nl_type, NLType::MutableReferencedStruct("SomeStruct"), "Wrong argument type.");
        }

        #[test]
        /// Testing the argument declaration reader.
        fn struct_owned() {
            let code = "(var: SomeStruct)";
            let args = pretty_read(code);

            assert_eq!(args.len(), 1, "Wrong number of args.");

            let arg = &args[0];
            assert_eq!(arg.name, "var", "Wrong argument name.");
            assert_eq!(arg.nl_type, NLType::OwnedStruct("SomeStruct"), "Wrong argument type.");
        }

        #[test]
        /// Testing the argument declaration reader.
        fn trait_reference() {
            let code = "(var: &dyn SomeTrait)";
            let args = pretty_read(code);

            assert_eq!(args.len(), 1, "Wrong number of args.");

            let arg = &args[0];
            assert_eq!(arg.name, "var", "Wrong argument name.");
            assert_eq!(arg.nl_type, NLType::ReferencedTrait("SomeTrait"), "Wrong argument type.");
        }

        #[test]
        /// Testing the argument declaration reader.
        fn mutable_trait_reference() {
            let code = "(var: &mut dyn SomeTrait)";
            let args = pretty_read(code);

            assert_eq!(args.len(), 1, "Wrong number of args.");

            let arg = &args[0];
            assert_eq!(arg.name, "var", "Wrong argument name.");
            assert_eq!(arg.nl_type, NLType::MutableReferencedTrait("SomeTrait"), "Wrong argument type.");
        }

        #[test]
        /// Testing the argument declaration reader.
        fn trait_owned() {
            let code = "(var: dyn SomeTrait)";
            let args = pretty_read(code);

            assert_eq!(args.len(), 1, "Wrong number of args.");

            let arg = &args[0];
            assert_eq!(arg.name, "var", "Wrong argument name.");
            assert_eq!(arg.nl_type, NLType::OwnedTrait("SomeTrait"), "Wrong argument type.");
        }
    }

    mod nl_methods {
        use super::*;

        fn pretty_read_method(input: &str) -> (&str, NLMethod) {
            let result = read_method(input);
            match result {
                Ok(tuple) => {
                    let (s, method) = tuple;
                    match method {
                        NLImplementor::Method(method) => {
                            (s, method)
                        },
                        _ => {
                            panic!("Did not get a method.");
                        }
                    }
                },
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
            assert_eq!(method.block.is_none(), true, "Method should not have been implemented.");
        }

        #[test]
        /// Construct a blank and unimplemented method.
        fn method_no_arg_return_i32_no_impl() {
            let code = "met my_method() -> i32;";

            let (_, method) = pretty_read_method(code);

            assert_eq!(method.name, "my_method", "Method had wrong name.");
            assert_eq!(method.arguments.len(), 0, "Wrong number of arguments.");
            assert_eq!(method.return_type, NLType::I32, "Wrong return type.");
            assert_eq!(method.block.is_none(), true, "Method should not have been implemented.");
        }

        #[test]
        /// Construct a blank and unimplemented method.
        fn method_no_arg_no_return_implemented() {
            let code = "met my_method() {}";

            let (_, method) = pretty_read_method(code);

            assert_eq!(method.name, "my_method", "Method had wrong name.");
            assert_eq!(method.arguments.len(), 0, "Wrong number of arguments.");
            assert_eq!(method.return_type, NLType::None, "Wrong return type.");
            assert_eq!(method.block.is_none(), false, "Method should have been implemented.");
        }

        #[test]
        /// Construct a blank and unimplemented method.
        fn method_no_arg_return_i32_implemented() {
            let code = "met my_method() -> i32 {}";

            let (_, method) = pretty_read_method(code);

            assert_eq!(method.name, "my_method", "Method had wrong name.");
            assert_eq!(method.arguments.len(), 0, "Wrong number of arguments.");
            assert_eq!(method.return_type, NLType::I32, "Wrong return type.");
            assert_eq!(method.block.is_none(), false, "Method should have been implemented.");
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
                        NLImplementor::Getter(getter) => {
                            (s, getter)
                        },
                        _ => {
                            panic!("Did not get a getter.");
                        }
                    }
                },
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

            assert_eq!(getter.name, "my_getter", "Getter did not have expected name.");
            assert_eq!(getter.block, NLEncapsulationBlock::Default, "Getter did not state use of default implementation.");
            assert_eq!(getter.nl_type, NLType::None, "Getter did not have correct return type.");
        }

        #[test]
        /// A simple test of the getter syntax.
        fn getter_impl() {
            let code = "get my_getter(&self) -> i32 {}";
            let (_, getter) = pretty_read_getter(code);

            assert_eq!(getter.name, "my_getter", "Getter did not have expected name.");
            assert_ne!(getter.block, NLEncapsulationBlock::Default, "Getter did not state use of default implementation.");
            assert_ne!(getter.block, NLEncapsulationBlock::None, "Getter did not state use of default implementation.");
            assert_eq!(getter.nl_type, NLType::I32, "Getter did not have correct return type.");
        }

        #[test]
        /// A simple test of the getter syntax.
        fn getter_no_impl() {
            let code = "get my_getter(&self) -> i32;";
            let (_, getter) = pretty_read_getter(code);

            assert_eq!(getter.name, "my_getter", "Getter did not have expected name.");
            assert_eq!(getter.block, NLEncapsulationBlock::None, "Getter did not state use of no implementation.");
            assert_eq!(getter.nl_type, NLType::I32, "Getter did not have correct return type.");
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
                        NLImplementor::Setter(setter) => {
                            (s, setter)
                        },
                        _ => {
                            panic!("Did not get a setter.");
                        }
                    }
                },
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

            assert_eq!(setter.name, "my_setter", "Setter did not have expected name.");
            assert_eq!(setter.block, NLEncapsulationBlock::Default, "Setter did not state use of default implementation.");
            assert_eq!(setter.args.len(), 0, "Setter did not have correct arguments.");
        }

        #[test]
        /// A simple test of the getter syntax.
        fn getter_impl() {
            let code = "set my_setter(value: i32) {}";
            let (_, setter) = pretty_read_setter(code);

            assert_eq!(setter.name, "my_setter", "Getter did not have expected name.");
            assert_ne!(setter.block, NLEncapsulationBlock::Default, "Setter did not state use of default implementation.");
            assert_ne!(setter.block, NLEncapsulationBlock::None, "Setter did not state use of default implementation.");

            assert_eq!(setter.args.len(), 1, "Setter did not have correct number of arguments.");
            let arg = &setter.args[0];
            assert_eq!(arg.name, "value", "Variable did not have expected name.");
            assert_eq!(arg.nl_type, NLType::I32, "Variable did not have expected type.");
        }

        #[test]
        /// A simple test of the getter syntax.
        fn getter_no_impl() {
            let code = "set my_setter(value: i32);";
            let (_, setter) = pretty_read_setter(code);

            assert_eq!(setter.name, "my_setter", "Setter did not have expected name.");
            assert_eq!(setter.block, NLEncapsulationBlock::None, "Setter did not state use of no implementation.");

            assert_eq!(setter.args.len(), 1, "Setter did not have correct number of arguments.");
            let arg = &setter.args[0];
            assert_eq!(arg.name, "value", "Variable did not have expected name.");
            assert_eq!(arg.nl_type, NLType::I32, "Variable did not have expected type.");
        }
    }
}

mod executable_blocks {
    use super::*;

    mod constants {
        use super::*;

        fn pretty_read(input: &str) -> NLOperation {
            let result = read_constant(input);
            match result {
                Ok(tuple) => {
                    let (_, op) = tuple;

                    op
                },
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
        fn number() {
            let code = "5";
            let constant = pretty_read(code);

            match constant {
                NLOperation::Constant(constant) => {
                    match constant {
                        OpConstant::Integer(constant, cast) => {
                            assert_eq!(constant, 5, "Constant had wrong value.");
                            assert_eq!(cast, NLType::None, "Wrong type cast recommendation.");
                        },
                        _ => panic!("Expected integer for constant type."),
                    }
                },
                _ => panic!("Expected constant"),
            }
        }

        #[test]
        fn negative_number() {
            let code = "-5";
            let constant = pretty_read(code);

            match constant {
                NLOperation::Constant(constant) => {
                    match constant {
                        OpConstant::Integer(constant, cast) => {
                            assert_eq!(constant, -5, "Constant had wrong value.");
                            assert_eq!(cast, NLType::None, "Wrong type cast recommendation.");
                        },
                        _ => panic!("Expected i32 for constant type."),
                    }
                },
                _ => panic!("Expected constant"),
            }
        }

        #[test]
        fn casted_number() {
            let code = "5 as i64";
            let constant = pretty_read(code);

            match constant {
                NLOperation::Constant(constant) => {
                    match constant {
                        OpConstant::Integer(constant, cast) => {
                            assert_eq!(constant, 5, "Constant had wrong value.");
                            assert_eq!(cast, NLType::I64, "Wrong type cast recommendation.");
                        },
                        _ => panic!("Expected i64 for constant type."),
                    }
                },
                _ => panic!("Expected constant"),
            }
        }

        #[test]
        fn negative_casted_number() {
            let code = "-5 as i64";
            let constant = pretty_read(code);

            match constant {
                NLOperation::Constant(constant) => {
                    match constant {
                        OpConstant::Integer(constant, cast) => {
                            assert_eq!(constant, -5, "Constant had wrong value.");
                            assert_eq!(cast, NLType::I64, "Wrong type cast recommendation.");
                        },
                        _ => panic!("Expected i64 for constant type."),
                    }
                },
                _ => panic!("Expected constant"),
            }
        }

        #[test]
        fn float() {
            let code = "5.5";
            let constant = pretty_read(code);

            match constant {
                NLOperation::Constant(constant) => {
                    match constant {
                        OpConstant::Float(constant, cast) => {
                            assert_eq!(constant, 5.5, "Constant had wrong value.");
                            assert_eq!(cast, NLType::None, "Wrong type cast recommendation.");
                        },
                        _ => panic!("Expected integer for constant type."),
                    }
                },
                _ => panic!("Expected constant"),
            }
        }

        #[test]
        fn float_with_cast() {
            let code = "5.5 as f64";
            let constant = pretty_read(code);

            match constant {
                NLOperation::Constant(constant) => {
                    match constant {
                        OpConstant::Float(constant, cast) => {
                            assert_eq!(constant, 5.5, "Constant had wrong value.");
                            assert_eq!(cast, NLType::F64, "Wrong type cast recommendation.");
                        },
                        _ => panic!("Expected integer for constant type."),
                    }
                },
                _ => panic!("Expected constant"),
            }
        }

        #[test]
        fn boolean_true() {
            let code = "true";
            let constant = pretty_read(code);

            match constant {
                NLOperation::Constant(constant) => {
                    match constant {
                        OpConstant::Boolean(constant) => {
                            assert_eq!(constant, true, "Constant had wrong value.");
                        },
                        _ => panic!("Expected boolean for constant type."),
                    }
                },
                _ => panic!("Expected constant"),
            }
        }

        #[test]
        fn boolean_false() {
            let code = "false";
            let constant = pretty_read(code);

            match constant {
                NLOperation::Constant(constant) => {
                    match constant {
                        OpConstant::Boolean(constant) => {
                            assert_eq!(constant, false, "Constant had wrong value.");
                        },
                        _ => panic!("Expected boolean for constant type."),
                    }
                },
                _ => panic!("Expected constant"),
            }
        }

        #[test]
        fn simple_string() {
            let code = "\"A simple string.\"";
            let constant = pretty_read(code);

            match constant {
                NLOperation::Constant(constant) => {
                    match constant {
                        OpConstant::String(string) => {
                            assert_eq!(string, "A simple string.", "Constant had wrong value.");
                        },
                        _ => panic!("Expected string for constant type."),
                    }
                },
                _ => panic!("Expected constant"),
            }
        }
    }

    mod tuples {
        use super::*;

        #[test]
        fn tuple_empty() {
            let code = "()";
            let (_, tuple) = read_tuple_operation(code).unwrap();

            match tuple {
                NLOperation::Tuple(tuple) => {
                    assert_eq!(tuple.len(), 0, "Wrong number of items in tuple.");
                },
                _ => panic!("Expected none."),
            }
        }

        #[test]
        fn tuple_one_item() {
            let code = "(1)";
            let (_, tuple) = read_tuple_operation(code).unwrap();

            match tuple {
                NLOperation::Tuple(tuple) => {
                    assert_eq!(tuple.len(), 1, "Wrong number of items in tuple.");
                    assert_eq!(tuple[0], NLOperation::Constant(OpConstant::Integer(1, NLType::None)), "Wrong value used for first value.");
                },
                _ => panic!("Expected none."),
            }
        }

        #[test]
        fn tuple_two_items() {
            let code = "(1, 2)";
            let (_, tuple) = read_tuple_operation(code).unwrap();

            match tuple {
                NLOperation::Tuple(tuple) => {
                    assert_eq!(tuple.len(), 2, "Wrong number of items in tuple.");
                    assert_eq!(tuple[0], NLOperation::Constant(OpConstant::Integer(1, NLType::None)), "Wrong value used for first value.");
                    assert_eq!(tuple[1], NLOperation::Constant(OpConstant::Integer(2, NLType::None)), "Wrong value used for second value.");
                },
                _ => panic!("Expected none."),
            }
        }

        #[test]
        fn tuple_three_items() {
            let code = "(1, 2, 3)";
            let (_, tuple) = read_tuple_operation(code).unwrap();

            match tuple {
                NLOperation::Tuple(tuple) => {
                    assert_eq!(tuple.len(), 3, "Wrong number of items in tuple.");
                    assert_eq!(tuple[0], NLOperation::Constant(OpConstant::Integer(1, NLType::None)), "Wrong value used for first value.");
                    assert_eq!(tuple[1], NLOperation::Constant(OpConstant::Integer(2, NLType::None)), "Wrong value used for second value.");
                    assert_eq!(tuple[2], NLOperation::Constant(OpConstant::Integer(3, NLType::None)), "Wrong value used for third value.");
                },
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
                    assert_eq!(assign.to_assign.len(), 1, "Wrong number of values being assigned.");
                    assert_eq!(assign.type_assignment, NLType::None, "Unexpected type specified.");

                    let variable = &assign.to_assign[0];

                    assert_eq!(variable.name, "five", "Wrong name given to variable.");


                },
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
                    assert_eq!(assign.to_assign.len(), 1, "Wrong number of values being assigned.");
                    assert_eq!(assign.type_assignment, NLType::I32, "Unexpected type specified.");

                    assert_eq!(assign.assignment,
                               Box::new(NLOperation::Constant(OpConstant::Integer(5, NLType::None))), "Wrong assignment.");

                    let variable = &assign.to_assign[0];

                    assert_eq!(variable.name, "five", "Wrong name given to variable.");


                },
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
                    assert_eq!(assign.to_assign.len(), 2, "Wrong number of values being assigned.");
                    assert_eq!(assign.type_assignment, NLType::None, "Unexpected type specified.");

                    assert_eq!(assign.assignment,
                               Box::new(NLOperation::Tuple(vec![
                                    NLOperation::Constant(OpConstant::Integer(4, NLType::None)),
                                    NLOperation::Constant(OpConstant::Integer(5, NLType::None))
                               ])),
                               "Wrong assignment."
                    );


                    let variable = &assign.to_assign[0];
                    assert_eq!(variable.name, "fore", "Wrong name given to variable.");

                    let variable = &assign.to_assign[1];
                    assert_eq!(variable.name, "five", "Wrong name given to variable.");


                },
                _ => panic!("Expected assignment operation."),
            };
        }
    }
}