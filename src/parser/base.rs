use nom::{
    IResult,
    error::{Error, ErrorKind},
    branch::alt,
    multi::{many0, many0_count, many_till},
    combinator::recognize,
    sequence::{delimited, pair, separated_pair, preceded},
    character::complete::{self, alpha1, alphanumeric1, multispace0, anychar, none_of},
    bytes::complete::{take, tag},
};
use crate::base::{
    chunks::*,
    commands::{
        self,
        Command,
        Value,
    },
};

pub fn identifier(input: &str) -> IResult<&str, &str> {
    recognize(
      pair(
          alt((alpha1, tag("_"))),
          many0_count(alt((alphanumeric1, tag("_"))))
          )
      )(input)
}

pub fn till_unescaped_ws_or_sc(input: &str) -> IResult<&str, &str> {
    recognize(
        many0_count(alt((
                preceded(complete::char('\\'), anychar),
                none_of(" ;"),
                )))
        )(input)
}

pub fn till_unescaped_quote(input: &str) -> IResult<&str, &str> {
    recognize(
        many0_count(alt((
                preceded(complete::char('\\'), anychar),
                none_of("\""),
                )))
        )(input)
}

pub fn value(input: &str) -> IResult<&str, Value> {
    let (input, _) = multispace0(input)?;
    if let Ok((s, c)) = take::<usize, &str, Error<&str>>(1usize)(input) {
        match c {
            "\"" => {
                // It's a string value
                let value = delimited(complete::char('"'), till_unescaped_quote, complete::char('"'))(input)?;
                return Ok((value.0, Value::STRING(value.1.to_string())));
            },
            "-" => {
                // It's an atom
                let value = identifier(s)?;
                return Ok((value.0, Value::FLAG(value.1.to_string())));
            },
            "{" => {
                // It's a command block
                let value = delimited(complete::char('{'), many0(command), complete::char('}'))(input)?;
                return Ok((value.0, Value::BLOCK(value.1)));
            },
            _ => {},
        }
    }
    let (o, (t,v)) = separated_pair(identifier, complete::char(':'), till_unescaped_ws_or_sc)(input)?;
    match t {
        "f" => return Ok((o, Value::FPATH(v.to_string()))),
        "url" => return Ok((o, Value::URL(v.to_string()))),
        _ => {},
    }
    Err(nom::Err::Error(Error::new(input, ErrorKind::Fail)))
}

pub fn chunk(input: &str) -> IResult<&str, Chunk> {
    let (input, _) = multispace0(input)?;
    let (i, (t,s)) = delimited(complete::char('['), separated_pair(identifier, complete::char(':'), identifier), complete::char(']'))(input)?;
    let (o, cmds) = many0(command)(i)?;
    Ok((o, match t {
        "target" => {
            Chunk::new(ChunkType::TARGET, s.to_string(), cmds)
        },
        _ => return Err(nom::Err::Error(Error::new(input, ErrorKind::Fail))),
    }))
}

pub fn command(input: &str) -> IResult<&str, Command> {
    let (input, _) = multispace0(input)?;
    let (o, (i, (args, _))) = pair(identifier, many_till(value, complete::char(';')))(input)?;
    Ok((o, Command::new(match i {
        "fetch" => commands::fetch,
        "traverse" => commands::traverse,
        "run" => commands::run,
        _ => return Err(nom::Err::Error(Error::new(input, ErrorKind::Fail))),
    }, args)))
}

