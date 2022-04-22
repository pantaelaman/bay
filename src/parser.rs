use nom::{
    IResult,
    branch::alt,
    multi::{many0, many0_count},
    combinator::recognize,
    sequence::{delimited, pair, separated_pair, terminated},
    character::complete::{self, alpha1, alphanumeric1},
    bytes::complete::{is_not, tag},
};
use crate::base::chunks::Chunk;

pub mod base;

use base::chunk;

pub fn parsebayfile(input: &str) -> IResult<&str, Vec<Chunk>> {
    many0(chunk)(input)
}

