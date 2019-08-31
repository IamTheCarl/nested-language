
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
    /// Compile a file with struct with a variable with an invalid type.
    fn struct_with_bad_variable_type() {
        let file_name = "tests/struct_with_single_variable_bad_type.nl";
        let file = parse_file(&mut Path::new(file_name), &|_file: &NLFile| {});

        match file {
            Err(error) => {
                // Everything is fine! ... in a way.
                assert!(error.description().contains("unknown variable type"));
            },
            Ok(_) => {
                panic!("No error when one was expected.");
            }
        }
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
}

mod argument_list {
    use super::*;

    fn pretty_read(input: &str) -> Vec<NLArgument> {
        let result = read_argument_decleration_list(input);
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
        assert_eq!(arg.nl_type, NLType::ReferencedStruct("SomeTrait"), "Wrong argument type.");
    }

    #[test]
    /// Testing the argument declaration reader.
    fn mutable_trait_reference() {
        let code = "(var: &mut dyn SomeTrait)";
        let args = pretty_read(code);

        assert_eq!(args.len(), 1, "Wrong number of args.");

        let arg = &args[0];
        assert_eq!(arg.name, "var", "Wrong argument name.");
        assert_eq!(arg.nl_type, NLType::MutableReferencedStruct("SomeTrait"), "Wrong argument type.");
    }

    #[test]
    /// Testing the argument declaration reader.
    fn trait_owned() {
        let code = "(var: dyn SomeTrait)";
        let args = pretty_read(code);

        assert_eq!(args.len(), 1, "Wrong number of args.");

        let arg = &args[0];
        assert_eq!(arg.name, "var", "Wrong argument name.");
        assert_eq!(arg.nl_type, NLType::OwnedStruct("SomeTrait"), "Wrong argument type.");
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
        let code = "get my_getter:default -> i32;";
        let (_, getter) = pretty_read_getter(code);

        assert_eq!(getter.name, "my_getter", "Getter did not have expected name.");
        assert_eq!(getter.block, NLEncapsulationBlock::Default, "Getter did not state use of default implementation.");
        assert_eq!(getter.nl_type, NLType::I32, "Getter did not have correct return type.");
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