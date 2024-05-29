use nom::{
    branch::alt,
    character::complete::{ alpha1, alphanumeric1 },
    bytes::complete::{ tag },
    combinator::{ recognize, verify },
    multi::many0_count,
    sequence::{ pair, preceded },
    IResult
};
use nom_locate::LocatedSpan;
use crate::atom::span::Span;
use string_interner::symbol::SymbolU32;

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

pub fn parse_iden(s: Input) -> IResult<Input, &str> {
    let (s, iden) = recognize(
        pair(
            alt((alpha1, tag("_"))),
            many0_count(alt((alphanumeric1, tag("_"))))
        )
    )(s)?;
    Ok((s, iden.fragment()))
}
