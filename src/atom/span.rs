use string_interner::symbol::SymbolU32;

pub struct Span {
    lo: u32,
    hi: u32,
    src: SymbolU32
}

impl Span {

    fn merge(&self, other: &Span) -> Span {
        Span {
            lo: self.lo.min(other.lo),
            hi: self.hi.max(other.hi),
            src: self.src
        }
    }
}
