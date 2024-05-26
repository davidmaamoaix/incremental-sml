use nom::AsBytes;

pub struct ParseCtx<'a> {
    offset: u32,
    source: &'a str,

    line: u32,
    col: u32
}

impl ParseCtx<'_> {
    pub fn new(source: &str) -> ParseCtx {
        ParseCtx {
            offset: 0,
            source,
            line: 1,
            col: 1
        }
    }
}

impl<'a> AsBytes for ParseCtx<'a> {
    fn as_bytes(&self) -> &[u8] {
        self.source.as_bytes()
    }
}


