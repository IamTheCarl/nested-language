
use std::fmt::Formatter;

use nom::Err as NomErr;
use nom::sequence::delimited;
use nom::IResult;
use nom::bytes::complete::take_while1;
use nom::bytes::complete::tag;
use nom::character::complete::alphanumeric0;
use nom::error::VerboseError;
use nom::error::convert_error;
use nom::combinator::recognize;
use nom::character::complete::multispace0;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use nom::branch::alt;
use nom::sequence::preceded;
use nom::sequence::terminated;
use nom::bytes::complete::take_until;
use nom::multi::many0_count;
use nom::combinator::value;
use nom::character::complete::char;
use nom::error::VerboseErrorKind;
use nom::multi::many0;
use nom::multi::many1;
use nom::sequence::tuple;
use nom::combinator::opt;
use nom::character::complete::alphanumeric1;
use nom::bytes::complete::take_while;
use nom::character::is_alphanumeric;

// All tests are kept in their own module.
#[cfg(test)]
mod test;

pub type ParserResult<'a, O> = IResult<&'a str, O, VerboseError<&'a str>>;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum NLAccessRule {
    Internal,
    External,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum NLType {
    None,
    Boolean,
    I8, I16, I32, I64,
    U8, U16, U32, U64,
    Tuple(Vec<NLType>),
    OwnedStruct(String),
    BorrowedStruct(String),
    OwnedTrait(String),
    BorrowedTrait(String),
}

pub struct NLStructVariable {
    name: String,
    my_type: NLType,
}

impl NLStructVariable {
    pub fn get_name(&self) -> &str { &self.name }
    pub fn get_type(&self) -> &NLType { &self.my_type }
}

pub struct NLArgument {
    name: String,
    nl_type: NLType,
}

impl NLArgument {
    pub fn get_name(&self) -> &str { &self.name }
    pub fn get_type(&self) -> &NLType { &self.nl_type }
}

pub struct NLBlock {

}

pub struct NLMethod {
    name: String,
    arguments: Vec<NLArgument>,
    return_type: NLType,
    block: Option<NLBlock>,
}

impl NLMethod {
    pub fn get_name(&self) -> &str { &self.name }
    pub fn get_arguments(&self) -> &Vec<NLArgument> { &self.arguments }
    pub fn get_return_type(&self) -> &NLType { &self.return_type }
    pub fn get_block(&self) -> &Option<NLBlock> { &self.block }
}

pub struct NLStruct {
    name: String,
    variables: Vec<NLStructVariable>,
    implementations: Vec<NLImplementation>,
}

impl NLStruct {
    pub fn get_name(&self) -> &str { &self.name }
    pub fn get_variables(&self) -> &Vec<NLStructVariable> { &self.variables }
    pub fn get_implementations(&self) -> &Vec<NLImplementation> { &self.implementations }
}

pub struct NLTrait {
    name: String,
}

impl NLTrait {
    pub fn get_name(&self) -> &str { &self.name }
}

pub struct NLImplementation {
    name: String,
    methods: Vec<NLMethod>,
}

impl NLImplementation {
    pub fn get_name(&self) -> &str { &self.name }
    pub fn get_methods(&self) -> &Vec<NLMethod> { &self.methods }
}

enum CoreDeceleration {
    Struct(NLStruct),
    Trait(NLTrait),
}

pub struct NLFile {
    name: String,
    structs: Vec<NLStruct>,
    traits: Vec<NLTrait>,
}

impl NLFile {
    pub fn get_name(&self) -> &str { &self.name }
    pub fn get_structs(&self) -> &Vec<NLStruct> { &self.structs }
    pub fn get_traits(&self) -> &Vec<NLTrait> { &self.traits }
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

fn read_comment(input: &str) -> ParserResult<&str> {
    alt((
        preceded(tag("//"), terminated(take_until("\n"), tag("\n"))),
        preceded(tag("/*"), terminated(take_until("*/"), tag("*/"))),
    ))(input)
}

fn read_comments(input: &str) -> ParserResult<&str> {
    recognize(
        many0_count(terminated(read_comment, multispace0))
    )(input)
}

fn blank(input: &str) -> ParserResult<()> {
    value((), preceded(multispace0, read_comments))(input)
}

fn is_name(c: char) -> bool {
    match c {
        '_' => true,
        _ => (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')
    }
}

fn read_struct_or_trait_name(input: &str) -> ParserResult<&str> {
    delimited(blank, alphanumeric1, blank)(input)
}

fn is_method_char(input: char) -> bool {
    match input {
        '_' => true,
        _ => is_alphanumeric(input as u8)
    }
}

fn read_method_name(input: &str) -> ParserResult<&str> {
    delimited(blank, take_while1(is_method_char), blank)(input)
}

fn read_code_block(input: &str) -> ParserResult<NLBlock> {
    // Filler function.

    let (input, _) = char('{')(input)?;
    let (input, _) = blank(input)?;
    let (input, _) = char('}')(input)?;

    Ok((input, NLBlock{}))
}

fn read_method_argument(input: &str) -> ParserResult<NLArgument> {
    let (input, _) = blank(input)?;
    let (input, name) = read_variable_name(input)?;
    let (input, _) = blank(input)?;
    let (input, _) = char(':')(input)?;
    let (input, _) = blank(input)?;
    let (input, nl_type) = read_variable_type(input)?;
    let (input, _) = blank(input)?;

    let arg = NLArgument {
        name: String::from(name),
        nl_type
    };

    Ok((input, arg))
}

fn read_method_argument_list(input: &str) -> ParserResult<Vec<NLArgument>> {
    let (input, arg_input) = delimited(char('('), take_while(|c| c != ')'), char(')'))(input)?;
    let (arg_input, mut arguments) = many0(terminated(read_method_argument, char(',')))(arg_input)?;
    let (_, last_arg) = opt(terminated(read_method_argument, tuple((blank, char(')')))))(arg_input)?;
    match last_arg {
        Some(arg) => {
            arguments.push(arg);
        },
        _ => {} // Do nothing if there was no argument.
    }

    Ok((input, arguments))
}

fn read_return_type(input: &str) -> ParserResult<NLType> {
    let (input, _) = blank(input)?;
    let (input, tagged) = opt(tag("->"))(input)?;

    if tagged.is_some() {
        let (input, _) = blank(input)?;
        let (input, nl_type) = read_variable_type(input)?;
        let (input, _) = blank(input)?;

        Ok((input, nl_type))
    } else {
        Ok((input, NLType::None))
    }
}

fn read_method(input: &str) -> ParserResult<NLMethod> {
    let (input, _) = blank(input)?;
    let (input, _) = tag("met")(input)?;
    let (input, _) = blank(input)?;
    let (input, name) = read_method_name(input)?;
    let (input, _) = blank(input)?;
    let (input, args) = read_method_argument_list(input)?;
    let (input, _) = blank(input)?;
    let (input, return_type) = read_return_type(input)?;
    let (input, _) = blank(input)?;
    let (input, block) = opt(read_code_block)(input)?;

    let method = NLMethod {
        name: String::from(name),
        arguments: args,
        return_type,
        block
    };

    // No block, we expect a semicolon.
    if method.block.is_none() {
        let (input, _) = char(';')(input)?;

        Ok((input, method))
    } else {
        Ok((input, method))
    }
}

fn read_trait(input: &str) -> ParserResult<CoreDeceleration> {
    let (input, _) = blank(input)?;
    let (input, _) = tag("trait")(input)?;
    let (input, _) = blank(input)?;
    let (input, name) = read_struct_or_trait_name(input)?;

    let new_trait = NLTrait {
        name: String::from(name)
    };

    let (input, _) = char('{')(input)?;
    let (input, _) = char('}')(input)?;

    /*do_parse!(input,
        char!("{") //>>
        //exprs: many0!(terminated((read_name), char!(';'))) >>
        char!("}") >>
        ()
    );*/

    Ok((input, CoreDeceleration::Trait(new_trait)))
}

fn read_variable_name(input: &str) -> ParserResult<&str> {
    take_while1(is_name)(input)
}

fn read_variable_type(input: &str) -> ParserResult<NLType> {
    // let (input, type_name) = take!(input, 5 )?;
    let (input, type_name) = alphanumeric0(input)?;

    // TODO figure out how to differentiate traits and structs.
    match type_name {
        "i8"   => Ok((input, NLType::I8)),
        "i16"  => Ok((input, NLType::I16)),
        "i32"  => Ok((input, NLType::I32)),
        "i64"  => Ok((input, NLType::I64)),
        "u8"   => Ok((input, NLType::U8)),
        "u16"  => Ok((input, NLType::U16)),
        "u32"  => Ok((input, NLType::U32)),
        "u64"  => Ok((input, NLType::U64)),
        "bool" => Ok((input, NLType::Boolean)),
        _ => {

            let vek = VerboseErrorKind::Context("unknown variable type");

            let ve = VerboseError {
                errors: vec![(input, vek)]
            };

            Err(NomErr::Failure(ve))
        }
    }
}

fn read_struct_variable(input: &str) -> ParserResult<NLStructVariable> {

    let (input, _) = blank(input)?;
    let (input, name) = read_variable_name(input)?;

    let (input, _) = blank(input)?;
    let (input, _) = char(':')(input)?; // That : between the variable name and its type.
    let (input, _) = blank(input)?;
    let (input, nl_type) = read_variable_type(input)?;

    let var = NLStructVariable {
        name: String::from(name),
        my_type: nl_type,
    };

    Ok((input, var))
}

fn read_implementation(input: &str) -> ParserResult<NLImplementation> {
    let (input, _) = blank(input)?;
    let (input, _) = tag("impl")(input)?;
    let (input, name) = read_struct_or_trait_name(input)?;
    let (input, _) = char('{')(input)?;
    let (input, _) = blank(input)?;
    let (input, methods) = many0(read_method)(input)?;
    let (input, _) = blank(input)?;
    let (input, _) = char('}')(input)?;

    let implementation = NLImplementation {
        name: String::from(name),
        methods,
    };

    Ok((input, implementation))
}

fn read_struct(input: &str) -> ParserResult<CoreDeceleration> {
    let (input, _) = blank(input)?;
    let (input, _) = tag("struct")(input)?;
    let (input, _) = blank(input)?;
    let (input, name) = read_struct_or_trait_name(input)?;
    let (input, _) = blank(input)?;
    let (input, _) = char('{')(input)?;
    let (input, _) = blank(input)?;
    let (input, mut variables) = many0(
        terminated(read_struct_variable, tuple((blank, char(','))))
    )(input)?;
    let (input, _) = blank(input)?;

    // Need to read the last struct.
    let (input, last_var) = opt(read_struct_variable)(input)?;
    match last_var {
        Some(var) => {
            variables.push(var);
        }
        _ => {} // Do nothing if we didn't have a last one.
    }

    let (input, _) = blank(input)?;
    let (input, _) = char('}')(input)?;
    let (input, implementations) = many0(read_implementation)(input)?;

    let nl_struct = NLStruct {
        name: String::from(name),
        variables,
        implementations
    };

    Ok((input, CoreDeceleration::Struct(nl_struct)))
}

fn parse_file_internal(input: &str) -> ParserResult<NLFile> {
    let mut file = NLFile {
        name: String::new(),
        structs: vec![],
        traits: vec![],
    };

    if !input.is_empty() {
        let (input, core_defs) = many1(alt((read_struct, read_trait)))(input)?;

        for core_def in core_defs {
            match core_def {
                CoreDeceleration::Struct(nl_struct) => {
                    file.structs.push(nl_struct);
                },
                CoreDeceleration::Trait(nl_trait) => {
                    file.traits.push(nl_trait);
                }
            }
        }

        Ok((input, file))
    } else {
        Ok((input, file))
    }
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