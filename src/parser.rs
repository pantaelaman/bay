use nom::{
    IResult,
    multi::{many_till},
    combinator::eof,
    character::complete::multispace0,
};
use crate::base::chunks::Chunk;

pub mod base;

use base::chunk;

pub fn parsebayfile(input: &str) -> IResult<&str, Vec<Chunk>> {
    let (input, _) = multispace0(input)?;
    let (o, (c, _)) = many_till(chunk, eof)(input)?;
    Ok((o, c))
}

