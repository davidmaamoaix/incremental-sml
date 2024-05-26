struct Span {
    lo: u32,
    hi: u32,
}

impl Span {

    fn merge(&self, other: &Span) -> Span {
        Span {
            lo: self.lo.min(other.lo),
            hi: self.hi.max(other.hi)
        }
    }
}
