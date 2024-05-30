use string_interner::symbol::SymbolU32;

pub struct Span {
    pub lo: usize,
    pub hi: usize,
    pub src: SymbolU32
}

impl Span {

    pub fn merge(&self, other: &Span) -> Span {
        Span {
            lo: self.lo.min(other.lo),
            hi: self.hi.max(other.hi),

            // There will never be merging of spans across source files.
            // TODO: specify this in design doc.
            src: self.src
        }
    }
}
