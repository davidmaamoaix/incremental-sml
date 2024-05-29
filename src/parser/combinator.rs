use nom::{
    branch::alt, bytes::complete::tag, character::complete::{ alpha1, alphanumeric1 }, combinator::{ recognize, verify }, error::ErrorKind, multi::many0_count, sequence::{ pair, preceded }, IResult
};
use nom_locate::LocatedSpan;
use crate::atom::span::Span;
use string_interner::symbol::SymbolU32;

use super::check::{ is_keyword, is_reserved_op };

// While `Input` is similar with the `Span` defined in this compiler, the
// latter is used specifically for denoting a region in the source location.
// Therefore it is used instead of `LocatedSpan` pretty much everywhere else
// in the compiler as there is no need to keep track of fragments.
type Input<'a> = LocatedSpan<&'a str, SymbolU32>;

fn to_span(s: Input) -> Span {
    let offset = s.location_offset();
    Span {
        lo: offset,
        hi: offset + s.fragment().len(),
        src: s.extra
    }
}

fn err<A, B>(s: A, code: ErrorKind) -> IResult<A, B> {
    Err(nom::Err::Error(nom::error::Error{input: s, code: ErrorKind::Fail}))
}

pub fn parse_name_id(s: Input) -> IResult<Input, Input> {
    let (res, iden) = recognize(
        pair(
            alt((alpha1, tag("_"))),
            many0_count(alt((alphanumeric1, tag("_"))))
        )
    )(s)?;

    let text = iden.fragment();
    if is_keyword(text) {
        return err(res, ErrorKind::Fail);
    }

    Ok((res, iden))
}


