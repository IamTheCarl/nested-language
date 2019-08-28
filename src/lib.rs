
use std::fmt::Formatter;

use nom::Err as NomErr;
use nom::sequence::delimited;
use nom::IResult;
use nom::bytes::complete::take_while1;
use nom::combinator::opt;
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
use nom::character::complete::alpha1;
use nom::error::VerboseErrorKind;

// All tests are kept in their own module.
#[cfg(test)]
mod test;

pub type ParserResult<'a, O> = IResult<&'a str, O, VerboseError<&'a str>>;

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
pub enum NLAccessRule {
    Hidden,
    Immutable,
    Mutable,
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

pub struct NLStructVariable {
    name: String,
    my_type: NLType,
    access: NLAccessRule,
}

impl NLStructVariable {
    pub fn get_name(&self) -> &str { &self.name }
    pub fn get_type(&self) -> &NLType { &self.my_type }
    pub fn get_access_rule(&self) -> &NLAccessRule { &self.access }
}

pub struct NLStruct {
    name: String,
    variables: Vec<NLStructVariable>,
}

impl NLStruct {
    pub fn get_name(&self) -> &str { &self.name }
    pub fn get_variables(&self) -> &Vec<NLStructVariable> { &self.variables }
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

fn read_struct_or_trait_name(data: &str) -> ParserResult<&str> {
    delimited(blank, take_while1(is_name), blank)(data)
}

fn read_trait(input: &str) -> ParserResult<CoreDeceleration> {
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

fn read_visibility(input: &str) -> ParserResult<NLAccessRule> {
    let (input, _) = blank(input)?;
    let (input, tag) = alpha1(input)?;
    let (input, _) = blank(input)?;

    match tag {

        "hid" => Ok((input, NLAccessRule::Hidden)),
        "imm" => Ok((input, NLAccessRule::Immutable)),
        "mut" => Ok((input, NLAccessRule::Mutable)),

        _ =>     Ok((input, NLAccessRule::Hidden)), // Hidden by default.
    }
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

fn read_var_definition(input: &str) -> ParserResult<NLStructVariable> {

    let (input, _) = blank(input)?;
    let (input, vision) = read_visibility(input)?;
    let (input, name) = read_variable_name(input)?;
    let (input, _) = blank(input)?;
    let (input, _) = char(':')(input)?; // That : between the variable name and its type.
    let (input, _) = blank(input)?;
    let (input, nl_type) = read_variable_type(input)?;

    let var = NLStructVariable {
        name: String::from(name),
        my_type: nl_type,
        access: vision,
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

    let (input, _) = char('{')(input)?;
    let mut input = input;

    loop {
        let (new_input, var_definition) = opt(read_var_definition)(input)?;
        input = new_input;

        match var_definition {
            Some(new_variable) => {
                vars.push(new_variable);
                let (new_input, comma) = opt(char(','))(input)?;
                input = new_input;
                match comma {
                    // We have a comma, so it is optional that we have another deceleration.
                    Some(_) => {
                        // Just go back to the top of the loop.
                        continue;
                    },
                    None => {
                        // No comma, we must end the loop.
                        break;
                    }
                }
            },
            None => {
                break;
            }
        }
    }

    let (input, _) = blank(input)?;
    let (input, _) = char('}')(input)?;

    Ok((input, CoreDeceleration::Struct(new_struct)))
}

fn parse_file_internal(input: &str) -> ParserResult<NLFile> {
    let mut file = NLFile {
        name: String::new(),
        structs: vec![],
        traits: vec![],
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
                        let vek = VerboseErrorKind::Context("root of file. Only traits and structs may be defined here.");

                        let ve = VerboseError {
                            errors: vec![(input, vek)]
                        };

                        return Err(NomErr::Failure(ve));
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