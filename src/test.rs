
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
    let file = parse_file(&mut Path::new(file_name)).unwrap();

    assert_eq!(file.name, "empty_struct_and_trait.nl", "File name not copied correctly.");

    assert_eq!(file.traits.len(), 1, "Wrong number of traits.");
    let my_trait = &file.traits[0];
    assert_eq!(my_trait.name, "MyTrait", "Wrong name for trait.");

    assert_eq!(file.structs.len(), 1, "Wrong number of structs.");
    let my_struct = &file.structs[0];
    assert_eq!(my_struct.name, "MyStruct", "Wrong name for struct.");
}

#[test]
/// Compile a file with struct with a variable with an invalid type.
fn bad_root() {
    let file_name = "tests/bad_root.nl";
    let file = parse_file(&mut Path::new(file_name));

    match file {
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
        let file = parse_file(&mut Path::new(file_name)).unwrap();

        assert_eq!(file.name, "single_struct_empty.nl", "File name not copied correctly.");

        assert_eq!(file.structs.len(), 1, "Wrong number of structs.");
        let my_struct = &file.structs[0];
        assert_eq!(my_struct.name, "MyStruct", "Wrong name for struct.");

        assert_eq!(file.traits.len(), 0, "Wrong number of traits.");
    }

    #[test]
    /// Compile a single struct with a single variable.
    fn single_variable_struct() {
        let file_name = "tests/struct_with_single_variable.nl";
        let file = parse_file(&mut Path::new(file_name)).unwrap();

        assert_eq!(file.structs.len(), 1, "Wrong number of structs.");
        let my_struct = &file.structs[0];
        assert_eq!(my_struct.name, "MyStruct", "Wrong name for struct.");
        assert_eq!(my_struct.variables.len(), 1, "Wrong number of variables.");
        let variable = &my_struct.variables[0];
        assert_eq!(variable.name, "variable", "Variable had wrong name.");
        assert_eq!(variable.my_type, NLType::I32, "Variable had wrong type.");
    }

    #[test]
    /// Compile a single struct with a single variable. We don't put the trailing comma after this one.
    fn single_variable_struct_no_ending_comma() {
        let file_name = "tests/struct_with_single_variable_no_comma.nl";
        let file = parse_file(&mut Path::new(file_name)).unwrap();

        assert_eq!(file.structs.len(), 1, "Wrong number of structs.");
        let my_struct = &file.structs[0];
        assert_eq!(my_struct.name, "MyStruct", "Wrong name for struct.");
        assert_eq!(my_struct.variables.len(), 1, "Wrong number of variables.");
        let variable = &my_struct.variables[0];
        assert_eq!(variable.name, "variable", "Variable had wrong name.");
        assert_eq!(variable.my_type, NLType::I32, "Variable had wrong type.");
    }

    #[test]
    /// Compile a single struct with two variables. We don't put the trailing comma after the last one.
    fn two_variable_struct_no_ending_comma() {
        let file_name = "tests/struct_with_two_variables_no_ending_comma.nl";
        let file = parse_file(&mut Path::new(file_name)).unwrap();

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
    }

    #[test]
    /// Compile a file with an empty struct and an empty trait. This one is special because it has single line comments in it.
    fn empty_struct_and_trait_single_line_comments() {
        let file_name = "tests/empty_struct_and_trait_with_single_line_comments.nl";
        let file = parse_file(&mut Path::new(file_name)).unwrap();

        assert_eq!(file.name, "empty_struct_and_trait_with_single_line_comments.nl", "File name not copied correctly.");

        assert_eq!(file.traits.len(), 1, "Wrong number of traits.");
        let my_trait = &file.traits[0];
        assert_eq!(my_trait.name, "MyTrait", "Wrong name for trait.");

        assert_eq!(file.structs.len(), 1, "Wrong number of structs.");
        let my_struct = &file.structs[0];
        assert_eq!(my_struct.name, "MyStruct", "Wrong name for struct.");
    }

    #[test]
    /// Compile a file with an empty struct and an empty trait. This one is special because it has multi line comments in it.
    fn empty_struct_and_trait_multi_line_comments() {
        let file_name = "tests/empty_struct_and_trait_with_multi_line_comments.nl";
        let file = parse_file(&mut Path::new(file_name)).unwrap();

        assert_eq!(file.name, "empty_struct_and_trait_with_multi_line_comments.nl", "File name not copied correctly.");

        assert_eq!(file.traits.len(), 1, "Wrong number of traits.");
        let my_trait = &file.traits[0];
        assert_eq!(my_trait.name, "MyTrait", "Wrong name for trait.");

        assert_eq!(file.structs.len(), 1, "Wrong number of structs.");
        let my_struct = &file.structs[0];
        assert_eq!(my_struct.name, "MyStruct", "Wrong name for struct.");
    }

    #[test]
    /// Compile a file with struct with a variable with an invalid type.
    fn struct_with_bad_variable_type() {
        let file_name = "tests/struct_with_single_variable_bad_type.nl";
        let file = parse_file(&mut Path::new(file_name));

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
    fn struct_variable_access_rules() {
        let file_name = "tests/struct_variable_access_rules.nl";
        let file = parse_file(&mut Path::new(file_name)).unwrap();

        assert_eq!(file.structs.len(), 1, "Wrong number of structs.");
        let my_struct = &file.structs[0];

        assert_eq!(my_struct.variables.len(), 4);

        fn check_access(var: &NLStructVariable, rule: NLAccessRule) {
            assert_eq!(var.access, rule);
        }

        check_access(&my_struct.variables[0], NLAccessRule::Hidden);
        check_access(&my_struct.variables[1], NLAccessRule::Hidden);
        check_access(&my_struct.variables[2], NLAccessRule::Immutable);
        check_access(&my_struct.variables[3], NLAccessRule::Mutable);
    }

    #[test]
    /// Compile a file with an empty struct and an empty trait. This one is special because it has multi line comments in it.
    fn struct_empty_self_implementation() {
        let file_name = "tests/struct_with_empty_self_implementation.nl";
        let file = parse_file(&mut Path::new(file_name)).unwrap();

        assert_eq!(file.structs.len(), 1, "Wrong number of structs.");
        let my_struct = &file.structs[0];

        assert_eq!(my_struct.implementations.len(), 1, "Wrong number of implementations.");
        let implementation = &my_struct.implementations[0];

        assert_eq!(implementation.name, "Self", "Implementation had wrong name.");
    }
}

mod nl_trait {
    use super::*;

    #[test]
    /// Compile a file with a single empty trait. We should get no errors or warnings.
    fn single_empty_trait() {
        let file_name = "tests/single_trait_empty.nl";
        let file = parse_file(&mut Path::new(file_name)).unwrap();

        assert_eq!(file.name, "single_trait_empty.nl", "File name not copied correctly.");

        assert_eq!(file.traits.len(), 1, "Wrong number of traits.");
        let my_trait = &file.traits[0];
        assert_eq!(my_trait.name, "MyTrait", "Wrong name for trait.");

        assert_eq!(file.structs.len(), 0, "Wrong number of structs.");
    }
}

mod nl_methods {
    use super::*;

    fn pretty_read_method(input: &str) -> (&str, NLMethod) {
        let result = read_method(input);
        match result {
            Ok(r) => r,
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
