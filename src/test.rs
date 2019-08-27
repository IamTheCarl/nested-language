
use super::*;

#[test]
/// Compile an empty file. We should get no errors or warnings. Name should match.
fn empty_file() {
    let file = "";
    let file_name = "test_file.nl";

    let file = parse_string(file, file_name).unwrap();

    assert_eq!(file.name, file_name, "File name not copied correctly.");
    assert_eq!(file.warnings.len(), 0, "Unwarranted warning.");
}

#[test]
/// Compile an empty file. We should get no errors or warnings. Name should match.
fn file_name_with_humpback() {
    let file = "";
    let file_name = "testFile.nl";

    let file = parse_string(file, file_name).unwrap();

    assert_eq!(file.name, file_name, "File name not copied correctly.");
    assert_eq!(file.warnings.len(), 1, "Wrong number of warnings.");

    let warning = &file.warnings[0];

    assert_eq!(warning.line, 0);
    assert_eq!(warning.column, 0);
    assert_eq!(warning.message, FILE_NAMING_CONVENTION_WARNING_MESSAGE);
}

#[test]
/// Compile an empty file. We should get an error since we have failed to meet file naming conventions.
fn file_name_with_missing_extension() {
    let file = "";
    let file_name = "test_file";

    let file = parse_string(file, file_name).unwrap();

    assert_eq!(file.name, file_name, "File name not copied correctly.");
    assert_eq!(file.warnings.len(), 1, "Wrong number of warnings.");

    let warning = &file.warnings[0];

    assert_eq!(warning.line, 0);
    assert_eq!(warning.column, 0);
    assert_eq!(warning.message, FILE_NAMING_CONVENTION_WARNING_MESSAGE);
}

#[test]
/// Compile an empty file. We should get an error since we have failed to meet file naming conventions.
fn file_name_with_wrong_extension() {
    let file = "";
    let file_name = "test_file.txt";

    let file = parse_string(file, file_name).unwrap();

    assert_eq!(file.name, file_name, "File name not copied correctly.");
    assert_eq!(file.warnings.len(), 1, "Wrong number of warnings.");

    let warning = &file.warnings[0];

    assert_eq!(warning.line, 0);
    assert_eq!(warning.column, 0);
    assert_eq!(warning.message, FILE_NAMING_CONVENTION_WARNING_MESSAGE);
}

#[test]
/// Compile a file with a single empty struct. We should get no errors or warnings.
fn single_empty_struct() {
    let file_name = "tests/single_struct_empty.nl";
    let file = parse_file(&mut Path::new(file_name)).unwrap();

    assert_eq!(file.name, "single_struct_empty.nl", "File name not copied correctly.");
    assert_eq!(file.warnings.len(), 0, "Unwarranted warning.");

    assert_eq!(file.structs.len(), 1, "Wrong number of structs.");
    let my_struct = &file.structs[0];
    assert_eq!(my_struct.name, "MyStruct", "Wrong name for struct.");

    assert_eq!(file.traits.len(), 0, "Wrong number of traits.");
}

#[test]
/// Compile a file with a single empty trait. We should get no errors or warnings.
fn single_empty_trait() {
    let file_name = "tests/single_trait_empty.nl";
    let file = parse_file(&mut Path::new(file_name)).unwrap();

    assert_eq!(file.name, "single_trait_empty.nl", "File name not copied correctly.");
    assert_eq!(file.warnings.len(), 0, "Unwarranted warning.");

    assert_eq!(file.traits.len(), 1, "Wrong number of traits.");
    let my_trait = &file.traits[0];
    assert_eq!(my_trait.name, "MyTrait", "Wrong name for trait.");

    assert_eq!(file.structs.len(), 0, "Wrong number of structs.");
}

#[test]
/// Compile a file with an empty struct and an empty trait. We should get no errors or warnings.
fn empty_struct_and_trait() {
    let file_name = "tests/empty_struct_and_trait.nl";
    let file = parse_file(&mut Path::new(file_name)).unwrap();

    assert_eq!(file.name, "empty_struct_and_trait.nl", "File name not copied correctly.");
    assert_eq!(file.warnings.len(), 0, "Unwarranted warning.");

    assert_eq!(file.traits.len(), 1, "Wrong number of traits.");
    let my_trait = &file.traits[0];
    assert_eq!(my_trait.name, "MyTrait", "Wrong name for trait.");

    assert_eq!(file.structs.len(), 1, "Wrong number of structs.");
    let my_struct = &file.structs[0];
    assert_eq!(my_struct.name, "MyStruct", "Wrong name for struct.");
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
    assert_eq!(file.warnings.len(), 0, "Unwarranted warning.");

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
    assert_eq!(file.warnings.len(), 0, "Unwarranted warning.");

    assert_eq!(file.traits.len(), 1, "Wrong number of traits.");
    let my_trait = &file.traits[0];
    assert_eq!(my_trait.name, "MyTrait", "Wrong name for trait.");

    assert_eq!(file.structs.len(), 1, "Wrong number of structs.");
    let my_struct = &file.structs[0];
    assert_eq!(my_struct.name, "MyStruct", "Wrong name for struct.");
}