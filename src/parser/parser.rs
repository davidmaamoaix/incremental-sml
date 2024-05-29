use nom::{ AsBytes, Compare, IResult };
use nom::bytes::complete::tag;
use nom_locate::{ LocatedSpan };
use crate::atom::span::Span;
use string_interner::symbol::SymbolU32;

type Input<'a> = LocatedSpan<&'a str, SymbolU32>;

fn to_span(s: Input) -> Span {
    let offset = s.location_offset();
    Span {
        lo: offset,
        hi: offset + s.fragment().len(),
        src: s.extra
    }
}

fn parse_iden(s: Input) -> IResult<Input, &str> {
    todo!()
}
