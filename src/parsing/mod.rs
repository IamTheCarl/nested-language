use nom::Err as NomErr;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while, take_while1},
    character::{
        complete::{alpha1, alphanumeric0, alphanumeric1, char, digit1, multispace0, one_of},
        is_alphanumeric,
    },
    combinator::{opt, recognize, value},
    error::{convert_error, FromExternalError, VerboseError, VerboseErrorKind},
    multi::{many0, many0_count, many1},
    sequence::tuple,
    sequence::{delimited, preceded, terminated},
    IResult,
};
use std::{fmt::Formatter, fs::File, io::Read, path::Path, str::FromStr};

// All tests are kept in their own module.
#[cfg(test)]
mod tests;

pub type ParserResult<'a, O> = IResult<&'a str, O, VerboseError<&'a str>>;

// TODO replace all the getters with reference handles and mut_handles.

#[derive(PartialOrd, PartialEq, Debug, Clone)]
pub enum NLType<'a> {
    None,
    Boolean,
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    OwnedString,
    BorrowedString,
    Tuple(Vec<NLType<'a>>),
    OwnedStruct(&'a str),
    ReferencedStruct(&'a str),
    MutableReferencedStruct(&'a str),
    OwnedTrait(&'a str),
    ReferencedTrait(&'a str),
    MutableReferencedTrait(&'a str),
    Enum(&'a str),
    SelfReference,
    MutableSelfReference,
}

impl<'a> NLType<'a> {
    pub fn num_bits(&self) -> u16 {
        match self {
            NLType::Boolean => 1,
            NLType::I8 => 8,
            NLType::I16 => 16,
            NLType::I32 => 32,
            NLType::I64 => 64,
            NLType::U8 => 8,
            NLType::U16 => 16,
            NLType::U32 => 32,
            NLType::U64 => 64,
            _ => 0,
        }
    }

    pub fn is_boolean(&self) -> bool {
        match self {
            NLType::Boolean => true,
            _ => false,
        }
    }

    pub fn is_integer(&self) -> bool {
        match self {
            NLType::I8 => true,
            NLType::I16 => true,
            NLType::I32 => true,
            NLType::I64 => true,
            NLType::U8 => true,
            NLType::U16 => true,
            NLType::U32 => true,
            NLType::U64 => true,
            _ => false,
        }
    }

    pub fn is_unsigned(&self) -> bool {
        match self {
            NLType::U8 => true,
            NLType::U16 => true,
            NLType::U32 => true,
            NLType::U64 => true,
            _ => false,
        }
    }

    pub fn is_signed(&self) -> bool {
        match self {
            NLType::I8 => true,
            NLType::I16 => true,
            NLType::I32 => true,
            NLType::I64 => true,
            _ => false,
        }
    }

    pub fn is_float(&self) -> bool {
        match self {
            NLType::F32 => true,
            NLType::F64 => true,
            _ => false,
        }
    }
}

pub struct NLStructVariable<'a> {
    name: &'a str,
    my_type: NLType<'a>,
}

impl<'a> NLStructVariable<'a> {
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_type(&self) -> &NLType {
        &self.my_type
    }
}

#[derive(PartialOrd, PartialEq, Debug)]
pub struct NLArgument<'a> {
    name: &'a str,
    nl_type: NLType<'a>,
}

impl<'a> NLArgument<'a> {
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_type(&self) -> &NLType {
        &self.nl_type
    }
}

#[derive(PartialOrd, PartialEq, Debug)]
pub struct NLBlock<'a> {
    operations: Vec<NLOperation<'a>>,
}

impl<'a> NLBlock<'a> {
    pub fn get_operations(&self) -> &Vec<NLOperation<'a>> {
        &self.operations
    }
}

pub struct NLFunction<'a> {
    name: &'a str,
    arguments: Vec<NLArgument<'a>>,
    return_type: NLType<'a>,
    block: Option<NLBlock<'a>>,
}

pub enum NLImplementor<'a> {
    Method(NLFunction<'a>),
    Getter(NLGetter<'a>),
    Setter(NLSetter<'a>),
}

impl<'a> NLFunction<'a> {
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_arguments(&self) -> &Vec<NLArgument> {
        &self.arguments
    }
    pub fn get_return_type(&self) -> &NLType {
        &self.return_type
    }
    pub fn get_block(&self) -> &Option<NLBlock> {
        &self.block
    }
}

#[derive(PartialOrd, PartialEq, Debug)]
pub enum NLEncapsulationBlock<'a> {
    Some(NLBlock<'a>),
    None,
    Default,
}

pub struct NLGetter<'a> {
    name: String,
    args: Vec<NLArgument<'a>>,
    nl_type: NLType<'a>,
    block: NLEncapsulationBlock<'a>,
}

impl<'a> NLGetter<'a> {
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_arguments(&self) -> &Vec<NLArgument> {
        &self.args
    }
    pub fn get_type(&self) -> &NLType {
        &self.nl_type
    }
    pub fn get_block(&self) -> &NLEncapsulationBlock {
        &self.block
    }
}

pub struct NLSetter<'a> {
    name: &'a str,
    args: Vec<NLArgument<'a>>,
    block: NLEncapsulationBlock<'a>,
}

impl<'a> NLSetter<'a> {
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_arguments(&self) -> &Vec<NLArgument> {
        &self.args
    }
    pub fn get_block(&self) -> &NLEncapsulationBlock {
        &self.block
    }
}

pub struct NLStruct<'a> {
    name: &'a str,
    variables: Vec<NLStructVariable<'a>>,
    implementations: Vec<NLImplementation<'a>>,
}

impl<'a> NLStruct<'a> {
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_variables(&self) -> &Vec<NLStructVariable> {
        &self.variables
    }
    pub fn get_implementations(&self) -> &Vec<NLImplementation> {
        &self.implementations
    }
}

pub struct NLTrait<'a> {
    name: &'a str,
    implementors: Vec<NLImplementor<'a>>,
}

impl<'a> NLTrait<'a> {
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_implementors(&self) -> &Vec<NLImplementor> {
        &self.implementors
    }
}

pub struct NLImplementation<'a> {
    name: &'a str,
    implementors: Vec<NLImplementor<'a>>,
}

impl<'a> NLImplementation<'a> {
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_implementors(&self) -> &Vec<NLImplementor> {
        &self.implementors
    }
}

#[derive(PartialOrd, PartialEq, Debug)]
pub struct EnumVariant<'a> {
    name: &'a str,
    arguments: Vec<NLArgument<'a>>,
}

impl<'a> EnumVariant<'a> {
    pub fn get_name(&self) -> &str {
        self.name
    }

    pub fn get_arguments(&self) -> &Vec<NLArgument<'a>> {
        &self.arguments
    }
}

pub struct NLEnum<'a> {
    name: &'a str,
    variants: Vec<EnumVariant<'a>>,
}

impl<'a> NLEnum<'a> {
    pub fn get_name(&self) -> &str {
        self.name
    }

    pub fn get_variants(&self) -> &Vec<EnumVariant> {
        &self.variants
    }
}

enum RootDeceleration<'a> {
    Struct(NLStruct<'a>),
    Trait(NLTrait<'a>),
    Function(NLFunction<'a>),
    Enum(NLEnum<'a>),
}

#[derive(PartialOrd, PartialEq, Debug)]
pub enum OpConstant<'a> {
    Boolean(bool),
    Unsigned(u64, NLType<'a>),
    Signed(i64, NLType<'a>),
    Float32(f32),
    Float64(f64),
    String(&'a str),
    // TODO add support for defining a constant enum.
}

#[derive(PartialOrd, PartialEq, Debug)]
pub struct OpVariable<'a> {
    name: &'a str,
}

impl<'a> OpVariable<'a> {
    pub fn get_name(&self) -> &str {
        self.name
    }
}

#[derive(PartialOrd, PartialEq, Debug)]
pub struct OpAssignment<'a> {
    is_new: bool,
    to_assign: Vec<OpVariable<'a>>,
    type_assignments: Vec<NLType<'a>>,
    assignment: Box<NLOperation<'a>>,
}

impl<'a> OpAssignment<'a> {
    pub fn is_new(&self) -> bool {
        self.is_new
    }
    pub fn get_variable_to_assign(&self) -> &Vec<OpVariable> {
        &self.to_assign
    }
    pub fn get_types(&self) -> &Vec<NLType> {
        &self.type_assignments
    }
    pub fn get_value(&self) -> &Box<NLOperation> {
        &self.assignment
    }
}

#[derive(PartialOrd, PartialEq, Debug)]
pub enum OpOperator<'a> {
    CompareEqual((Box<NLOperation<'a>>, Box<NLOperation<'a>>)),
    CompareNotEqual((Box<NLOperation<'a>>, Box<NLOperation<'a>>)),
    CompareGreater((Box<NLOperation<'a>>, Box<NLOperation<'a>>)),
    CompareLess((Box<NLOperation<'a>>, Box<NLOperation<'a>>)),
    CompareGreaterEqual((Box<NLOperation<'a>>, Box<NLOperation<'a>>)),
    CompareLessEqual((Box<NLOperation<'a>>, Box<NLOperation<'a>>)),

    LogicalNegate(Box<NLOperation<'a>>),

    LogicalAnd((Box<NLOperation<'a>>, Box<NLOperation<'a>>)),
    LogicalOr((Box<NLOperation<'a>>, Box<NLOperation<'a>>)),
    LogicalXor((Box<NLOperation<'a>>, Box<NLOperation<'a>>)),

    BitAnd((Box<NLOperation<'a>>, Box<NLOperation<'a>>)),
    BitOr((Box<NLOperation<'a>>, Box<NLOperation<'a>>)),
    BitXor((Box<NLOperation<'a>>, Box<NLOperation<'a>>)),

    ArithmeticNegate(Box<NLOperation<'a>>),
    BitNegate(Box<NLOperation<'a>>),

    BitLeftShift((Box<NLOperation<'a>>, Box<NLOperation<'a>>)),
    BitRightShift((Box<NLOperation<'a>>, Box<NLOperation<'a>>)),

    PropError(Box<NLOperation<'a>>), // TODO implement.

    ArithmeticMod((Box<NLOperation<'a>>, Box<NLOperation<'a>>)),
    ArithmeticAdd((Box<NLOperation<'a>>, Box<NLOperation<'a>>)),
    ArithmeticSub((Box<NLOperation<'a>>, Box<NLOperation<'a>>)),
    ArithmeticMul((Box<NLOperation<'a>>, Box<NLOperation<'a>>)),
    ArithmeticDiv((Box<NLOperation<'a>>, Box<NLOperation<'a>>)),

    Range((Box<NLOperation<'a>>, Box<NLOperation<'a>>)),
}

#[derive(PartialOrd, PartialEq, Debug)]
pub struct IfStatement<'a> {
    condition: Box<NLOperation<'a>>,
    true_block: NLBlock<'a>,
    false_block: NLBlock<'a>,
}

#[derive(PartialOrd, PartialEq, Debug)]
pub struct WhileLoop<'a> {
    condition: Box<NLOperation<'a>>,
    block: NLBlock<'a>,
}

#[derive(PartialOrd, PartialEq, Debug)]
pub struct ForLoop<'a> {
    variable: OpVariable<'a>,
    iterator: Box<NLOperation<'a>>,
    block: NLBlock<'a>,
}

#[derive(PartialOrd, PartialEq, Debug)]
struct MatchEnumBranch<'a> {
    nl_enum: &'a str,
    variant: &'a str,
    variables: Vec<&'a str>,
}

#[derive(PartialOrd, PartialEq, Debug)]
enum MatchBranch<'a> {
    Enum(MatchEnumBranch<'a>),
    Constant(OpConstant<'a>),
    Range((i128, i128)),
    AllOther, // TODO implement.
}

#[derive(PartialOrd, PartialEq, Debug)]
pub struct Match<'a> {
    input: Box<NLOperation<'a>>,
    branches: Vec<(MatchBranch<'a>, NLOperation<'a>)>,
}

#[derive(PartialOrd, PartialEq, Debug)]
pub struct FunctionCall<'a> {
    path: &'a str,
    arguments: Vec<&'a str>,
}

#[derive(PartialOrd, PartialEq, Debug)]
pub enum NLOperation<'a> {
    Block(NLBlock<'a>),
    Constant(OpConstant<'a>),
    Assign(OpAssignment<'a>),
    VariableAccess(OpVariable<'a>),
    Tuple(Vec<NLOperation<'a>>),
    Operator(OpOperator<'a>),
    If(IfStatement<'a>),
    Loop(NLBlock<'a>),
    WhileLoop(WhileLoop<'a>),
    ForLoop(ForLoop<'a>),
    Break,
    Match(Match<'a>),
    FunctionCall(FunctionCall<'a>),
}

pub struct NLFile<'a> {
    name: String,
    structs: Vec<NLStruct<'a>>,
    traits: Vec<NLTrait<'a>>,
    functions: Vec<NLFunction<'a>>,
    enums: Vec<NLEnum<'a>>,
}

impl<'a> NLFile<'a> {
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_structs(&self) -> &Vec<NLStruct> {
        &self.structs
    }
    pub fn get_traits(&self) -> &Vec<NLTrait> {
        &self.traits
    }
    pub fn get_functions(&self) -> &Vec<NLFunction> {
        &self.functions
    }
    pub fn get_enums(&self) -> &Vec<NLEnum> {
        &self.enums
    }
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

fn verbose_error<'a>(input: &'a str, message: &'static str) -> NomErr<VerboseError<&'a str>> {
    let vek = VerboseErrorKind::Context(message);

    let ve = VerboseError {
        errors: vec![(input, vek)],
    };

    NomErr::Error(ve)
}

fn read_comment(input: &str) -> ParserResult<&str> {
    alt((
        preceded(tag("//"), terminated(take_until("\n"), tag("\n"))),
        preceded(tag("/*"), terminated(take_until("*/"), tag("*/"))),
    ))(input)
}

fn read_comments(input: &str) -> ParserResult<&str> {
    recognize(many0_count(terminated(read_comment, multispace0)))(input)
}

fn blank(input: &str) -> ParserResult<()> {
    value((), preceded(multispace0, read_comments))(input)
}

fn is_name(c: char) -> bool {
    match c {
        '_' => true,
        '.' => true, // Used for scoped names.
        _ => (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z'),
    }
}

fn read_struct_or_trait_name(input: &str) -> ParserResult<&str> {
    delimited(blank, alphanumeric1, blank)(input)
}

fn is_method_char(input: char) -> bool {
    match input {
        '_' => true,
        _ => is_alphanumeric(input as u8),
    }
}

fn read_method_name(input: &str) -> ParserResult<&str> {
    delimited(blank, take_while1(is_method_char), blank)(input)
}

fn read_tuple_of_variable_names(input: &str) -> ParserResult<Vec<&str>> {
    let (input, tuple_str) = delimited(char('('), take_while(|c| c != ')'), char(')'))(input)?;

    let (tuple_str, mut variables) = many0(terminated(
        read_variable_name,
        tuple((blank, char(','), blank)),
    ))(tuple_str)?;

    let (_, last_var) = opt(terminated(read_variable_name, blank))(tuple_str)?;
    match last_var {
        Some(var) => {
            variables.push(var);
        }
        _ => {} // Do nothing if there was no argument.
    }

    Ok((input, variables))
}

fn read_tuple(input: &str) -> ParserResult<NLOperation> {
    let (input, _) = blank(input)?;
    let (input, tuple_str) = delimited(char('('), take_while(|c| c != ')'), char(')'))(input)?;

    let (tuple_str, mut tuple) =
        many0(terminated(read_operation, tuple((blank, char(','), blank))))(tuple_str)?;

    let (_, last_item) = opt(terminated(read_operation, blank))(tuple_str)?;
    match last_item {
        Some(item) => {
            tuple.push(item);
        }
        _ => {} // Do nothing if there was no argument.
    }

    Ok((input, NLOperation::Tuple(tuple)))
}

fn read_single_variable(input: &str) -> ParserResult<Vec<&str>> {
    let (input, name) = read_variable_name(input)?;
    Ok((input, vec![name]))
}

fn read_boolean_constant(input: &str) -> ParserResult<OpConstant> {
    let (input, value) = alpha1(input)?;
    match value {
        "true" => Ok((input, OpConstant::Boolean(true))),
        "false" => Ok((input, OpConstant::Boolean(false))),
        _ => Err(verbose_error(input, "boolean must be true or false")),
    }
}

// TODO this is to be used for casting variable types, not constant types.
fn read_cast(input: &str) -> ParserResult<NLType> {
    let (input, _) = blank(input)?;
    let (input, _) = tag("as")(input)?;
    let (input, _) = blank(input)?;

    read_variable_type(input)
}

struct ParsedInteger<'a> {
    text: &'a str,
    radix: u32,
}

fn parse_decimal(input: &str) -> ParserResult<ParsedInteger> {
    let (input, text) =
        recognize(many1(terminated(one_of("-0123456789"), many0(char('_')))))(input)?;

    let product = ParsedInteger { text, radix: 10 };
    Ok((input, product))
}

fn parse_hexadecimal(input: &str) -> ParserResult<ParsedInteger> {
    let (input, text) = preceded(
        alt((tag("0x"), tag("0X"))),
        recognize(many1(terminated(
            one_of("0123456789abcdefABCDEF"),
            many0(char('_')),
        ))),
    )(input)?;

    let product = ParsedInteger { text, radix: 16 };
    Ok((input, product))
}

fn parse_octal(input: &str) -> ParserResult<ParsedInteger> {
    let (input, text) = preceded(
        alt((tag("0o"), tag("0O"))),
        recognize(many1(terminated(one_of("01234567"), many0(char('_'))))),
    )(input)?;

    let product = ParsedInteger { text, radix: 8 };
    Ok((input, product))
}

fn parse_binary(input: &str) -> ParserResult<ParsedInteger> {
    let (input, text) = preceded(
        alt((tag("0b"), tag("0B"))),
        recognize(many1(terminated(one_of("01"), many0(char('_'))))),
    )(input)?;

    let product = ParsedInteger { text, radix: 2 };
    Ok((input, product))
}

fn parse_integer(input: &str) -> ParserResult<ParsedInteger> {
    alt((parse_hexadecimal, parse_binary, parse_octal, parse_decimal))(input)
}

fn parse_float(input: &str) -> ParserResult<&str> {
    fn parse_decimal(input: &str) -> ParserResult<&str> {
        recognize(many1(terminated(one_of("0123456789"), many0(char('_')))))(input)
    }

    alt((
        recognize(tuple((
            opt(char('-')),
            char('.'),
            parse_decimal,
            opt(tuple((one_of("eE"), opt(one_of("+-")), parse_decimal))),
        ))),
        recognize(tuple((
            opt(char('-')),
            parse_decimal,
            opt(preceded(char('.'), parse_decimal)),
            one_of("eE"),
            opt(one_of("+-")),
            parse_decimal,
        ))),
        recognize(tuple((opt(char('-')), parse_decimal, char('.'), opt(parse_decimal)))),
    ))(input)
}

fn read_numerical_constant(input: &str) -> ParserResult<OpConstant> {

    // Try to read as a float first.
    let float_attempt = parse_float(input);

    if let Ok((input, number)) = float_attempt {
        // It's a float.

        fn parse_number<T>(input: &str) -> ParserResult<T>
        where
            T: std::str::FromStr,
        {
            let value = input.parse::<T>();
            match value {
                Ok(value) => {
                    // Its a valid integer.
                    Ok((input, value))
                }
                _ => {
                    let vek = VerboseErrorKind::Context("parse constant integer");
                    let ve = VerboseError {
                        errors: vec![(input, vek)],
                    };
                    Err(NomErr::Error(ve))
                }
            }
        }

        // Figure out the type.
        match read_variable_type_primitive_no_whitespace(input) {
            Ok((input, nl_type)) => match nl_type {
                // It must be a floating point type.
                NLType::F32 => {
                    let (_, number) = parse_number::<f32>(number)?;
                    Ok((input, OpConstant::Float32(number)))
                },
                NLType::F64 => {
                    let (_, number) = parse_number::<f64>(number)?;
                    Ok((input, OpConstant::Float64(number)))
                },
                _ => Err(verbose_error(
                    input,
                    "Cannot represent a fractional number as anything other than a floating point type.",
                )),
            },
            Err(_) => {
                // If unspecified, assume 32bit.
                let (_, number) = parse_number::<f32>(number)?;
                Ok((input, OpConstant::Float32(number)))
            }, 
        }
    } else {
        // We attempt to read an integer.
        let (input, integer) = parse_integer(input)?;

        // Figure out the type.
        let (input, nl_type) = match read_variable_type_primitive_no_whitespace(input) {
            Ok((input, nl_type)) => match nl_type {
                // It can't be a boolean type.
                NLType::Boolean => Err(verbose_error(
                    input,
                    "Cannot represent a number as a boolean.",
                )),
                _ => Ok((input, nl_type)), // Okay we're good. Use the type.
            },
            Err(_) => Ok((input, NLType::I32)), // If unspecified, assume 32bit.
        }?;

        if nl_type.is_signed() {
            match i64::from_str_radix(integer.text, integer.radix) {
                Ok(number) => Ok((input, OpConstant::Signed(number, nl_type))),
                Err(_error) => Err(verbose_error(input, "Failed to parse integer.")),
            }
        } else {
            match u64::from_str_radix(integer.text, integer.radix) {
                Ok(number) => Ok((input, OpConstant::Unsigned(number, nl_type))),
                Err(_error) => Err(verbose_error(input, "Failed to parse integer.")),
            }
        }
    }

    // let (input, number) = terminated(take_while1(is_number), blank)(input)?;
    // let (input, nl_type) = match read_variable_type_primitive_no_whitespace(input) {
    //     Ok(result) => result,
    //     Err(_) => {
    //         // Default to a 32bit type if unspecified.
    //         if number.contains(".") {
    //             // It's probably a floating point type.
    //             (input, NLType::F32)
    //         } else {
    //             // It's probably an integer type.
    //             (input, NLType::I32)
    //         }
    //     }
    // };

    // if nl_type.is_integer() {
    //     // FIXME need to support hexdecimal.
    //     if nl_type.is_signed() {
    //         let (_, value) = parse_number::<i64>(number)?;
    //         Ok((input, OpConstant::Integer(value as u64, nl_type)))
    //     } else {
    //         let (_, value) = parse_number::<u64>(number)?;
    //         Ok((input, OpConstant::Integer(value, nl_type)))
    //     }
    // } else {
    //     // Has to be a floating point number.]
    //     // FIXME there's a lot of different styles of float that need to be tested.
    //     match nl_type {
    //         NLType::F32 => {
    //             let (_, value) = parse_number::<f32>(number)?;
    //             Ok((input, OpConstant::Float32(value)))
    //         }
    //         NLType::F64 => {
    //             let (_, value) = parse_number::<f64>(number)?;
    //             Ok((input, OpConstant::Float64(value)))
    //         }
    //         _ => unreachable!(),
    //     }
    // }
}

fn read_string_constant(input: &str) -> ParserResult<OpConstant> {
    // String constants are not pre-escaped. The escape can't be preformed without memory copying, and I want to avoid that in the
    // parsing phase.

    // FIXME make sure escaped quotes are treated correctly.
    let (input, _) = blank(input)?;
    let (input, string) = delimited(char('"'), take_while(|c| c != '\"'), char('"'))(input)?;
    Ok((input, OpConstant::String(string)))
}

fn read_constant_raw(input: &str) -> ParserResult<OpConstant> {
    let (input, _) = blank(input)?;
    let (input, constant) = alt((
        read_boolean_constant,
        read_numerical_constant,
        read_string_constant,
    ))(input)?;
    Ok((input, constant))
}

fn read_constant(input: &str) -> ParserResult<NLOperation> {
    let (input, constant) = read_constant_raw(input)?;
    Ok((input, NLOperation::Constant(constant)))
}

fn read_assignment(input: &str) -> ParserResult<NLOperation> {
    // Are we defining?
    let (input, _) = blank(input)?;
    let (input, is_new) = opt(tag("let"))(input)?;
    let is_new = is_new.is_some();

    // What is our name?
    let (input, _) = blank(input)?;
    let (input, names) = alt((read_tuple_of_variable_names, read_single_variable))(input)?;

    let mut variables = Vec::new();
    variables.reserve(names.len());

    for name in names {
        let variable = OpVariable { name };
        variables.push(variable);
    }

    // Are we given a type specification?
    let (input, _) = blank(input)?;
    let (input, has_type_assignment) = opt(char(':'))(input)?;
    let has_type_assignment = has_type_assignment.is_some();
    let (input, type_assignments) = if !has_type_assignment {
        (input, vec![])
    } else {
        let (input, assignment) = read_variable_type(input)?;
        let assignment = match assignment {
            NLType::Tuple(tuple) => tuple,
            _ => vec![assignment],
        };
        (input, assignment)
    };

    // Consume equal sign.
    let (input, _) = blank(input)?;
    let (input, _) = char('=')(input)?;
    let (input, _) = blank(input)?;

    // What's the value we are assigning to?
    let (input, _) = blank(input)?;
    let (input, assignment) = read_operation(input)?;

    let assignment = OpAssignment {
        is_new,
        to_assign: variables,
        type_assignments,
        assignment: Box::new(assignment),
    };

    Ok((input, NLOperation::Assign(assignment)))
}

fn take_operator_symbol(input: &str) -> ParserResult<&str> {
    fn is_operator_symbol(c: char) -> bool {
        match c {
            '=' | '!' | '~' | '|' | '&' | '^' | '%' | '+' | '-' | '*' | '/' | '<' | '>' | '.' => {
                true
            }
            _ => false,
        }
    }

    take_while1(is_operator_symbol)(input)
}

fn read_urinary_operator(input: &str) -> ParserResult<NLOperation> {
    let (input, _) = blank(input)?;
    let (input, operator) = take_operator_symbol(input)?;

    let (input, _) = blank(input)?;
    let (input, operand) = read_operation(input)?;
    let operand = Box::new(operand);

    match operator {
        "!" => {
            let operator = OpOperator::LogicalNegate(operand);
            Ok((input, NLOperation::Operator(operator)))
        }
        "~" => {
            let operator = OpOperator::BitNegate(operand);
            Ok((input, NLOperation::Operator(operator)))
        }
        "-" => {
            let operator = OpOperator::ArithmeticNegate(operand);
            Ok((input, NLOperation::Operator(operator)))
        }

        _ => Err(verbose_error(input, "unknown operator")),
    }
}

fn read_binary_operator(input: &str) -> ParserResult<NLOperation> {
    let (input, _) = blank(input)?;
    let (input, operand_a) = read_sub_operation(input)?;
    let operand_a = Box::new(operand_a);

    let (input, _) = blank(input)?;
    let (input, operator) = take_operator_symbol(input)?;

    let (input, _) = blank(input)?;
    let (input, operand_b) = read_sub_operation(input)?;
    let operand_b = Box::new(operand_b);

    match operator {
        // Logical operators.
        "==" => {
            let operator = OpOperator::CompareEqual((operand_a, operand_b));
            Ok((input, NLOperation::Operator(operator)))
        }
        "!=" => {
            let operator = OpOperator::CompareNotEqual((operand_a, operand_b));
            Ok((input, NLOperation::Operator(operator)))
        }
        // TODO create formal errors for => and =< operators to help the noobs.
        ">=" => {
            let operator = OpOperator::CompareGreaterEqual((operand_a, operand_b));
            Ok((input, NLOperation::Operator(operator)))
        }
        "<=" => {
            let operator = OpOperator::CompareLessEqual((operand_a, operand_b));
            Ok((input, NLOperation::Operator(operator)))
        }

        ">" => {
            let operator = OpOperator::CompareGreater((operand_a, operand_b));
            Ok((input, NLOperation::Operator(operator)))
        }
        "<" => {
            let operator = OpOperator::CompareLess((operand_a, operand_b));
            Ok((input, NLOperation::Operator(operator)))
        }
        "&&" => {
            let operator = OpOperator::LogicalAnd((operand_a, operand_b));
            Ok((input, NLOperation::Operator(operator)))
        }
        "||" => {
            let operator = OpOperator::LogicalOr((operand_a, operand_b));
            Ok((input, NLOperation::Operator(operator)))
        }
        "^^" => {
            let operator = OpOperator::LogicalXor((operand_a, operand_b));
            Ok((input, NLOperation::Operator(operator)))
        }

        // Bitwise operators.
        "&" => {
            let operator = OpOperator::BitAnd((operand_a, operand_b));
            Ok((input, NLOperation::Operator(operator)))
        }
        "|" => {
            let operator = OpOperator::BitOr((operand_a, operand_b));
            Ok((input, NLOperation::Operator(operator)))
        }
        "^" => {
            let operator = OpOperator::BitXor((operand_a, operand_b));
            Ok((input, NLOperation::Operator(operator)))
        }
        "<<" => {
            let operator = OpOperator::BitLeftShift((operand_a, operand_b));
            Ok((input, NLOperation::Operator(operator)))
        }
        ">>" => {
            let operator = OpOperator::BitRightShift((operand_a, operand_b));
            Ok((input, NLOperation::Operator(operator)))
        }

        // Arithmetic operators.
        "+" => {
            let operator = OpOperator::ArithmeticAdd((operand_a, operand_b));
            Ok((input, NLOperation::Operator(operator)))
        }
        "-" => {
            let operator = OpOperator::ArithmeticSub((operand_a, operand_b));
            Ok((input, NLOperation::Operator(operator)))
        }
        "%" => {
            let operator = OpOperator::ArithmeticMod((operand_a, operand_b));
            Ok((input, NLOperation::Operator(operator)))
        }
        "/" => {
            let operator = OpOperator::ArithmeticDiv((operand_a, operand_b));
            Ok((input, NLOperation::Operator(operator)))
        }
        "*" => {
            let operator = OpOperator::ArithmeticMul((operand_a, operand_b));
            Ok((input, NLOperation::Operator(operator)))
        }
        ".." => {
            let operator = OpOperator::Range((operand_a, operand_b));
            Ok((input, NLOperation::Operator(operator)))
        }

        _ => Err(verbose_error(input, "unknown operator")),
    }
}

fn read_if_statement(input: &str) -> ParserResult<NLOperation> {
    let (input, _) = blank(input)?;
    let (input, _) = tag("if")(input)?;
    let (input, _) = blank(input)?;
    let (input, condition) = read_operation(input)?;
    let (input, _) = blank(input)?;
    let (input, true_block) = read_code_block(input)?;
    let (input, _) = blank(input)?;
    let (input, else_tag) = opt(tag("else"))(input)?;

    let (input, false_block) = if else_tag.is_some() {
        // We have an else block.
        let (input, block) = read_code_block(input)?;

        let block = match block {
            NLOperation::Block(block) => block,
            _ => panic!("Got something other than a block when it should have been a block."),
        };

        (input, block)
    } else {
        (input, NLBlock { operations: vec![] })
    };

    let true_block = match true_block {
        NLOperation::Block(block) => block,
        _ => panic!("Got something other than a block when it should have been a block."),
    };

    Ok((
        input,
        NLOperation::If(IfStatement {
            condition: Box::new(condition),
            true_block,
            false_block,
        }),
    ))
}

fn read_basic_loop(input: &str) -> ParserResult<NLOperation> {
    let (input, _) = blank(input)?;
    let (input, _) = tag("loop")(input)?;
    let (input, _) = blank(input)?;
    let (input, block) = read_code_block_raw(input)?;

    Ok((input, NLOperation::Loop(block)))
}

fn read_while_loop(input: &str) -> ParserResult<NLOperation> {
    let (input, _) = blank(input)?;
    let (input, _) = tag("while")(input)?;
    let (input, _) = blank(input)?;
    let (input, condition) = read_operation(input)?;
    let (input, _) = blank(input)?;
    let (input, block) = read_code_block_raw(input)?;

    Ok((
        input,
        NLOperation::WhileLoop(WhileLoop {
            condition: Box::new(condition),
            block,
        }),
    ))
}

fn read_for_loop(input: &str) -> ParserResult<NLOperation> {
    let (input, _) = blank(input)?;
    let (input, _) = tag("for")(input)?;
    let (input, _) = blank(input)?;
    let (input, variable) = read_variable_access_raw(input)?;
    let (input, _) = blank(input)?;
    let (input, _) = tag("in")(input)?;
    let (input, _) = blank(input)?;
    let (input, iterator) = read_operation(input)?;
    let (input, _) = blank(input)?;
    let (input, block) = read_code_block_raw(input)?;

    Ok((
        input,
        NLOperation::ForLoop(ForLoop {
            variable,
            iterator: Box::new(iterator),
            block,
        }),
    ))
}

fn read_break_keyword(input: &str) -> ParserResult<NLOperation> {
    let (input, break_keyword) = opt(tag("break"))(input)?;

    if break_keyword.is_some() {
        Ok((input, NLOperation::Break))
    } else {
        Err(verbose_error(input, "This is not a break operation."))
    }
}

fn read_variable_access_raw(input: &str) -> ParserResult<OpVariable> {
    let (input, _) = blank(input)?;
    let (input, name) = read_variable_name(input)?;

    Ok((input, OpVariable { name }))
}

fn read_variable_access(input: &str) -> ParserResult<NLOperation> {
    let (input, variable) = read_variable_access_raw(input)?;

    Ok((input, NLOperation::VariableAccess(variable)))
}

fn read_function_call(input: &str) -> ParserResult<NLOperation> {
    let (input, _) = blank(input)?;
    let (input, path) = read_variable_name(input)?;
    let (input, _) = blank(input)?;
    let (input, arg_input) = delimited(char('('), take_while(|c| c != ')'), char(')'))(input)?;

    let (arg_input, mut arguments) = many0(terminated(read_variable_name, char(',')))(arg_input)?;

    let (_, last_arg) = opt(read_variable_name)(arg_input)?;
    if let Some(arg) = last_arg {
        arguments.push(arg);
    };

    Ok((
        input,
        NLOperation::FunctionCall(FunctionCall { path, arguments }),
    ))
}

fn read_match(input: &str) -> ParserResult<NLOperation> {
    let (input, _) = blank(input)?;
    let (input, _) = tag("match")(input)?;
    let (input, _) = blank(input)?;
    let (input, input_operation) = read_operation(input)?;

    let (input, _) = blank(input)?;
    let (input, _) = char('{')(input)?;

    fn read_branch_body(input: &str) -> ParserResult<NLOperation> {
        let (input, _) = blank(input)?;
        let (input, _) = tag("=>")(input)?;
        let (input, _) = blank(input)?;

        read_operation(input)
    }

    fn read_enum_branch(input: &str) -> ParserResult<(MatchBranch, NLOperation)> {
        let (input, _) = blank(input)?;
        let (input, nl_enum) = read_variable_name(input)?;
        let (input, _) = blank(input)?;
        let (input, _) = tag("::")(input)?;
        let (input, _) = blank(input)?;
        let (input, variant) = read_variable_name(input)?;
        let (input, _) = blank(input)?;

        let (input, var_input) =
            opt(delimited(char('('), take_while(|c| c != ')'), char(')')))(input)?;

        let variables = if let Some(var_input) = var_input {
            let (var_input, mut variables) =
                many0(terminated(read_variable_name, char(',')))(var_input)?;
            let (_, last_arg) = opt(read_variable_name)(var_input)?;
            if let Some(arg) = last_arg {
                variables.push(arg);
            };

            variables
        } else {
            Vec::new()
        };

        let (input, operation) = read_branch_body(input)?;

        let match_branch = MatchBranch::Enum(MatchEnumBranch {
            nl_enum,
            variant,
            variables,
        });

        Ok((input, (match_branch, operation)))
    }

    fn read_constant_branch(input: &str) -> ParserResult<(MatchBranch, NLOperation)> {
        let (input, _) = blank(input)?;
        let (input, constant) = read_constant_raw(input)?;
        let (input, _) = blank(input)?;

        let (input, operation) = read_branch_body(input)?;

        Ok((input, (MatchBranch::Constant(constant), operation)))
    }

    fn read_range_branch(input: &str) -> ParserResult<(MatchBranch, NLOperation)> {
        let (input, _) = blank(input)?;
        let (input, lower) = digit1(input)?;
        let (_, lower) = parse_integer(lower)?;

        let (input, _) = blank(input)?;
        let (input, _) = tag("..")(input)?;

        let (input, _) = blank(input)?;
        let (input, higher) = digit1(input)?;
        let (_, higher) = parse_integer(higher)?;

        let (input, _) = blank(input)?;
        let (input, operation) = read_branch_body(input)?;

        // TODO make work with the new implementation.
        unimplemented!()
        // Ok((input, (MatchBranch::Range((lower, higher)), operation)))
    }

    fn read_branch(input: &str) -> ParserResult<(MatchBranch, NLOperation)> {
        alt((read_range_branch, read_constant_branch, read_enum_branch))(input)
    }

    let (input, _) = blank(input)?;
    let (input, mut branches) = many0(terminated(read_branch, char(',')))(input)?;

    let (input, _) = blank(input)?;
    let (input, last_branch) = opt(read_branch)(input)?;

    if let Some(arg) = last_branch {
        branches.push(arg);
    }

    let (input, _) = blank(input)?;

    let (input, _) = char('}')(input)?;

    Ok((
        input,
        NLOperation::Match(Match {
            input: Box::new(input_operation),
            branches,
        }),
    ))
}

fn read_code_block_raw(input: &str) -> ParserResult<NLBlock> {
    let (input, _) = blank(input)?;
    let (input, _) = char('{')(input)?;

    let (input, operations) = many0(read_operation)(input)?;

    let (input, _) = blank(input)?;
    let (input, _) = char('}')(input)?;

    Ok((input, NLBlock { operations }))
}

fn read_code_block(input: &str) -> ParserResult<NLOperation> {
    let (input, block) = read_code_block_raw(input)?;

    Ok((input, NLOperation::Block(block)))
}

fn read_sub_operation(input: &str) -> ParserResult<NLOperation> {
    alt((
        read_code_block,
        read_tuple,
        read_function_call,
        read_assignment,
        read_constant,
        read_urinary_operator,
        read_variable_access,
    ))(input)
}

fn read_operation(input: &str) -> ParserResult<NLOperation> {
    alt((
        read_code_block,
        read_if_statement,
        read_match,
        read_break_keyword,
        read_basic_loop,
        read_while_loop,
        read_for_loop,
        read_tuple,
        read_function_call,
        read_assignment,
        read_binary_operator,
        read_constant,
        read_urinary_operator,
        read_variable_access,
    ))(input)
}

fn read_argument_declaration(input: &str) -> ParserResult<NLArgument> {
    let (input, _) = blank(input)?;
    let (input, name) = opt(read_variable_name)(input)?;

    match name {
        Some(name) => {
            let (input, _) = blank(input)?;
            let (input, _) = char(':')(input)?;
            let (input, _) = blank(input)?;
            let (input, nl_type) = read_variable_type(input)?;
            let (input, _) = blank(input)?;

            let arg = NLArgument { name, nl_type };

            Ok((input, arg))
        }
        None => {
            let (post_input, is_ref) = opt(char('&'))(input)?;
            let is_ref = is_ref.is_some();

            if is_ref {
                let input = post_input;

                let (input, _) = blank(input)?;
                let (input, tagged) = opt(tag("self"))(input)?;
                if tagged.is_some() {
                    let arg = NLArgument {
                        name: "self",
                        nl_type: NLType::SelfReference,
                    };

                    return Ok((input, arg));
                }

                let (input, tagged) = opt(tag("mut"))(input)?;
                if tagged.is_some() {
                    let (input, _) = blank(input)?;
                    let (input, _) = tag("self")(input)?;

                    let arg = NLArgument {
                        name: "self",
                        nl_type: NLType::MutableSelfReference,
                    };

                    return Ok((input, arg));
                }
            }

            if !input.is_empty() {
                Err(verbose_error(
                    input,
                    "could not read deceleration of argument correctly",
                ))
            } else {
                Err(verbose_error(input, "there is no argument"))
            }
        }
    }
}

fn read_argument_deceleration_list(input: &str) -> ParserResult<Vec<NLArgument>> {
    let (input, arg_input) = delimited(char('('), take_while(|c| c != ')'), char(')'))(input)?;

    let (arg_input, mut arguments) =
        many0(terminated(read_argument_declaration, char(',')))(arg_input)?;

    let (_, last_arg) = opt(terminated(read_argument_declaration, blank))(arg_input)?;
    match last_arg {
        Some(arg) => {
            arguments.push(arg);
        }
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

fn read_method(input: &str) -> ParserResult<NLImplementor> {
    let (input, _) = blank(input)?;
    let (input, _) = tag("met")(input)?;
    let (input, _) = blank(input)?;
    let (input, name) = read_method_name(input)?;
    let (input, _) = blank(input)?;
    let (input, args) = read_argument_deceleration_list(input)?;
    let (input, _) = blank(input)?;
    let (input, return_type) = read_return_type(input)?;
    let (input, _) = blank(input)?;
    let (input, block) = opt(read_code_block)(input)?;
    let block = match block {
        Some(block) => match block {
            NLOperation::Block(block) => Some(block),
            _ => None,
        },
        _ => None,
    };

    let method = NLFunction {
        name,
        arguments: args,
        return_type,
        block,
    };

    // No block, we expect a semicolon.
    if method.block.is_none() {
        let (input, _) = char(';')(input)?;

        Ok((input, NLImplementor::Method(method)))
    } else {
        Ok((input, NLImplementor::Method(method)))
    }
}

fn read_function(input: &str) -> ParserResult<RootDeceleration> {
    let (input, _) = blank(input)?;
    let (input, _) = tag("fn")(input)?;
    let (input, _) = blank(input)?;
    let (input, name) = read_method_name(input)?;
    let (input, _) = blank(input)?;
    let (input, args) = read_argument_deceleration_list(input)?;
    let (input, _) = blank(input)?;
    let (input, return_type) = read_return_type(input)?;
    let (input, _) = blank(input)?;
    let (input, block) = opt(read_code_block)(input)?;
    let block = match block {
        Some(block) => match block {
            NLOperation::Block(block) => Some(block),
            _ => None,
        },
        _ => None,
    };

    let function = NLFunction {
        name,
        arguments: args,
        return_type,
        block,
    };

    // No block, we expect a semicolon.
    if function.block.is_none() {
        let (input, _) = char(';')(input)?;

        Ok((input, RootDeceleration::Function(function)))
    } else {
        Ok((input, RootDeceleration::Function(function)))
    }
}

fn read_variant_enum(input: &str) -> ParserResult<RootDeceleration> {
    let (input, _) = blank(input)?;
    let (input, _) = tag("enum")(input)?;
    let (input, _) = blank(input)?;
    let (input, name) = read_method_name(input)?;

    let (input, _) = blank(input)?;
    let (input, _) = char('{')(input)?;

    fn read_variant(input: &str) -> ParserResult<EnumVariant> {
        let (input, _) = blank(input)?;
        let (input, name) = read_variable_name(input)?;
        let (input, _) = blank(input)?;

        let (input, args) = opt(read_argument_deceleration_list)(input)?;

        let arguments = if let Some(args) = args {
            args
        } else {
            Vec::new()
        };

        Ok((input, EnumVariant { name, arguments }))
    }

    let (input, _) = blank(input)?;
    let (input, mut variants) = many0(terminated(read_variant, char(',')))(input)?;
    let (input, _) = blank(input)?;
    let (input, last_variant) = opt(read_variant)(input)?;
    if let Some(arg) = last_variant {
        variants.push(arg);
    }

    let (input, _) = blank(input)?;
    let (input, _) = char('}')(input)?;

    Ok((input, RootDeceleration::Enum(NLEnum { name, variants })))
}

fn read_getter(input: &str) -> ParserResult<NLImplementor> {
    let (input, _) = blank(input)?;
    let (input, _) = tag("get")(input)?;
    let (input, name) = read_method_name(input)?;
    let (input, _) = blank(input)?;
    let (input, is_default) = opt(tuple((char(':'), blank, tag("default"), blank)))(input)?;

    if is_default.is_some() {
        let (input, nl_type) = read_return_type(input)?;
        let (input, _) = char(';')(input)?;

        let getter = NLGetter {
            name: String::from(name),
            args: vec![],
            nl_type,
            block: NLEncapsulationBlock::Default,
        };

        Ok((input, NLImplementor::Getter(getter)))
    } else {
        let (input, args) = read_argument_deceleration_list(input)?;
        let (input, nl_type) = read_return_type(input)?;
        let (input, block) = opt(read_code_block)(input)?;

        let block = match block {
            Some(block) => match block {
                NLOperation::Block(block) => Some(block),
                _ => None,
            },
            _ => None,
        };

        match block {
            Some(block) => {
                let getter = NLGetter {
                    name: String::from(name),
                    args,
                    nl_type,
                    block: NLEncapsulationBlock::Some(block),
                };

                Ok((input, NLImplementor::Getter(getter)))
            }
            None => {
                let (input, _) = char(';')(input)?;

                let getter = NLGetter {
                    name: String::from(name),
                    args,
                    nl_type,
                    block: NLEncapsulationBlock::None,
                };

                Ok((input, NLImplementor::Getter(getter)))
            }
        }
    }
}

fn read_setter(input: &str) -> ParserResult<NLImplementor> {
    let (input, _) = blank(input)?;
    let (input, _) = tag("set")(input)?;
    let (input, name) = read_method_name(input)?;
    let (input, _) = blank(input)?;
    let (input, is_default) =
        opt(tuple((char(':'), blank, tag("default"), blank, char(';'))))(input)?;

    if is_default.is_some() {
        let setter = NLSetter {
            name,
            args: vec![],
            block: NLEncapsulationBlock::Default,
        };

        Ok((input, NLImplementor::Setter(setter)))
    } else {
        let (input, args) = read_argument_deceleration_list(input)?;
        let (input, _) = blank(input)?;
        let (input, block) = opt(read_code_block)(input)?;
        let block = match block {
            Some(block) => match block {
                NLOperation::Block(block) => Some(block),
                _ => None,
            },
            _ => None,
        };

        match block {
            Some(block) => {
                let setter = NLSetter {
                    name,
                    args,
                    block: NLEncapsulationBlock::Some(block),
                };

                Ok((input, NLImplementor::Setter(setter)))
            }
            None => {
                let (input, _) = char(';')(input)?;

                let setter = NLSetter {
                    name,
                    args,
                    block: NLEncapsulationBlock::None,
                };

                Ok((input, NLImplementor::Setter(setter)))
            }
        }
    }
}

// TODO make it so you can specify required traits.
fn read_trait(input: &str) -> ParserResult<RootDeceleration> {
    let (input, _) = blank(input)?;
    let (input, _) = tag("trait")(input)?;
    let (input, _) = blank(input)?;
    let (input, name) = read_struct_or_trait_name(input)?;

    let (input, _) = blank(input)?;
    let (input, _) = char('{')(input)?;
    let (input, _) = blank(input)?;

    let (input, implementors) = many0(alt((read_method, read_getter, read_setter)))(input)?;

    let (input, _) = blank(input)?;
    let (input, _) = char('}')(input)?;

    let new_trait = NLTrait { name, implementors };

    Ok((input, RootDeceleration::Trait(new_trait)))
}

fn read_variable_name(input: &str) -> ParserResult<&str> {
    let (input, _) = blank(input)?;
    take_while1(is_name)(input)
}

fn identify_struct_or_trait_type(input: &str) -> ParserResult<NLType> {
    let (input, is_reference) = opt(char('&'))(input)?;
    let is_reference = is_reference.is_some();

    let (input, _) = blank(input)?;

    let (input, is_mutable) = if is_reference {
        let (input, is_mutable) = opt(tag("mut"))(input)?;
        let is_mutable = is_mutable.is_some();

        let (input, _) = blank(input)?;

        (input, is_mutable)
    } else {
        // If not a reference, this does not matter.
        (input, false)
    };

    let (input, is_struct) = opt(tag("dyn"))(input)?;
    let is_struct = is_struct.is_none();

    let (input, name) = read_struct_or_trait_name(input)?;

    if is_struct {
        // Its a struct.
        if is_reference {
            if is_mutable {
                Ok((input, NLType::MutableReferencedStruct(name)))
            } else {
                Ok((input, NLType::ReferencedStruct(name)))
            }
        } else {
            Ok((input, NLType::OwnedStruct(name)))
        }
    } else {
        // Its a trait.
        if is_reference {
            if is_mutable {
                Ok((input, NLType::MutableReferencedTrait(name)))
            } else {
                Ok((input, NLType::ReferencedTrait(name)))
            }
        } else {
            Ok((input, NLType::OwnedTrait(name)))
        }
    }
}

fn read_variable_type_primitive_no_whitespace(input: &str) -> ParserResult<NLType> {
    let (input, type_name) = alphanumeric0(input)?;

    match type_name {
        "i8" => Ok((input, NLType::I8)),
        "i16" => Ok((input, NLType::I16)),
        "i32" => Ok((input, NLType::I32)),
        "i64" => Ok((input, NLType::I64)),
        "u8" => Ok((input, NLType::U8)),
        "u16" => Ok((input, NLType::U16)),
        "u32" => Ok((input, NLType::U32)),
        "u64" => Ok((input, NLType::U64)),
        "f32" => Ok((input, NLType::F32)),
        "f64" => Ok((input, NLType::F64)),
        "bool" => Ok((input, NLType::Boolean)),

        _ => Err(verbose_error(
            input,
            "Constants must be primative types: i8-64, u8-64, f32-64, or bool.",
        )),
    }
}

fn read_variable_type_no_whitespace(input: &str) -> ParserResult<NLType> {
    fn read_advanced_types(input: &str) -> ParserResult<NLType> {
        // Could it be a referenced string?
        let (input, _) = blank(input)?;
        let (input, is_referenced_string) = opt(preceded(blank, tag("str")))(input)?;
        let is_referenced_string = is_referenced_string.is_some();
        if is_referenced_string {
            return Ok((input, NLType::BorrowedString));
        } else {
            // Okay so we ether have Struct or Trait. Could even be a reference.
            return identify_struct_or_trait_type(input);
        }
    }

    alt((
        read_variable_type_primitive_no_whitespace,
        read_advanced_types,
    ))(input)
}

fn read_variable_type(input: &str) -> ParserResult<NLType> {
    let (input, _) = blank(input)?;
    read_variable_type_no_whitespace(input)
}

fn read_struct_variable(input: &str) -> ParserResult<NLStructVariable> {
    let (input, _) = blank(input)?;
    let (input, name) = read_variable_name(input)?;

    let (input, _) = blank(input)?;
    let (input, _) = char(':')(input)?; // That : between the variable name and its type.
    let (input, _) = blank(input)?;
    let (input, nl_type) = read_variable_type(input)?;

    let var = NLStructVariable {
        name,
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
    let (input, methods) = many0(alt((read_method, read_getter, read_setter)))(input)?;
    let (input, _) = blank(input)?;
    let (input, _) = char('}')(input)?;

    let implementation = NLImplementation {
        name,
        implementors: methods,
    };

    Ok((input, implementation))
}

fn read_struct(input: &str) -> ParserResult<RootDeceleration> {
    let (input, _) = blank(input)?;
    let (input, _) = tag("struct")(input)?;
    let (input, _) = blank(input)?;
    let (input, name) = read_struct_or_trait_name(input)?;
    let (input, _) = blank(input)?;
    let (input, _) = char('{')(input)?;
    let (input, _) = blank(input)?;
    let (input, mut variables) =
        many0(terminated(read_struct_variable, tuple((blank, char(',')))))(input)?;
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
        name,
        variables,
        implementations,
    };

    Ok((input, RootDeceleration::Struct(nl_struct)))
}

fn parse_file_root(input: &str) -> ParserResult<NLFile> {
    let mut file = NLFile {
        name: String::new(),
        structs: vec![],
        traits: vec![],
        functions: vec![],
        enums: vec![],
    };

    if !input.is_empty() {
        let (input, root_defs) = many1(alt((
            read_struct,
            read_trait,
            read_function,
            read_variant_enum,
        )))(input)?;

        for root_def in root_defs {
            match root_def {
                RootDeceleration::Struct(nl_struct) => {
                    file.structs.push(nl_struct);
                }
                RootDeceleration::Trait(nl_trait) => {
                    file.traits.push(nl_trait);
                }
                RootDeceleration::Function(nl_func) => {
                    file.functions.push(nl_func);
                }
                RootDeceleration::Enum(nl_enum) => {
                    file.enums.push(nl_enum);
                }
            }
        }

        Ok((input, file))
    } else {
        Ok((input, file))
    }
}

pub fn parse_string<'a>(input: &'a str, file_name: &str) -> Result<NLFile<'a>, ParseError> {
    let file = parse_file_root(input);

    match file {
        Result::Err(err) => {
            match err {
                nom::Err::Error(e) | nom::Err::Failure(e) => {
                    let message = convert_error(input, e);

                    // Makes our error messages more readable when running tests.
                    #[cfg(test)]
                    println!("{}", message);

                    Err(ParseError { message })
                }
                nom::Err::Incomplete(_) => Err(ParseError {
                    message: "Unexpected end of file.".to_string(),
                }),
            }
        }
        Result::Ok(result) => {
            let (_, mut file) = result;

            file.name = file_name.to_string();

            Ok(file)
        }
    }
}

pub fn parse_file<T>(
    path: &Path,
    function: &dyn Fn(&NLFile) -> T,
) -> Result<T, Box<dyn std::error::Error>> {
    let mut input_file = File::open(&path)?;

    let mut contents = String::new();
    input_file.read_to_string(&mut contents)?;

    // This should *always* have a name since we shouldn't have been able to get to this point if it wasn't actually a file.
    let result = parse_string(&contents, &path.file_name().unwrap().to_str().unwrap());

    match result {
        Ok(result) => Ok(function(&result)),
        Err(error) => Err(Box::new(error)),
    }
}
