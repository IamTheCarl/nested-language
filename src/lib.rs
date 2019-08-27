
use std::fmt::Formatter;

use nom::sequence::delimited;
use nom::bytes::complete::take_while;
use nom::IResult;
use nom::bytes::complete::take_while1;
use nom::combinator::opt;
use nom::bytes::complete::tag;
use nom::character::complete::alphanumeric0;
use nom::error::VerboseError;
use nom::error::convert_error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

// All tests are kept in their own module.
#[cfg(test)]
mod test;

/*
enum Type {
    bool,
    i8, i16, i32, i64,
    u8, u16, u32, u64,
    OwnedStruct (&Struct),
    BorrowedStruct (&Struct),
    Tuple(count: u32, Vec<Type>),
}

Variable {
    type: Type,
    lifetime_start_line: u32,
    lifetine_end_line: u32,
}

Trait {
    traits: Vec<&Trait>,
    public_functions: Vec<Function>,
    private_functions: Vec<Function>,
}

Struct {
    traits: Vec<&Trait>,
    variables: Vec<&Type>,
    public_functions,
    private_functions,
}

Instruction {
    condition: u8,
    op: u8,
    line: u32,
}

Function {
    arguments: Vec<Type>,
    return_type: Type,
    doc_comment: String,
    instructions: Vec<Instruction>,
}
*/

pub type ParserResult<'a, O> = IResult<&'a str, O, VerboseError<&'a str>>;

const FILE_NAMING_CONVENTION_WARNING_MESSAGE: &str = "File name fails to meet conventions. Should be snake case and end in \".nl\".";

pub struct CompileMessage {
    line: u32,
    column: u32,
    message: String,
}

impl CompileMessage {
    pub fn get_location(&self) -> (u32, u32) {
        (self.line, self.column)
    }

    pub fn get_message(&self) -> &str { &self.message }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum NLType {
    Boolean,
    I8, I16, I32, I64,
    U8, U16, U32, U64,
    OwnedStruct(String),
    BorrowedStruct(String),
    OwnedTrait(String),
    BorrowedTrait(String),
}

pub struct NLVariable {
    name: String,
    my_type: NLType,
}

impl NLVariable {
    pub fn get_name(&self) -> &str { &self.name }
    pub fn get_type(&self) -> &NLType { &self.my_type }
}

pub struct NLStruct {
    name: String,
    variables: Vec<NLVariable>,
}

impl NLStruct {
    pub fn get_name(&self) -> &str { &self.name }
    pub fn get_variables(&self) -> &Vec<NLVariable> { &self.variables }
}

pub struct NLTrait {
    name: String,
}

impl NLTrait {
    pub fn get_name(&self) -> &str { &self.name }
}

enum CoreDeceleration {
    Struct(NLStruct),
    Trait(NLTrait),
    Unknown,
}

pub struct NLFile {
    name: String,
    structs: Vec<NLStruct>,
    traits: Vec<NLTrait>,
    warnings: Vec<CompileMessage>,
}

impl NLFile {
    pub fn get_name(&self) -> &str { &self.name }
    pub fn get_structs(&self) -> &Vec<NLStruct> { &self.structs }
    pub fn get_traits(&self) -> &Vec<NLTrait> { &self.traits }
    pub fn get_warnings(&self) -> &Vec<CompileMessage> { &self.warnings }
}

#[derive(Debug)]
pub struct ParseError {
    message: String,
}

impl std::error::Error for ParseError {
    fn description(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.message)
    }
}

fn is_white_space(c: char) -> bool {
    c == ' ' || c == '\t' || c == '\r' || c == '\n'
}

fn is_name(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')
}

fn read_struct_or_trait_name(data: &str) -> ParserResult<&str> {
    delimited(take_while(is_white_space), take_while1(is_name), take_while(is_white_space))(data)
}

fn read_trait(input: &str) -> ParserResult<CoreDeceleration> {
    let (input, name) = read_struct_or_trait_name(input)?;

    let new_trait = NLTrait {
        name: String::from(name)
    };

    let (input, _) = tag("{")(input)?;
    let (input, _) = tag("}")(input)?;

    /*do_parse!(input,
        char!("{") //>>
        //exprs: many0!(terminated((read_name), char!(';'))) >>
        char!("}") >>
        ()
    );*/

    Ok((input, CoreDeceleration::Trait(new_trait)))
}

fn is_var_name_end(c: char) -> bool {
    is_white_space(c) || c == ':'
}

fn read_variable_name(data: &str) -> ParserResult<&str> {
    delimited(take_while(is_white_space), take_while1(is_name), take_while(is_var_name_end))(data)
}

fn is_type_name_end(c: char) -> bool {
    is_white_space(c) || c == ','
}

fn read_variable_type(input: &str) -> ParserResult<NLType> {
    // let (input, type_name) = take!(input, 5 )?;
    let (input, type_name) =
        delimited(take_while(is_white_space), alphanumeric0, take_while(is_type_name_end))(input)?;

    // TODO figure out how to differentiate traits and structs.
    let the_type = match type_name {
        "i8" => NLType::I8,
        "i16" => NLType::I16,
        "i32" => NLType::I32,
        "i64" => NLType::I64,
        "u8" => NLType::U8,
        "u16" => NLType::U16,
        "u32" => NLType::U32,
        "u64" => NLType::U64,
        "bool" => NLType::Boolean,
        _ => {
            panic!("Unknown variable type."); // TODO pass this error.
        }
    };

    Ok((input, the_type))
}

fn read_var_definition(input: &str) -> ParserResult<NLVariable> {

    let (input, name) = read_variable_name(input)?;
    let (input, _) = tag(":")(input)?; // That : between the variable name and its type.
    let (input, nl_type) = read_variable_type(input)?;
    let (input, _) = opt(tag(","))(input)?;

    let var = NLVariable {
        name: String::from(name),
        my_type: nl_type,
    };

    Ok((input, var))
}

fn read_struct(input: &str) -> ParserResult<CoreDeceleration> {
    let (input, name) = read_struct_or_trait_name(input)?;

    let mut new_struct = NLStruct {
        name: String::from(name),
        variables: vec![]
    };

    let vars = &mut new_struct.variables;

    let (input, _) = tag("{")(input)?;
    let mut input = input;

    loop {
        let (new_input, var_definition) = opt(read_var_definition)(input)?;
        input = new_input;

        match var_definition {
            Some(new_variable) => {
                vars.push(new_variable);
            },
            None => {
                break;
            }
        }
    }
    let (input, _) = tag("}")(input)?;

    Ok((input, CoreDeceleration::Struct(new_struct)))
}

fn parse_file_internal(input: &str) -> ParserResult<NLFile> {
    let mut file = NLFile {
        name: String::new(),
        structs: vec![],
        traits: vec![],
        warnings: vec![],
    };

    let mut input = input;

    loop {
        let (new_input, core_decoder_tag) = opt(read_struct_or_trait_name)(input)?;
        input = new_input;

        match core_decoder_tag {
            Some(core_decoder_tag) => {
                let (new_input, core_def) = match core_decoder_tag {
                    "struct" => {
                        read_struct(input)?
                    }
                    "trait" => {
                        read_trait(input)?
                    },
                    _ => {
                        (input, CoreDeceleration::Unknown)
                    }
                };
                input = new_input;

                match core_def {
                    CoreDeceleration::Struct(nl_struct) => {
                        file.structs.push(nl_struct);
                    },
                    CoreDeceleration::Trait(nl_trait) => {
                        file.traits.push(nl_trait);
                    },
                    CoreDeceleration::Unknown => {
                        // TODO make this error correctly.
                        // Err(nom::error::VerboseError)
                        panic!("Invalid core def.");
                    }
                }
            },
            None => {
                break;
            }
        }
    }

    Ok((input, file))
}

pub fn parse_string(input: &str, file_name: &str) -> Result<NLFile, ParseError> {

    let file = parse_file_internal(input);

    match file {
        Result::Err(err) => {
            match err {
                nom::Err::Error(e) | nom::Err::Failure(e) => {
                    let message = convert_error(input, e);

                    // Makes our error messages more readable when running tests.
                    #[cfg(test)]
                    println!("{}", message);

                    Err(ParseError {
                        message
                    })
                }
                nom::Err::Incomplete(_) => {
                    Err(ParseError {
                        message: "Unexpected end of file.".to_string()
                    })
                }
            }
        },
        Result::Ok(result) => {
            let (_, mut file) = result;

            if !file_name.ends_with(".nl") || voca_rs::case::snake_case(file_name) != file_name {
                file.warnings.push(CompileMessage {
                    line: 0,
                    column: 0,
                    message: String::from(FILE_NAMING_CONVENTION_WARNING_MESSAGE),
                })
            }

            file.name = file_name.to_string();

            Ok(file)
        }
    }
}

pub fn parse_file(path: &Path) -> Result<NLFile, Box<dyn std::error::Error>> {
    let mut input_file = File::open(&path)?;

    let mut contents = String::new();
    input_file.read_to_string(&mut contents)?;

    // This should *always* have a name since we shouldn't have been able to get to this point if it wasn't actually a file.
    let result = parse_string(&contents, &path.file_name().unwrap().to_str().unwrap());

    match result {
        Ok(result) => {
            Ok(result)
        },
        Err(error) => {
            Err(Box::new(error))
        }
    }
}